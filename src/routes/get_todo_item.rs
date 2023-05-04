use crate::domain::entities::todo_item::TodoItem;
use crate::domain::value_objects::priority_level::PriorityLevel;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::PgPool;
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

/// Errors that can happen in the get_todo_item route
#[derive(Error, Debug)]
pub enum GetTodoItemError {
    #[error("Todo Item with Id '{0}' not found")]
    TodoItemNotFound(Uuid),

    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for GetTodoItemError {
    fn into_response(self) -> Response {
        let status_code = match self {
            GetTodoItemError::TodoItemNotFound(_) => StatusCode::OK,
            GetTodoItemError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({ "message": self.to_string() }));

        (status_code, body).into_response()
    }
}

pub async fn get_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<TodoItem>, GetTodoItemError> {
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

    let todo_item = db_result.map_err(|e| match e {
        sqlx::Error::RowNotFound => GetTodoItemError::TodoItemNotFound(todo_item_id),
        _ => {
            error!("unable to excecute getTodoItem database Query: {}", e);
            GetTodoItemError::InternalServerError
        }
    })?;

    Ok(Json(todo_item))
}
