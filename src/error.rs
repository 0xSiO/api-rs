use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tracing::{error, warn};
use uuid::Uuid;

pub struct Error<M: Display> {
    id: Uuid,
    status: StatusCode,
    message: M,
    details: Option<Value>,
}

impl<M: Display> Error<M> {
    pub fn new(status: StatusCode, message: M) -> Self {
        Self {
            id: Uuid::new_v4(),
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
        let id = self.id.to_string();
        let status = self.status.as_u16();
        let description = self.message.to_string();
        let details = self.details.unwrap_or_else(|| json!({}));

        if self.status.is_client_error() {
            warn!(%status, error_id = %id, %description, %details, "client error");
        }

        if self.status.is_server_error() {
            error!(%status, error_id = %id, %description, %details, "server error");
        }

        (
            self.status,
            Json(json!({ "id": id, "message": description, "details": details })),
        )
            .into_response()
    }
}
