use crate::domain::todo_item::{PriorityLevel, TodoItem};
use axum::{extract::Path, Json};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<Json<TodoItem>, String> {
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

    match db_result {
        Ok(r) => Ok(Json(r)),
        Err(i) => {
            println!("Matched {:?}!", i);
            return Err(String::from("Something Went wrong!"));
        }
    }
}
