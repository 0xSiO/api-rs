use axum::{
    http::StatusCode,
    routing::{any, get},
    Router,
};

use crate::{controller::meta, error::Error};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(meta::health))
        .fallback(any(|| async {
            Error::new(
                StatusCode::NOT_FOUND,
                "The requested endpoint could not be found.",
            )
        }))
}
