use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// TodoItem represents a single item in a to-do list.
///
/// It contains information such as title, note, priority level, reminder, and status.
#[derive(Debug, Serialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub note: Option<String>,
    pub priority: PriorityLevel,
    pub reminder: Option<DateTime<Utc>>,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents errors that can occur when creating a TodoItem.
#[derive(Error, PartialEq, Debug)]
pub enum TodoItemValidationError {
    #[error("Title cannot be longer then 25 characters")]
    TitleTooLong,
    #[error("Title cannot be a empty")]
    TitleEmpty,
}

impl TodoItem {
    /// Tries to create a new TodoItem with the provided title, note, and priority.
    pub fn try_create(
        title: String,
        note: Option<String>,
        priority: PriorityLevel,
    ) -> Result<Self, TodoItemValidationError> {
        if title.trim().is_empty() {
            return Err(TodoItemValidationError::TitleEmpty);
        }

        if title.chars().count() > 25 {
            return Err(TodoItemValidationError::TitleTooLong);
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

#[derive(PartialEq, Debug, Deserialize, Serialize, sqlx::Type)]
#[repr(i32)]
pub enum PriorityLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

#[cfg(test)]
mod tests {
    use crate::domain::entities::todo_item::{PriorityLevel, TodoItemValidationError};

    use super::TodoItem;

    #[test]
    fn try_create_ok() {
        let result = TodoItem::try_create(
            String::from("Do the dishes"),
            None,
            super::PriorityLevel::Low,
        );

        let todo_item = result.unwrap();
        assert_eq!(todo_item.title, String::from("Do the dishes"));
        assert_eq!(todo_item.note, None);
        assert_eq!(todo_item.priority, PriorityLevel::Low);
        assert!(!todo_item.done);
    }

    #[test]
    fn try_create_title_empty() {
        let result = TodoItem::try_create(String::from(" "), None, super::PriorityLevel::Low);
        let error = result.unwrap_err();
        assert_eq!(error, TodoItemValidationError::TitleEmpty);
    }

    #[test]
    fn try_create_title_too_long() {
        let result = TodoItem::try_create(
            String::from("Do the dishes, and also do the laundry and than watch some television"),
            None,
            super::PriorityLevel::Low,
        );

        let error = result.unwrap_err();
        assert_eq!(error, TodoItemValidationError::TitleTooLong);
    }
}
