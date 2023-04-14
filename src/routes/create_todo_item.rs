use crate::domain::entities::todo_item::{PriorityLevel, TodoItem};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};
use thiserror::Error;

#[derive(Deserialize)]
pub struct CreateTodoItemRequest {
    title: String,
    note: Option<String>,
    priority: PriorityLevel,
}

#[derive(Serialize)]
pub struct CreateTodoItemResponse {
    todo_item_id: String,
}

/// Errors that can happen in the create_todo_item route
#[derive(Error, Debug)]
pub enum CreateTodoItemError {
    #[error("Invalid Todo Item")]
    InvalidTodoItem,

    #[error("Internal Server Error")]
    InternalServerError,
}

pub async fn create_todo_item(
    db: axum::Extension<PgPool>,
    Json(body): Json<CreateTodoItemRequest>,
) -> Result<Json<CreateTodoItemResponse>, (StatusCode, String)> {
    let todo_item = TodoItem::try_create(body.title, body.note, body.priority).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            CreateTodoItemError::InvalidTodoItem.to_string(),
        )
    })?;

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

    if let Err(e) = db_result {
        println!("Matched {:?}!", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CreateTodoItemError::InternalServerError.to_string(),
        ));
    }

    Ok(Json(CreateTodoItemResponse {
        todo_item_id: todo_item.id.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use hyper::{header, Method, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn correct_request() {
        let app = crate::test_util::setup_api().await;
        let request = Request::builder()
            .method(Method::POST)
            .uri("/todoitem")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                r#"{
                    "title": "no one",
                    "note": "my note",
                    "priority": "Medium"
                }"#,
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8_lossy(&body[..]);
        assert!(body.contains(r#"{"todo_item_id":""#));
    }

    #[tokio::test]
    async fn incorrect_request() {
        let app = crate::test_util::setup_api().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/todoitem")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                          "note": "my note",
                          "priority": "Medium"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn invalid_domain() {
        let app = crate::test_util::setup_api().await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/todoitem")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                          "title": "TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE",
                          "note": "my note",
                          "priority": "Medium"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
