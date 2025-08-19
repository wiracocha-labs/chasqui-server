//! API Module
//!
//! This module handles the HTTP API implementation including routes and handlers
//! for different resources in the application.
//!
//! # Module Structure
//! - `routes`: API route configuration and setup
//! - `task_handlers`: Task-related request handlers
//! - `user_handlers`: User-related request handlers
//!
//! # Usage
//! ```rust
//! use chasqui_server::interfaces::api::{routes, task_handlers, user_handlers};
//! ```
//!
//! The API module is designed to:
//! - Define HTTP endpoints
//! - Handle API requests and responses
//! - Implement resource handlers
//! - Manage API routing logic

pub mod routes;
pub mod task_handlers;
pub mod user_handlers;