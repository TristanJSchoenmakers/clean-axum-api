use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DeleteTodoItemResponse {
    success: bool,
}

/// Errors that can happen in the delete_todo_item route
#[derive(Error, Debug)]
pub enum DeleteTodoItemError {
    #[error("Todo Item with Id not found")]
    TodoItemNotFound,

    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for DeleteTodoItemError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DeleteTodoItemError::TodoItemNotFound => StatusCode::OK,
            DeleteTodoItemError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = self.to_string();
        let body = Json(json!({ "message": message }));

        (status_code, body).into_response()
    }
}

pub async fn delete_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<DeleteTodoItemResponse>, DeleteTodoItemError> {
    let db_result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
        r#"
            DELETE
            FROM public.todo_items
            WHERE todo_items.id = $1;
        "#,
        todo_item_id
    )
    .execute(&*db)
    .await;

    db_result.map_err(|e| match e {
        sqlx::Error::RowNotFound => DeleteTodoItemError::TodoItemNotFound,
        _ => {
            error!("unable to excecute deleteTodoItem database Query: {}", e);
            DeleteTodoItemError::InternalServerError
        }
    })?;

    Ok(Json(DeleteTodoItemResponse { success: true }))
}
