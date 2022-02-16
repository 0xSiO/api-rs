use axum::http::Request;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, MakeSpan, TraceLayer},
};
use tracing::{Level, Span};

use crate::middleware::request_id::RequestId;

#[derive(Clone, Copy)]
pub struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let RequestId(id) = request.extensions().get().unwrap();

        tracing::info_span!(
            "request",
            %id,
            method = %request.method(),
            uri = %request.uri(),
            http_version = ?request.version(),
            headers = ?request.headers(),
            api_version = env!("CARGO_PKG_VERSION"),
        )
    }
}

pub fn middleware() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, CustomMakeSpan> {
    TraceLayer::new_for_http()
        .make_span_with(CustomMakeSpan)
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .include_headers(true),
        )
        .on_failure(DefaultOnFailure::new())
}
