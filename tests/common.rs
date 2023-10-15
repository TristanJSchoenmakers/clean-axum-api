// This is imported by different tests that use different functions.
#![allow(dead_code)]

use axum::body::{Body, BoxBody};
use axum::http::{request, Request};
use axum::response::Response;
use hyper::body::HttpBody;

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

pub async fn get_body_string(response: Response<BoxBody>) -> String {
    let body = hyper::body::to_bytes(response.into_body())
        .await
        .expect("error reading response body");
    String::from_utf8_lossy(&body[..]).to_string()
}

pub async fn get_body_json(response: &mut Response<BoxBody>) -> serde_json::Value {
    let body = response.body_mut();

    let mut bytes = Vec::new();

    while let Some(res) = body.data().await {
        let chunk = res.expect("error reading response body");

        bytes.extend_from_slice(&chunk[..]);
    }

    serde_json::from_slice(&bytes).expect("failed to read response body as json")
}
