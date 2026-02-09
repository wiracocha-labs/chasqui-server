//! Infrastructure Module
//!
//! This module provides core infrastructure implementations and services
//! for database access, authentication, logging, and WebSocket communication.
//!
//! # Module Structure
//! - `database`: Database connection and query implementations
//! - `auth`: Authentication and authorization services
//! - `logging`: System-wide logging facilities
//! - `websocket`: WebSocket infrastructure for real-time chat
//!
//! # Usage
//! ```rust,ignore
//! use actix_crud::infrastructure::{database, auth, logging, websocket};
//! ```
//!
//! The infrastructure layer is designed to:
//! - Manage database connections
//! - Handle authentication flows
//! - Provide logging capabilities
//! - Support core system services
//! - Enable real-time communication

pub mod auth;
pub mod database;
pub mod logging;
pub mod websocket;
