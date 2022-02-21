use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tracing::{error, warn};

pub struct Error<M: Display> {
    status: StatusCode,
    message: M,
    details: Option<Value>,
}

impl<M: Display> Error<M> {
    pub fn new(status: StatusCode, message: M) -> Self {
        Self {
            status,
            message,
            details: None,
        }
    }

    pub fn details(mut self, value: Value) -> Self {
        self.details = Some(value);
        self
    }
}

impl<M: Display> IntoResponse for Error<M> {
    fn into_response(self) -> axum::response::Response {
        let status = self.status.as_u16();
        let details = self.details.unwrap_or_else(|| json!({}));

        if self.status.is_client_error() {
            warn!(%status, description = %self.message, %details, "client error");
        }

        if self.status.is_server_error() {
            error!(%status, description = %self.message, %details, "server error");
        }

        (
            self.status,
            Json(json!({ "message": format!("{}", self.message), "details": details })),
        )
            .into_response()
    }
}
