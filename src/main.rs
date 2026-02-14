//! Application Entry Point
//! Initializes and runs the HTTP server with all necessary middleware and configurations.

use actix::Actor;
use actix_crud::infrastructure::database::surrealdb::Database;
use actix_crud::interfaces::api::routes;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::env;
use std::sync::Arc;

use actix_crud::application::services::conversation_service::ConversationService;
use actix_crud::application::services::message_service::MessageService;
use actix_crud::infrastructure::database::repositories::surreal_conversation::SurrealConversationRepository;
use actix_crud::infrastructure::database::repositories::surreal_message::SurrealMessageRepository;
use actix_crud::infrastructure::websocket::chat_server::ChatServer;

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

    // Check for --list-api argument
    if std::env::args().any(|arg| arg == "--list-api") {
        actix_crud::interfaces::api::api_doc::print_routes();
        return Ok(());
    }

    // Check for --list-ws argument
    if std::env::args().any(|arg| arg == "--list-ws") {
        actix_crud::interfaces::api::api_doc::print_ws_docs();
        return Ok(());
    }

    // Initialize database connection
    // This creates a new SurrealDB instance with configured credentials
    let db = Database::init()
        .await
        .expect("Error connecting to database");
    println!("Database connection established.");

    // Wrap database connection in web::Data for thread-safe sharing
    let db_data = web::Data::new(db.clone());

    // Initialize repositories
    let message_repo = Arc::new(SurrealMessageRepository::new(db.clone()));
    let conversation_repo = Arc::new(SurrealConversationRepository::new(db.clone()));

    // Initialize services
    let message_service = Arc::new(MessageService::new(
        message_repo.clone(),
        conversation_repo.clone(),
    ));
    let conversation_service = Arc::new(ConversationService::new(conversation_repo.clone()));

    // Initialize ChatServer actor for WebSockets with injected services
    let chat_server =
        ChatServer::new(message_service.clone(), conversation_service.clone()).start();
    let chat_server_data = web::Data::new(chat_server);

    // Prepare web::Data for services to fix extractor issues
    let message_service_data = web::Data::from(message_service.clone());
    let conversation_service_data = web::Data::from(conversation_service.clone());

    println!("Starting the HTTP server...");
    // Configure and launch HTTP server
    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin() // Permitir cualquier origen por ahora (ideal para desarrollo)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors) // Enable CORS
            .wrap(Logger::default()) // Enable request logging
            .app_data(db_data.clone()) // Share database connection
            .app_data(chat_server_data.clone()) // Share chat server actor
            .app_data(message_service_data.clone()) // Share message service
            .app_data(conversation_service_data.clone()) // Share conversation service
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
