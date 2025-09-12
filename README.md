README.md
===

# Cross-compilation

## With Cargo

```bash
# Add target
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin

# Compile for target
cargo build --target x86_64-pc-windows-gnu
cargo build --target aarch64-unknown-linux-gnu
```

## With docker

```bash
docker build --platform linux/amd64,linux/arm64  -t rust-skel .
```


# Questionable 


## Install cross-compilation toolchains

brew install musl-cross

or 
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-musl
brew install x86_64-unknown-linux-musl


## With cross

Somehow doesn't work now

```bash
# Install cross
cargo install cross

# Use it (instead of cargo)
cross build --target x86_64-pc-windows-gnu
cross build --target aarch64-unknown-linux-gnu
cross test --target armv7-unknown-linux-gnueabihf
```


## Build for different targets (uses Docker internally)

Need
docker pull --platform linux/x86_64 ghcr.io/cross-rs/x86_64-unknown-linux-gnu

docker run --privileged --rm tonistiigi/binfmt --install amd64
rustup toolchain install stable-x86_64-unknown-linux-gnu --force-non-host


cross build --release --target x86_64-unknown-linux-musl --force-non-host
cross build --release --target aarch64-unknown-linux-musl --force-non-hos

