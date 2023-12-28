use crate::domain::entities::todo_item::TodoItem;
use crate::domain::value_objects::priority_level::PriorityLevel;
use crate::routes::response_builders::internal_error;
use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<TodoItem>, (StatusCode, axum::Json<Value>)> {
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
        sqlx::Error::RowNotFound => {
            let body = json!({ "code": "NOT_FOUND".to_string(), "message": format!("Todo Item with id '{}' not found", todo_item_id) });
            (StatusCode::NOT_FOUND, Json(body))
        },
        e => internal_error(e),
    })?;

    Ok(Json(todo_item))
}
