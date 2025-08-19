//! Helpers to execute use cases against data-access traits.
//!
//! This module centralizes the execution of closures that operate on the
//! application data-access traits (`TaskDataTrait`, `UserDataTrait`). It enables
//! dependency injection by passing the database (which implements these traits)
//! to a user-provided async function/closure.
//!
use actix_web::web::Data;
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::task_data_trait::TaskDataTrait;
use crate::models::traits::user_data_trait::UserDataTrait;

/// Utility type that hosts helpers to run closures against data-access traits.
///
/// It exposes two async methods to execute closures that depend on
/// `TaskDataTrait` or `UserDataTrait`. The database instance is injected via
/// Actix's `Data<Database>` and passed to the closure as a trait object.
pub struct DataTraitExecutor;

impl DataTraitExecutor {
    /// Execute a closure that operates on `TaskDataTrait`.
    ///
    /// Generics:
    /// - `F`: a closure/function that receives `&dyn TaskDataTrait` and returns a future.
    /// - `Fut`: the future returned by `F`.
    /// - `R`: the output type of the future.
    ///
    /// The database is injected as `&Data<Database>` and passed to the closure
    /// as a trait object (`&dyn TaskDataTrait`).
    pub async fn execute_task_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn TaskDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the closure with the database (which implements TaskDataTrait)
        // and await the result. This enables dependency injection of the DB.
        f(db.as_ref()).await
    }

    /// Execute a closure that operates on `UserDataTrait`.
    ///
    /// Generics:
    /// - `F`: a closure/function that receives `&dyn UserDataTrait` and returns a future.
    /// - `Fut`: the future returned by `F`.
    /// - `R`: the output type of the future.
    ///
    /// The database is injected as `&Data<Database>` and passed to the closure
    /// as a trait object (`&dyn UserDataTrait`).
    pub async fn execute_user_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn UserDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the closure with the database (which implements UserDataTrait)
        // and await the result. This enables dependency injection of the DB.
        f(db.as_ref()).await
    }
}