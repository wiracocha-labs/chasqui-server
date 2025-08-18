use serde::Deserialize;

// AppConfig struct to hold application configuration
#[derive(Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
}

impl AppConfig {
    // Load configuration from environment variables
    pub fn from_env() -> Self {
        // Load environment variables from .env file if present
        dotenv::dotenv().ok();
        
        // Retrieve DATABASE_URL from environment variables
        // Panic if not set
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        // Retrieve SECRET_KEY from environment variables
        // Panic if not set
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        
        // Create and return AppConfig instance
        AppConfig { database_url, secret_key }
    }
}
