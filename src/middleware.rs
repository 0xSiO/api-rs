use std::time::Instant;

use axum::{http::Request, response::IntoResponse};
use axum_extra::middleware::Next;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RequestId(pub Uuid);

pub async fn request_id<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(RequestId(id));
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert("x-request-id", id.to_string().try_into().unwrap());
    res
}

#[instrument(skip_all, fields(
    id = %req.extensions().get::<RequestId>().unwrap().0,
    method = %req.method(),
    path = %req.uri().path(),
    http_version = ?req.version(),
    api_version = env!("CARGO_PKG_VERSION"),
))]
pub async fn trace<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    info!(headers = ?req.headers(), "request");
    let start = Instant::now();
    let res = next.run(req).await;
    info!(
        status = res.status().as_u16(),
        // If your requests are taking longer than u128::MAX, you've got a problem
        duration = start.elapsed().as_millis() as usize,
        headers = ?res.headers(),
        "response"
    );
    res
}
