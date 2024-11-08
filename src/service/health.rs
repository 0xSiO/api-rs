use std::time::Instant;

use anyhow::Context;
use serde_json::{json, Value};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::AppState;

#[instrument(skip_all)]
pub async fn db_check(state: &AppState) -> Value {
    let start = Instant::now();

    // This will acquire a connection and ping the DB
    let result = state.db.acquire().await.context("failed to get connection");
    let elapsed = start.elapsed().as_millis() as usize;

    match result {
        Ok(conn) => json!({
            "status": "up",
            "duration": elapsed,
            "server_version": conn.server_version_num(),
        }),
        Err(error) => {
            let error_id = Uuid::now_v7();
            error!(%error_id, description = ?error, "database health check failed");
            json!({
                "status": "down",
                "duration": elapsed,
                "error_id": error_id.to_string(),
                "error": error.to_string(),
            })
        }
    }
}
