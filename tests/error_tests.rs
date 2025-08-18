use actix_crud::error::task_error::TaskError;
use actix_web::http::{header::CONTENT_TYPE, StatusCode};
use actix_web::ResponseError; // bring trait into scope

#[test]
fn task_error_status_codes() {
    assert_eq!(TaskError::NoTasksFound.status_code(), StatusCode::NOT_FOUND);
    assert_eq!(
        TaskError::TaskCreationError.status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(TaskError::NoTaskFoundWithId.status_code(), StatusCode::NOT_FOUND);
}

#[test]
fn task_error_response_is_json() {
    let resp = TaskError::NoTasksFound.error_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let ct = resp
        .headers()
        .get(CONTENT_TYPE)
        .expect("content-type header present");
    assert_eq!(ct.to_str().unwrap(), "application/json");
}
