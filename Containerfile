FROM docker.io/library/rust:alpine AS build
RUN apk add --no-cache libc-dev
WORKDIR /build
COPY Cargo.toml Cargo.lock .
RUN mkdir src && touch src/lib.rs && cargo build --release && rm src/lib.rs
COPY src src
RUN cargo build --release && strip target/release/api

FROM docker.io/library/alpine:latest
WORKDIR /app
COPY --from=build /build/target/release/api .
CMD ["./api"]
