use axum::{extract::Extension, response::IntoResponse, Json};
use serde_json::json;
use tracing::instrument;

use crate::{service::health, State};

#[instrument(skip_all)]
pub async fn health(Extension(state): Extension<State>) -> impl IntoResponse {
    Json(json!({
        "database": health::db_check(&state).await,
    }))
}

#[instrument(skip_all)]
pub async fn version() -> impl IntoResponse {
    Json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
