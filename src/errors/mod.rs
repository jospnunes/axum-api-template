use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;
use tracing::error;
use validator::ValidationErrors;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    ResourceAlreadyExists(String),
    ValidationError(ValidationErrors),
    InternalServerError(String),
    DatabaseError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    code: String,
    message: String,
    details: Option<serde_json::Value>,
    path: Option<String>,
    timestamp: Option<String>,
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ResourceAlreadyExists(_) => StatusCode::CONFLICT,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> String {
        match self {
            ApiError::BadRequest(_) => "BAD_REQUEST".to_string(),
            ApiError::Unauthorized(_) => "UNAUTHORIZED".to_string(),
            ApiError::Forbidden(_) => "FORBIDDEN".to_string(),
            ApiError::NotFound(_) => "NOT_FOUND".to_string(),
            ApiError::ResourceAlreadyExists(_) => "RESOURCE_ALREADY_EXISTS".to_string(),
            ApiError::ValidationError(_) => "VALIDATION_ERROR".to_string(),
            ApiError::InternalServerError(_) => "INTERNAL_SERVER_ERROR".to_string(),
            ApiError::DatabaseError(_) => "DATABASE_ERROR".to_string(),
        }
    }

    pub fn details(&self) -> Option<serde_json::Value> {
        match self {
            ApiError::ValidationError(errors) => {
                Some(serde_json::to_value(errors).unwrap_or_default())
            }
            _ => None,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::ResourceAlreadyExists(msg) => write!(f, "Resource already exists: {}", msg),
            ApiError::ValidationError(_) => write!(f, "Validation error"),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match &self {
            ApiError::InternalServerError(_) | ApiError::DatabaseError(_) => {
                error!("Internal error: {}", self);
            }
            _ => {
                tracing::info!("API error: {}", self);
            }
        }

        let status = self.status_code();
        let error_response = ErrorResponse {
            status: status.to_string(),
            code: self.error_code(),
            message: self.to_string(),
            details: self.details(),
            path: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        ApiError::ValidationError(errors)
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::DatabaseError(error.to_string()),
        }
    }
}
