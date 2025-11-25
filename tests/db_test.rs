 /// Initializes a test database connection using TEST_ prefixed env vars.
    /// 
    /// This method is only available in test builds and uses separate configuration
    /// to avoid interfering with production/development databases.
    ///
    /// # Errors
    /// Returns `Err` if any step fails (connection, authentication, or selection).
    #[test]
    pub async fn init_for_tests() -> Result<Self, Error> {
        info!("SurrealDB: starting TEST connection");
        
        let host = env::var("TEST_SURREALDB_HOST")
            .unwrap_or_else(|_| "ws://127.0.0.1:8002".to_string());
        
        info!("SurrealDB TEST: connecting to {}", host);
        let client = surrealdb::engine::any::connect(&host).await?;
        
        let username = env::var("TEST_SURREALDB_USER")
            .unwrap_or_else(|_| "root".to_string());
        let password = env::var("TEST_SURREALDB_PASS")
            .unwrap_or_else(|_| "root".to_string());
        
        client
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;
        
        let namespace = env::var("TEST_SURREALDB_NAMESPACE")
            .unwrap_or_else(|_| "test_surreal".to_string());
        let db_name = env::var("TEST_SURREALDB_DATABASE")
            .unwrap_or_else(|_| "test_task".to_string());
        
        client.use_ns(&namespace).use_db(&db_name).await?;
        
        info!("SurrealDB TEST: ready (ns={}, db={})", namespace, db_name);
        
        Ok(Database {
            client,
            namespace,
            db_name,
        })
    }