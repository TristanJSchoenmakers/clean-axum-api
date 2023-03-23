use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// TodoItem represents a single item in a to-do list.
///
/// It contains information such as title, note, priority level, reminder, and status.
#[derive(Serialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub list_id: Uuid,
    /// Title for the TodoItem
    pub title: String,
    pub note: Option<String>,
    pub priority: PriorityLevel,
    pub reminder: Option<DateTime<Utc>>,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents errors that can occur when creating a TodoItem.
#[derive(Error, Debug)]
pub enum CreateTodoItemError {
    /// Error occurs when a TodoItem fails validation.
    #[error("Failed to create todo item")]
    InvalidTodoItem,
}

impl TodoItem {
    /// Tries to create a new TodoItem with the provided title, note, and priority.
    pub fn try_create(
        title: String,
        note: Option<String>,
        priority: PriorityLevel,
    ) -> Result<Self, CreateTodoItemError> {
        if title.trim().is_empty() || title.len() > 100 {
            return Err(CreateTodoItemError::InvalidTodoItem);
        }

        Ok(TodoItem {
            id: Uuid::new_v4(),
            list_id: Uuid::new_v4(),
            title,
            note,
            priority,
            reminder: None,
            done: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

// #[derive(Deserialize, sqlx::Type)]
// #[sqlx(type_name = "PRIORITY")]
#[derive(Deserialize, Serialize, sqlx::Type)]
#[repr(i32)]
pub enum PriorityLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}
