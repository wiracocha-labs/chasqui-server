use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::infrastructure::auth::jwt::hash_password;

// User struct represents a user in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // Optional ID field, skipped during serialization if None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String, // This should store the hashed password
}

impl User {
    // Create a new user with a hashed password
    pub fn new(username: String, password: String) -> Result<Self, bcrypt::BcryptError> {
        // Hash the provided password
        let hashed_password = hash_password(&password)?;
        
        // Return a new User instance
        Ok(User {
            id: None,  // ID is None for new users, will be set by the database
            username,
            password: hashed_password,
        })
    }
}