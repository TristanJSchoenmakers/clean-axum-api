use crate::routes::response_builders::internal_error;
use crate::{
    domain::value_objects::priority_level::PriorityLevel, routes::extractors::ValidatedJson,
};
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct UpdateTodoItemRequest {
    #[validate(length(min = 1, max = 25, message = "must be between 1 and 25 characters"))]
    title: Option<String>,
    #[validate(length(min = 1, message = "must be atleast 1 character"))]
    note: Option<String>,
    priority: Option<PriorityLevel>,
    done: Option<bool>,
}

#[derive(Serialize)]
pub struct UpdateTodoItemResponse {
    success: bool,
}

pub async fn update_todo_item(
    Path(todo_item_id): Path<Uuid>,
    db: Extension<PgPool>,
    ValidatedJson(body): ValidatedJson<UpdateTodoItemRequest>,
) -> Result<Json<UpdateTodoItemResponse>, (StatusCode, axum::Json<Value>)> {
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

    let db_result = db_result.map_err(internal_error)?;

    if db_result.rows_affected() == 0 {
        let body = json!({ "code": "NOT_FOUND".to_string(), "message": format!("Todo Item with id '{}' not found", todo_item_id) });
        Err((StatusCode::NOT_FOUND, Json(body)))
    } else {
        Ok(Json(UpdateTodoItemResponse { success: true }))
    }
}
