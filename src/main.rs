//! Application Entry Point
//! Initializes and runs the HTTP server with all necessary middleware and configurations.

use actix::Actor;
use chasqui_server::infrastructure::database::surrealdb::Database;
use chasqui_server::interfaces::api::routes;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use std::env;
use std::sync::Arc;

use chasqui_server::application::services::conversation_service::ConversationService;
use chasqui_server::application::services::message_service::MessageService;
use chasqui_server::infrastructure::database::repositories::surreal_conversation::SurrealConversationRepository;
use chasqui_server::infrastructure::database::repositories::surreal_message::SurrealMessageRepository;
use chasqui_server::infrastructure::websocket::chat_server::ChatServer;

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
        chasqui_server::interfaces::api::api_doc::print_routes();
        return Ok(());
    }

    // Check for --list-ws argument
    if std::env::args().any(|arg| arg == "--list-ws") {
        chasqui_server::interfaces::api::api_doc::print_ws_docs();
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
        // Build CORS policy:
        // - If ALLOWED_ORIGINS env var is set (comma-separated), only those origins are allowed.
        // - Otherwise allow any localhost/127.0.0.1 origin (useful for local dev and ngrok previews).
        // - Always allow methods/headers and set a reasonable max_age.
        // Read optional env var with a comma-separated list of allowed origins (production)
        let allowed_origins = env::var("ALLOWED_ORIGINS").ok();

        // Simplified CORS policy: allow explicit dev origins, localhost variants,
        // ngrok subdomains and common public TLDs used in previews (.io, .app).
        // If ALLOWED_ORIGINS is set, only those exact origins are allowed.
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                if let Ok(origin_str) = origin.to_str() {
                    // If ALLOWED_ORIGINS provided, allow only exact matches from the list
                    if let Some(ref list) = allowed_origins {
                        return list.split(',').any(|o| o.trim() == origin_str);
                    }

                    // Allow common local dev origins (localhost / 127.0.0.1 with port)
                    if origin_str.starts_with("http://127.0.0.1:")
                        || origin_str.starts_with("http://localhost:")
                        || origin_str.starts_with("https://localhost:")
                    {
                        return true;
                    }

                    // Allow ngrok public domains and common preview TLDs (.io, .app)
                    if origin_str.ends_with(".ngrok.io")
                        || origin_str.ends_with(".ngrok-free.app")
                        || origin_str.ends_with(".ngrok.app")
                        || origin_str.ends_with(".io")
                        || origin_str.ends_with(".app")
                    {
                        return true;
                    }
                }
                false
            })
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
        // with fallback to default values.
        // PORT is the standard variable used by Fly.io and Railway.
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT")
            .or_else(|_| env::var("PORT"))
            .unwrap_or_else(|_| "8080".to_string());
        format!("{}:{}", host, port)
    })?
    .run()
    .await // Start the server and wait for it to complete
}
