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
            id: Uuid::now_v7(),
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

macro_rules! impl_from_rejection {
    ($type:ty) => {
        impl From<$type> for Error {
            fn from(rej: $type) -> Self {
                Self::new(rej.status(), rej)
            }
        }
    };
}

impl_from_rejection!(axum::extract::rejection::BytesRejection);
impl_from_rejection!(axum::extract::rejection::ExtensionRejection);
impl_from_rejection!(axum::extract::rejection::FormRejection);
impl_from_rejection!(axum::extract::rejection::JsonRejection);
impl_from_rejection!(axum::extract::rejection::NestedPathRejection);
impl_from_rejection!(axum::extract::rejection::PathRejection);
impl_from_rejection!(axum::extract::rejection::QueryRejection);
impl_from_rejection!(axum::extract::rejection::RawFormRejection);
impl_from_rejection!(axum::extract::rejection::RawPathParamsRejection);
impl_from_rejection!(axum::extract::rejection::StringRejection);
impl_from_rejection!(axum_extra::extract::rejection::HostRejection);
