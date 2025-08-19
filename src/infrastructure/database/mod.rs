//! Database Infrastructure Module
//!
//! This module provides database connectivity and operations for the application.
//! It implements the database layer using SurrealDB as the backend.
//!
//! # Module Structure
//! - `surrealdb`: SurrealDB implementation and connection management
//!
//! # Usage
//! ```rust
//! use chasqui_server::infrastructure::database::surrealdb::Database;
//! ```
//!
//! The database infrastructure is designed to:
//! - Manage database connections
//! - Handle query operations
//! - Provide transaction support
//! - Enable database configuration

pub mod surrealdb;