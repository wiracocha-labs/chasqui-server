use std::sync::Arc;
use surrealdb::sql::Thing;
use surrealdb::Error;

use crate::interfaces::repositories::conversation::ConversationRepository;
use crate::models::entities::conversation::Conversation;

pub struct ConversationService {
    conversation_repo: Arc<dyn ConversationRepository>,
}

impl ConversationService {
    pub fn new(conversation_repo: Arc<dyn ConversationRepository>) -> Self {
        Self { conversation_repo }
    }

    pub async fn create_direct_conversation(
        &self,
        user_a: Thing,
        user_b: Thing,
    ) -> Result<Conversation, Error> {
        // For direct conversations, we could check if one already exists
        // but for now we just create a new one.
        let conversation = Conversation::new_direct(user_a, user_b);
        self.conversation_repo.create(conversation).await
    }

    pub async fn create_group_conversation(
        &self,
        name: String,
        creator_id: Thing,
        participants: Vec<Thing>,
    ) -> Result<Conversation, Error> {
        let mut final_participants = participants;
        if !final_participants.contains(&creator_id) {
            final_participants.push(creator_id);
        }

        let conversation = Conversation::new_group(final_participants, Some(name));
        self.conversation_repo.create(conversation).await
    }

    pub async fn get_user_conversations(&self, user_id: Thing) -> Result<Vec<Conversation>, Error> {
        self.conversation_repo.find_by_user(user_id).await
    }

    pub async fn add_participant(
        &self,
        conversation_id: Thing,
        user_id: Thing,
    ) -> Result<(), Error> {
        self.conversation_repo
            .add_participant(conversation_id, user_id)
            .await
    }
}
