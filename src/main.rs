//! Application Entry Point
//! Initializes and runs the HTTP server with all necessary middleware and configurations.

use actix::Actor;
use actix_crud::infrastructure::database::surrealdb::Database;
use actix_crud::interfaces::api::routes;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::env;

/// Main application entry point
/// Initializes the following components:
/// - Environment variables
/// - Logging system
/// - Database connection
/// - HTTP server with middleware
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok(); // Load .env file if present
                           // Setup logging with default environment configuration
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Starting the application...");

    // Initialize database connection
    // This creates a new SurrealDB instance with configured credentials
    let db = Database::init()
        .await
        .expect("Error connecting to database");
    println!("Database connection established.");

    // Wrap database connection in web::Data for thread-safe sharing
    // This allows the connection to be used across different request handlers
    let db_data = web::Data::new(db);

    // Initialize ChatServer actor for WebSockets
    let chat_server = actix_crud::infrastructure::websocket::chat_server::ChatServer::new().start();
    let chat_server_data = web::Data::new(chat_server);

    println!("Starting the HTTP server...");
    // Configure and launch HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Enable request logging
            .app_data(db_data.clone()) // Share database connection
            .app_data(chat_server_data.clone()) // Share chat server actor
            .configure(routes::config) // Setup API routes
    })
    .bind({
        // Read server configuration from environment variables
        // with fallback to default values
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
        format!("{}:{}", host, port)
    })?
    .run()
    .await // Start the server and wait for it to complete
}
