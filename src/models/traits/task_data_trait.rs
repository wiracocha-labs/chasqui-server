use crate::models::entities::task::Task;
use crate::infrastructure::database::surrealdb::Database;
// use actix_web::web::Data;
use surrealdb::Error;
use async_trait::async_trait;

// TaskDataTrait defines the interface for task-related database operations
#[async_trait]
pub trait TaskDataTrait {
    // Retrieve all tasks from the database
    async fn get_all_tasks(&self) -> Option<Vec<Task>>;
    
    // Add a new task to the database
    async fn add_task(&self, new_task: Task) -> Option<Task>;
    
    // Update an existing task in the database
    async fn update_task(&self, uuid: String) -> Option<Task>;
}

// Implementation of TaskDataTrait for the Database struct
#[async_trait]
impl TaskDataTrait for Database {
    // Retrieve all tasks from the database
    async fn get_all_tasks(&self) -> Option<Vec<Task>> {
        println!("Attempting to retrieve all tasks...");
        let result = self.client.select("task").await;
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
        let created_task = self
            .client
            .create(("task", new_task.uuid.clone()))
            .content(new_task)
            .await;
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
        let find_task: Result<Option<Task>, Error> = self.client.select(("task", &uuid)).await;

        match find_task {
            Ok(found) => {
                match found {
                    Some(_found_task) => {
                        let updated_task: Result<Option<Task>, Error> = self
                            .client
                            .update(("task", &uuid))
                            .merge(Task {
                                uuid,
                                task_name: String::from("Completed")
                            })
                            .await;
                        match updated_task {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
                    
            },
            Err(_) => None,
        }
    }
}
