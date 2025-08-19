//! Entities Module
//!
//! This module defines the core domain entities and their relationships
//! for the application.
//!
//! # Module Structure
//! - `task`: Task entity definitions and related types
//! - `user`: User entity definitions and related types
//!
//! # Usage
//! ```rust,ignore
//! use actix_crud::models::entities::{task, user};
//! ```
//!
//! The entities module is designed to:
//! - Define core domain models
//! - Establish entity relationships
//! - Provide data structures
//! - Define entity validation rules

pub mod task;
pub mod user;