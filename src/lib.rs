//! Chasqui Server Library
//!
//! This is the root module of the Chasqui Server application, implementing
//! a webhook handling and data processing system.
//!
//! # Module Structure
//! - `models`: Core domain models and traits
//! - `interfaces`: API endpoints and data access
//! - `infrastructure`: Technical implementations
//! - `config`: Application configuration
//! - `error`: Error handling system
//! - `application`: Business logic and services
//!
//! # Architecture
//! The application follows clean architecture principles with layers:
//! - Domain (models)
//! - Application (services)
//! - Infrastructure (technical details)
//! - Interfaces (API and repositories)

pub mod models;
pub mod interfaces;
pub mod infrastructure;
pub mod config;
pub mod error;
pub mod application;