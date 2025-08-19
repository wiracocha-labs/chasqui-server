//! Authentication helpers: password hashing/verification and JWT creation.
//!
//! This module provides utility functions to:
//! - Hash and verify passwords using `bcrypt`.
//! - Generate JSON Web Tokens (JWT) with a 24-hour expiration.
//!
//! Security notes:
//! - Ensure `SECRET_KEY` is set in the environment for production. The fallback
//!   default ("change_me_please") is only for development/testing and should be
//!   replaced with a strong, random secret.
//!
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use std::env;

/// JWT payload (claims) used by this application.
///
/// Fields:
/// - `sub`: Subject — the user identifier (e.g., database ID).
/// - `exp`: Expiration as a Unix timestamp (seconds since epoch).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    sub: String,
    /// Expiration time (Unix timestamp, seconds)
    exp: i64,
}

/// Hash a plaintext password using `bcrypt` with `DEFAULT_COST`.
///
/// Returns the password hash on success.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verify a plaintext password against a previously stored `bcrypt` hash.
///
/// Returns `true` if the password matches, otherwise `false`.
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

/// Generate a signed JWT for the given user identifier.
///
/// The token uses a default header and includes:
/// - `sub`: the stringified `user_id`.
/// - `exp`: expiration set to 24 hours from the current time.
///
/// The secret is read from the `SECRET_KEY` environment variable; if missing,
/// it falls back to "change_me_please" (not suitable for production).
pub fn generate_token(user_id: &Thing) -> Result<String, jsonwebtoken::errors::Error> {
    // Set token expiration to 24 hours from now
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Valid timestamp")
        .timestamp();

    // Create claims for the token
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    // Create a default header
    let header = Header::default();
    // Create an encoding key from a secret
    // Read SECRET_KEY from environment variables
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "change_me_please".to_string());
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    // Encode the JWT
    encode(&header, &claims, &encoding_key)
}
