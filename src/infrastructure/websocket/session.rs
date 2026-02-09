//! WebSocket Session Actor
//!
//! Handles individual WebSocket connections.
//! Each connected client gets their own Session actor.

use actix::prelude::*;
use actix_web_actors::ws;
use log::{debug, error, info};
use std::time::{Duration, Instant};
use surrealdb::sql::Thing;

use super::chat_server::{ChatServer, Connect, Disconnect, JoinRoom, ServerMessage};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// WebSocket session actor
pub struct WsSession {
    /// Unique session ID
    pub id: usize,

    /// User ID
    pub user_id: Thing,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Chat server address
    pub server: Addr<ChatServer>,
}

impl WsSession {
    pub fn new(user_id: Thing, server: Addr<ChatServer>) -> Self {
        WsSession {
            id: 0,
            user_id,
            hb: Instant::now(),
            server,
        }
    }

    /// Helper method that sends ping to client every HEARTBEAT_INTERVAL
    /// Also checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // Check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // Heartbeat timed out
                info!("WebSocket Client heartbeat failed, disconnecting!");

                // Notify chat server
                act.server.do_send(Disconnect { session_id: act.id });

                // Stop actor
                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // Start heartbeat process
        self.hb(ctx);

        // Register self in chat server
        let addr = ctx.address();
        self.server
            .send(Connect {
                addr: addr.recipient(),
                user_id: self.user_id.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(session_id) => {
                        act.id = session_id;
                        info!(
                            "WebSocket session {} started for user {}",
                            session_id, act.user_id
                        );
                    }
                    Err(err) => {
                        error!("Failed to connect to chat server: {}", err);
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // Notify chat server
        self.server.do_send(Disconnect {
            session_id: self.id,
        });
        Running::Stop
    }
}

/// Handle messages from chat server
impl Handler<ServerMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.content);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                error!("WebSocket error: {}", err);
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                debug!("Received text message: {}", text);

                // Parse JSON message
                // Expected format: {"type": "join", "conversation_id": "..."}
                // or {"type": "message", "conversation_id": "...", "content": "..."}
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => {
                        if let Some(msg_type) = json.get("type").and_then(|v| v.as_str()) {
                            match msg_type {
                                "join" => {
                                    if let Some(conv_id) =
                                        json.get("conversation_id").and_then(|v| v.as_str())
                                    {
                                        self.server.do_send(JoinRoom {
                                            session_id: self.id,
                                            conversation_id: conv_id.to_string(),
                                        });
                                    }
                                }
                                "message" => {
                                    // Handle sending message
                                    // This will be implemented with the repository layer
                                    debug!("Message sending not yet implemented");
                                }
                                _ => {
                                    error!("Unknown message type: {}", msg_type);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to parse JSON: {}", err);
                    }
                }
            }
            ws::Message::Binary(_) => {
                debug!("Binary messages not supported");
            }
            ws::Message::Close(reason) => {
                info!("WebSocket closing: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => {}
        }
    }
}
