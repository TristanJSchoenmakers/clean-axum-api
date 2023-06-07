use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::routes::response::{internal_error, json_error};

#[derive(Serialize)]
pub struct DeleteTodoItemResponse {
    success: bool,
}

pub async fn delete_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<DeleteTodoItemResponse>, (StatusCode, String)> {
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

    let db_result = db_result.map_err(internal_error)?;

    if db_result.rows_affected() == 0 {
        Err(json_error(
            "NOT_FOUND".to_string(),
            format!("Todo Item with id '{}' not found", todo_item_id),
        ))
    } else {
        Ok(Json(DeleteTodoItemResponse { success: true }))
    }
}
