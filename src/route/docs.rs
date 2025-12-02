use axum::{Router, routing::get};

use crate::{AppState, controller::docs};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/openapi.json", get(docs::openapi))
        .route("/", get(docs::swagger))
}
