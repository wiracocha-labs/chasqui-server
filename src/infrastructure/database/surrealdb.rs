//! SurrealDB database implementation for the application.
//!
//! This module provides a `Database` struct that wraps a SurrealDB client and
//! handles connection setup, authentication, and namespace/database selection.
//! It reads configuration from environment variables with sensible defaults.

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
    /// Initializes a new database connection with the following steps:
    /// 1. Connects to the SurrealDB server using `SURREALDB_HOST` (default: 127.0.0.1:8002)
    /// 2. Authenticates using `SURREALDB_USER`/`SURREALDB_PASS` (default: root/root)
    /// 3. Selects namespace/database from `SURREALDB_NAMESPACE`/`SURREALDB_DATABASE`
    ///    (default: "surreal"/"task")
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
}