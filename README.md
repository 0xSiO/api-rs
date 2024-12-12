# api-rs

An opinionated template for lightweight server-side web APIs written in Rust.

## Preface

The goal of this template is to provide a simple, scalable project skeleton for
a typical web API service. The opinions laid out below are not gospel. Make
adjustments to suit the needs of your application and your team.

## Configuration

Use a `.env` file to store configuration locally. This is loaded by
[`dotenvy`](https://crates.io/crates/dotenvy) upon startup. In a cloud
environment, use the provider's tools to configure environment variables set
during deployment.

## Instrumentation

Use [`tracing`](https://crates.io/crates/tracing) for instrumentation and
structured logging. You should choose a subscriber implementation that uses a
format suitable for your log management service - see the list of crates
[here](https://docs.rs/tracing/latest/tracing/#related-crates).

## Database

Use [`sqlx`](https://crates.io/crates/sqlx) with
[PostgreSQL](https://www.postgresql.org/) for persistence. Use
[`sqlx-cli`](https://crates.io/crates/sqlx-cli) to handle migrations. Don't be
afraid of SQL. Try to ensure your queries are checked at compile-time.

Place models in `src/model.rs`, and derive `sqlx::FromRow` to allow you to use
your models with `sqlx::query_as!`.

## Routes

Routers should be created for each "section" of your service and nested under
the `Router` defined in `src/route.rs`. How you define "section" is up to you.
You can create a new router by creating a submodule of `src/route.rs`.

### Examples
- See `src/route/meta.rs` and `src/route/docs.rs`.
- A hypothetical `src/route/user.rs` might contain a router pointing to
  handlers that perform CRUD operations on user models.
- A hypothetical `src/route/authn.rs` might contain a router pointing to
  several authentication-related handlers.

## Middleware

Some of your routes might require
[middleware](https://docs.rs/axum/latest/axum/middleware/index.html#writing-middleware)
- if the middleware is specific to a few related routes, add it to the module
that also defines those routes. If the middleware can be shared across many
routes, add it to a submodule of `src/middleware.rs`, or directly to
`src/middleware.rs` if you prefer.

### Examples
- By default, every request is instrumented using `tracing` and contains a
  request ID stored in the request extensions. See `src/middleware.rs` for
  details on how this works.
- If you want your middleware to apply to the entire service, you can modify
  the `ServiceBuilder` layers in `src/main.rs`.

## Controllers

Controllers are collections of
[handlers](https://docs.rs/axum/latest/axum/handler/index.html). Your routes
should ultimately point to these handlers. The two main concerns of a handler
should be to interpret/validate a request and to construct a response. Defer
more complex tasks to service modules.

If interpreting/validating the request or building the response involves
behavior that can be shared across many routes, consider separating this
behavior into middleware.

## Service Modules

If non-trivial logic is required for a particular handler, extract the logic to
a submodule of `src/service.rs` and call that from the handler instead.

### Examples
- By default, the `controller::meta::health` handler calls upon the
  `service::health` module to perform health checks. The
  `controller::meta::version` handler is trivial, so it does not require a
  service module.
- A hypothetical `controller::orders` module might have a handler to calculate
  the shipping cost for some `order_id`. The handler should extract the
  `order_id` from the request, and call a service function in the
  `service::orders` module, which fetches the order details from the database,
  performs any needed calculations, and returns the result back to the handler.
  The handler should then use this result to construct the final response.

## Errors

Use [`anyhow`](https://crates.io/crates/anyhow) to add context to your errors
and `Result`s. If you are constructing an error response, you should include a
final line of user-friendly context, as well as any additional details.

Wrap handler errors in a `crate::Error`. For axum extractors, wrap rejections
in a `crate::Error` using
[`WithRejection`](https://docs.rs/axum-extra/latest/axum_extra/extract/struct.WithRejection.html).

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

## Documentation

API documentation is maintained inside the `docs` directory, which is a
Node.js/TypeScript project. Using the [OpenAPI
specification](https://spec.openapis.org/oas/latest.html), write your
documentation in `docs/openapi.ts`.

Inside the `docs` directory:
- Run `npm run build` to generate an `openapi.json`, which is used by the
  `/docs/openapi.json` endpoint.
- Run `npm run check` to type-check `openapi.ts` with `tsc`.
- Run `npm run format` for format `openapi.ts` with `prettier`.

## Testing

Coming soon.

## Contributing

- Contributions to this project must be submitted under the [project's license](./LICENSE).
- Contributors to this project must attest to the [Developer Certificate of Origin](https://developercertificate.org/) by including a `Signed-off-by` statement in all commit messages.
- All commits must have a valid digital signature.
