use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::value_objects::priority_level::PriorityLevel;

/// The TodoItem entiry represents a single item in a to-do list.
///
/// It contains information such as title, note, priority level, reminder, and status.
#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub list_id: Uuid,
    #[validate(length(min = 1, message = "must be atleast 1 character"))]
    #[validate(length(max = 25, message = "cannot be longer than 25 characters"))]
    pub title: String,
    pub note: Option<String>,
    pub priority: PriorityLevel,
    pub reminder: Option<DateTime<Utc>>,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TodoItem {
    /// Tries to create a new TodoItem with the provided title, note, and priority.
    pub fn new(
        title: String,
        note: Option<String>,
        priority: PriorityLevel,
    ) -> Result<Self, validator::ValidationErrors> {
        // sanitize
        let title: String = title.trim().to_string();

        let todo_item = TodoItem {
            id: Uuid::new_v4(),
            list_id: Uuid::new_v4(),
            title,
            note,
            priority,
            reminder: None,
            done: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        match todo_item.validate() {
            Ok(_) => Ok(todo_item),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
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
        let err = format!("{}", result.unwrap_err());
        assert_eq!(err, "title: must be atleast 1 character");
    }

    #[test]
    fn new_title_too_long() {
        let result = TodoItem::new(
            "Do the dishes, and also do the laundry and than watch some television".to_string(),
            None,
            PriorityLevel::Low,
        );

        let err = format!("{}", result.unwrap_err());
        assert_eq!(err, "title: cannot be longer than 25 characters");
    }
}
