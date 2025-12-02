use axum::{Router, routing::get};

use crate::{AppState, controller::meta};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(meta::health))
        .route("/version", get(meta::version))
}
