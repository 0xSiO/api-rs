use serde_json::{json, Value};
use sqlx::postgres::PgConnectionInfo;
use std::time::Instant;
use tracing::instrument;

use crate::State;

#[instrument(skip_all)]
pub async fn db_check(state: &State) -> Value {
    let start = Instant::now();

    // This will acquire a connection and ping the DB
    let result = state.db.acquire().await;
    let elapsed = start.elapsed().as_millis() as usize;

    match result {
        Ok(conn) => json!({
            "status": "up",
            "duration": elapsed,
            "server_version": conn.server_version_num(),
        }),
        Err(err) => json!({
            "status": "down",
            "duration": elapsed,
            "error": err.to_string(),
        }),
    }
}
