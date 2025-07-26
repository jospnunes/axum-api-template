use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use axum_api_template::app;
use axum_api_template::db::models::user::{User, NewUser};
use axum_api_template::user::user_repository;
use axum_api_template::auth::auth_hashing::hash_password;
use axum_api_template::auth::auth_tokens::generate_access_token;
use axum::Router;
use once_cell::sync::Lazy;
use std::env;
use uuid::Uuid;

pub static TEST_POOL: Lazy<Pool<ConnectionManager<PgConnection>>> = Lazy::new(|| {
    dotenv::dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(1))
        .connection_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(60)))
        .build(manager)
        .expect("Failed to create test pool")
});

pub fn setup_test_app() -> Router {
    app()
}

pub fn setup_test_db() -> diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    let mut conn = TEST_POOL.get().expect("Failed to get DB connection from pool");
    cleanup_test_data(&mut conn);
    conn
}

fn cleanup_test_data(conn: &mut diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>) {
    use diesel::sql_query;
    let _ = sql_query("DELETE FROM refresh_tokens").execute(conn);
    let _ = sql_query("DELETE FROM revoked_tokens").execute(conn);
    let _ = sql_query("DELETE FROM users").execute(conn);
}

pub fn create_test_user_with_email(conn: &mut PgConnection, email: &str) -> User {
    let new_user = NewUser {
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: email.to_string(),
        password_hash: hash_password("password123").expect("Failed to hash password"),
        is_active: Some(true),
        is_verified: Some(true),
    };

    user_repository::create_user(&new_user, conn)
        .expect("Failed to create test user")
}

pub fn create_test_user(conn: &mut PgConnection) -> User {
    let unique_email = format!("test-{}@example.com", uuid::Uuid::new_v4());
    create_test_user_with_email(conn, &unique_email)
}

pub fn generate_test_token(user_id: Uuid) -> String {
    generate_access_token(user_id)
}
