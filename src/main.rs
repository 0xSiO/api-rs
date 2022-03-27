use axum::{extract::Extension, middleware::from_fn};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

pub mod error;
pub mod middleware;
pub mod route;

#[derive(Debug, Clone)]
struct State {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let state = State { db };

    let app = route::router().layer(
        ServiceBuilder::new()
            .layer(from_fn(middleware::request_id))
            .layer(from_fn(middleware::trace))
            .layer(Extension(state)),
    );

    let server =
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    tracing::info!(addr = %server.local_addr(), "starting server");
    server.await.unwrap();
}
