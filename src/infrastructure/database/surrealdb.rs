//! SurrealDB database implementation for the application.
//!
//! This module provides a `Database` struct that wraps a SurrealDB client and
//! handles connection setup, authentication, and namespace/database selection.
//! It reads configuration from environment variables.
//!
//! Notes:
//! - Uses parameterized queries to avoid injection.
//! - User IDs are SurrealDB Things in the form `user:<uuid-v4>`.
//! - Inherent helper `find_user_by_email` returns only users with a password set
//!   (filters legacy rows) via `AND password != NONE LIMIT 1`.
//! - Connects to a cloud-hosted SurrealDB instance (wss://) via `any::connect`.
//!
//! Env:
//! - SURREALDB_HOST (wss:// cloud endpoint)
//! - SURREALDB_USER, SURREALDB_PASS
//! - SURREALDB_NAMESPACE, SURREALDB_DATABASE
//!
//! For tests, use TEST_ prefixed variables:
//! - TEST_SURREALDB_HOST, TEST_SURREALDB_USER, etc.

use crate::models::entities::user::User;
use log::{debug, error, info};
use std::env;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

/// A connection to a SurrealDB database instance.
///
/// This struct holds the SurrealDB client along with the namespace and database
/// names being used. It's designed to be cloned and shared across threads.
#[derive(Clone)]
pub struct Database {
    /// The underlying SurrealDB client instance.
    pub client: Surreal<any::Any>,
    /// The namespace currently in use.
    pub namespace: String,
    /// The name of the currently selected database.
    pub db_name: String,
}

impl Database {
    /// Initializes a new database connection to a cloud-hosted SurrealDB instance.
    ///
    /// Reads required environment variables:
    /// - SURREALDB_HOST (wss:// cloud endpoint)
    /// - SURREALDB_NAMESPACE
    /// - SURREALDB_DATABASE
    ///
    /// Returns an authenticated client with namespace and database selected.
    ///
    /// # Errors
    /// Returns `Err` if any step fails (connection, authentication, or selection).
    ///
    /// # Panics
    /// Panics if required environment variables are not set.
    pub async fn init() -> Result<Self, Error> {
        info!("SurrealDB: starting connection");

        // Read environment variables with descriptive error messages
        let host = env::var("SURREALDB_HOST")
            .expect("FATAL: SURREALDB_HOST environment variable is not set. Please configure it in your .env file.");
        let namespace = env::var("SURREALDB_NAMESPACE")
            .expect("FATAL: SURREALDB_NAMESPACE environment variable is not set. Please configure it in your .env file.");
        let db_name = env::var("SURREALDB_DATABASE")
            .expect("FATAL: SURREALDB_DATABASE environment variable is not set. Please configure it in your .env file.");
        let username = env::var("SURREALDB_USER")
            .expect("FATAL: SURREALDB_USER environment variable is not set. Please configure it in your .env file.");
        let password = env::var("SURREALDB_PASS")
            .expect("FATAL: SURREALDB_PASS environment variable is not set. Please configure it in your .env file.");

        info!("SurrealDB: environment variables loaded successfully");

        // Connect to the cloud database
        info!("SurrealDB: attempting connection to cloud instance");
        let db = any::connect(&host).await.map_err(|e| {
            error!("SurrealDB: FAILED to connect to cloud instance: {:?}", e);
            e
        })?;
        info!("SurrealDB: ✓ connection established");

        // Authenticate
        info!(
            "SurrealDB: attempting authentication as user '{}'",
            username
        );
        db.signin(Root {
            username: &username,
            password: &password,
        })
        .await
        .map_err(|e| {
            error!("SurrealDB: FAILED to authenticate: {:?}", e);
            e
        })?;
        info!("SurrealDB: ✓ authentication successful");

        // Select namespace and database
        info!(
            "SurrealDB: selecting namespace '{}' and database '{}'",
            namespace, db_name
        );
        db.use_ns(&namespace).use_db(&db_name).await.map_err(|e| {
            error!("SurrealDB: FAILED to select namespace/database: {:?}", e);
            e
        })?;
        info!("SurrealDB: ✓ namespace and database selected successfully");

        info!("SurrealDB: ✓ fully connected to {}/{}", namespace, db_name);

        Ok(Database {
            client: db,
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
        debug!("SurrealDB: find_user_by_email {}", email);
        let sql = "SELECT * FROM user WHERE email = $email AND password != NONE LIMIT 1";
        match self
            .client
            .query(sql)
            .bind(("email", email.to_owned()))
            .await
        {
            Ok(mut resp) => {
                let out = resp.take::<Option<User>>(0).ok().flatten();
                if out.is_some() {
                    debug!("SurrealDB: user found by email");
                } else {
                    debug!("SurrealDB: user not found by email");
                }
                out
            }
            Err(e) => {
                error!("SurrealDB query error (find_user_by_email): {:?}", e);
                None
            }
        }
    }
}
