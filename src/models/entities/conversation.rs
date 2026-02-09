//! Conversation Entity Module
//!
//! Represents a chat conversation (1-to-1 or group) in the system.
//!
//! # Fields
//! - `id`: SurrealDB Thing with schema `conversation:<uuid-v4>`
//! - `participants`: List of user IDs in the conversation
//! - `conversation_type`: Direct (1-to-1) or Group
//! - `name`: Optional name for group conversations
//! - `created_at`: Timestamp when conversation was created
//! - `updated_at`: Timestamp of last activity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid::Uuid;

/// Type of conversation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConversationType {
    /// Direct 1-to-1 conversation
    Direct,
    /// Group conversation with multiple participants
    Group,
}

/// Represents a chat conversation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    /// Database identifier (SurrealDB Thing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,

    /// List of participant user IDs
    pub participants: Vec<Thing>,

    /// Type of conversation
    pub conversation_type: ConversationType,

    /// Optional name for group conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// When the conversation was created
    #[serde(default = "default_timestamp")]
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    #[serde(default = "default_timestamp")]
    pub updated_at: DateTime<Utc>,
}

fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

impl Conversation {
    /// Creates a new direct (1-to-1) conversation
    ///
    /// # Arguments
    /// * `user1` - First participant
    /// * `user2` - Second participant
    pub fn new_direct(user1: Thing, user2: Thing) -> Self {
        let uuid = Uuid::new_v4().to_string();

        Conversation {
            id: Some(Thing::from(("conversation", uuid.as_str()))),
            participants: vec![user1, user2],
            conversation_type: ConversationType::Direct,
            name: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Creates a new group conversation
    ///
    /// # Arguments
    /// * `participants` - List of participant user IDs
    /// * `name` - Optional group name
    pub fn new_group(participants: Vec<Thing>, name: Option<String>) -> Self {
        let uuid = Uuid::new_v4().to_string();

        Conversation {
            id: Some(Thing::from(("conversation", uuid.as_str()))),
            participants,
            conversation_type: ConversationType::Group,
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a participant to the conversation
    ///
    /// # Arguments
    /// * `user_id` - The user to add
    ///
    /// # Returns
    /// `true` if added, `false` if already a participant
    pub fn add_participant(&mut self, user_id: Thing) -> bool {
        if !self.participants.contains(&user_id) {
            self.participants.push(user_id);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Remove a participant from the conversation
    ///
    /// # Arguments
    /// * `user_id` - The user to remove
    ///
    /// # Returns
    /// `true` if removed, `false` if not a participant
    pub fn remove_participant(&mut self, user_id: &Thing) -> bool {
        let original_len = self.participants.len();
        self.participants.retain(|p| p != user_id);

        if self.participants.len() != original_len {
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Check if a user is a participant
    ///
    /// # Arguments
    /// * `user_id` - The user to check
    pub fn has_participant(&self, user_id: &Thing) -> bool {
        self.participants.contains(user_id)
    }

    /// Update the last activity timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Validate conversation
    ///
    /// # Returns
    /// `true` if valid, `false` otherwise
    pub fn is_valid(&self) -> bool {
        match self.conversation_type {
            ConversationType::Direct => self.participants.len() == 2,
            ConversationType::Group => self.participants.len() >= 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_direct_conversation() {
        let user1 = Thing::from(("user", "user1"));
        let user2 = Thing::from(("user", "user2"));

        let conv = Conversation::new_direct(user1.clone(), user2.clone());

        assert!(conv.id.is_some());
        assert_eq!(conv.conversation_type, ConversationType::Direct);
        assert_eq!(conv.participants.len(), 2);
        assert!(conv.has_participant(&user1));
        assert!(conv.has_participant(&user2));
        assert!(conv.name.is_none());
        assert!(conv.is_valid());
    }

    #[test]
    fn test_new_group_conversation() {
        let users = vec![
            Thing::from(("user", "user1")),
            Thing::from(("user", "user2")),
            Thing::from(("user", "user3")),
        ];
        let name = Some("Test Group".to_string());

        let conv = Conversation::new_group(users.clone(), name.clone());

        assert!(conv.id.is_some());
        assert_eq!(conv.conversation_type, ConversationType::Group);
        assert_eq!(conv.participants.len(), 3);
        assert_eq!(conv.name, name);
        assert!(conv.is_valid());
    }

    #[test]
    fn test_add_remove_participant() {
        let user1 = Thing::from(("user", "user1"));
        let user2 = Thing::from(("user", "user2"));
        let user3 = Thing::from(("user", "user3"));

        let mut conv = Conversation::new_direct(user1.clone(), user2.clone());

        // Add new participant
        assert!(conv.add_participant(user3.clone()));
        assert_eq!(conv.participants.len(), 3);

        // Try to add duplicate
        assert!(!conv.add_participant(user3.clone()));
        assert_eq!(conv.participants.len(), 3);

        // Remove participant
        assert!(conv.remove_participant(&user3));
        assert_eq!(conv.participants.len(), 2);

        // Try to remove non-existent
        assert!(!conv.remove_participant(&user3));
    }

    #[test]
    fn test_conversation_validation() {
        let user1 = Thing::from(("user", "user1"));
        let user2 = Thing::from(("user", "user2"));

        // Valid direct conversation
        let direct = Conversation::new_direct(user1.clone(), user2.clone());
        assert!(direct.is_valid());

        // Valid group conversation
        let group = Conversation::new_group(vec![user1.clone(), user2.clone()], None);
        assert!(group.is_valid());

        // Invalid: direct with wrong number of participants
        let mut invalid_direct = Conversation::new_direct(user1.clone(), user2.clone());
        invalid_direct
            .participants
            .push(Thing::from(("user", "user3")));
        assert!(!invalid_direct.is_valid());
    }
}
