//! Authentication infrastructure module root.
//!
//! Groups authentication-related helpers (e.g., JWT, password hashing). The
//! submodules are exposed so other layers (API handlers, services, tests) can
//! use them.
//!
//! Example import:
//! `use crate::infrastructure::auth::jwt;`
 
pub mod jwt; // Expose JWT-related helpers