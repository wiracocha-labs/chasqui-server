// Module declarations for the main components of the application

pub mod models;       // Data models and traits
pub mod interfaces;   // API interfaces and handlers
pub mod infrastructure; // Database and other infrastructure components
pub mod config;       // Application configuration
pub mod error;        // Error handling
pub mod application;  // Application logic and services

// This file serves as the root of the library crate, organizing the
// application's modules and making them publicly accessible.
// It provides a clear structure for the project and allows other
// parts of the application (or external crates) to import these modules.