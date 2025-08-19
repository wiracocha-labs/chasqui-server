//! Configuration layer root.
//!
//! This module groups configuration-related code for the application and
//! publicly exposes submodules so other layers (e.g., application services,
//! infrastructure, tests) can access configuration types and helpers.
//!
//! Example import:
//! `use crate::config::app_config::AppConfig;`
 
pub mod app_config; // Expose the `app_config` submodule to the rest of the crate