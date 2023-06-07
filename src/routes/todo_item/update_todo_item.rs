use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::{
    domain::value_objects::priority_level::PriorityLevel,
    routes::response::{internal_error, json_error},
};

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

pub async fn update_todo_item(
    Path(todo_item_id): Path<Uuid>,
    db: Extension<PgPool>,
    Json(body): Json<UpdateTodoItemRequest>,
) -> Result<Json<UpdateTodoItemResponse>, (StatusCode, String)> {
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
        sqlx::Error::RowNotFound => json_error(
            "NOT_FOUND".to_string(),
            format!("Todo Item with id '{}' not found", todo_item_id),
        ),
        e => internal_error(e),
    })?;

    Ok(Json(UpdateTodoItemResponse { success: true }))
}
