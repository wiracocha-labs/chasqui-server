use actix_web::{web, HttpResponse, Responder};
use crate::models::entities::task::{AddTaskRequest, UpdateTaskUrl, Task};
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::task_data_trait::TaskDataTrait;
use crate::error::TaskError;
use validator::Validate;
use uuid::Uuid;

// Handler for retrieving all tasks
pub async fn get_task(db: web::Data<Database>) -> impl Responder {
    // Attempt to retrieve all tasks from the database
    let tasks = <Database as TaskDataTrait>::get_all_tasks(&db).await;  
    match tasks {
        Some(found_tasks) => HttpResponse::Ok().json(found_tasks),
        None => HttpResponse::NotFound().json(TaskError::NoTasksFound)
    }
}

// Handler for adding a new task
pub async fn add_task(body: web::Json<AddTaskRequest>, db: web::Data<Database>) -> impl Responder {
    // Validate the request body
    if let Err(_) = body.validate() {
        return HttpResponse::BadRequest().json(TaskError::TaskCreationError);
    }

    let task_name = body.task_name.clone();
    let new_uuid = Uuid::new_v4().to_string();

    // Attempt to add the new task to the database
    let new_task = <Database as TaskDataTrait>::add_task(&db, Task::new(new_uuid, task_name)).await;

    match new_task {
        Some(created) => HttpResponse::Ok().json(created),
        None => HttpResponse::InternalServerError().json(TaskError::TaskCreationError),
    }
}

// Handler for updating an existing task
pub async fn update_task(
    update_task_url: web::Path<UpdateTaskUrl>,
    db: web::Data<Database>
) -> impl Responder {
    let uuid = update_task_url.into_inner().uuid;
    // Attempt to update the task in the database
    let update_result = <Database as TaskDataTrait>::update_task(&db, uuid).await;

    match update_result {
        Some(updated_task) => HttpResponse::Ok().json(updated_task),
        None => HttpResponse::NotFound().json(TaskError::NoTaskFoundWithId),
    }
}