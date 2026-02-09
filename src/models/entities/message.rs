//! Message Entity Module
//!
//! Represents a chat message in the system.
//!
//! # Fields
//! - `id`: SurrealDB Thing with schema `message:<uuid-v4>`
//! - `conversation_id`: Reference to the conversation
//! - `sender_id`: Reference to the user who sent the message
//! - `content`: Message text content
//! - `message_type`: Type of message (Text, Image, File)
//! - `created_at`: Timestamp when message was created
//! - `read_by`: List of user IDs who have read the message

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid::Uuid;

/// Type of message content
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// Plain text message
    Text,
    /// Image message with URL
    Image,
    /// File attachment with URL
    File,
}

/// Represents a chat message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// Database identifier (SurrealDB Thing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,

    /// Reference to the conversation this message belongs to
    pub conversation_id: Thing,

    /// Reference to the user who sent the message
    pub sender_id: Thing,

    /// Message content (text, URL, etc.)
    pub content: String,

    /// Type of message
    #[serde(default = "default_message_type")]
    pub message_type: MessageType,

    /// When the message was created
    #[serde(default = "default_timestamp")]
    pub created_at: DateTime<Utc>,

    /// List of user IDs who have read this message
    #[serde(default)]
    pub read_by: Vec<Thing>,
}

fn default_message_type() -> MessageType {
    MessageType::Text
}

fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

impl Message {
    /// Creates a new message
    ///
    /// # Arguments
    /// * `conversation_id` - The conversation this message belongs to
    /// * `sender_id` - The user sending the message
    /// * `content` - The message content
    /// * `message_type` - Type of message (defaults to Text)
    pub fn new(
        conversation_id: Thing,
        sender_id: Thing,
        content: String,
        message_type: Option<MessageType>,
    ) -> Self {
        let uuid = Uuid::new_v4().to_string();

        Message {
            id: Some(Thing::from(("message", uuid.as_str()))),
            conversation_id,
            sender_id,
            content,
            message_type: message_type.unwrap_or(MessageType::Text),
            created_at: Utc::now(),
            read_by: Vec::new(),
        }
    }

    /// Mark message as read by a user
    ///
    /// # Arguments
    /// * `user_id` - The user who read the message
    ///
    /// # Returns
    /// `true` if the user was added, `false` if they already read it
    pub fn mark_as_read(&mut self, user_id: Thing) -> bool {
        if !self.read_by.contains(&user_id) {
            self.read_by.push(user_id);
            true
        } else {
            false
        }
    }

    /// Check if a user has read this message
    ///
    /// # Arguments
    /// * `user_id` - The user to check
    pub fn is_read_by(&self, user_id: &Thing) -> bool {
        self.read_by.contains(user_id)
    }

    /// Validate message content
    ///
    /// # Returns
    /// `true` if valid, `false` otherwise
    pub fn is_valid(&self) -> bool {
        !self.content.trim().is_empty() && self.content.len() <= 10000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message() {
        let conv_id = Thing::from(("conversation", "test-conv"));
        let sender_id = Thing::from(("user", "test-user"));
        let content = "Hello, World!".to_string();

        let message = Message::new(conv_id.clone(), sender_id.clone(), content.clone(), None);

        assert!(message.id.is_some());
        assert_eq!(message.conversation_id, conv_id);
        assert_eq!(message.sender_id, sender_id);
        assert_eq!(message.content, content);
        assert_eq!(message.message_type, MessageType::Text);
        assert!(message.read_by.is_empty());
    }

    #[test]
    fn test_mark_as_read() {
        let conv_id = Thing::from(("conversation", "test-conv"));
        let sender_id = Thing::from(("user", "sender"));
        let reader_id = Thing::from(("user", "reader"));

        let mut message = Message::new(conv_id, sender_id, "Test".to_string(), None);

        assert!(message.mark_as_read(reader_id.clone()));
        assert!(!message.mark_as_read(reader_id.clone())); // Already read
        assert!(message.is_read_by(&reader_id));
    }

    #[test]
    fn test_message_validation() {
        let conv_id = Thing::from(("conversation", "test"));
        let sender_id = Thing::from(("user", "test"));

        let valid_msg = Message::new(
            conv_id.clone(),
            sender_id.clone(),
            "Valid".to_string(),
            None,
        );
        assert!(valid_msg.is_valid());

        let empty_msg = Message::new(conv_id.clone(), sender_id.clone(), "".to_string(), None);
        assert!(!empty_msg.is_valid());

        let too_long = "a".repeat(10001);
        let long_msg = Message::new(conv_id, sender_id, too_long, None);
        assert!(!long_msg.is_valid());
    }
}
