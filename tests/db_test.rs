use actix_crud::infrastructure::database::surrealdb::Database;
use std::env;
use surrealdb::engine::any;
use surrealdb::opt::auth::Root;

#[actix_rt::test]
async fn test_database_connection() {
    // Configure TEST database connection parameters if not already set
    // In a real scenario, these might come from a .env file loaded by dotenv
    // or be set in the CI environment.
    let host =
        env::var("TEST_SURREALDB_HOST").unwrap_or_else(|_| "ws://127.0.0.1:8002".to_string());

    let username = env::var("TEST_SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
    let password = env::var("TEST_SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());
    let namespace =
        env::var("TEST_SURREALDB_NAMESPACE").unwrap_or_else(|_| "test_surreal".to_string());
    let db_name = env::var("TEST_SURREALDB_DATABASE").unwrap_or_else(|_| "test_task".to_string());

    println!("Connecting to SurrealDB at {}...", host);

    // 1. Connect
    let client_result = any::connect(&host).await;

    match client_result {
        Ok(client) => {
            // 2. Sign in
            let signin_result = client
                .signin(Root {
                    username: &username,
                    password: &password,
                })
                .await;

            if let Err(e) = signin_result {
                panic!("Failed to sign in to SurrealDB: {:?}", e);
            }

            // 3. Select Namespace/Database
            let use_result = client.use_ns(&namespace).use_db(&db_name).await;
            if let Err(e) = use_result {
                panic!("Failed to select namespace/database: {:?}", e);
            }

            // 4. Create Database struct (manually since we can't use the private fields if we wanted to,
            // but here we are just testing connectivity so we can just assert success)
            // Note: We can't construct `Database` directly if its fields are private or if we don't want to duplicate logic too much.
            // However, the user asked to test if it connects. We have proven it connects above.

            // If we really wanted to return a Database struct, we would need to make sure we can construct it.
            // Checking src/infrastructure/database/surrealdb.rs...
            // The fields `client`, `namespace`, `db_name` are public!

            let db = Database {
                client,
                namespace: namespace.clone(),
                db_name: db_name.clone(),
            };

            println!("Successfully connected to {}/{}", db.namespace, db.db_name);
            assert_eq!(db.namespace, namespace);
            assert_eq!(db.db_name, db_name);
        }
        Err(e) => {
            // If we can't connect, we panic to fail the test
            // This is expected if no local DB is running
            panic!("Failed to connect to test database: {:?}", e);
        }
    }
}
