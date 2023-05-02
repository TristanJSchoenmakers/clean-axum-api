use serde::{Deserialize, Serialize};

/// PriorityLevel represents the different levels of priority that a task can have in a todo list.
///
/// This ValueObject is used to enforce a consistent set of priority values across the todo list API, and to make it easier to reason about the importance of tasks within the list.
#[derive(PartialEq, Debug, Deserialize, Serialize, sqlx::Type)]
#[repr(i32)]
pub enum PriorityLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}
