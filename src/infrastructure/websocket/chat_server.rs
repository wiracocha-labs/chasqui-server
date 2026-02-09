//! Chat Server Actor
//!
//! Central actor that manages all WebSocket connections and message routing.
//! Handles:
//! - Connection registration/deregistration
//! - Room management (conversations)
//! - Message broadcasting
//! - User presence tracking

use actix::prelude::*;
use log::{debug, info};
use std::collections::{HashMap, HashSet};
use surrealdb::sql::Thing;

/// Chat server manages all WebSocket connections and rooms
pub struct ChatServer {
    /// Map of conversation_id -> set of session IDs
    rooms: HashMap<String, HashSet<usize>>,

    /// Map of session_id -> user_id
    sessions: HashMap<usize, Thing>,

    /// Map of user_id -> session_id (for presence tracking)
    user_sessions: HashMap<String, usize>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
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

        // Store session
        self.sessions.insert(session_id, msg.user_id.clone());

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
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(sessions) = self.rooms.get(&msg.conversation_id) {
            debug!(
                "Broadcasting message to {} sessions in room {}",
                sessions.len(),
                msg.conversation_id
            );

            // Note: In a real implementation, you would:
            // 1. Store the message in the database
            // 2. Get the session recipients
            // 3. Send the message to each recipient
            // For now, this is a placeholder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_chat_server_creation() {
        let server = ChatServer::new();
        assert_eq!(server.rooms.len(), 0);
        assert_eq!(server.sessions.len(), 0);
    }
}
