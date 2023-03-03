use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    lastname: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    user_id: i32,
}

pub async fn create_user(
    db: axum::Extension<PgPool>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    println!("Create User Request: {}", body.lastname);

    // sqlx::query!(
    //     r#"INSERT INTO public.transaction (transaction_id, lastname)
    //         VALUES(gen_random_uuid(), 'wat');
    //     "#
    // )
    // .execute(&mut db);

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*db)
        .await
        .unwrap();

    println!("QUERY RESULT: {}", row.0);

    //INSERT INTO public."transaction"
    //(transaction_id, lastname)
    //VALUES(gen_random_uuid(), 'wat');
    //let _v: Result<CreateUserRequest, serde_json::Error> = serde_json::from_str(&body);

    /*if let Err(w) = v {
        return Json("wtf");
    }*/

    Json(CreateUserResponse { user_id: 73 })
}
