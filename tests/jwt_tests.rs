//! JWT Authentication Tests Module
//! Tests password hashing and JWT token generation/validation.

use actix_crud::infrastructure::auth::jwt::{generate_token, hash_password, verify_password};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

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
    sub: String,       // Subject (user ID)
    exp: usize,        // Expiration timestamp
    iat: usize,        // Issued-at timestamp
    username: String,  // Username
    roles: Vec<String> // Roles
}

/// Test JWT token generation and decoding
#[test]
fn generate_and_decode_token() {
    // Set a fixed secret key for testing
    std::env::set_var("SECRET_KEY", "testing_secret_key");

    // Create a test user ID (UUID string) and generate token
    let user_id = Uuid::new_v4().to_string();
    let username = "john_doe".to_string();
    let roles = vec!["user".to_string()];

    let token = generate_token(&user_id, &username, &roles).expect("token generation should succeed");

    // Decode and validate the token
    let decoding_key = DecodingKey::from_secret(b"testing_secret_key");
    let data = decode::<TestClaims>(&token, &decoding_key, &Validation::default())
        .expect("token should decode");

    // Verify token claims
    assert_eq!(data.claims.sub, user_id);
    assert_eq!(data.claims.username, username);
    assert_eq!(data.claims.roles, roles);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;
    assert!(data.claims.exp > now);
    assert!(data.claims.iat <= now);
}
