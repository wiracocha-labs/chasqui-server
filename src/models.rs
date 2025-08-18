// Sub-modules for different types of models

pub mod entities;  // Concrete data structures representing domain objects
pub mod traits;    // Trait definitions for data access and manipulation

// Re-exports of commonly used types for easier access
pub use entities::task::Task;
pub use entities::user::User;

// This file organizes the data models used in the application.
// It separates concrete entities from trait definitions and
// re-exports frequently used types to simplify imports in other parts of the code.
// This structure promotes modularity and makes it easier to manage and extend
// the application's data model.