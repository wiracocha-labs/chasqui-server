// Import the task_error module
pub mod task_error;
// Re-export the TaskError type for easier access
pub use task_error::TaskError;

// This file serves as a central point for error handling in the application.
// It imports and re-exports error types from various modules, making them
// easily accessible throughout the project.