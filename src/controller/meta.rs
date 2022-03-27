use axum::{extract::Extension, response::IntoResponse, Json};
use serde_json::json;

use crate::{service::health, State};

pub async fn health(Extension(state): Extension<State>) -> impl IntoResponse {
    Json(json!({
        "db": health::db_check(state.db).await,
    }))
}
