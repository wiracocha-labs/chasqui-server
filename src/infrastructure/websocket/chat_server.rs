//! Chat Server Actor
//!
//! Central actor that manages all WebSocket connections and message routing.
//! Handles:
//! - Connection registration/deregistration
//! - Room management (conversations)
//! - Message broadcasting
//! - User presence tracking

use actix::prelude::*;
use log::{debug, error, info};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use surrealdb::sql::Thing;

use crate::application::services::conversation_service::ConversationService;
use crate::application::services::message_service::MessageService;
use crate::models::entities::message::{Message, MessageType};

/// Chat server manages all WebSocket connections and rooms
pub struct ChatServer {
    /// Map of conversation_id -> set of session IDs
    rooms: HashMap<String, HashSet<usize>>,

    /// Map of session_id -> user_id
    sessions: HashMap<usize, Thing>,

    /// Map of user_id -> session_id (for presence tracking)
    user_sessions: HashMap<String, usize>,

    /// Map of session_id -> Recipient (to send messages back to sessions)
    recipients: HashMap<usize, Recipient<ServerMessage>>,

    /// Services for persistence and business logic
    message_service: Arc<MessageService>,
    #[allow(dead_code)]
    conversation_service: Arc<ConversationService>,
}

impl ChatServer {
    pub fn new(
        message_service: Arc<MessageService>,
        conversation_service: Arc<ConversationService>,
    ) -> Self {
        ChatServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
            recipients: HashMap::new(),
            message_service,
            conversation_service,
        }
    }

    /// Helper to send message to a specific session
    fn send_message_to_session(&self, session_id: usize, msg: &str) {
        if let Some(recipient) = self.recipients.get(&session_id) {
            recipient.do_send(ServerMessage {
                content: msg.to_string(),
            });
        }
    }

    /// Broadcast message to all participants in a room (conversation)
    fn broadcast_to_room(&self, conversation_id: &str, msg: &str, _skip_session: Option<usize>) {
        if let Some(sessions) = self.rooms.get(conversation_id) {
            for session_id in sessions {
                self.send_message_to_session(*session_id, msg);
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("ChatServer actor started");
    }
}

/// Message to connect a new session
#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Recipient<ServerMessage>,
    pub user_id: Thing,
}

/// Message to disconnect a session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: usize,
}

/// Message to join a conversation room
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub session_id: usize,
    pub conversation_id: String,
}

/// Message to leave a conversation room
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub session_id: usize,
    pub conversation_id: String,
}

/// Message to send a chat message to a room
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub session_id: usize,
    pub conversation_id: String,
    pub message: String,
    pub sender_id: Thing,
}

/// Message sent from server to client
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub content: String,
}

/// Handler for Connect message
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        // Generate unique session ID
        let session_id = rand::random::<usize>();

        // Store session and recipient
        self.sessions.insert(session_id, msg.user_id.clone());
        self.recipients.insert(session_id, msg.addr);

        // Track user presence
        let user_id_str = format!("{}", msg.user_id);
        self.user_sessions.insert(user_id_str.clone(), session_id);

        info!("User {} connected with session {}", user_id_str, session_id);

        session_id
    }
}

/// Handler for Disconnect message
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(user_id) = self.sessions.remove(&msg.session_id) {
            let user_id_str = format!("{}", user_id);
            self.user_sessions.remove(&user_id_str);
            self.recipients.remove(&msg.session_id);

            // Remove from all rooms
            for (_room_id, sessions) in self.rooms.iter_mut() {
                sessions.remove(&msg.session_id);
            }

            info!(
                "User {} disconnected (session {})",
                user_id_str, msg.session_id
            );
        }
    }
}

/// Handler for JoinRoom message
impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Context<Self>) -> Self::Result {
        let room = self
            .rooms
            .entry(msg.conversation_id.clone())
            .or_insert_with(HashSet::new);
        room.insert(msg.session_id);

        debug!(
            "Session {} joined room {}",
            msg.session_id, msg.conversation_id
        );
    }
}

/// Handler for LeaveRoom message
impl Handler<LeaveRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.conversation_id) {
            room.remove(&msg.session_id);

            // Clean up empty rooms
            if room.is_empty() {
                self.rooms.remove(&msg.conversation_id);
            }

            debug!(
                "Session {} left room {}",
                msg.session_id, msg.conversation_id
            );
        }
    }
}

/// Handler for SendMessage - broadcasts to all in room
impl Handler<SendMessage> for ChatServer {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let conversation_id = msg.conversation_id.clone();
        let session_id = msg.session_id;
        let message_service = self.message_service.clone();

        // Parse conversation_id into a Thing
        let parts: Vec<&str> = conversation_id.split(':').collect();
        if parts.len() != 2 {
            error!("Invalid conversation_id format: {}", conversation_id);
            let error_payload = serde_json::json!({
                "type": "Error",
                "message": "Invalid conversation ID format"
            })
            .to_string();
            self.send_message_to_session(session_id, &error_payload);
            return Box::pin(async {}.into_actor(self));
        }

        let conv_thing = Thing::from((parts[0], parts[1]));

        let chat_msg = Message {
            id: None,
            conversation_id: conv_thing.clone(),
            sender_id: msg.sender_id.clone(),
            content: msg.message.clone(),
            message_type: MessageType::Text,
            created_at: chrono::Utc::now(),
            read_by: vec![],
        };

        // Use wrap_future to run async logic within the actor
        Box::pin(
            async move { message_service.send_message(chat_msg).await }
                .into_actor(self)
                .map(move |result, act, _ctx| {
                    match result {
                        Ok(saved_msg) => {
                            // Broadcast the saved message with its real ID and timestamp
                            let broadcast_payload = serde_json::json!({
                                "type": "NewMessage",
                                "message": saved_msg
                            })
                            .to_string();

                            act.broadcast_to_room(&conversation_id, &broadcast_payload, None);
                        }
                        Err(e) => {
                            error!("Failed to save message: {:?}", e);
                            // Notify ONLY the sender of the error
                            let error_payload = serde_json::json!({
                                "type": "Error",
                                "message": e.to_string()
                            })
                            .to_string();
                            act.send_message_to_session(session_id, &error_payload);
                        }
                    }
                }),
        )
    }
}
