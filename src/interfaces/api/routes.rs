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
            .route(
                "/tasks",
                web::get().to(crate::interfaces::api::task_handlers::get_task),
            )
            // POST endpoint for adding a new task
            .route(
                "/tasks",
                web::post().to(crate::interfaces::api::task_handlers::add_task),
            )
            // PATCH endpoint for updating an existing task
            .route(
                "/tasks/{uuid}",
                web::patch().to(crate::interfaces::api::task_handlers::update_task),
            )
            // POST endpoint for user registration
            .route(
                "/register",
                web::post().to(crate::interfaces::api::user_handlers::register),
            )
            // POST endpoint for user login
            .route(
                "/login",
                web::post().to(crate::interfaces::api::user_handlers::login),
            )
            // GET endpoint to retrieve all users
            .route(
                "/users",
                web::get().to(crate::interfaces::api::user_handlers::get_all_users),
            )
            // DELETE endpoint to remove users with wallets
            .route(
                "/users/wallets",
                web::delete().to(crate::interfaces::api::user_handlers::delete_wallet_users),
            )
            // WebSocket endpoint for chat
            .route(
                "/ws/chat",
                web::get().to(crate::interfaces::api::chat_handlers::chat_ws),
            )
            // REST endpoints for chat
            .route(
                "/conversations",
                web::post().to(crate::interfaces::api::chat_handlers::create_conversation),
            )
            .route(
                "/conversations",
                web::get().to(crate::interfaces::api::chat_handlers::get_conversations),
            )
            .route(
                "/conversations/{id}/messages",
                web::get().to(crate::interfaces::api::chat_handlers::get_messages),
            )
            .route(
                "/conversations/{id}/participants",
                web::post().to(crate::interfaces::api::chat_handlers::add_participant),
            ),
    );
}
