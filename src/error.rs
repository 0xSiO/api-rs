use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use tracing::{error, instrument, warn};
use uuid::Uuid;

pub struct Error {
    id: Uuid,
    status: StatusCode,
    inner: anyhow::Error,
    details: Option<Value>,
}

impl Error {
    pub fn new(status: StatusCode, error: impl Into<anyhow::Error>) -> Self {
        Self {
            id: Uuid::new_v4(),
            status,
            inner: error.into(),
            details: None,
        }
    }

    pub fn details(mut self, value: Value) -> Self {
        self.details = Some(value);
        self
    }
}

impl IntoResponse for Error {
    #[instrument(skip_all)]
    fn into_response(self) -> axum::response::Response {
        let error_id = self.id.to_string();
        let status = self.status.as_u16();
        let description = self.inner.to_string();
        let details = self.details.unwrap_or_else(|| json!({}));

        if self.status.is_client_error() {
            warn!(%status, %error_id, description = %self.inner, %details, "client error");
        }

        if self.status.is_server_error() {
            error!(%status, %error_id, description = ?self.inner, %details, "server error");
        }

        (
            self.status,
            Json(json!({ "id": error_id, "message": description, "details": details })),
        )
            .into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            err.context("an internal error occurred"),
        )
    }
}
