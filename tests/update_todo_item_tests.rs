use axum::http::{Request, StatusCode};
use chrono::{TimeZone, Utc};
use common::get_body_json;
use common::RequestBuilderExt;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    sqlx::query!(r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('55555555-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', 'some_title', '', 0, $1, false, $1, $1);
        "#,
        Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap()
    )
     .execute(&pool)
     .await?;
    let app = api::app(pool);
    let request = Request::patch("/todoitem/55555555-1111-2222-3333-444444444444").json(json! {
        {
            "title": "no one",
            "note": "my note",
            "priority": "Medium"
        }
    });

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_body_json(response).await;
    assert_eq!(body["success"].as_bool(), Some(true));
    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::patch("/todoitem/8ccf4b24-7b25-4781-8e4e-b22931dd6558").json(json! {
        {
            "title": "Some updated Title",
            "note": "Some updated Note",
            "priority": "High",
            "done": true
        }
    });

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = get_body_json(response).await;
    assert_eq!(body["code"].as_str(), Some("NOT_FOUND"));
    assert_eq!(
        body["message"].as_str(),
        Some("Todo Item with id '8ccf4b24-7b25-4781-8e4e-b22931dd6558' not found")
    );
    Ok(())
}
