//! JWT Authentication Tests Module
//! Tests password hashing and JWT token generation/validation.

use actix_crud::infrastructure::auth::jwt::{generate_token, hash_password, verify_password};
use jsonwebtoken::{decode, DecodingKey, Validation};
use surrealdb::sql::Thing;

/// Test password hashing and verification functionality
#[test]
fn hash_and_verify_password() {
    let pwd = "Super$ecret123";
    // Test successful password hashing and verification
    let hashed = hash_password(pwd).expect("hash should succeed");
    assert!(verify_password(pwd, &hashed));
    // Test failed verification with wrong password
    assert!(!verify_password("wrong", &hashed));
}

/// Claims structure for JWT token validation
#[derive(Debug, serde::Deserialize)]
struct TestClaims {
    sub: String,  // Subject (user ID)
    exp: i64,     // Expiration timestamp
}

/// Test JWT token generation and decoding
#[test]
fn generate_and_decode_token() {
    // Set a fixed secret key for testing
    std::env::set_var("SECRET_KEY", "testing_secret_key");

    // Create a test user ID and generate token
    let user_id = Thing::from(("user", "abc123"));
    let token = generate_token(&user_id).expect("token generation should succeed");

    // Decode and validate the token
    let decoding_key = DecodingKey::from_secret(b"testing_secret_key");
    let data = decode::<TestClaims>(&token, &decoding_key, &Validation::default())
        .expect("token should decode");

    // Verify token claims
    assert_eq!(data.claims.sub, user_id.to_string());
    assert!(data.claims.exp > chrono::Utc::now().timestamp());
}
