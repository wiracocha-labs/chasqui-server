use async_trait::async_trait;
use mockall::automock;
use actix_crud::models::entities::user::User;
use actix_crud::models::entities::role::Role;

#[derive(Debug, thiserror::Error)]
pub enum MockError {
    #[error("Mock error: {0}")]
    Error(String),
}

// El automock genera un struct MockUserRepository que implementa este trait
#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, MockError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, MockError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, MockError>;
    async fn add_user_role(&self, user_id: &str, role: Role) -> Result<(), MockError>;
}