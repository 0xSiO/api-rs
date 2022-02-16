use axum_extra::middleware::from_fn;
use tower::ServiceBuilder;
use tracing::*;
use tracing_subscriber::EnvFilter;

pub mod middleware;
pub mod routing;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = routing::router().layer(
        ServiceBuilder::new()
            .layer(from_fn(middleware::request_id::middleware))
            .layer(middleware::tracing::middleware()),
    );

    info!("starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
