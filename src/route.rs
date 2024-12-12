use anyhow::anyhow;
use axum::{
    http::StatusCode,
    routing::{any, get},
    Router,
};

use crate::{controller::meta, AppState, Error};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest(
            "/meta",
            Router::new()
                .route("/health", get(meta::health))
                .route("/version", get(meta::version)),
        )
        .fallback(any(|| async {
            Error::new(
                StatusCode::NOT_FOUND,
                anyhow!("requested endpoint not found"),
            )
        }))
}
