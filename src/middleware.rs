use std::time::Instant;

use axum::{
    http::{header, HeaderMap, Request},
    middleware::Next,
    response::IntoResponse,
};
use tracing::{info, instrument};
use uuid::Uuid;

const SENSITIVE_HEADERS: &[header::HeaderName] = &[
    header::AUTHORIZATION,
    header::PROXY_AUTHORIZATION,
    header::COOKIE,
    header::SET_COOKIE,
];

#[repr(transparent)]
struct RequestId(Uuid);

pub async fn request_id<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(RequestId(id));
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert("x-request-id", id.to_string().try_into().unwrap());
    res
}

fn redact_sensitive(map: &HeaderMap) -> HeaderMap {
    let mut map = map.clone();
    for name in SENSITIVE_HEADERS {
        if let header::Entry::Occupied(mut entry) = map.entry(name) {
            entry.insert_mult("[REDACTED]".parse().unwrap());
        }
    }
    map
}

#[instrument(skip_all, fields(
    request_id = %req.extensions().get::<RequestId>().unwrap().0,
    method = %req.method(),
    path = %req.uri().path(),
    http_version = ?req.version(),
    api_version = %env!("CARGO_PKG_VERSION"),
))]
pub async fn trace<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    info!(headers = ?redact_sensitive(req.headers()), "request");
    let start = Instant::now();
    let res = next.run(req).await;
    info!(
        status = res.status().as_u16(),
        duration = start.elapsed().as_millis() as usize,
        headers = ?redact_sensitive(res.headers()),
        "response"
    );
    res
}
