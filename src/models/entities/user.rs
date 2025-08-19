//! User Entity Module
//!
//! - `id`: SurrealDB Thing con esquema `user:<uuid-v4>` (se genera al crear).
//! - `username`: único.
//! - `password`: hash bcrypt; opcional para compatibilidad con filas legacy.
//! - `email`: opcional para compatibilidad con filas legacy.
//!
//! Seguridad:
//! - El constructor `User::new` aplica hash bcrypt (coste configurable vía BCRYPT_COST).
//! - No exponer el hash en respuestas públicas (usar DTO si es necesario).
//! - Validar formato de email y unicidad en la creación de usuarios.

use serde::{Deserialize, Serialize};
use crate::infrastructure::auth::jwt::hash_password;
use bcrypt::BcryptError;
use surrealdb::sql::Thing;
use uuid::Uuid;

/// Represents a User in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Database identifier (SurrealDB Thing). We'll store a UUID v4 as the record id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    /// Unique username for the user
    pub username: String,
    /// Hashed password string (optional for legacy rows)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Email address of the user (optional for compat with legacy rows)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl User {
    /// Creates a new User with:
    /// - id = Thing("user", <uuid-v4>)
    /// - password = bcrypt(password)
    /// - email stored
    ///
    /// Returns hashing errors from bcrypt on failure.
    pub fn new(username: String, email: String, password: String) -> Result<Self, BcryptError> {
        // Hash the provided password (propagate bcrypt error)
        let hashed_password = hash_password(&password)?;
        let uuid = Uuid::new_v4().to_string();
        Ok(User {
            id: Some(Thing::from(("user", uuid.as_str()))), // user:<uuid>
            username,
            password: Some(hashed_password),
            email: Some(email),
        })
    }
}