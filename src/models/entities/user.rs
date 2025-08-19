//! User Entity Module
//! Defines the core user-related data structures and their behavior.

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::infrastructure::auth::jwt::hash_password;

/// Represents a User in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Database-assigned identifier, optional for new users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    /// Unique username for the user
    pub username: String,
    /// Hashed password string
    pub password: String,
}

impl User {
    /// Creates a new User instance with a hashed password
    /// 
    /// # Arguments
    /// * `username` - Unique identifier for the user
    /// * `password` - Plain text password to be hashed
    ///
    /// # Returns
    /// * `Result<User, BcryptError>` - New user instance or hashing error
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