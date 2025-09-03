mod config;
mod domain;
mod error;
mod infrastructure;
mod repo;
mod routes;
mod service;
mod state;
mod handlers;

use crate::config::Config;
use crate::infrastructure::db::{create_pool, run_migrations};
use crate::repo::sqlx_book_repo::SqlxBookRepo;
use crate::service::book_service::BookService;
use crate::state::AppState;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env()?;
    let pool = create_pool(&cfg.database_url).await?;
    run_migrations(&pool).await?;

    let repo: Arc<dyn crate::repo::BookRepository> = Arc::new(SqlxBookRepo::new(pool.clone()));
    let service = Arc::new(BookService::<dyn crate::repo::BookRepository>::new(repo));
    let state = AppState { book_service: service };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT])
        .expose_headers([axum::http::header::LOCATION])
        .allow_credentials(false);

    let app: Router = routes::router(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("shutdown signal received");
}
