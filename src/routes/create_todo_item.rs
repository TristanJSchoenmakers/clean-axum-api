use crate::domain::todo_item::{PriorityLevel, TodoItem};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;

#[derive(Deserialize)]
pub struct CreateTodoItemRequest {
    title: String,
    note: Option<String>,
    priority: PriorityLevel,
}

#[derive(Serialize)]
pub struct CreateTodoItemResponse {
    todo_item_id: String,
}

#[derive(Error, Debug)]
pub enum CreateTodoItemError {
    #[error("Failed to create todo item")]
    InvalidTodoItem,

    #[error("Database error")]
    DatabaseError,
}

pub async fn create_todo_item(
    db: axum::Extension<PgPool>,
    Json(body): Json<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, String> {
    let todo_item = TodoItem::try_create(body.title, body.note, body.priority)
        .map_err(|_| CreateTodoItemError::InvalidTodoItem.to_string())?;

    let db_result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
        r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done)
            VALUES ($1, $2, $3, $4, $5, '2023-03-20 12:00:00', false);
        "#,
        todo_item.id,
        todo_item.list_id,
        todo_item.title,
        todo_item.note,
        todo_item.priority as _
    )
    .execute(&*db)
    .await;

    if let Err(e) = db_result {
        println!("Matched {:?}!", e);
        return Err(CreateTodoItemError::DatabaseError.to_string());
    }

    Ok(Json(CreateTodoItemResponse {
        todo_item_id: todo_item.id.to_string(),
    }))
}
