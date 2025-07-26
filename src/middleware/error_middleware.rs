use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use bytes::Bytes;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub async fn error_handling_middleware(_cookies: Cookies, req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let response = next.run(req).await;

    if response.status().is_success() {
        return response;
    }

    enrich_error_response(response, path, method).await
}

async fn enrich_error_response(response: Response, path: String, method: String) -> Response {
    let status = response.status();
    let (parts, body) = response.into_parts();

    let bytes = match collect_body(body).await {
        Ok(bytes) => bytes,
        Err(_) => return create_default_error_response(status, &path),
    };

    if let Ok(mut json_value) = serde_json::from_slice::<Value>(&bytes) {
        if let Some(obj) = json_value.as_object_mut() {
            obj.insert("path".to_string(), json!(path));
            
            if cfg!(debug_assertions) {
                obj.insert("debug_info".to_string(), json!({
                    "method": method,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }));
            }

            let body = axum::body::Body::from(
                serde_json::to_vec(&json_value).unwrap_or_default()
            );
            return Response::from_parts(parts, body);
        }
    }

    create_default_error_response(status, &path)
}

async fn collect_body(body: Body) -> Result<Bytes, String> {
    body.collect()
        .await
        .map(|collected| collected.to_bytes())
        .map_err(|_| "Failed to collect body".to_string())
}

fn create_default_error_response(status: StatusCode, path: &str) -> Response {
    let error_response = json!({
        "status": status.as_u16(),
        "code": "UNKNOWN_ERROR",
        "message": "An error occurred",
        "path": path,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    (status, Json(error_response)).into_response()
}
