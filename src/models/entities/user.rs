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


use crate::infrastructure::auth::jwt::hash_password;
use crate::models::entities::role::roles;
use crate::models::entities::role::{Role, Permission};
use serde::{Deserialize, Serialize};
use bcrypt::BcryptError;
use surrealdb::sql::Thing;
use uuid::Uuid;
use log::debug;
use std::collections::HashSet;


/// Represents a User in the system
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    /// Roles del usuario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role>,
}

impl User {
    /// Creates a new User with:
    /// - id = Thing("user", <uuid-v4>)
    /// - password = bcrypt(password)
    /// - email stored
    ///
    /// Returns hashing errors from bcrypt on failure.
    pub fn new(username: String, email: String, password: String) -> Result<Self, BcryptError> {
        debug!("User::new creating user with username={} email={}", username, email);
        // Hash the provided password (propagate bcrypt error)
        let hashed_password = hash_password(&password)?;
        let uuid = Uuid::new_v4().to_string();

        let mut user = User {
            id: Some(Thing::from(("user", uuid.as_str()))),
            username,
            password: Some(hashed_password),
            email: Some(email),
            roles: Vec::new(),
        };

        user.add_role(roles::user());

        Ok(user)
    }
    
    //--------- Roles Methods ---------

    pub fn has_role(&self, role_name: &str) -> bool {
        self.roles.iter().any(|r| r.name == role_name)
    }

    pub fn add_role(&mut self, role: Role) -> bool {
            if !self.has_role(&role.name) {
                self.roles.push(role);
                true
            } else {
                false
            }
        }

    pub fn remove_role(&mut self, role_name: &str) -> bool {
        let original_len = self.roles.len();
        self.roles.retain(|r| r.name != role_name);
        self.roles.len() != original_len
    }

    pub fn has_all_roles(&self, role_names: &[&str]) -> bool {
        let user_roles: HashSet<_> = self.roles.iter().map(|r| r.name.as_str()).collect();
        role_names.iter().all(|&r| user_roles.contains(r))
    }

    pub fn has_any_role(&self, role_names: &[&str]) -> bool {
        let user_roles: HashSet<_> = self.roles.iter().map(|r| r.name.as_str()).collect();
        role_names.iter().any(|&r| user_roles.contains(r))
    }

    pub fn has_permission(&self, permission: Permission) -> bool {
        self.roles.iter().any(|role| role.has_permission(permission))
    }

    pub fn has_all_permissions(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|&p| self.has_permission(p))
    }

    pub fn has_any_permission(&self, permissions: &[Permission]) -> bool {
        permissions.iter().any(|&p| self.has_permission(p))
    }

    //--------Convenience Methods--------

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    pub fn is_moderator(&self) -> bool {
        self.has_role("moderator")
    }

    pub fn is_standard_user(&self) -> bool {
        self.has_role("user") && !self.is_admin() && !self.is_moderator()
    }

}

    