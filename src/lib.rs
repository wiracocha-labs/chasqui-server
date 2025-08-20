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

/// Inicializa logging usando una única variable APP_LOG_LEVEL.
/// Valores válidos: error | warn | info | debug | trace.
/// Default: info.
pub fn init_logging_from_env() {
	use env_logger::Builder;
	use log::LevelFilter;
	use std::str::FromStr;

	let val = std::env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
	let lvl = LevelFilter::from_str(&val).unwrap_or(LevelFilter::Info);

	let mut builder = Builder::new();
	// Global y módulos clave al mismo nivel
	builder.filter_level(lvl);
	builder.filter_module("actix_web", lvl);
	builder.filter_module("actix_crud", lvl);
	let _ = builder.try_init();
}