use axum::extract::Path;
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;
use uuid::Uuid;

/// Errors that can happen in the delete_todo_item route
#[derive(Error, Debug)]
pub enum DeleteTodoItemError {
    #[error("Todo Item with Id not found")]
    TodoItemNotFound,

    #[error("Internal Server Error")]
    InternalServerError,
}

pub async fn delete_todo_item(
    db: axum::Extension<PgPool>,
    Path(todo_item_id): Path<Uuid>,
) -> Result<String, String> {
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

    match db_result {
        Ok(_) => Ok(todo_item_id.to_string()),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(DeleteTodoItemError::TodoItemNotFound.to_string()),
            _ => {
                println!("Matched {:?}!", e);
                Err(DeleteTodoItemError::InternalServerError.to_string())
            }
        },
    }
}
