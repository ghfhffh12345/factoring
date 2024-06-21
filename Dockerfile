FROM rust:latest as builder

WORKDIR     /rust

# Download the cargo target
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

# create dummy application, s.t. cargo can download all dependencies
RUN         mkdir -p /rust/app/src && echo 'fn main(){}' > app/src/main.rs
WORKDIR     /rust/app
COPY static ./static

# Build & cache dependencies
COPY        Cargo.toml Cargo.lock build.rs ./
RUN         cargo build --release --target x86_64-unknown-linux-musl

# Copy application code
COPY        src ./src

# Build production binary
RUN         touch src/main.rs && apt-get install musl-tools && cargo build --release --target x86_64-unknown-linux-musl

# Production container
FROM        scratch
COPY        --from=builder /rust/app/target/x86_64-unknown-linux-musl/release/factoring /app
EXPOSE 8080
ENTRYPOINT  ["/app"]
