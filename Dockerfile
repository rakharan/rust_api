# ---------------------------------------------------
# 1. Builder Stage
# ---------------------------------------------------
FROM rust:alpine AS builder

# pkgconfig: helps rust find the libraries
# openssl-dev: headers for compiling
# openssl-libs-static: THE FIX. Contains libssl.a and libcrypto.a
# musl-dev: standard C library headers
RUN apk add --no-cache pkgconfig openssl-dev musl-dev openssl-libs-static

WORKDIR /usr/src/app
COPY . .

# Build for release.
RUN cargo build --release

# ---------------------------------------------------
# 2. Runtime Stage
# ---------------------------------------------------
FROM alpine:latest

# We need the runtime library because we are on Alpine
RUN apk add --no-cache openssl libgcc

COPY --from=builder /usr/src/app/target/release/rust_api /usr/local/bin/rust_api

WORKDIR /usr/local/bin
EXPOSE 3000

CMD ["rust_api"]