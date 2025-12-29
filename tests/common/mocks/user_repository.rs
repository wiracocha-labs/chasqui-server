use async_trait::async_trait;
use mockall::automock;
use actix_crud::models::entities::user::User;
use actix_crud::models::entities::role::Role;
use std::fmt;

#[derive(Debug)]
pub struct MockError(String);

impl std::error::Error for MockError {}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mock error: {}", self.0)
    }
}

impl From<String> for MockError {
    fn from(err: String) -> Self {
        MockError(err)
    }
}

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, MockError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, MockError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, MockError>;
    async fn add_user_role(&self, user_id: &str, role: Role) -> Result<(), MockError>;
}