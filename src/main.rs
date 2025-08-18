use actix_web::{HttpServer, App, web, middleware::Logger};
use actix_crud::infrastructure::database::surrealdb::Database;
use actix_crud::interfaces::api::routes;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env if present
    dotenv::dotenv().ok();
    // Initialize the logger with default settings
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Starting the application...");
    // Initialize the database connection
    let db = Database::init().await.expect("Error connecting to database");
    println!("Database connection established.");
    // Wrap the database connection in a web::Data for sharing across threads
    let db_data = web::Data::new(db);

    println!("Starting the HTTP server...");
    // Create and run the HTTP server
    HttpServer::new(move || {
        App::new()
            // Add logger middleware
            .wrap(Logger::default())
            // Share the database connection
            .app_data(db_data.clone())
            // Configure the API routes
            .configure(routes::config)
    })
    .bind({
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
        format!("{}:{}", host, port)
    })?  // Bind the server using environment variables
    .run()
    .await
}