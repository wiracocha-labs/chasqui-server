//! Models Module
//!
//! This module organizes the core domain model components of the application.
//! It provides entity definitions and trait implementations.
//!
//! # Module Structure
//! - `entities`: Domain entities and value objects
//! - `traits`: Core behavior definitions
//!
//! # Usage
//! ```rust
//! use chasqui_server::models::{Task, User};
//! use chasqui_server::models::traits::TaskDataTrait;
//! ```
//!
//! The models module is designed to:
//! - Centralize domain model definitions
//! - Provide easy access to common types
//! - Organize related model components
//! - Support clean architecture principles

// Core domain model modules
pub mod entities;
pub mod traits;

// Re-export commonly used types
pub use entities::task::Task;
pub use entities::user::User;