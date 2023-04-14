use axum::{
    extract::{Extension, Path},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::todo_item::PriorityLevel;

#[derive(Deserialize)]
pub struct UpdateTodoItemRequest {
    title: Option<String>,
    note: Option<String>,
    priority: Option<PriorityLevel>,
    done: Option<bool>,
}

#[derive(Serialize)]
pub struct UpdateTodoItemResponse {
    success: bool,
}

/// Errors that can happen in the update_todo_item route
#[derive(Error, Debug)]
pub enum UpdateTodoItemError {
    #[error("Todo Item with Id not found")]
    TodoItemNotFound,

    #[error("Internal Server Error")]
    InternalServerError,
}

pub async fn update_todo_item(
    Path(id): Path<Uuid>,
    db: Extension<PgPool>,
    Json(body): Json<UpdateTodoItemRequest>,
) -> Result<Json<UpdateTodoItemResponse>, String> {
    let db_result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
        r#"
            UPDATE todo_items
            SET
                title = COALESCE($2, title),
                note = COALESCE($3, note),
                priority = COALESCE($4, priority),
                done = COALESCE($5, done),
                updated_at = $6
            WHERE id = $1;
        "#,
        id,
        body.title,
        body.note,
        body.priority as _,
        body.done,
        Utc::now()
    )
    .execute(&*db)
    .await;

    match db_result {
        Ok(_) => Ok(Json(UpdateTodoItemResponse { success: true })),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(UpdateTodoItemError::TodoItemNotFound.to_string()),
            _ => {
                println!("Matched {:?}!", e);
                Err(UpdateTodoItemError::InternalServerError.to_string())
            }
        },
    }
}
