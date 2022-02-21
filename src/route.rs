use crate::error::Error;
use axum::{
    http::StatusCode,
    routing::{any, get},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .fallback(any(|| async {
            Error::new(
                StatusCode::NOT_FOUND,
                "The requested endpoint could not be found.",
            )
        }))
}
