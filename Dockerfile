FROM rust:latest as builder
WORKDIR /usr/src/factoring
COPY . .
RUN cargo build --release

# 실행 스테이지
FROM debian:buster-slim
COPY --from=builder /usr/src/factoring/target/release/factoring /usr/local/bin/factoring
CMD ["factoring"]
