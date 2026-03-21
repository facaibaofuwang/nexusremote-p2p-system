FROM rust:1.75-slim

WORKDIR /app

# 复制项目文件
COPY nexusremote/Cargo.toml nexusremote/Cargo.lock ./
COPY nexusremote/src ./src

# 创建虚拟main.rs以缓存依赖
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# 复制实际源代码
COPY nexusremote/src ./src

# 构建项目
RUN cargo build --release

# 运行测试
RUN cargo test

CMD ["cargo", "run", "--release"]