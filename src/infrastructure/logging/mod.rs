//! Logging Infrastructure Module
//!
//! This module provides logging functionality for the Chasqui Server application.
//! It contains middleware and utilities for handling logging across the system.
//!
//! # Module Structure
//! - `middleware`: Contains logging middleware components for HTTP request/response logging
//!   and other logging interceptors
//!
//! # Usage
//! ```rust
//! use chasqui_server::infrastructure::logging;
//! ```
//!
//! The logging infrastructure is designed to:
//! - Provide structured logging capabilities
//! - Handle request/response logging
//! - Support different logging levels
//! - Enable logging customization through middleware

pub mod middleware;