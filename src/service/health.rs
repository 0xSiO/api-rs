use serde_json::{json, Value};
use sqlx::postgres::PgConnectionInfo;
use std::time::Instant;
use tracing::{error, instrument};
use uuid::Uuid;

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
        Err(error) => {
            let error_id = Uuid::new_v4();
            error!(%error_id, description = %error, "database health check failed");
            json!({
                "status": "down",
                "duration": elapsed,
                "error_id": error_id.to_string(),
                "error": error.to_string(),
            })
        }
    }
}
