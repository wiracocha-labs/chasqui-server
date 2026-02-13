//! WebSocket Infrastructure Module
//!
//! This module provides WebSocket functionality for real-time chat.
//!
//! # Module Structure
//! - `session`: Individual WebSocket connection actor
//! - `chat_server`: Central chat server managing all connections and rooms

pub mod chat_server;
pub mod session;
