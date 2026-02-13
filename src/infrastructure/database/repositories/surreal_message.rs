use async_trait::async_trait;
use surrealdb::sql::Thing;
use surrealdb::Error;

use crate::infrastructure::database::surrealdb::Database;
use crate::interfaces::repositories::message::MessageRepository;
use crate::models::entities::message::Message;

pub struct SurrealMessageRepository {
    db: Database,
}

impl SurrealMessageRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MessageRepository for SurrealMessageRepository {
    async fn create(&self, message: Message) -> Result<Message, Error> {
        let created: Option<Message> = self.db.client.create("message").content(message).await?;

        // Using Error::Db with a Thrown variant as suggested by the compiler
        created.ok_or_else(|| {
            Error::Db(surrealdb::error::Db::Thrown(
                "Failed to create message".to_string(),
            ))
        })
    }

    async fn find_by_conversation(
        &self,
        conversation_id: Thing,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Message>, Error> {
        let sql = "SELECT * FROM message WHERE conversation_id = $conv ORDER BY created_at DESC LIMIT $limit START $offset";
        let mut response = self
            .db
            .client
            .query(sql)
            .bind(("conv", conversation_id))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?;

        let messages: Vec<Message> = response.take(0)?;
        Ok(messages)
    }

    async fn mark_as_read(&self, message_id: Thing, user_id: Thing) -> Result<(), Error> {
        let sql = "UPDATE $id SET read_by += $user";
        self.db
            .client
            .query(sql)
            .bind(("user", user_id))
            .bind(("id", message_id))
            .await?;
        Ok(())
    }

    async fn delete(&self, id: Thing) -> Result<(), Error> {
        self.db.client.query("DELETE $id").bind(("id", id)).await?;
        Ok(())
    }
}
