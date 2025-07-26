use crate::{
    app::AppState,
    auth::{
        auth_dto::{LoginRequest, RegisterRequest, RefreshTokenRequest, ForgotPasswordRequest, ResetPasswordRequest, AuthResponse},
        auth_service,
        auth_middleware::AuthUser,
    },
    errors::ApiResult,
};
use axum::{
    extract::State,
    response::Json,
    Extension,
};
use validator::Validate;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    payload.validate()?;
    let auth_response = auth_service::login(payload, state.pool).await?;
    Ok(Json(auth_response))
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    payload.validate()?;
    let auth_response = auth_service::register(payload, state.pool).await?;
    Ok(Json(auth_response))
}

pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> ApiResult<Json<AuthResponse>> {
    let auth_response = auth_service::refresh_token(payload, state.pool).await?;
    Ok(Json(auth_response))
}

pub async fn forgot_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    payload.validate()?;
    auth_service::forgot_password(payload, state.pool).await?;
    Ok(Json(serde_json::json!({"message": "If the email exists, a password reset link has been sent"})))
}

pub async fn reset_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    payload.validate()?;
    auth_service::reset_password(payload, state.pool).await?;
    Ok(Json(serde_json::json!({"message": "Password reset successfully"})))
}

pub async fn logout_handler(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    auth_service::logout(authenticated_user.0, state.pool).await?;
    Ok(Json(serde_json::json!({"message": "Logged out successfully"})))
}
