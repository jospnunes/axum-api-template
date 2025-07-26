use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_health_endpoint() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "healthy");
    assert_eq!(json["service"], "axum-api-template");
    assert!(json["timestamp"].is_string());
}

#[tokio::test]
async fn test_readiness_endpoint() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/ready")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ready");
    assert_eq!(json["database"], "connected");
    assert!(json["timestamp"].is_string());
}
