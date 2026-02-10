use crate::models::entities::conversation::Conversation;
use async_trait::async_trait;
use surrealdb::sql::Thing;
use surrealdb::Error;

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn create(&self, conversation: Conversation) -> Result<Conversation, Error>;
    async fn find_by_user(&self, user_id: Thing) -> Result<Vec<Conversation>, Error>;
    async fn find_by_id(&self, id: Thing) -> Result<Option<Conversation>, Error>;
    async fn add_participant(&self, conversation_id: Thing, user_id: Thing) -> Result<(), Error>;
    async fn remove_participant(&self, conversation_id: Thing, user_id: Thing)
        -> Result<(), Error>;
}
