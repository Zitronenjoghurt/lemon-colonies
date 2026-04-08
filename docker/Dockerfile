FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /build
RUN rustup target add wasm32-unknown-unknown

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin lemon-colonies-server
RUN cargo build --release --target wasm32-unknown-unknown --bin lemon-colonies-app

RUN mkdir -p /build/static && \
    cp /build/target/wasm32-unknown-unknown/release/lemon-colonies-app.wasm /build/static/ && \
    cp /build/app/index.html /build/static/ && \
    cp -r /build/app/public /build/static/

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
WORKDIR /app
COPY --from=builder /build/target/release/lemon-colonies-server /usr/local/bin/server
COPY --from=builder /build/static /app/static
CMD ["/usr/local/bin/server"]