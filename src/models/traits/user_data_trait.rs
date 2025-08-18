use crate::models::entities::user::User;
use crate::infrastructure::database::surrealdb::Database;
use async_trait::async_trait;

// UserDataTrait defines the interface for user-related database operations
#[async_trait]
pub trait UserDataTrait {
    // Add a new user to the database
    async fn add_user(&self, new_user: User) -> Option<User>;
    // Find a user by their username
    async fn find_user_by_username(&self, username: &str) -> Option<User>;
}

// Implementation of UserDataTrait for the Database struct
#[async_trait]
impl UserDataTrait for Database {
    // Add a new user to the database
    async fn add_user(&self, new_user: User) -> Option<User> {
        println!("Attempting to add a new user...");
        let created_user = self
            .client
            .create("user")
            .content(&new_user)
            .await;
        
        match created_user {
            Ok(mut users) => {
                if let Some(user) = users.pop() {
                    println!("User created successfully.");
                    Some(user)
                } else {
                    println!("Failed to create user.");
                    None
                }
            },
            Err(e) => {
                println!("Error creating user: {:?}", e);
                None
            },
        }
    }

    // Find a user by their username
    async fn find_user_by_username(&self, username: &str) -> Option<User> {
        println!("Searching for user by username: {}", username);
        let result = self
            .client
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username))
            .await;
        
        match result {
            Ok(mut response) => {
                match response.take::<Vec<User>>(0) {
                    Ok(users) => users.into_iter().next(),
                    Err(e) => {
                        println!("Error deserializing user: {:?}", e);
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error searching for user: {:?}", e);
                None
            },
        }
    }
}