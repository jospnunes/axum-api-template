use crate::auth::auth_repository;
use crate::config::database::{establish_connection_pool, DbPool};
use crate::routes::create_routes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceExt;
use axum::{Extension, Router};

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub fn app() -> Router {
    dotenv::dotenv().ok();
    
    let pool = establish_connection_pool();
    let app_state = AppState { pool: pool.clone() };

    setup_token_cleanup_tasks(pool.clone());

    create_routes(app_state)
        .layer(Extension(pool))
}

fn setup_token_cleanup_tasks(pool: DbPool) {
    let pool_init = pool.clone();
    tokio::spawn(async move {
        if let Ok(mut conn) = pool_init.get() {
            let _ = auth_repository::clean_expired_refresh_tokens(&mut conn);
            let _ = auth_repository::clean_expired_revoked_tokens(&mut conn);
        }
    });

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        loop {
            interval.tick().await;
            if let Ok(mut conn) = pool.get() {
                let _ = auth_repository::clean_expired_refresh_tokens(&mut conn);
                let _ = auth_repository::clean_expired_revoked_tokens(&mut conn);
            }
        }
    });
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let app = app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("Servidor iniciado em http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let app = app.clone();

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| app.clone().oneshot(req)))
                .await
            {
                tracing::error!("Erro ao atender conexão: {:?}", err);
            }
        });
    }
}
