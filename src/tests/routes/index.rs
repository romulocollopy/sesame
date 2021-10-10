use crate::tests::setup;
use axum::body::Body;
use axum::http::{self, Request, StatusCode};
use tower::ServiceExt; // for `app.oneshot()`

#[tokio::test]
async fn hello_world() {
    let app = setup::build_test_app().await;

    // `BoxRoute<Body>` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}

#[tokio::test]
async fn hello_post() {
    let app = setup::build_test_app().await;

    // `BoxRoute<Body>` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .method(http::Method::POST)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Post World, mdf!");
}

#[tokio::test]
async fn not_found() {
    let app = setup::build_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/does-not-exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(body.is_empty());
}
