use crate::domain::todo_item::{PriorityLevel, TodoItem};
use axum::{extract::Path, Json};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can happen in the get_todo_item route
#[derive(Error, Debug)]
pub enum GetTodoItemError {
    #[error("Todo Item with Id not found")]
    TodoItemNotFound,

    #[error("Internal Server Error")]
    InternalServerError,
}

pub async fn get_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<TodoItem>, String> {
    let db_result: Result<TodoItem, sqlx::Error> = sqlx::query_as!(
        TodoItem,
        r#"
            SELECT id, list_id, title, note, priority AS "priority: PriorityLevel", reminder, done, created_at, updated_at
            FROM public.todo_items
            WHERE todo_items.id = $1;
        "#,
        todo_item_id
    )
    .fetch_one(&*db)
    .await;

    match db_result {
        Ok(r) => Ok(Json(r)),
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(GetTodoItemError::TodoItemNotFound.to_string()),
            _ => {
                println!("Matched {:?}!", e);
                return Err(GetTodoItemError::InternalServerError.to_string());
            }
        },
    }
}
