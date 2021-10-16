use crate::tests::setup;
use axum::body::Body;
use axum::http::{self, Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt; // for `app.oneshot()`

#[tokio::test]
async fn json_post() {
    let app = setup::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
}

#[tokio::test]
async fn json_get() {
    let app = setup::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!(1));
}
