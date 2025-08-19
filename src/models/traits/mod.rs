//! Traits Module
//!
//! This module defines the core traits that establish contracts and behaviors
//! for the domain models and entities.
//!
//! # Module Structure
//! - `task_data_trait`: Trait definitions for task-related behaviors
//! - `user_data_trait`: Trait definitions for user-related behaviors
//!
//! # Usage
//! ```rust
//! use chasqui_server::models::traits::{task_data_trait, user_data_trait};
//! ```
//!
//! The traits module is designed to:
//! - Define common behaviors
//! - Establish interface contracts
//! - Enable polymorphic operations
//! - Support dependency inversion

pub mod task_data_trait;
pub mod user_data_trait;
