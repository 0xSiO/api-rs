use anyhow::anyhow;
use axum::{http::StatusCode, routing::any, Router};

use crate::error::Error;

mod meta;

pub fn router() -> Router {
    Router::new().merge(meta::router()).fallback(any(|| async {
        Error::new(
            StatusCode::NOT_FOUND,
            anyhow!("requested endpoint could not be found"),
        )
    }))
}
