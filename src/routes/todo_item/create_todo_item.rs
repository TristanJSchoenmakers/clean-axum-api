use crate::domain::entities::todo_item::TodoItem;
use crate::domain::value_objects::priority_level::PriorityLevel;
use crate::routes::extractors::ValidatedJson;
use crate::routes::response_builders::{internal_error, validation_error};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgQueryResult, PgPool};
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreateTodoItemRequest {
    #[validate(length(min = 1, message = "must be atleast 1 character"))]
    #[validate(length(max = 25, message = "cannot be longer than 25 characters"))]
    title: String,
    note: Option<String>,
    priority: PriorityLevel,
}

#[derive(Serialize)]
pub struct CreateTodoItemResponse {
    todo_item_id: String,
}

pub async fn create_todo_item(
    db: axum::Extension<PgPool>,
    ValidatedJson(body): ValidatedJson<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, (StatusCode, axum::Json<Value>)> {
    let todo_item =
        TodoItem::new(body.title, body.note, body.priority).map_err(validation_error)?;

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

    db_result.map_err(internal_error)?;

    Ok(Json(CreateTodoItemResponse {
        todo_item_id: todo_item.id.to_string(),
    }))
}
