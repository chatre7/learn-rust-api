# Multi-stage build
FROM rust:1.80 as builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
COPY migrations ./migrations
# Use nightly to satisfy dependencies that require edition2024 during build
RUN rustup toolchain install nightly --profile minimal \
    && rustup default nightly \
    && cargo build --release

FROM gcr.io/distroless/cc-debian12:latest
WORKDIR /app
ENV RUST_LOG=info
COPY --from=builder /app/target/release/rust_api /app/rust_api
COPY migrations /app/migrations
EXPOSE 8080
CMD ["/app/rust_api"]
