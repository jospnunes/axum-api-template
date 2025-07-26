use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_user_profile_requires_auth() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/api/user/profile")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_profile_requires_auth() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/api/user/profile")
        .method("PUT")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


