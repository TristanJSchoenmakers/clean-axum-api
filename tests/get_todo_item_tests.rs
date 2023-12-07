use axum::http::{Request, StatusCode};
use chrono::TimeZone;
use chrono::Utc;
use common::get_body_json;
use common::RequestBuilderExt;
use pretty_assertions::assert_eq;
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
fn correct_request(pool: PgPool) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO todo_items (id, list_id, title, note, priority, reminder, done, created_at, updated_at)
            VALUES ('22222222-1111-2222-3333-444444444444', '11111111-1111-2222-3333-444444444444', 'some_title', '', 0, $1, false, $1, $1);
        "#,
        Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap()
    )
    .execute(&pool)
    .await
    .unwrap();
    let app = api::app(pool);
    let request = Request::get("/todoitem/22222222-1111-2222-3333-444444444444").empty_body();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_body_json(response).await;
    assert_eq!(
        body["id"].as_str(),
        Some("22222222-1111-2222-3333-444444444444")
    );
    assert_eq!(
        body["list_id"].as_str(),
        Some("11111111-1111-2222-3333-444444444444")
    );
    assert_eq!(body["title"].as_str(), Some("some_title"));
    assert_eq!(body["created_at"].as_str(), Some("2014-07-08T09:10:11Z"));
    Ok(())
}

#[sqlx::test]
fn not_found(pool: PgPool) -> sqlx::Result<()> {
    let app = api::app(pool);
    let request = Request::get("/todoitem/8ccf4b24-7b25-4781-8e4e-b22931dd6558").empty_body();

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
