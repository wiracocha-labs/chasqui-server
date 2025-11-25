//! Test utilities and helpers
use crate::mocks::user_repository::MockUserRepository;
use actix_crud::models::entities::user::User;


/// Create a test user with default values
pub fn create_test_user() -> User {
    User::new(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "password123".to_string()
    ).expect("Failed to create test user")
}

/// Create a mock user repository with default expectations
pub fn create_mock_user_repository() -> MockUserRepository {
    let mock = MockUserRepository::new();
    // Configura los mocks por defecto aquí
    mock
}