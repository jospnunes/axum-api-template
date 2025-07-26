mod app;
mod auth;
mod config;
mod db;
mod errors;
mod middleware;
mod routes;
mod schema;
mod user;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    app::start_server().await
}
