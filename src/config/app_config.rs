//! Configuration module: loads application settings from environment variables.
//!
//! This module provides the `AppConfig` struct and a helper to construct it from
//! environment variables. It first attempts to load a local `.env` file (if
//! present) using `dotenv`, then reads required variables from the process
//! environment.
//!
//! Required variables:
//! - `DATABASE_URL`: database connection string
//! - `SECRET_KEY`: secret used for signing/encryption (e.g., JWT or crypto)
//!
use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// Fields:
/// - `database_url`: connection string for the database (from `DATABASE_URL`)
/// - `secret_key`: secret for signing/encryption (from `SECRET_KEY`)
#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
}

impl AppConfig {
    /// Construct `AppConfig` by reading environment variables.
    ///
    /// Behavior:
    /// - Attempts to load `.env` (non-fatal if the file doesn't exist).
    /// - Reads `DATABASE_URL` and `SECRET_KEY`; panics with a clear message if
    ///   any is missing. This is intentional to fail fast during startup.
    pub fn from_env() -> Self {
        // Load environment variables from `.env` if present (ignore errors)
        dotenv::dotenv().ok();
        
        // Retrieve `DATABASE_URL`; panic early if not provided
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        // Retrieve `SECRET_KEY`; panic early if not provided
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        
        // Return assembled configuration
        AppConfig { database_url, secret_key }
    }
}
