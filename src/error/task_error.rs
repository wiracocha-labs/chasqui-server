//! Error types and Actix-Web integration for task-related operations.
//!
//! This module defines `TaskError`, an application error enum used by the task
//! domain. It integrates with Actix-Web by implementing `ResponseError` so
//! HTTP handlers can return errors directly and get a consistent JSON response
//! body plus an appropriate status code.
//!
use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
use serde::Serialize;

/// Task-level errors that can occur during task operations.
///
/// The enum derives:
/// - `Debug` for developer-friendly formatting
/// - `Display` (via `derive_more`) for human-readable messages
/// - `Serialize` so it can be returned as JSON in HTTP responses
#[derive(Debug, Display, Serialize)]
pub enum TaskError {
    /// No tasks were found in the data store.
    NoTasksFound = 0,
    /// Failed to create a new task due to an internal error.
    TaskCreationError = 1,
    /// No task exists with the specified ID.
    NoTaskFoundWithId = 2,
}

// Integrate `TaskError` with Actix-Web error handling.
impl ResponseError for TaskError {
    // Render the error as a JSON response with a proper Content-Type header.
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .json(self)
    }

    // Map each error variant to its corresponding HTTP status code.
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NoTasksFound => StatusCode::NOT_FOUND,
            TaskError::TaskCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::NoTaskFoundWithId => StatusCode::NOT_FOUND,
        }
    }
}