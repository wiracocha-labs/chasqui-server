use std::sync::Arc;
use surrealdb::sql::Thing;
use surrealdb::Error;

use crate::interfaces::repositories::conversation::ConversationRepository;
use crate::interfaces::repositories::message::MessageRepository;
use crate::models::entities::message::Message;

pub struct MessageService {
    message_repo: Arc<dyn MessageRepository>,
    conversation_repo: Arc<dyn ConversationRepository>,
}

impl MessageService {
    pub fn new(
        message_repo: Arc<dyn MessageRepository>,
        conversation_repo: Arc<dyn ConversationRepository>,
    ) -> Self {
        Self {
            message_repo,
            conversation_repo,
        }
    }

    pub async fn send_message(&self, message: Message) -> Result<Message, Error> {
        // 1. Basic validation
        if !message.is_valid() {
            return Err(Error::Db(surrealdb::error::Db::Thrown(
                "Invalid message content or structure".to_string(),
            )));
        }

        // 2. Security: Verify sender is part of the conversation
        let conversation = self
            .conversation_repo
            .find_by_id(message.conversation_id.clone())
            .await?;

        match conversation {
            Some(conversation) => {
                if !conversation.has_participant(&message.sender_id) {
                    return Err(Error::Db(surrealdb::error::Db::Thrown(
                        "User is not a participant in this conversation".to_string(),
                    )));
                }
            }
            None => {
                return Err(Error::Db(surrealdb::error::Db::Thrown(
                    "Conversation not found".to_string(),
                )));
            }
        }

        // 3. Persistence
        self.message_repo.create(message).await
    }

    pub async fn get_conversation_history(
        &self,
        conversation_id: Thing,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Message>, Error> {
        self.message_repo
            .find_by_conversation(conversation_id, limit, offset)
            .await
    }

    pub async fn mark_as_read(&self, message_id: Thing, user_id: Thing) -> Result<(), Error> {
        self.message_repo.mark_as_read(message_id, user_id).await
    }
}
