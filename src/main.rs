use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tracing::*;

pub mod logging;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    logging::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(ServiceBuilder::new().layer(logging::middleware()));

    info!("starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
