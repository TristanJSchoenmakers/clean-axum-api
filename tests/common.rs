// This is imported by different tests that use different functions.
#![allow(dead_code)]

use axum::body::to_bytes;
use axum::body::Body;
use axum::http::{request, Request};
use axum::response::Response;

pub trait RequestBuilderExt {
    fn json(self, json: serde_json::Value) -> Request<Body>;

    fn empty_body(self) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
    fn json(self, json: serde_json::Value) -> Request<Body> {
        self.header("Content-Type", "application/json")
            .body(Body::from(json.to_string()))
            .expect("failed to build request")
    }

    fn empty_body(self) -> Request<Body> {
        self.body(Body::empty()).expect("failed to build request")
    }
}

pub async fn get_body_string(response: Response<Body>) -> String {
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("error reading response body");
    String::from_utf8_lossy(&body[..]).to_string()
}

pub async fn get_body_json(response: Response<Body>) -> serde_json::Value {
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("error reading response body");

    serde_json::from_slice(&body[..]).expect("failed to read response body as json")
}
