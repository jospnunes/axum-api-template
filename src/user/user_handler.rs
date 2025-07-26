use crate::{
    app::AppState,
    auth::auth_middleware::AuthUser,
    errors::ApiResult,
    user::{
        user_dto::{UpdateUserProfileRequest, UserProfileResponse},
        user_service,
    },
};
use axum::{
    extract::State,
    response::Json,
    Extension,
};
use validator::Validate;

pub async fn get_user_profile_handler(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<AuthUser>,
) -> ApiResult<Json<UserProfileResponse>> {
    let profile = user_service::get_user_profile(authenticated_user.0, state.pool).await?;
    Ok(Json(profile))
}

pub async fn update_user_profile_handler(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<AuthUser>,
    Json(payload): Json<UpdateUserProfileRequest>,
) -> ApiResult<Json<UserProfileResponse>> {
    payload.validate()?;
    let profile = user_service::update_user_profile(
        authenticated_user.0,
        payload,
        state.pool,
    ).await?;
    Ok(Json(profile))
}
