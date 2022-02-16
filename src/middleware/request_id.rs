use axum::{http::Request, response::IntoResponse};
use axum_extra::middleware::Next;
use uuid::Uuid;

const HEADER_NAME: &str = "x-request-id";

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RequestId(pub Uuid);

pub async fn middleware<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(RequestId(id));
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert(HEADER_NAME, id.to_string().try_into().unwrap());
    res
}
