#!/bin/bash
# NexusRemote 构建脚本

set -e

echo "🚀 开始构建 NexusRemote v3.0..."

# 检查 Rust 工具链
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust 工具链未安装"
    echo "正在安装 Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "✅ Rust 版本: $(rustc --version)"
echo "✅ Cargo 版本: $(cargo --version)"

# 检查项目结构
echo "📁 检查项目结构..."
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Cargo.toml 不存在"
    exit 1
fi

# 更新依赖
echo "📦 更新依赖..."
cargo update

# 运行 clippy 检查
echo "🔍 运行代码检查..."
cargo clippy -- -D warnings || {
    echo "⚠️ Clippy 检查发现问题，但继续构建..."
}

# 运行测试
echo "🧪 运行单元测试..."
cargo test --lib || {
    echo "⚠️ 部分测试失败，但继续构建..."
}

# 构建项目
echo "🔨 构建项目..."
cargo build --release

# 构建 CLI
echo "🛠️ 构建 CLI 工具..."
cargo build --release --bin nexusremote

echo "✅ 构建完成！"
echo ""
echo "📊 构建结果:"
echo "  - 二进制文件: target/release/nexusremote"
echo "  - 库文件: target/release/libnexusremote.rlib"
echo ""
echo "🚀 运行测试: cargo test"
echo "📦 发布构建: cargo build --release"
echo "🔍 代码检查: cargo clippy"
