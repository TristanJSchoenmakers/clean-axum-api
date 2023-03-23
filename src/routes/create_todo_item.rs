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

/// Errors that can happen in the create_todo_item route
#[derive(Error, Debug)]
pub enum CreateTodoItemError {
    #[error("Failed to create todo item")]
    InvalidTodoItem,

    #[error("Internal Server Error")]
    InternalServerError,
}

pub async fn create_todo_item(
    db: axum::Extension<PgPool>,
    Json(body): Json<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, String> {
    let todo_item = TodoItem::try_create(body.title, body.note, body.priority)
        .map_err(|_| CreateTodoItemError::InvalidTodoItem.to_string())?;

    let db_result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
        r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
        "#,
        todo_item.id,
        todo_item.list_id,
        todo_item.title,
        todo_item.note,
        todo_item.priority as _,
        todo_item.reminder,
        todo_item.done,
        todo_item.created_at,
        todo_item.updated_at
    )
    .execute(&*db)
    .await;

    if let Err(e) = db_result {
        println!("Matched {:?}!", e);
        return Err(CreateTodoItemError::InternalServerError.to_string());
    }

    Ok(Json(CreateTodoItemResponse {
        todo_item_id: todo_item.id.to_string(),
    }))
}
