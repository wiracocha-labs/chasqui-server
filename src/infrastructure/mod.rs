//! Infrastructure Module
//!
//! This module provides core infrastructure implementations and services
//! for database access, authentication, and logging.
//!
//! # Module Structure
//! - `database`: Database connection and query implementations
//! - `auth`: Authentication and authorization services
//! - `logging`: System-wide logging facilities
//!
//! # Usage
//! ```rust
//! use chasqui_server::infrastructure::{database, auth, logging};
//! ```
//!
//! The infrastructure layer is designed to:
//! - Manage database connections
//! - Handle authentication flows
//! - Provide logging capabilities
//! - Support core system services

pub mod database;
pub mod auth;
pub mod logging;