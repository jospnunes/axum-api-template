use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub jti: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub jti: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate_access_token(user_id: Uuid) -> String {
    let now = Utc::now();
    let exp = now + Duration::minutes(15);
    
    let claims = AccessTokenClaims {
        sub: user_id,
        jti: Uuid::new_v4().to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    let secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to generate access token")
}

pub fn generate_refresh_token(user_id: Uuid) -> String {
    let now = Utc::now();
    let exp = now + Duration::days(7);
    
    let claims = RefreshTokenClaims {
        sub: user_id,
        jti: Uuid::new_v4().to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to generate refresh token")
}

pub fn validate_access_token(token: &str) -> Result<AccessTokenClaims, String> {
    let secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
    
    match decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            if token_data.claims.exp < Utc::now().timestamp() {
                Err("Access token expired".to_string())
            } else {
                Ok(token_data.claims)
            }
        }
        Err(_) => Err("Invalid access token".to_string()),
    }
}

pub fn validate_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    
    match decode::<RefreshTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            if token_data.claims.exp < Utc::now().timestamp() {
                Err("Refresh token expired".to_string())
            } else {
                Ok(token_data.claims)
            }
        }
        Err(_) => Err("Invalid refresh token".to_string()),
    }
}
