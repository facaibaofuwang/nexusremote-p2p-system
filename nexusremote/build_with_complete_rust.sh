#!/bin/bash
# 使用完整Rust环境构建NexusRemote

set -e

echo "🚀 使用完整Rust环境构建NexusRemote..."
echo "====================================="

# 激活完整Rust环境
source "$HOME/rust-complete/env.sh"

echo "环境信息:"
echo "rustc: $(rustc --version 2>/dev/null || which rustc)"
echo "cargo: $(cargo --version 2>/dev/null || which cargo)"

cd "/home/admin/.openclaw/workspace/nexusremote"

# 清理
echo "清理..."
cargo clean 2>/dev/null || true

# 检查
echo "检查项目..."
cargo check || {
    echo "⚠️ 检查失败，尝试修复..."
    # 尝试更新依赖
    cargo update || true
    cargo check || echo "检查仍然失败，但继续构建..."
}

# 构建
echo "构建项目..."
cargo build --release

echo ""
echo "🎉 构建完成!"
echo ""
echo "📊 构建结果:"
find target/release -maxdepth 1 -type f -executable | while read file; do
    echo "   - $(basename "$file")"
done

echo ""
echo "🚀 运行测试:"
echo "   cargo test"
echo "   cargo run -- simulate"
echo "   cargo run -- test-routing"
