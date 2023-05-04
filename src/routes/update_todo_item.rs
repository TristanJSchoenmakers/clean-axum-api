use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;
use tracing::error;
use uuid::Uuid;

use crate::domain::value_objects::priority_level::PriorityLevel;

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
    #[error("Todo Item with Id '{0}' not found")]
    TodoItemNotFound(Uuid),

    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for UpdateTodoItemError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UpdateTodoItemError::TodoItemNotFound(_) => StatusCode::OK,
            UpdateTodoItemError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({ "message": self.to_string() }));

        (status_code, body).into_response()
    }
}

pub async fn update_todo_item(
    Path(todo_item_id): Path<Uuid>,
    db: Extension<PgPool>,
    Json(body): Json<UpdateTodoItemRequest>,
) -> Result<Json<UpdateTodoItemResponse>, UpdateTodoItemError> {
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
        todo_item_id,
        body.title,
        body.note,
        body.priority as _,
        body.done,
        Utc::now()
    )
    .execute(&*db)
    .await;

    db_result.map_err(|e| match e {
        sqlx::Error::RowNotFound => UpdateTodoItemError::TodoItemNotFound(todo_item_id),
        _ => {
            error!("unable to excecute updateTodoItem database Query: {}", e);
            UpdateTodoItemError::InternalServerError
        }
    })?;

    Ok(Json(UpdateTodoItemResponse { success: true }))
}
