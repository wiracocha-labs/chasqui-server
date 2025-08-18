use actix_crud::infrastructure::auth::jwt::{generate_token, hash_password, verify_password};
use jsonwebtoken::{decode, DecodingKey, Validation};
use surrealdb::sql::Thing;

#[test]
fn hash_and_verify_password() {
    let pwd = "Super$ecret123";
    let hashed = hash_password(pwd).expect("hash should succeed");
    assert!(verify_password(pwd, &hashed));
    assert!(!verify_password("wrong", &hashed));
}

#[derive(Debug, serde::Deserialize)]
struct TestClaims {
    sub: String,
    exp: i64,
}

#[test]
fn generate_and_decode_token() {
    // Ensure deterministic secret within this test
    std::env::set_var("SECRET_KEY", "testing_secret_key");

    let user_id = Thing::from(("user", "abc123"));
    let token = generate_token(&user_id).expect("token generation should succeed");

    let decoding_key = DecodingKey::from_secret(b"testing_secret_key");
    let data = decode::<TestClaims>(&token, &decoding_key, &Validation::default())
        .expect("token should decode");

    assert_eq!(data.claims.sub, user_id.to_string());
    assert!(data.claims.exp > chrono::Utc::now().timestamp());
}
