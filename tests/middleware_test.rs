use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_cors_headers_present() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/login")
        .method("OPTIONS")
        .header(header::ORIGIN, "http://localhost:3000")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert!(response.headers().contains_key("access-control-allow-origin"));
}

#[tokio::test]
async fn test_error_middleware_enriches_response() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/api/user/profile")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    
    assert!(body.get("path").is_some());
    assert!(body.get("timestamp").is_some());
}

#[tokio::test]
async fn test_rate_limiting_allows_normal_requests() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    for _ in 0..3 {
        let request = Request::builder()
            .uri("/auth/login")
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{}"))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        
        assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }
}

#[tokio::test]
async fn test_json_content_type_required_for_post() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/login")
        .method("POST")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_ne!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_invalid_json_returns_error() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/auth/login")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("invalid json"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[tokio::test]
async fn test_nonexistent_route_returns_404() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request = Request::builder()
        .uri("/nonexistent/route")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
