use crate::models::entities::message::Message;
use async_trait::async_trait;
use surrealdb::sql::Thing;
use surrealdb::Error;

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn create(&self, message: Message) -> Result<Message, Error>;
    async fn find_by_conversation(
        &self,
        conversation_id: Thing,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Message>, Error>;
    async fn mark_as_read(&self, message_id: Thing, user_id: Thing) -> Result<(), Error>;
    async fn delete(&self, id: Thing) -> Result<(), Error>;
}
