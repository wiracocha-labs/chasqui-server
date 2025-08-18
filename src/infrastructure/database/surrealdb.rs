use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use std::env;

// Database struct represents a connection to SurrealDB
#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,  // The SurrealDB client
    pub namespace: String,        // The namespace being used
    pub db_name: String,          // The name of the database
}

impl Database {
    // Initialize the database connection
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