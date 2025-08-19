//! Error Handling Module
//!
//! This module provides centralized error handling for the application.
//! It defines and manages all error types used throughout the system.
//!
//! # Module Structure
//! - `task_error`: Task-related error definitions
//!
//! # Usage
//! ```rust,ignore
//! use actix_crud::error::TaskError;
//! ```
//!
//! The error module is designed to:
//! - Provide consistent error handling
//! - Define domain-specific error types
//! - Enable error propagation
//! - Facilitate error reporting

// Import and re-export error types
pub mod task_error;
pub use task_error::TaskError;