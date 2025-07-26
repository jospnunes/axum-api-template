use crate::auth::{auth_handler, auth_middleware::auth_middleware};
use crate::app::AppState;
use crate::middleware::error_middleware::error_handling_middleware;
use crate::middleware::rate_limiter::{rate_limit_middleware, RateLimiter};
use crate::user::user_handler;
use axum::http::{HeaderName, Method};
use axum::middleware::from_fn_with_state;
use axum::{middleware::from_fn, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use std::env;

pub fn create_routes(app_state: AppState) -> Router {
    let cors_origin = env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let cors = CorsLayer::new()
        .allow_origin([cors_origin.parse().unwrap()])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true);

    let strict_limiter = RateLimiter::new(5, 60);
    let normal_limiter = RateLimiter::new(60, 60);

    let auth_routes = Router::new()
        .route("/login", axum::routing::post(auth_handler::login_handler))
        .route("/register", axum::routing::post(auth_handler::register_handler))
        .route("/refresh", axum::routing::post(auth_handler::refresh_token_handler))
        .route("/forgot-password", axum::routing::post(auth_handler::forgot_password_handler))
        .route("/reset-password", axum::routing::post(auth_handler::reset_password_handler))
        .layer(from_fn_with_state(strict_limiter, rate_limit_middleware));

    let user_routes = Router::new()
        .route("/profile", axum::routing::get(user_handler::get_user_profile_handler))
        .route("/profile", axum::routing::put(user_handler::update_user_profile_handler));

    let protected_routes = Router::new()
        .nest("/user", user_routes)
        .route("/logout", axum::routing::post(auth_handler::logout_handler))
        .layer(from_fn_with_state(app_state.pool.clone(), auth_middleware))
        .layer(from_fn_with_state(normal_limiter, rate_limit_middleware));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/api", protected_routes)
        .layer(from_fn(error_handling_middleware))
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .with_state(app_state)
}
