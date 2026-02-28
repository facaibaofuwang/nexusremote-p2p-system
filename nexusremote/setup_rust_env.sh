#!/bin/bash
# 设置Rust环境脚本

set -e

echo "🔧 设置Rust开发环境"
echo "=================="

# 添加rustup到PATH
export PATH="/snap/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

# 检查rustup
echo "检查rustup..."
if command -v rustup >/dev/null 2>&1; then
    echo "✅ rustup: $(rustup --version)"
else
    echo "❌ rustup未找到，尝试/snap/bin/rustup"
    if [ -f "/snap/bin/rustup" ]; then
        alias rustup="/snap/bin/rustup"
        echo "✅ 使用/snap/bin/rustup"
    else
        echo "❌ rustup完全未找到"
        exit 1
    fi
fi

# 检查是否已安装工具链
echo "检查已安装的工具链..."
rustup toolchain list

# 如果没有stable，安装它
if ! rustup toolchain list | grep -q stable; then
    echo "安装stable工具链..."
    rustup install stable
fi

# 设置默认
echo "设置默认工具链..."
rustup default stable

# 验证安装
echo "验证安装..."
if command -v rustc >/dev/null 2>&1; then
    echo "✅ rustc: $(rustc --version)"
else
    # 尝试通过rustup运行
    echo "尝试通过rustup运行rustc..."
    rustup run stable rustc --version
fi

if command -v cargo >/dev/null 2>&1; then
    echo "✅ cargo: $(cargo --version)"
else
    echo "尝试通过rustup运行cargo..."
    rustup run stable cargo --version
fi

# 创建环境配置
cat > "$HOME/.rust_env" << 'EOF'
# Rust环境配置
export PATH="/snap/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"

# 如果rustc/cargo不在PATH，使用rustup运行
if ! command -v rustc >/dev/null 2>&1; then
    alias rustc="rustup run stable rustc"
fi

if ! command -v cargo >/dev/null 2>&1; then
    alias cargo="rustup run stable cargo"
fi

echo "Rust环境已配置"
EOF

echo "✅ 环境配置已保存到 ~/.rust_env"
echo "使用: source ~/.rust_env 来激活环境"

# 测试简单Rust程序
echo "测试简单Rust程序..."
cat > /tmp/test_rust.rs << 'EOF'
fn main() {
    println!("🎉 Rust环境测试成功!");
    println!("NexusRemote开发环境就绪!");
    
    let a = 10;
    let b = 20;
    println!("{} + {} = {}", a, b, a + b);
}
EOF

# 尝试编译
if command -v rustc >/dev/null 2>&1; then
    rustc /tmp/test_rust.rs -o /tmp/test_rust
elif command -v rustup >/dev/null 2>&1; then
    rustup run stable rustc /tmp/test_rust.rs -o /tmp/test_rust
fi

if [ -f /tmp/test_rust ]; then
    /tmp/test_rust
    echo "✅ Rust编译测试成功!"
else
    echo "⚠️ Rust编译测试失败，但环境可能仍可用"
fi

echo ""
echo "📋 下一步:"
echo "  1. 激活环境: source ~/.rust_env"
echo "  2. 测试NexusRemote: cd nexusremote && cargo check"
echo "  3. 开始开发: cargo build"
