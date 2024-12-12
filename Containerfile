FROM docker.io/library/rust:alpine AS build
RUN apk add --no-cache libc-dev nodejs npm
WORKDIR /build
COPY Cargo.toml Cargo.lock .
RUN mkdir src && touch src/lib.rs && cargo build --release && rm src/lib.rs
COPY docs docs
RUN cd docs && npm install && npm run check && npm run build
COPY src src
RUN cargo build --release && strip target/release/api

FROM scratch
COPY --from=build /build/target/release/api .
CMD ["./api"]
