use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::domain::todo_item::{PriorityLevel, TodoItem};

#[derive(Deserialize)]
pub struct CreateTodoItemRequest {
    title: String,
    note: String,
    priority: PriorityLevel,
}

#[derive(Serialize)]
pub struct CreateTodoItemResponse {
    todo_item_id: String,
}

pub async fn create_todo_item(
    db: axum::Extension<PgPool>,
    Json(body): Json<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, String> {
    let todo_item = TodoItem::try_create(body.title, body.note, body.priority);

    let db_result = sqlx::query!(
        r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done)
            VALUES ($1, $2, $3, $4, $5, '2023-03-20 12:00:00', false);
        "#,
        todo_item.id,
        todo_item.list_id,
        todo_item.title,
        todo_item.note,
        todo_item.priority as _
    )
    .execute(&*db)
    .await;

    if let Err(i) = db_result {
        println!("Matched {:?}!", i);
        return Err(String::from("Something Went wrong!"));
    }

    Ok(Json(CreateTodoItemResponse {
        todo_item_id: todo_item.id.to_string(),
    }))
}
