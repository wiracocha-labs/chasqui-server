//! Authentication helpers: password hashing/verification and JWT creation.
//!
//! This module provides utility functions to:
//! - Hash and verify passwords using `bcrypt`.
//! - Generate JSON Web Tokens (JWT) with a configurable expiration.
//!
//! Security notes:
//! - SECRET_KEY debe estar definido en el entorno (es obligatorio). Si falta, se retorna error.
//! - Ajusta JWT_EXP_SECONDS (segundos) para cambiar la expiración del token.
//! - Ajusta BCRYPT_COST para controlar el “work factor” del hash.
//!
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::errors::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT payload (claims) used by this application.
///
/// Fields:
/// - `sub`: Subject — the user identifier (e.g., UUID).
/// - `exp`: Expiration as a Unix timestamp (seconds since epoch).
/// - `iat`: Issued-at as a Unix timestamp (seconds since epoch).
/// - `username`: Username of the user.
/// - `roles`: List of roles.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    username: String,
    roles: Vec<String>,
}

/// Hash a plaintext password using `bcrypt`.
///
/// Returns the password hash on success.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
	// Permitir configurar el coste via BCRYPT_COST, sino usar DEFAULT_COST
	let cost = env::var("BCRYPT_COST")
		.ok()
		.and_then(|v| v.parse::<u32>().ok())
		.unwrap_or(DEFAULT_COST);
	hash(password, cost)
}

/// Verify a plaintext password against a previously stored `bcrypt` hash.
///
/// Returns `true` if the password matches, otherwise `false`.
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

/// Generate a signed JWT for the given user identifier and metadata.
///
/// The token uses a default header and includes:
/// - `sub`: the user_id (UUID string)
/// - `exp`: expiration set from `JWT_EXP_SECONDS` (default 24h if no env var)
/// - `iat`: current timestamp
/// - `username`: provided username
/// - `roles`: provided roles
///
/// SECRET_KEY must be set in the environment.
pub fn generate_token(user_id: &str, username: &str, roles: &[String]) -> Result<String, Error> {
    // Leer SECRET_KEY obligatoriamente (sin fallback)
    let secret = env::var("SECRET_KEY").map_err(|_| Error::from(ErrorKind::InvalidToken))?;

    // Timestamps (seconds since epoch)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // Expiración configurable via JWT_EXP_SECONDS, por defecto 24h
    let exp_secs = env::var("JWT_EXP_SECONDS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(24 * 3600);
    let exp = now + exp_secs;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat: now,
        username: username.to_string(),
        roles: roles.to_vec(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}