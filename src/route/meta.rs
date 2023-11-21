use axum::{routing::get, Router};

use crate::{controller::meta, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(meta::health))
        .route("/version", get(meta::version))
}
