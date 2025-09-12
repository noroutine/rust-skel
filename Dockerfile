# Multi-stage Dockerfile for static Rust binary - simplified!

# Build stage
FROM --platform=$BUILDPLATFORM rust:slim as builder

# Install musl targets and cross-compilation toolchain
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y musl-tools gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu

# Determine target based on platform
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
        "linux/amd64") echo "x86_64-unknown-linux-musl" > /target ;; \
        "linux/arm64") echo "aarch64-unknown-linux-musl" > /target ;; \
        *) echo "Unsupported: $TARGETPLATFORM" && exit 1 ;; \
    esac

WORKDIR /usr/src/app

# Copy everything (including .cargo/config.toml)
COPY . .
RUN cp .cargo/config.toml.docker .cargo/config.toml

# Build for target platform
RUN cargo build --release --target $(cat /target) && \
    cp target/$(cat /target)/release/rust-skel /rust-skel

# Runtime stage - scratch!
FROM scratch
COPY --from=builder /rust-skel /rust-skel
CMD ["/rust_skel"]