//! Task Entity Module
//! Defines the core task-related data structures and their behavior.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request payload for creating a new task
#[derive(Validate, Serialize, Deserialize)]
pub struct AddTaskRequest {
    /// Name of the task, must not be empty
    #[validate(length(min=1, message="task name required"))]
    pub task_name: String,
}

/// URL parameters for task update operations
#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateTaskUrl {
    /// Unique identifier for the task
    pub uuid: String,
}

/// Represents a Task entity in the system
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Task {
    /// Unique identifier for the task
    pub uuid: String,
    /// Name/description of the task
    pub task_name: String,
}

impl Task {
    /// Creates a new Task instance
    /// 
    /// # Arguments
    /// * `uuid` - Unique identifier for the task
    /// * `task_name` - Name/description of the task
    pub fn new(uuid: String, task_name: String) -> Task {
        Task { uuid, task_name }
    }
}