//! User Data Trait Module
//! Defines the interface for user-related database operations.
//!
//! Implementación actual (SurrealDB):
//! - add_user: inserta en tabla `user`, retorna Option<User>.
//! - find_user_by_username/email: consultas parametrizadas con `LIMIT 1`
//!   y filtro `AND password != NONE` para evitar filas legacy sin hash.
//!
//! Notas:
//! - Retorna None ante errores de DB o deserialización.
//! - Deserializa directamente a Option<User> con `response.take::<Option<User>>(0)`.
//! - Las funciones son asíncronas y retornan resultados envueltos en Option.

use crate::infrastructure::database::surrealdb::Database;
use crate::models::entities::user::User;
use async_trait::async_trait;
use log::{debug, error, info, warn}; // añadido

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
        info!("DB add_user: start");
        // SurrealDB: create -> Result<Vec<User>, Error> when creating in a table
        let created_users = self
            .client
            .create("user") // Create in 'user' table
            .content(new_user) // Set the record content
            .await;

        // Handle the database response
        match created_users {
            Ok(users) => {
                let user_opt = users.into_iter().next();
                if user_opt.is_some() {
                    info!("DB add_user: success");
                } else {
                    warn!("DB add_user: no user returned");
                }
                user_opt
            }
            Err(e) => {
                error!("DB add_user error: {:?}", e);
                None
            }
        }
    }

    // Find a user by their username
    async fn find_user_by_username(&self, username: &str) -> Option<User> {
        debug!("DB find_user_by_username: {}", username);
        let result = self
            .client
            .query("SELECT * FROM user WHERE username = $username AND password != NONE LIMIT 1")
            .bind(("username", username.to_owned()))
            .await;

        // Process the query result
        match result {
            Ok(mut response) => match response.take::<Option<User>>(0) {
                Ok(user_opt) => {
                    if user_opt.is_some() {
                        debug!("DB find_user_by_username: found");
                    } else {
                        debug!("DB find_user_by_username: not found");
                    }
                    user_opt
                }
                Err(e) => {
                    error!("DB find_user_by_username deserialization error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                error!("DB find_user_by_username query error: {:?}", e);
                None
            }
        }
    }

    // Find a user by their email
    async fn find_user_by_email(&self, email: &str) -> Option<User> {
        debug!("DB find_user_by_email: {}", email);
        let result = self
            .client
            .query("SELECT * FROM user WHERE email = $email AND password != NONE LIMIT 1")
            .bind(("email", email.to_owned()))
            .await;

        // Process the query result
        match result {
            Ok(mut response) => match response.take::<Option<User>>(0) {
                Ok(user_opt) => {
                    if user_opt.is_some() {
                        debug!("DB find_user_by_email: found");
                    } else {
                        debug!("DB find_user_by_email: not found");
                    }
                    user_opt
                }
                Err(e) => {
                    error!("DB find_user_by_email deserialization error: {:?}", e);
                    None
                }
            },
            Err(e) => {
                error!("DB find_user_by_email query error: {:?}", e);
                None
            }
        }
    }
}
