use crate::auth::{auth_tokens::validate_access_token, auth_repository};
use crate::config::database::DbPool;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthUser(pub Uuid);

pub async fn auth_middleware(
    State(pool): State<DbPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match validate_access_token(token) {
        Ok(claims) => {
            if let Ok(mut conn) = pool.get() {
                if auth_repository::is_token_revoked(&claims.jti, &mut conn).unwrap_or(false) {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }

            req.extensions_mut().insert(AuthUser(claims.sub));
            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}
