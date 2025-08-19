//! Error Handling Tests Module
//! Validates error types and their HTTP response characteristics.

use actix_crud::error::task_error::TaskError;
use actix_web::http::{header::CONTENT_TYPE, StatusCode};
use actix_web::ResponseError;

/// Test that task errors map to correct HTTP status codes
#[test]
fn task_error_status_codes() {
    // Verify each error type returns the expected status code
    assert_eq!(TaskError::NoTasksFound.status_code(), StatusCode::NOT_FOUND);
    assert_eq!(
        TaskError::TaskCreationError.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(TaskError::NoTaskFoundWithId.status_code(), StatusCode::NOT_FOUND);
}

/// Test that error responses are properly formatted as JSON
#[test]
fn task_error_response_is_json() {
    // Create and verify error response
    let resp = TaskError::NoTasksFound.error_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Verify response content type is JSON
    let ct = resp
        .headers()
        .get(CONTENT_TYPE)
        .expect("content-type header present");
    assert_eq!(ct.to_str().unwrap(), "application/json");
}
