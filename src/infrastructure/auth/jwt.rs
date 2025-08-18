use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use std::env;

// Claims struct represents the payload of the JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,  // Subject (user ID)
    exp: i64,     // Expiration time
}

// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

// Generate a JWT for a given user ID
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
