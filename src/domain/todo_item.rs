use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl TodoItem {
    pub fn try_create(title: String, note: Option<String>, priority: PriorityLevel) -> Self {
        TodoItem {
            id: Uuid::new_v4(),
            list_id: Uuid::new_v4(),
            title,
            note,
            priority,
            reminder: None,
            done: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
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
