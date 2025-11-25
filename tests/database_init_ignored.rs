//! Database Integration Test Module
//! Tests database connectivity with a running SurrealDB instance.
//!
//! Tests use a separate configuration with TEST_ prefixed environment variables
//! to avoid interfering with production/development databases.

use actix_crud::infrastructure::database::surrealdb::Database;

/// Test database connection initialization
/// This test requires a running local SurrealDB instance and is ignored by default.
/// 
/// To run this test:
/// 1. Start local SurrealDB: `surreal start --bind 127.0.0.1:8002 --user root --pass root`
/// 2. Run test: `cargo test -- --ignored`
#[test]
#[ignore]
fn database_init_connects() {
    // Configure TEST database connection parameters
    std::env::set_var("TEST_SURREALDB_HOST", 
        std::env::var("TEST_SURREALDB_HOST").unwrap_or_else(|_| "ws://127.0.0.1:8002".into()));
    std::env::set_var("TEST_SURREALDB_USER", 
        std::env::var("TEST_SURREALDB_USER").unwrap_or_else(|_| "root".into()));
    std::env::set_var("TEST_SURREALDB_PASS", 
        std::env::var("TEST_SURREALDB_PASS").unwrap_or_else(|_| "root".into()));
    std::env::set_var("TEST_SURREALDB_NAMESPACE", 
        std::env::var("TEST_SURREALDB_NAMESPACE").unwrap_or_else(|_| "test_surreal".into()));
    std::env::set_var("TEST_SURREALDB_DATABASE", 
        std::env::var("TEST_SURREALDB_DATABASE").unwrap_or_else(|_| "test_task".into()));

    // Attempt to initialize test database connection
    let res = futures::executor::block_on(Database::init_for_tests());
    assert!(res.is_ok(), "Database::init_for_tests should succeed when local SurrealDB is available: {:?}", res.err());
}
