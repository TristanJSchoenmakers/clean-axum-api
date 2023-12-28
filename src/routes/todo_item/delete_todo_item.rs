use crate::routes::response_builders::internal_error;
use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

#[derive(Serialize)]
pub struct DeleteTodoItemResponse {
    success: bool,
}

pub async fn delete_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<DeleteTodoItemResponse>, (StatusCode, axum::Json<Value>)> {
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
        let body = json!({ "code": "NOT_FOUND".to_string(), "message": format!("Todo Item with id '{}' not found", todo_item_id) });
        Err((StatusCode::NOT_FOUND, Json(body)))
    } else {
        Ok(Json(DeleteTodoItemResponse { success: true }))
    }
}
