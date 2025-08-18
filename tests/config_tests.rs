use actix_crud::config::app_config::AppConfig;

#[test]
fn from_env_reads_values() {
    std::env::set_var(
        "DATABASE_URL",
        "surrealdb+ws://root:root@127.0.0.1:8002/surreal/task",
    );
    std::env::set_var("SECRET_KEY", "unit_test_secret");

    let cfg = AppConfig::from_env();

    assert_eq!(
        cfg.database_url,
        "surrealdb+ws://root:root@127.0.0.1:8002/surreal/task"
    );
    assert_eq!(cfg.secret_key, "unit_test_secret");
}
