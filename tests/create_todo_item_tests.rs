use axum::http::{Request, StatusCode};
use common::get_body_json;
use common::get_body_string;
use common::RequestBuilderExt;
use pretty_assertions::assert_eq;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::post("/todoitem").json(json! {
        {
            "title": "no one",
            "note": "my note",
            "priority": "Medium"
        }
    });

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_body_json(response).await;
    assert!(body["todo_item_id"].is_string());
    Ok(())
}

#[sqlx::test]
fn incorrect_request(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::post("/todoitem").json(json! {
        {
          "note": "my note",
          "priority": "Medium"
        }
    });

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[sqlx::test]
fn invalid_domain(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::post("/todoitem").json(json! {
        {
          "title": "TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE, TO LONG TITLE",
          "note": "my note",
          "priority": "Medium"
        }
    });

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = get_body_string(response).await;
    assert_eq!(
        body,
        json! {
            {
                "code": "VALIDATION_ERROR",
                "errors": {
                    "title": ["must be between 1 and 25 characters"]},
                "message": "Validation error occurred"
            }
        }
        .to_string()
    );
    Ok(())
}
