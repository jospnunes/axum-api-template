use crate::{
    auth::{
        auth_dto::{LoginRequest, RegisterRequest, RefreshTokenRequest, ForgotPasswordRequest, ResetPasswordRequest, AuthResponse, UserInfo},
        auth_hashing::{hash_password, verify_password},
        auth_tokens::{generate_access_token, generate_refresh_token, validate_refresh_token},
        auth_repository,
    },
    config::database::DbPool,
    db::models::{
        user::{User, NewUser},
        refresh_token::NewRefreshToken,
    },
    errors::{ApiError, ApiResult},
    user::user_repository,
};
use chrono::{Duration, Utc};
use rand::{Rng};

pub async fn register(request: RegisterRequest, pool: DbPool) -> ApiResult<AuthResponse> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    if user_repository::find_user_by_email(&request.email, &mut conn).is_ok() {
        return Err(ApiError::ResourceAlreadyExists("Email already registered".to_string()));
    }

    let password_hash = hash_password(&request.password)
        .map_err(|e| ApiError::InternalServerError(format!("Password hashing failed: {}", e)))?;

    let new_user = NewUser {
        first_name: request.first_name,
        last_name: request.last_name,
        email: request.email,
        password_hash,
        is_active: Some(true),
        is_verified: Some(false),
    };

    let user = user_repository::create_user(&new_user, &mut conn)?;

    let access_token = generate_access_token(user.id);
    let refresh_token = generate_refresh_token(user.id);

    let new_refresh_token = NewRefreshToken {
        user_id: user.id,
        token: refresh_token.clone(),
        expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
    };

    auth_repository::create_refresh_token(&new_refresh_token, &mut conn)?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: user_to_info(user),
    })
}

pub async fn login(request: LoginRequest, pool: DbPool) -> ApiResult<AuthResponse> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let user = user_repository::find_user_by_email(&request.email, &mut conn)
        .map_err(|_| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    if !verify_password(&request.password, &user.password_hash)
        .map_err(|e| ApiError::InternalServerError(format!("Password verification failed: {}", e)))?
    {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    if !user.is_active.unwrap_or(false) {
        return Err(ApiError::Forbidden("Account is deactivated".to_string()));
    }

    let access_token = generate_access_token(user.id);
    let refresh_token = generate_refresh_token(user.id);

    let new_refresh_token = NewRefreshToken {
        user_id: user.id,
        token: refresh_token.clone(),
        expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
    };

    auth_repository::create_refresh_token(&new_refresh_token, &mut conn)?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        user: user_to_info(user),
    })
}

pub async fn refresh_token(request: RefreshTokenRequest, pool: DbPool) -> ApiResult<AuthResponse> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let claims = validate_refresh_token(&request.refresh_token)
        .map_err(|_| ApiError::Unauthorized("Invalid refresh token".to_string()))?;

    let stored_token = auth_repository::find_refresh_token(&request.refresh_token, &mut conn)
        .map_err(|_| ApiError::Unauthorized("Refresh token not found".to_string()))?;

    if stored_token.user_id != claims.sub {
        return Err(ApiError::Unauthorized("Token user mismatch".to_string()));
    }

    let user = user_repository::find_user_by_id(claims.sub, &mut conn)?;

    auth_repository::delete_refresh_token(&request.refresh_token, &mut conn)?;

    let new_access_token = generate_access_token(user.id);
    let new_refresh_token = generate_refresh_token(user.id);

    let refresh_token_record = NewRefreshToken {
        user_id: user.id,
        token: new_refresh_token.clone(),
        expires_at: (Utc::now() + Duration::days(7)).naive_utc(),
    };

    auth_repository::create_refresh_token(&refresh_token_record, &mut conn)?;

    Ok(AuthResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        user: user_to_info(user),
    })
}

pub async fn logout(user_id: uuid::Uuid, pool: DbPool) -> ApiResult<()> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    auth_repository::delete_user_refresh_tokens(user_id, &mut conn)?;

    Ok(())
}

pub async fn forgot_password(request: ForgotPasswordRequest, pool: DbPool) -> ApiResult<()> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    if let Ok(mut user) = user_repository::find_user_by_email(&request.email, &mut conn) {
        let reset_token: String = (0..32)
            .map(|_| rand::rng().random::<u8>() % 26 + b'a')
            .map(|b| b as char)
            .collect();

        user.password_reset_token = Some(reset_token.clone());
        user.password_reset_expires = Some((Utc::now() + Duration::hours(1)).naive_utc());

        user_repository::update_user(user.id, &user, &mut conn)?;

        tracing::info!("Password reset token generated for user: {}", user.email);
        tracing::info!("Reset token: {} (expires in 1 hour)", reset_token);
    }

    Ok(())
}

pub async fn reset_password(request: ResetPasswordRequest, pool: DbPool) -> ApiResult<()> {
    let mut conn = pool.get().map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let mut user = user_repository::find_user_by_reset_token(&request.token, &mut conn)
        .map_err(|_| ApiError::BadRequest("Invalid or expired reset token".to_string()))?;

    if let Some(expires) = user.password_reset_expires {
        if expires < Utc::now().naive_utc() {
            return Err(ApiError::BadRequest("Reset token has expired".to_string()));
        }
    } else {
        return Err(ApiError::BadRequest("Invalid reset token".to_string()));
    }

    let new_password_hash = hash_password(&request.new_password)
        .map_err(|e| ApiError::InternalServerError(format!("Password hashing failed: {}", e)))?;

    user.password_hash = new_password_hash;
    user.password_reset_token = None;
    user.password_reset_expires = None;

    user_repository::update_user(user.id, &user, &mut conn)?;

    auth_repository::delete_user_refresh_tokens(user.id, &mut conn)?;

    Ok(())
}

fn user_to_info(user: User) -> UserInfo {
    UserInfo {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    }
}
