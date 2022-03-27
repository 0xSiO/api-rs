use axum::{extract::Extension, response::IntoResponse, Json};
use serde_json::json;

use crate::{service::health, State};

pub async fn health(Extension(state): Extension<State>) -> impl IntoResponse {
    Json(json!({
        "database": health::db_check(&state).await,
    }))
}

pub async fn version() -> impl IntoResponse {
    Json(json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
