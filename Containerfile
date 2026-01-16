FROM docker.io/oven/bun:latest AS docs
WORKDIR /build
COPY docs docs
RUN cd docs && bun install && bun check && bun openapi.ts > openapi.json

FROM docker.io/library/rust:alpine AS build
WORKDIR /build
COPY Cargo.toml Cargo.lock .
RUN mkdir src && touch src/lib.rs && cargo build --release && rm src/lib.rs
COPY --from=docs /build/docs/openapi.json docs/
COPY src src
RUN cargo build --release && strip target/release/api

FROM scratch
COPY --from=build /build/target/release/api .
CMD ["./api"]
