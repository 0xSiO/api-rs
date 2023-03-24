use anyhow::anyhow;
use axum::{http::StatusCode, routing::any, Router};

use crate::Error;

mod meta;

pub fn router() -> Router {
    Router::new()
        .nest("/meta", meta::router())
        .fallback(any(|| async {
            Error::new(
                StatusCode::NOT_FOUND,
                anyhow!("requested endpoint not found"),
            )
        }))
}
