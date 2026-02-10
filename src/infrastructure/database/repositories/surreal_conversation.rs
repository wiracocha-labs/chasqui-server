use async_trait::async_trait;
use surrealdb::sql::Thing;
use surrealdb::Error;

use crate::infrastructure::database::surrealdb::Database;
use crate::interfaces::repositories::conversation::ConversationRepository;
use crate::models::entities::conversation::Conversation;

pub struct SurrealConversationRepository {
    db: Database,
}

impl SurrealConversationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ConversationRepository for SurrealConversationRepository {
    async fn create(&self, conversation: Conversation) -> Result<Conversation, Error> {
        let created: Option<Conversation> = self
            .db
            .client
            .create("conversation")
            .content(conversation)
            .await?;

        // Using Error::Db with a Thrown variant as suggested by the compiler
        created.ok_or_else(|| {
            Error::Db(surrealdb::error::Db::Thrown(
                "Failed to create conversation".to_string(),
            ))
        })
    }

    async fn find_by_user(&self, user_id: Thing) -> Result<Vec<Conversation>, Error> {
        let sql =
            "SELECT * FROM conversation WHERE participants CONTAINS $user ORDER BY updated_at DESC";
        let mut response = self.db.client.query(sql).bind(("user", user_id)).await?;

        let conversations: Vec<Conversation> = response.take(0)?;
        Ok(conversations)
    }

    async fn find_by_id(&self, id: Thing) -> Result<Option<Conversation>, Error> {
        let mut response = self
            .db
            .client
            .query("SELECT * FROM $id")
            .bind(("id", id))
            .await?;
        Ok(response.take(0)?)
    }

    async fn add_participant(&self, conversation_id: Thing, user_id: Thing) -> Result<(), Error> {
        let sql = "UPDATE $conv SET participants += $user, updated_at = time::now()";
        self.db
            .client
            .query(sql)
            .bind(("conv", conversation_id))
            .bind(("user", user_id))
            .await?;
        Ok(())
    }

    async fn remove_participant(
        &self,
        conversation_id: Thing,
        user_id: Thing,
    ) -> Result<(), Error> {
        let sql = "UPDATE $conv SET participants -= $user, updated_at = time::now()";
        self.db
            .client
            .query(sql)
            .bind(("conv", conversation_id))
            .bind(("user", user_id))
            .await?;
        Ok(())
    }
}
