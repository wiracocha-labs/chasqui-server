//! Application Module
//!
//! This module contains the core business logic and use cases of the application.
//! It serves as the main application layer in the clean architecture pattern.
//!
//! # Module Structure
//! - `services`: Contains core business logic and use case implementations
//!
//! # Ejemplo
//! ```rust,ignore
//! use actix_crud::application::services;
//! // ...example usage...
//! ```
//!
//! The application layer is designed to:
//! - Implement core business logic
//! - Define use cases
//! - Coordinate between different layers
//! - Maintain separation of concerns

pub mod services;
