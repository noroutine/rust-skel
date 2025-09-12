# Flamegraph

You need XCode
```bash
cargo install flamegraph
sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
cargo flamegraph --dev # for release you need to leave debug symbols in Cargo.toml

# in another terminal

wrk -t12 -c400 -d30s http://localhost:3000/
```

Tracing accounted for ~5k rps drop (86k -> 81k) with a simple static html handler

# Run jaeger

```bash
# 4317 for gRPC, 4318 for HTTP
docker run -p 16686:16686 -p 4317:4317 -p 4318:4318 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
```

Access UI at http://localhost:16686

# Control logging

You can get granular control with `RUST_LOG`! Here are a few approaches:

tower_http is configured to create spans at info level

RUST_LOG=info,rust_skel=debug cargo run

## Option 1: Target Only Your Crate

```bash
# Only show debug from your crate, others at default level
RUST_LOG=server=debug cargo run --bin server
RUST_LOG=client=debug cargo run --bin client
```

## Option 2: Your Crate Debug + Others at Higher Levels

```bash
# Your crate: debug, everything else: warn
RUST_LOG=server=debug,warn cargo run --bin server

# Your crate: debug, tokio: info, everything else: warn  
RUST_LOG=server=debug,tokio=info,warn cargo run --bin server
```

## Option 3: Suppress Noisy Dependencies

```bash
# Debug everything but silence the noisy ones
RUST_LOG=debug,hyper=warn,tokio=warn,opentelemetry=warn cargo run --bin server
```

## Option 4: Programmatic Control

You can also set this in code instead of environment variables:

```rust
tracing_subscriber::registry()
    .with(EnvFilter::new("server=debug,warn"))  // Your crate=debug, others=warn
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_opentelemetry::layer().with_tracer(tracer))
    .init();
```
# Tokio Console

Use tokio=trace,runtime=trace

```bash
RUST_LOG=debug,tokio=trace,runtime=trace cargo run
```