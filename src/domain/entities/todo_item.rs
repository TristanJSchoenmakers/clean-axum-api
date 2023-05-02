use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::value_objects::priority_level::PriorityLevel;

/// The TodoItem entiry represents a single item in a to-do list.
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
    pub fn new(
        title: String,
        note: Option<String>,
        priority: PriorityLevel,
    ) -> Result<Self, TodoItemValidationError> {
        // sanitize
        let sanitized_username: String = title.trim().to_lowercase();

        // validate
        if sanitized_username.is_empty() {
            Err(TodoItemValidationError::TitleEmpty)
        } else if title.chars().count() > 25 {
            Err(TodoItemValidationError::TitleTooLong)
        } else {
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
}

#[cfg(test)]
mod tests {
    use crate::domain::entities::todo_item::TodoItemValidationError;
    use crate::domain::value_objects::priority_level::PriorityLevel;

    use super::TodoItem;

    #[test]
    fn new_ok() {
        let result = TodoItem::new("Do the dishes".to_string(), None, PriorityLevel::Low);

        let todo_item = result.unwrap();
        assert_eq!(todo_item.title, "Do the dishes".to_string());
        assert_eq!(todo_item.note, None);
        assert_eq!(todo_item.priority, PriorityLevel::Low);
        assert!(!todo_item.done);
    }

    #[test]
    fn new_title_empty() {
        let result = TodoItem::new(" ".to_string(), None, PriorityLevel::Low);
        let error = result.unwrap_err();
        assert_eq!(error, TodoItemValidationError::TitleEmpty);
    }

    #[test]
    fn new_title_too_long() {
        let result = TodoItem::new(
            "Do the dishes, and also do the laundry and than watch some television".to_string(),
            None,
            PriorityLevel::Low,
        );

        let error = result.unwrap_err();
        assert_eq!(error, TodoItemValidationError::TitleTooLong);
    }
}
