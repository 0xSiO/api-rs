use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tracing::{error, warn};
use uuid::Uuid;

// TODO: Ability to add backtrace and cause
pub struct Error {
    id: Uuid,
    status: StatusCode,
    message: String,
    details: Option<Value>,
}

impl Error {
    pub fn new(status: StatusCode, message: impl ToString) -> Self {
        Self {
            id: Uuid::new_v4(),
            status,
            message: message.to_string(),
            details: None,
        }
    }

    pub fn details(mut self, value: Value) -> Self {
        self.details = Some(value);
        self
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error_id = self.id.to_string();
        let status = self.status.as_u16();
        let description = self.message;
        let details = self.details.unwrap_or_else(|| json!({}));

        if self.status.is_client_error() {
            warn!(%status, %error_id, %description, %details, "client error");
        }

        if self.status.is_server_error() {
            error!(%status, %error_id, %description, %details, "server error");
        }

        (
            self.status,
            Json(json!({ "id": error_id, "message": description, "details": details })),
        )
            .into_response()
    }
}

impl<E: std::error::Error> From<E> for Error {
    fn from(err: E) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, err)
    }
}
