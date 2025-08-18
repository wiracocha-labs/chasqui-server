use actix_web::web;

// Configuration function for setting up API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // Create a scope for all API routes
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
