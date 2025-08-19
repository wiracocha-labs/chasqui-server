//! User Data Trait Module
//! Defines the interface for user-related database operations.

use crate::models::entities::user::User;
use crate::infrastructure::database::surrealdb::Database;
use async_trait::async_trait;

/// Defines the interface for user-related database operations
#[async_trait]
pub trait UserDataTrait {
    /// Adds a new user to the database
    /// 
    /// # Arguments
    /// * `new_user` - The user to be added
    ///
    /// # Returns
    /// * `Option<User>` - Some(user) if created, None if error
    async fn add_user(&self, new_user: User) -> Option<User>;

    /// Finds a user by their username
    /// 
    /// # Arguments
    /// * `username` - The username to search for
    ///
    /// # Returns
    /// * `Option<User>` - Some(user) if found, None if not found or error
    async fn find_user_by_username(&self, username: &str) -> Option<User>;
}

// Implementation of UserDataTrait for the Database struct
#[async_trait]
impl UserDataTrait for Database {
    // Add a new user to the database
    async fn add_user(&self, new_user: User) -> Option<User> {
        println!("Attempting to add a new user...");
        // Create a new user record in the database
        // SurrealDB will automatically generate a unique ID
        let created_user = self
            .client
            .create("user") // Create in 'user' table
            .content(&new_user) // Set the record content
            .await;
        
        // Handle the database response
        match created_user {
            Ok(mut users) => {
                // created_user returns a vector, we want the first (and only) user
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
        // Execute a parameterized query to find user by username
        // This prevents SQL injection by using bind parameters
        let result = self
            .client
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username)) // Bind the parameter safely
            .await;
        
        // Process the query result
        match result {
            Ok(mut response) => {
                // Attempt to extract User objects from the response
                match response.take::<Vec<User>>(0) {
                    Ok(users) => users.into_iter().next(), // Return the first user found
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