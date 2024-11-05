#![forbid(unsafe_code)]

use anyhow::Context;
use axum::middleware::from_fn;
use sqlx::PgPool;
use tokio::net::TcpListener;

pub mod controller;
mod error;
pub mod middleware;
pub mod model;
pub mod route;
pub mod service;

pub use error::Error;

#[derive(Debug, Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db = PgPool::connect(&dotenvy::var("DATABASE_URL").unwrap())
        .await
        .context("failed to initialize DB pool")?;

    let app = route::router().with_state(AppState { db }).layer(
        tower::ServiceBuilder::new()
            .layer(from_fn(middleware::request_id))
            .layer(from_fn(middleware::trace)),
    );

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind server to address")?;

    tracing::info!(addr = %listener.local_addr()?, "starting server");
    Ok(axum::serve(listener, app).await?)
}
