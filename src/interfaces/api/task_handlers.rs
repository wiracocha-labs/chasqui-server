//! Task Handlers Module
//! Implements HTTP request handlers for task-related operations.

use actix_web::{web, HttpResponse, Responder};
use crate::models::entities::task::{AddTaskRequest, UpdateTaskUrl, Task};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::task_data_trait::TaskDataTrait;
use crate::error::TaskError;
use validator::Validate;
use uuid::Uuid;
use log::{info, debug, warn, error};

/// Retrieves all tasks from the database
/// 
/// # Returns
/// - 200 OK with tasks array if found
/// - 404 Not Found if no tasks exist
pub async fn get_task(db: web::Data<Database>) -> impl Responder {
    info!("GET /tasks: start");
    // Query the database for all tasks using the TaskDataTrait implementation
    let tasks = <Database as TaskDataTrait>::get_all_tasks(&db).await;  
    match tasks {
        Some(found_tasks) => {
            info!("GET /tasks: ok count={}", found_tasks.len());
            HttpResponse::Ok().json(found_tasks)
        },
        None => {
            warn!("GET /tasks: no tasks found");
            HttpResponse::NotFound().json(TaskError::NoTasksFound)
        }
    }
}

/// Creates a new task in the system
/// 
/// # Arguments
/// * `body` - JSON payload containing task details
/// * `db` - Database connection
///
/// # Returns
/// - 200 OK with created task if successful
/// - 400 Bad Request if validation fails
/// - 500 Internal Server Error if creation fails
pub async fn add_task(body: web::Json<AddTaskRequest>, db: web::Data<Database>) -> impl Responder {
    info!("POST /tasks: create requested");
    // Validate incoming request data
    if let Err(e) = body.validate() {
        warn!("POST /tasks: validation failed -> {:?}", e);
        return HttpResponse::BadRequest().json(TaskError::TaskCreationError);
    }

    // Generate new UUID and prepare task data
    let task_name = body.task_name.clone();
    let new_uuid = Uuid::new_v4().to_string();
    debug!("POST /tasks: creating uuid={} name={}", new_uuid, task_name);

    // Attempt to add the new task to the database
    let new_task = <Database as TaskDataTrait>::add_task(&db, Task::new(new_uuid.clone(), task_name)).await;

    match new_task {
        Some(created) => {
            info!("POST /tasks: created uuid={}", new_uuid);
            HttpResponse::Ok().json(created)
        },
        None => {
            error!("POST /tasks: DB create failed uuid={}", new_uuid);
            HttpResponse::InternalServerError().json(TaskError::TaskCreationError)
        },
    }
}

/// Updates an existing task by UUID
/// 
/// # Arguments
/// * `update_task_url` - URL parameters containing task UUID
/// * `db` - Database connection
///
/// # Returns
/// - 200 OK with updated task if successful
/// - 404 Not Found if task doesn't exist
pub async fn update_task(
    update_task_url: web::Path<UpdateTaskUrl>,
    db: web::Data<Database>
) -> impl Responder {
    let uuid = update_task_url.into_inner().uuid;
    info!("PUT /tasks/{uuid}: update requested");
    // Attempt to update the task in the database
    let update_result = <Database as TaskDataTrait>::update_task(&db, uuid.clone()).await;

    match update_result {
        Some(updated_task) => {
            info!("PUT /tasks/{uuid}: updated");
            HttpResponse::Ok().json(updated_task)
        },
        None => {
            warn!("PUT /tasks/{uuid}: not found");
            HttpResponse::NotFound().json(TaskError::NoTaskFoundWithId)
        },
    }
}