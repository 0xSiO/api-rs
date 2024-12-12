use axum::{routing::get, Router};

use crate::{controller::docs, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/openapi.json", get(docs::openapi))
        .route("/", get(docs::swagger))
}
