#!/bin/bash
# NexusRemote立即构建脚本

set -e

echo "🔨 开始构建NexusRemote..."

# 激活Rust环境
if [ -f "$HOME/.rust_env" ]; then
    source "$HOME/.rust_env"
fi

# 检查Rust
if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ cargo未找到"
    echo "尝试使用rustup..."
    if command -v rustup >/dev/null 2>&1; then
        alias cargo="rustup run stable cargo"
    else
        echo "❌ rustup也未找到"
        exit 1
    fi
fi

echo "✅ 使用: $(cargo --version)"

# 清理之前的构建
echo "清理..."
cargo clean 2>/dev/null || true

# 检查项目
echo "检查项目..."
cargo check

# 运行测试
echo "运行测试..."
cargo test --lib || {
    echo "⚠️ 部分测试失败，但继续构建..."
}

# 构建
echo "构建发布版本..."
cargo build --release

# 构建CLI
echo "构建CLI工具..."
cargo build --release --bin nexusremote

echo ""
echo "🎉 构建完成!"
echo ""
echo "📊 构建结果:"
echo "   - 主库: target/release/libnexusremote.rlib"
echo "   - CLI工具: target/release/nexusremote"
echo ""
echo "🚀 运行测试:"
echo "   cargo test"
echo "   cargo run -- simulate"
echo "   cargo run -- test-routing"
