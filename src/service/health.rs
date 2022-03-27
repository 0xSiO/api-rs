use serde_json::{json, Value};
use sqlx::{postgres::PgConnectionInfo, PgPool};
use std::time::Instant;

pub async fn db_check(db: PgPool) -> Value {
    let start = Instant::now();

    // This will acquire a connection and ping the DB
    let result = db.acquire().await;
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
