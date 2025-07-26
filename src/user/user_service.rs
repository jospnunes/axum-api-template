use crate::{
    config::database::DbPool,
    errors::{ApiError, ApiResult},
    user::{
        user_dto::{UpdateUserProfileRequest, UserProfileResponse},
        user_repository,
    },
    db::models::user::User,
};
use uuid::Uuid;

pub async fn get_user_profile(user_id: Uuid, pool: DbPool) -> ApiResult<UserProfileResponse> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let user = user_repository::find_user_by_id(user_id, &mut conn)?;

    Ok(user_to_profile_response(user))
}

pub async fn update_user_profile(
    user_id: Uuid,
    request: UpdateUserProfileRequest,
    pool: DbPool,
) -> ApiResult<UserProfileResponse> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let mut user = user_repository::find_user_by_id(user_id, &mut conn)?;

    if let Some(first_name) = request.first_name {
        user.first_name = first_name;
    }
    if let Some(last_name) = request.last_name {
        user.last_name = last_name;
    }
    if let Some(email) = request.email {
        if let Ok(_existing_user) = user_repository::find_user_by_email(&email, &mut conn) {
            return Err(ApiError::ResourceAlreadyExists("Email already in use".to_string()));
        }
        user.email = email;
    }

    let updated_user = user_repository::update_user(user_id, &user, &mut conn)?;

    Ok(user_to_profile_response(updated_user))
}

fn user_to_profile_response(user: User) -> UserProfileResponse {
    UserProfileResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        is_active: user.is_active.unwrap_or(true),
        is_verified: user.is_verified.unwrap_or(false),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}
