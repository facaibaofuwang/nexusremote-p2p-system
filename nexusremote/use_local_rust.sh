#!/bin/bash
# 使用本地安装的Rust（绕过rustup）

set -e

echo "🔧 使用本地Rust安装"
echo "=================="

# 直接设置路径
export RUSTC_PATH="$HOME/rust-local/rustc/bin/rustc"
export CARGO_PATH="$HOME/rust-local/cargo/bin/cargo"

if [ ! -f "$RUSTC_PATH" ]; then
    echo "❌ rustc未找到: $RUSTC_PATH"
    exit 1
fi

if [ ! -f "$CARGO_PATH" ]; then
    echo "❌ cargo未找到: $CARGO_PATH"
    exit 1
fi

echo "✅ rustc: $RUSTC_PATH"
echo "✅ cargo: $CARGO_PATH"

# 创建别名
alias rustc="$RUSTC_PATH"
alias cargo="$CARGO_PATH"

# 测试
echo ""
echo "🧪 测试本地Rust..."
cat > /tmp/local_test.rs << 'EOF'
fn main() {
    println!("🚀 本地Rust测试成功!");
    println!("版本: rustc 1.75.0");
    
    let x: i32 = 42;
    let y: i32 = 58;
    println!("计算: {} + {} = {}", x, y, x + y);
    
    // 测试数组
    let arr = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);
}
EOF

# 直接使用rustc编译
"$RUSTC_PATH" /tmp/local_test.rs -o /tmp/local_test

if [ -f /tmp/local_test ]; then
    /tmp/local_test
    echo "✅ 本地Rust编译成功!"
else
    echo "❌ 编译失败"
    exit 1
fi

# 创建cargo包装脚本
cat > "$HOME/.cargo_wrapper.sh" << EOF
#!/bin/bash
# Cargo包装脚本

export RUSTC="$RUSTC_PATH"
"$CARGO_PATH" "\$@"
EOF

chmod +x "$HOME/.cargo_wrapper.sh"

echo ""
echo "🎉 本地Rust环境就绪!"
echo ""
echo "📋 使用方法:"
echo "   编译Rust文件: $RUSTC_PATH your_file.rs"
echo "   使用Cargo: $CARGO_PATH build"
echo "   或使用包装脚本: $HOME/.cargo_wrapper.sh build"
echo ""
echo "🚀 立即构建NexusRemote:"
echo "   cd /home/admin/.openclaw/workspace/nexusremote"
echo "   $CARGO_PATH check"
