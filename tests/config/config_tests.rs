//! Configuration Tests Module
//! Tests the application configuration loading functionality.

use actix_crud::config::app_config::AppConfig;

/// Test that configuration values are correctly read from environment variables
#[test]
fn from_env_reads_values() {
    // Set up test environment variables
    std::env::set_var(
        "DATABASE_URL",
        "surrealdb+ws://root:root@127.0.0.1:8002/surreal/task",
    );
    std::env::set_var("SECRET_KEY", "unit_test_secret");

    // Load configuration from environment
    let cfg = AppConfig::from_env();

    // Verify configuration values match expected values
    assert_eq!(
        cfg.database_url,
        "surrealdb+ws://root:root@127.0.0.1:8002/surreal/task"
    );
    assert_eq!(cfg.secret_key, "unit_test_secret");
}
