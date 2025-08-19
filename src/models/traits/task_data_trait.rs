//! Task Data Trait Module
//! Defines the interface for task-related database operations.

use crate::models::entities::task::Task;
use crate::infrastructure::database::surrealdb::Database;
use surrealdb::Error;
use async_trait::async_trait;

/// Defines the interface for task-related database operations
#[async_trait]
pub trait TaskDataTrait {
    /// Retrieves all tasks from the database
    /// 
    /// # Returns
    /// * `Option<Vec<Task>>` - Some(tasks) if found, None if error or no tasks
    async fn get_all_tasks(&self) -> Option<Vec<Task>>;
    
    /// Adds a new task to the database
    /// 
    /// # Arguments
    /// * `new_task` - The task to be added
    ///
    /// # Returns
    /// * `Option<Task>` - Some(task) if created, None if error
    async fn add_task(&self, new_task: Task) -> Option<Task>;
    
    /// Updates an existing task in the database
    /// 
    /// # Arguments
    /// * `uuid` - Unique identifier of the task to update
    ///
    /// # Returns
    /// * `Option<Task>` - Some(task) if updated, None if not found or error
    async fn update_task(&self, uuid: String) -> Option<Task>;
}

// Implementation of TaskDataTrait for the Database struct
#[async_trait]
impl TaskDataTrait for Database {
    // Retrieve all tasks from the database
    async fn get_all_tasks(&self) -> Option<Vec<Task>> {
        println!("Attempting to retrieve all tasks...");
        // Execute a SELECT query on the "task" table to retrieve all tasks
        // This uses SurrealDB's built-in select operation
        let result = self.client.select("task").await;
        
        // Handle the query result, returning None if there's an error
        match result {
            Ok(all_tasks) => {
                println!("Tasks retrieved successfully.");
                Some(all_tasks)
            },
            Err(e) => {
                println!("Error retrieving tasks: {:?}", e);
                None
            },
        }
    }

    // Add a new task to the database
    async fn add_task(&self, new_task: Task) -> Option<Task> {
        println!("Attempting to add a new task...");
        // Create a new task record with the specified UUID
        // The create operation takes a tuple of (table_name, record_id)
        // content() method sets the record's content to our new_task
        let created_task = self
            .client
            .create(("task", new_task.uuid.clone()))
            .content(new_task)
            .await;

        // Handle the creation result
        match created_task {
            Ok(created) => {
                println!("Task created successfully.");
                created
            },
            Err(e) => {
                println!("Error creating task: {:?}", e);
                None
            },
        }
    }

    // Update an existing task in the database
    async fn update_task(&self, uuid: String) -> Option<Task> {
        // First, check if the task exists in the database
        // Using select with a specific record ID (uuid)
        let find_task: Result<Option<Task>, Error> = self.client.select(("task", &uuid)).await;

        match find_task {
            Ok(found) => {
                match found {
                    Some(_found_task) => {
                        // If task exists, update it using merge operation
                        // merge() combines existing data with new data
                        let updated_task: Result<Option<Task>, Error> = self
                            .client
                            .update(("task", &uuid))
                            .merge(Task {
                                uuid,
                                task_name: String::from("Completed") // Mark as completed
                            })
                            .await;
                        // Return the updated task or None if update failed
                        match updated_task {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    },
                    None => None, // Task not found
                }
            },
            Err(_) => None, // Database error
        }
    }
}
