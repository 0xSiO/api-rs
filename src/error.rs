use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use tracing::{error, warn};

// TODO: Add ability to include additional info
pub struct Error<M: Display>(pub StatusCode, pub M);

impl<M: Display> IntoResponse for Error<M> {
    fn into_response(self) -> axum::response::Response {
        if self.0.is_client_error() {
            warn!(status = %self.0.as_u16(), description = %self.1, "client error");
        }

        if self.0.is_server_error() {
            error!(status = %self.0.as_u16(), description = %self.1, "server error");
        }

        (self.0, Json(json!({ "message": format!("{}", self.1), }))).into_response()
    }
}
