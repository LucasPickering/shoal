# Build the binary in a builder image
FROM rust:1.88-alpine AS builder
WORKDIR /app
RUN apk add curl musl-dev
COPY . .
RUN cargo build --release

# Copy the binary to a thin image
FROM alpine:latest
ENV HOST=0.0.0.0:80
ENV RUST_BACKTRACE=1
EXPOSE 80
WORKDIR /app
COPY --from=builder /app/target/release/shoal .
CMD ["/app/shoal"]
