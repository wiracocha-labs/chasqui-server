use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
use serde::Serialize;

// TaskError enum represents different types of errors that can occur in task-related operations
#[derive(Debug, Display, Serialize)]
pub enum TaskError {
    NoTasksFound = 0,        // Error when no tasks are found in the database
    TaskCreationError = 1,   // Error when there's a problem creating a new task
    NoTaskFoundWithId = 2,   // Error when a task with a specific ID is not found
}

// Implement ResponseError trait for TaskError to integrate with actix-web error handling
impl ResponseError for TaskError {
    // Define how to convert the error into an HTTP response
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .json(self)
    }

    // Define the appropriate HTTP status code for each error type
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NoTasksFound => StatusCode::NOT_FOUND,
            TaskError::TaskCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::NoTaskFoundWithId => StatusCode::NOT_FOUND,
        }
    }
}