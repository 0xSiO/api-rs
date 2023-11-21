use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use tracing::instrument;

use crate::{service::health, AppState};

#[instrument(skip_all)]
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let (database,) = tokio::join!(health::db_check(&state));
    Json(json!({
        "database": database,
    }))
}

#[instrument(skip_all)]
pub async fn version() -> impl IntoResponse {
    Json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
