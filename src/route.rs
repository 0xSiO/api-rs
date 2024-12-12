use anyhow::anyhow;
use axum::{http::StatusCode, routing::any, Router};

use crate::{AppState, Error};

mod docs;
mod meta;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/docs", docs::router())
        .nest("/meta", meta::router())
        .fallback(any(|| async {
            Error::new(
                StatusCode::NOT_FOUND,
                anyhow!("requested endpoint not found"),
            )
        }))
}
