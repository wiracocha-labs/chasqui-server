//! Interfaces Module
//!
//! This module contains the external interfaces and data access layers for the application.
//! It provides API endpoints and repository interfaces for data operations.
//!
//! # Module Structure
//! - `api`: Contains HTTP API routes, handlers and endpoint definitions
//! - `repositories`: Data access interfaces and concrete implementations
//!
//! # Usage
//! ```rust
//! use chasqui_server::interfaces::{api, repositories};
//! ```
//!
//! The interfaces layer is designed to:
//! - Handle HTTP requests/responses
//! - Define repository contracts
//! - Implement data access patterns
//! - Manage API versioning and routing

pub mod api;
pub mod repositories;