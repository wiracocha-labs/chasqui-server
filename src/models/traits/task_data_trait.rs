//! Task Data Trait Module
//! Defines the interface for task-related database operations.

use crate::models::entities::task::Task;
use crate::infrastructure::database::surrealdb::Database;
use surrealdb::Error;
use async_trait::async_trait;
use log::{info, debug, warn, error}; // añadido

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
        info!("Tasks: retrieving all");
        let result = self.client.select("task").await;
        match result {
            Ok(all_tasks) => {
                info!("Tasks: retrieved {}", all_tasks.len());
                Some(all_tasks)
            },
            Err(e) => {
                error!("Tasks: error retrieving all -> {:?}", e);
                None
            },
        }
    }

    // Add a new task to the database
    async fn add_task(&self, new_task: Task) -> Option<Task> {
        // Guardar datos antes de mover new_task
        let uuid_dbg = new_task.uuid.clone();
        let name_dbg = new_task.task_name.clone();
        info!("Tasks: add start uuid={} name={}", uuid_dbg, name_dbg);

        let created_task = self
            .client
            .create(("task", uuid_dbg.clone()))
            .content(new_task)
            .await;

        match created_task {
            Ok(created) => {
                if created.is_some() {
                    info!("Tasks: add success uuid={}", uuid_dbg);
                } else {
                    warn!("Tasks: add returned None uuid={}", uuid_dbg);
                }
                created
            },
            Err(e) => {
                error!("Tasks: add DB error uuid={} -> {:?}", uuid_dbg, e);
                None
            },
        }
    }

    // Update an existing task in the database
    async fn update_task(&self, uuid: String) -> Option<Task> {
        debug!("Tasks: update check uuid={}", uuid);
        let find_task: Result<Option<Task>, Error> = self.client.select(("task", &uuid)).await;

        match find_task {
            Ok(found) => {
                match found {
                    Some(_found_task) => {
                        debug!("Tasks: updating uuid={}", uuid);
                        let updated_task: Result<Option<Task>, Error> = self
                            .client
                            .update(("task", &uuid))
                            .merge(Task {
                                uuid: uuid.clone(),
                                task_name: String::from("Completed")
                            })
                            .await;
                        match updated_task {
                            Ok(updated) => {
                                if updated.is_some() {
                                    info!("Tasks: update success uuid={}", uuid);
                                } else {
                                    warn!("Tasks: update returned None uuid={}", uuid);
                                }
                                updated
                            },
                            Err(e) => {
                                error!("Tasks: update DB error uuid={} -> {:?}", uuid, e);
                                None
                            },
                        }
                    },
                    None => {
                        warn!("Tasks: not found uuid={}", uuid);
                        None
                    },
                }
            },
            Err(e) => {
                error!("Tasks: find DB error uuid={} -> {:?}", uuid, e);
                None
            },
        }
    }
}
