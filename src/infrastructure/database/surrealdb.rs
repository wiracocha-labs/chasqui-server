//! SurrealDB database implementation for the application.
//!
//! This module provides a `Database` struct that wraps a SurrealDB client and
//! handles connection setup, authentication, and namespace/database selection.
//! It reads configuration from environment variables with sensible defaults.
//!
//! Notes:
//! - Uses parameterized queries to avoid injection.
//! - User IDs are SurrealDB Things in the form `user:<uuid-v4>`.
//! - Inherent helper `find_user_by_email` returns only users with a password set
//!   (filters legacy rows) via `AND password != NONE LIMIT 1`.
//!
//! Env:
//! - SURREALDB_HOST (ws endpoint), SURREALDB_USER, SURREALDB_PASS,
//!   SURREALDB_NAMESPACE, SURREALDB_DATABASE.

use crate::models::entities::user::User;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use std::env;

/// A connection to a SurrealDB database instance.
///
/// This struct holds the SurrealDB client along with the namespace and database
/// names being used. It's designed to be cloned and shared across threads.
#[derive(Clone)]
pub struct Database {
    /// The underlying SurrealDB client instance.
    pub client: Surreal<Client>,
    /// The namespace currently in use.
    pub namespace: String,
    /// The name of the currently selected database.
    pub db_name: String,
}

impl Database {
    /// Initializes a new database connection using env vars:
    /// - SURREALDB_HOST (default: 127.0.0.1:8002)
    /// - SURREALDB_USER/SURREALDB_PASS (default: root/root)
    /// - SURREALDB_NAMESPACE/SURREALDB_DATABASE (default: surreal/task)
    ///
    /// Returns an authenticated client with ns/db selected.
    ///
    /// # Errors
    /// Returns `Err` if any step fails (connection, authentication, or selection).
    pub async fn init() -> Result<Self, Error> {
        println!("Starting connection to SurrealDB...");
        // Create a new SurrealDB client and connect to the server
        let host = env::var("SURREALDB_HOST").unwrap_or_else(|_| "127.0.0.1:8002".to_string());
        let client = Surreal::new::<Ws>(host).await?;
        
        println!("Connection established. Signing in...");
        // Sign in to the database with root credentials
        // Read credentials from environment variables
        let username = env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
        let password = env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());
        client
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;
        
        println!("Signed in. Selecting namespace and database...");
        // Select the namespace and database to use
        let namespace = env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "surreal".to_string());
        let db_name = env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "task".to_string());
        client.use_ns(&namespace).use_db(&db_name).await?;
        
        println!("Namespace and database selected.");
        // Return a new Database instance
        Ok(Database {
            client,
            namespace,
            db_name,
        })
    }

    /// Find a user by email (filters legacy rows without password).
    ///
    /// Query:
    /// SELECT * FROM user WHERE email = $email AND password != NONE LIMIT 1
    ///
    /// Returns:
    /// - Some(User) when found and deserialized, None otherwise.
    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
        let sql = "SELECT * FROM user WHERE email = $email AND password != NONE LIMIT 1";
        match self
            .client
            .query(sql)
            .bind(("email", email))
            .await
        {
            Ok(mut resp) => resp.take::<Option<User>>(0).ok().flatten(),
            Err(_) => None,
        }
    }
}