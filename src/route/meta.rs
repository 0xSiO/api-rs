use axum::{routing::get, Router};

use crate::controller::meta;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(meta::health))
        .route("/version", get(meta::version))
}
