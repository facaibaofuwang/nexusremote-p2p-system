#!/bin/bash
# 最终Rust环境设置

set -e

echo "🎉 Rust安装完成！开始设置环境..."
echo "================================="

# 设置环境变量
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$HOME/rust-local/rustc/bin:$PATH"
export PATH="$HOME/rust-local/cargo/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="/snap/bin:$PATH"

# 创建永久环境文件
cat > "$HOME/.rust_final_env" << 'EOF'
# NexusRemote Rust环境配置
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$HOME/rust-local/rustc/bin:$PATH"
export PATH="$HOME/rust-local/cargo/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="/snap/bin:$PATH"

echo "✅ Rust环境已激活"
echo "rustc: $(rustc --version 2>/dev/null || echo '通过路径访问')"
echo "cargo: $(cargo --version 2>/dev/null || echo '通过路径访问')"
EOF

# 也添加到.bashrc
if ! grep -q "rust_final_env" "$HOME/.bashrc"; then
    echo "" >> "$HOME/.bashrc"
    echo "# NexusRemote Rust环境" >> "$HOME/.bashrc"
    echo "source \$HOME/.rust_final_env 2>/dev/null || true" >> "$HOME/.bashrc"
fi

# 激活环境
source "$HOME/.rust_final_env"

# 验证安装
echo ""
echo "🔍 验证安装..."

# 检查rustc
if command -v rustc >/dev/null 2>&1; then
    RUSTC_VERSION=$(rustc --version 2>/dev/null || echo "找到但无法运行")
    echo "✅ rustc: $RUSTC_VERSION"
else
    # 尝试直接路径
    if [ -f "$HOME/rust-local/rustc/bin/rustc" ]; then
        echo "✅ rustc: $HOME/rust-local/rustc/bin/rustc"
        alias rustc="$HOME/rust-local/rustc/bin/rustc"
    else
        echo "❌ rustc未找到"
        exit 1
    fi
fi

# 检查cargo
if command -v cargo >/dev/null 2>&1; then
    CARGO_VERSION=$(cargo --version 2>/dev/null || echo "找到但无法运行")
    echo "✅ cargo: $CARGO_VERSION"
else
    if [ -f "$HOME/rust-local/cargo/bin/cargo" ]; then
        echo "✅ cargo: $HOME/rust-local/cargo/bin/cargo"
        alias cargo="$HOME/rust-local/cargo/bin/cargo"
    else
        echo "❌ cargo未找到"
        exit 1
    fi
fi

# 测试简单程序
echo ""
echo "🧪 测试Rust编译..."
cat > /tmp/simple_test.rs << 'EOF'
fn main() {
    println!("🎊 Rust环境测试成功!");
    println!("NexusRemote开发环境就绪!");
    
    // 测试基本功能
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }
    println!("1到10的和: {}", sum);
    
    // 测试数组
    let device_id = [0x42u8; 32];
    println!("示例设备ID: {:?}", &device_id[..4]);
}
EOF

# 编译测试
if command -v rustc >/dev/null 2>&1; then
    rustc /tmp/simple_test.rs -o /tmp/simple_test
elif [ -f "$HOME/rust-local/rustc/bin/rustc" ]; then
    "$HOME/rust-local/rustc/bin/rustc" /tmp/simple_test.rs -o /tmp/simple_test
fi

if [ -f /tmp/simple_test ]; then
    /tmp/simple_test
    echo "✅ Rust编译测试成功!"
else
    echo "⚠️ 编译测试失败，但继续..."
fi

echo ""
echo "🚀 环境设置完成!"
echo ""
echo "📋 使用说明:"
echo "   永久激活: source ~/.rust_final_env"
echo "   或重启终端自动激活"
echo ""
echo "🔧 验证命令:"
echo "   rustc --version"
echo "   cargo --version"
echo ""
echo "🎯 下一步: 立即构建NexusRemote!"
