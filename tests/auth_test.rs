use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_auth_endpoints_exist() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_register_endpoint_exists() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/register")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_refresh_endpoint_exists() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/refresh")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_ne!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_protected_route_requires_auth() {
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
async fn test_protected_route_with_invalid_token() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/api/user/profile")
        .method("GET")
        .header(header::AUTHORIZATION, "Bearer invalid_token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_malformed_auth_header() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/api/user/profile")
        .method("GET")
        .header(header::AUTHORIZATION, "InvalidFormat token")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


