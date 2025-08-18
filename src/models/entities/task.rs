use serde::{Deserialize, Serialize};
use validator::Validate;

// Struct for adding a new task request
#[derive(Validate, Serialize, Deserialize)]
pub struct AddTaskRequest {
    #[validate(length(min=1, message="task name required"))]
    pub task_name: String,
}

// Struct for updating a task URL
#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateTaskUrl {
    pub uuid: String,
}

// Struct representing a Task
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Task {
    pub uuid: String,
    pub task_name: String,
}

impl Task {
    // Create a new task
    pub fn new(uuid: String, task_name: String) -> Task {
        Task { uuid, task_name }
    }
}