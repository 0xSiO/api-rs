# api-rs

An opinionated template for lightweight server-side web APIs written in Rust.

## Preface

The goal behind this template is to produce a simple, scalable project skeleton
for a typical web service.

The opinions laid out below are just that - opinions. Do not take them as
gospel. Make adjustments to suit the needs of your application and your team.

## Setup

Use a `.env` file to store configuration locally. This is loaded by
[`dotenvy`](https://crates.io/crates/dotenvy) upon startup. In a cloud
environment, use the provider's tools to configure environment variables set
during deployment.

Use [`tracing`](https://crates.io/crates/tracing) for instrumentation and
structured logging. You should choose a subscriber implementation that uses a
format suitable for your log management service (you're using a log management
service, right?) - see the list of crates
[here](https://docs.rs/tracing/latest/tracing/#related-crates).

Use [`sqlx`](https://crates.io/crates/sqlx) with
[PostgreSQL](https://www.postgresql.org/) for persistence. If you must use an
ORM, you should use [ormx](https://crates.io/crates/ormx) (simple) or
[sea-orm](https://crates.io/crates/sea-orm) (complex).

## Routes

Routers should be created for each "section" of your service and merged into a
single router in `src/route.rs`. How you define "section" is up to you. You can
create a new router by creating a submodule of `src/route.rs`.

### Examples
- By default, `src/route/meta.rs` contains a router with a `/health` and a
  `/version` endpoint.
- A hypothetical `src/route/user.rs` might contain a router pointing to
  handlers that perform CRUD operations on user models.
- A hypothetical `src/route/authn.rs` might contain a router pointing to
  several authentication-related handlers.

## Middleware

Some of your routes might require middleware - if the middleware is specific to
a few related routes, define it in the module that also defines those routes.
If the middleware can be shared across many routes, add it to a submodule of
`src/middleware.rs`, or directly to `src/middleware.rs` if you prefer.

### Examples
- By default, every request is instrumented using `tracing` and contains a
  request ID stored in the request extensions. Have a look at
  `src/middleware.rs` for details on how this works.
- In a hypothetical `src/route/secure.rs`, one might want middleware to
  extract/verify the value of the
  [`Authorization`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Authorization)
  header. Middleware can be
  [defined](https://docs.rs/axum/latest/axum/middleware/index.html#writing-middleware)
  in the same file and used in the required routes.
- Let's say you want the entire service to be behind authentication instead -
  you could place the middleware described above in
  `src/middleware/authorization.rs`, or perhaps in `src/middleware.rs`, and
  include it in the middleware stack of the main router in `src/route.rs` (or
  add it to the `ServiceBuilder` layers in `src/main.rs`).

## Controllers

Controllers are collections of
[handlers](https://docs.rs/axum/latest/axum/handler/index.html). Your routers
should ultimately point to these handlers. The two main concerns of the
controller handlers should be to interpret/validate a request and to construct
a response. Defer more complex tasks to service modules.

If interpreting/validating the request or building the response involves
behavior that can be shared across many routes, consider breaking this behavior
into middleware and add it to run before/after each applicable handler.

## Service Modules

If non-trivial logic is required for a particular handler, extract the logic to
a submodule of `src/service.rs` and call that from the handler instead.

### Examples
- By default, the `controller::meta::health` handler calls upon the
  `service::health` module to perform health checks. The
  `controller::meta::version` handler is trivial, so it does not require a
  service module.
- A hypothetical `controller::orders` module might have a handler to calculate
  the shipping cost for a given `order_id`. The handler should extract the
  `order_id` from the request, and call a service function in the
  `service::orders` module, which fetches the order details from the database,
  performs any needed calculations, and returns the result back to the handler.
  The handler should then use this result to construct the final response.

## Errors

Use [`anyhow`](https://crates.io/crates/anyhow) to add context to your errors
and `Result`s. If you are constructing an error response, you should include a
final line of user-friendly context, as well as any additional details.

Prefer wrapping errors with a `crate::Error`.

### Examples
```rust
use anyhow::{anyhow, bail, Context};
use axum::http::StatusCode;
use serde_json::json;

use crate::Error;

fn make_coffee() -> anyhow::Result<()> {
    bail!("making coffee currently unsupported")
}

async fn explicit_wrap() -> Result<impl IntoResponse, Error> {
    let _ = make_coffee()
        .context("failed to make coffee")
        .map_err(|err| {
            // Explicitly wrap the error
            Error::new(
                StatusCode::IM_A_TEAPOT,
                err.context("you can't make coffee, I'm a teapot!"),
            )
        })?;
    Ok(())
}

async fn implicit_wrap() -> Result<impl IntoResponse, Error> {
    // This will return a 500 error if it is not explicitly wrapped
    let _ = make_coffee().context("failed to make coffee")?;
    Ok(())
}

async fn with_details() -> Error {
    Error::new(
        StatusCode::SERVICE_UNAVAILABLE,
        anyhow!("service is temporarily down for maintenance"),
    )
    .details(json!({ "time_remaining": 999999 }))
}
```

## License

This template is available as open source under the terms of the MIT License.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you shall be licensed as MIT, without any
additional terms or conditions.
