use actix_web::web::Data;
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::task_data_trait::TaskDataTrait;
use crate::models::traits::user_data_trait::UserDataTrait;

// DataTraitExecutor is a utility struct for executing use cases
// It provides methods to execute functions that require TaskDataTrait or UserDataTrait
pub struct DataTraitExecutor;

impl DataTraitExecutor {
    // Execute a task data trait
    // This method takes a database reference and a function that operates on TaskDataTrait
    // F: The function to execute
    // Fut: The future returned by F
    // R: The result type of the future
    pub async fn execute_task_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn TaskDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the function f with a reference to the Database,
        // which implements TaskDataTrait, and await the result
        // This allows for dependency injection of the database implementation
        f(db.as_ref()).await
    }

    // Execute a user data trait
    // This method takes a database reference and a function that operates on UserDataTrait
    // F: The function to execute
    // Fut: The future returned by F
    // R: The result type of the future
    pub async fn execute_user_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn UserDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the function f with a reference to the Database,
        // which implements UserDataTrait, and await the result
        // This allows for dependency injection of the database implementation
        f(db.as_ref()).await
    }
}