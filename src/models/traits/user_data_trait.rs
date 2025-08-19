//! User Data Trait Module
//! Defines the interface for user-related database operations.
//!
//! Implementación actual (SurrealDB):
//! - add_user: inserta en tabla `user`, retorna Option<User>.
//! - find_user_by_username/email: consultas parametrizadas con `LIMIT 1`
//!   y filtro `AND defined(password)` para evitar filas legacy sin hash.
//!
//! Notas:
//! - Retorna None ante errores de DB o deserialización.
//! - Deserializa directamente a Option<User> con `response.take::<Option<User>>(0)`.
//! - Las funciones son asíncronas y retornan resultados envueltos en Option.

use crate::models::entities::user::User;
use crate::infrastructure::database::surrealdb::Database;
use async_trait::async_trait;

/// Defines the interface for user-related database operations
#[async_trait(?Send)]
pub trait UserDataTrait {
    /// Adds a new user to the database. Returns Some(User) on success, None on error.
    /// 
    /// # Arguments
    /// * `new_user` - The user to be added
    ///
    /// # Returns
    /// * `Option<User>` - Some(user) if created, None if error
    async fn add_user(&self, new_user: User) -> Option<User>;

    /// Finds a user by username. Returns a single user (LIMIT 1) with defined(password).
    /// 
    /// # Arguments
    /// * `username` - The username to search for
    ///
    /// # Returns
    /// * `Option<User>` - Some(user) if found, None if not found or error
    async fn find_user_by_username(&self, username: &str) -> Option<User>;

    /// Finds a user by email. Returns a single user (LIMIT 1) with defined(password).
    /// 
    /// # Arguments
    /// * `email` - The email to search for
    ///
    /// # Returns
    /// * `Option<User>` - Some(user) if found, None if not found or error
    async fn find_user_by_email(&self, email: &str) -> Option<User>;
}

// Implementation of UserDataTrait for the Database struct
#[async_trait(?Send)]
impl UserDataTrait for Database {
    // Add a new user to the database
    async fn add_user(&self, new_user: User) -> Option<User> {
        println!("Attempting to add a new user...");
        // SurrealDB: create -> Result<Vec<User>, Error> when creating in a table
        let created_users = self
            .client
            .create("user") // Create in 'user' table
            .content(&new_user) // Set the record content
            .await;

        // Handle the database response
        match created_users {
            Ok(users) => {
                let user_opt = users.into_iter().next();
                if user_opt.is_some() {
                    println!("User created successfully.");
                } else {
                    println!("Failed to create user.");
                }
                user_opt
            }
            Err(e) => {
                println!("Error creating user: {:?}", e);
                None
            }
        }
    }

    // Find a user by their username
    async fn find_user_by_username(&self, username: &str) -> Option<User> {
        println!("Searching for user by username: {}", username);
        let result = self
            .client
            .query("SELECT * FROM user WHERE username = $username AND password != NONE LIMIT 1")
            .bind(("username", username))
            .await;

        // Process the query result
        match result {
            Ok(mut response) => match response.take::<Option<User>>(0) {
                Ok(user_opt) => user_opt,
                Err(e) => {
                    println!("Error deserializing user: {:?}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error searching for user: {:?}", e);
                None
            }
        }
    }

    // Find a user by their email
    async fn find_user_by_email(&self, email: &str) -> Option<User> {
        println!("Searching for user by email: {}", email);
        let result = self
            .client
            .query("SELECT * FROM user WHERE email = $email AND password != NONE LIMIT 1")
            .bind(("email", email))
            .await;

        // Process the query result
        match result {
            Ok(mut response) => match response.take::<Option<User>>(0) {
                Ok(user_opt) => user_opt,
                Err(e) => {
                    println!("Error deserializing user: {:?}", e);
                    None
                }
            },
            Err(e) => {
                println!("Error searching for user: {:?}", e);
                None
            }
        }
    }
}