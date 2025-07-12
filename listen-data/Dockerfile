FROM rust:latest AS builder
WORKDIR /app

# 复制 Cargo 文件
COPY listen-data/Cargo.toml listen-data/Cargo.lock ./
COPY listen-tracing/Cargo.toml listen-tracing/Cargo.lock ./

# 创建虚拟项目以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码
COPY listen-data/src ./src
COPY listen-tracing/src ./listen-tracing/src

# 构建项目
RUN cargo build --release --bin indexer

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
	ca-certificates \
	openssl \
	libssl3 \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/indexer /usr/local/bin
ENTRYPOINT ["/usr/local/bin/indexer"]
