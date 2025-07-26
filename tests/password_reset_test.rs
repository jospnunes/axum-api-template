use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

mod common;



#[tokio::test]
async fn test_forgot_password_with_nonexistent_email() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request_data = json!({
        "email": "nonexistent@example.com"
    });

    let request = Request::builder()
        .uri("/auth/forgot-password")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    
    assert!(body["message"].as_str().unwrap().contains("password reset"));
}

#[tokio::test]
async fn test_forgot_password_with_invalid_email() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request_data = json!({
        "email": "invalid-email"
    });

    let request = Request::builder()
        .uri("/auth/forgot-password")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    
    assert_eq!(body["code"], "VALIDATION_ERROR");
}

#[tokio::test]
async fn test_reset_password_with_invalid_token() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request_data = json!({
        "token": "invalid_token",
        "new_password": "newpassword123"
    });

    let request = Request::builder()
        .uri("/auth/reset-password")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_reset_password_with_short_password() {
    let _conn = common::setup_test_db();
    let app = common::setup_test_app();

    let request_data = json!({
        "token": "some_token",
        "new_password": "123"
    });

    let request = Request::builder()
        .uri("/auth/reset-password")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&request_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    
    assert_eq!(body["code"], "VALIDATION_ERROR");
}
