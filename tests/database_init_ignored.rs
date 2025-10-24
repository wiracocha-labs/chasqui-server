//! Database Integration Test Module
//! Tests database connectivity with a running SurrealDB instance.

use actix_crud::infrastructure::database::surrealdb::Database;

/// Test database connection initialization
/// This test requires a running SurrealDB instance and is ignored by default.
/// Run with: `cargo test -- --ignored`
#[test]
#[ignore]
fn database_init_connects() {
    // Configure database connection parameters with defaults
    std::env::set_var("SURREALDB_HOST", std::env::var("SURREALDB_HOST").unwrap_or_else(|_| "127.0.0.1:8002".into()));
    std::env::set_var("SURREALDB_USER", std::env::var("SURREALDB_USER").unwrap_or_else(|_| "root".into()));
    std::env::set_var("SURREALDB_PASS", std::env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".into()));
    std::env::set_var("SURREALDB_NAMESPACE", std::env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "surreal".into()));
    std::env::set_var("SURREALDB_DATABASE", std::env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "task".into()));

    // Attempt to initialize database connection
    let res = futures::executor::block_on(Database::init());
    assert!(res.is_ok(), "Database::init should succeed when SurrealDB is available: {:?}", res.err());
}
