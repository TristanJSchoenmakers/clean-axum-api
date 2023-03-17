use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

pub struct TodoItem {
    pub id: Uuid,
    pub list_id: Uuid,
    /// Title for the TodoItem
    pub title: String,
    pub note: String,
    pub priority: PriorityLevel,
    pub reminder: Option<DateTime<Utc>>,
    pub done: bool,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

impl TodoItem {
    pub fn try_create(title: String, note: String, priority: PriorityLevel) -> Self {
        TodoItem {
            id: Uuid::new_v4(),
            list_id: Uuid::new_v4(),
            title,
            note,
            priority,
            reminder: None,
            done: false,
            created: Utc::now(),
            last_modified: Utc::now(),
        }
    }
}

#[derive(Deserialize, sqlx::Type)]
#[sqlx(type_name = "PRIORITY")]
pub enum PriorityLevel {
    None,
    Low,
    Medium,
    High,
}
