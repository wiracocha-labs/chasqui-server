//! Routes configuration module
//! This module defines and configures all API routes for the application.

use actix_web::web;

/// Configures all API routes for the application
///
/// # Arguments
/// * `cfg` - Service configuration instance from Actix-web
///
/// # Route Structure
/// All routes are prefixed with '/api' and include:
/// - GET    /tasks       -> Retrieve all tasks
/// - POST   /tasks       -> Create a new task
/// - PATCH  /tasks/{uuid}-> Update an existing task
/// - POST   /register    -> Register a new user
/// - POST   /login       -> Authenticate a user
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // Create a scope for all API routes under /api prefix
        web::scope("/api")
            // GET endpoint for retrieving tasks
            .route("/tasks", web::get().to(crate::interfaces::api::task_handlers::get_task))
            // POST endpoint for adding a new task
            .route("/tasks", web::post().to(crate::interfaces::api::task_handlers::add_task))
            // PATCH endpoint for updating an existing task
            .route("/tasks/{uuid}", web::patch().to(crate::interfaces::api::task_handlers::update_task))
            // POST endpoint for user registration
            .route("/register", web::post().to(crate::interfaces::api::user_handlers::register))
            // POST endpoint for user login
            .route("/login", web::post().to(crate::interfaces::api::user_handlers::login))
    );
}
