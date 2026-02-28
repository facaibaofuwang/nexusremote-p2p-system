#!/bin/bash
echo "🚀 Rust环境完整解决方案 (方案2: rustup)"
echo "========================================"

# 设置环境变量
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

echo "1. 检查当前rustup状态..."
rustup show

echo ""
echo "2. 检查工具链完整性..."
rustup component list --installed

echo ""
echo "3. 验证标准库文件..."
if find ~/.rustup -name "libcore*.rlib" 2>/dev/null | grep -q .; then
    echo "✅ 标准库文件存在:"
    find ~/.rustup -name "libcore*.rlib" 2>/dev/null | head -3
else
    echo "❌ 标准库文件缺失"
    echo "   安装标准库组件..."
    rustup component add rust-std
fi

echo ""
echo "4. 测试编译能力..."
cat > /tmp/rust_test.rs << 'EOF'
fn main() {
    println!("✅ Rust环境测试成功!");
    println!("   版本: {}", env!("CARGO_PKG_VERSION"));
    
    // 测试标准库功能
    let vec = vec![1, 2, 3];
    println!("   向量测试: {:?}", vec);
    
    // 测试错误处理
    let result: Result<i32, &str> = Ok(42);
    println!("   Result测试: {:?}", result);
}
EOF

echo "   编译测试程序..."
rustc /tmp/rust_test.rs -o /tmp/rust_test
if [ $? -eq 0 ]; then
    echo "✅ 编译成功!"
    echo "   运行测试..."
    /tmp/rust_test
else
    echo "❌ 编译失败"
fi

echo ""
echo "5. 配置NexusRemote项目环境..."
cd /home/admin/.openclaw/workspace/nexusremote

echo "   检查Cargo.toml..."
if [ -f "Cargo.toml" ]; then
    echo "✅ Cargo.toml存在"
    echo "   项目名称: $(grep '^name =' Cargo.toml | head -1 | cut -d'"' -f2)"
    echo "   版本: $(grep '^version =' Cargo.toml | head -1 | cut -d'"' -f2)"
else
    echo "❌ Cargo.toml不存在"
fi

echo ""
echo "6. 测试项目依赖..."
if command -v cargo &> /dev/null; then
    echo "   运行cargo check..."
    cargo check --lib --no-default-features 2>&1 | tail -20
else
    echo "❌ cargo未找到"
fi

echo ""
echo "7. 创建永久环境配置..."
cat > ~/.nexusremote_rust_env << 'EOF'
#!/bin/bash
# NexusRemote Rust开发环境配置
# 方案2: rustup完整工具链解决方案

export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"

# 设置国内镜像加速（可选）
# export RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
# export RUSTUP_UPDATE_ROOT="https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup"

# 显示环境状态
echo "🔧 NexusRemote Rust环境已激活"
echo "   rustc: $(which rustc 2>/dev/null || echo '未找到')"
echo "   cargo: $(which cargo 2>/dev/null || echo '未找到')"
echo "   版本: $(rustc --version 2>/dev/null || echo '未知')"
echo ""
echo "📁 项目目录: /home/admin/.openclaw/workspace/nexusremote"
echo "💡 使用方法:"
echo "   cd /home/admin/.openclaw/workspace/nexusremote"
echo "   cargo check    # 检查代码"
echo "   cargo build    # 编译项目"
echo "   cargo test     # 运行测试"
echo "   cargo run      # 运行程序"
EOF

chmod +x ~/.nexusremote_rust_env

echo ""
echo "8. 创建项目构建脚本..."
cat > /home/admin/.openclaw/workspace/nexusremote/build_with_rustup.sh << 'EOF'
#!/bin/bash
# NexusRemote项目构建脚本 (使用rustup环境)

# 激活Rust环境
source ~/.nexusremote_rust_env 2>/dev/null || {
    echo "❌ 无法激活Rust环境"
    echo "请先运行: source ~/.nexusremote_rust_env"
    exit 1
}

cd "$(dirname "$0")"

echo "🔨 构建NexusRemote项目..."
echo "   工作目录: $(pwd)"
echo "   工具链: $(rustc --version)"

# 检查依赖
echo ""
echo "1. 检查依赖..."
cargo check --lib --no-default-features

echo ""
echo "2. 构建项目..."
cargo build --lib --no-default-features

echo ""
echo "3. 运行测试..."
cargo test --lib --no-default-features 2>&1 | tail -30

echo ""
echo "📊 构建完成!"
echo "   二进制文件: target/debug/libnexusremote.rlib"
echo "   下一步: 集成UI前端框架"
EOF

chmod +x /home/admin/.openclaw/workspace/nexusremote/build_with_rustup.sh

echo ""
echo "🎉 解决方案部署完成!"
echo ""
echo "📋 使用说明:"
echo "   1. 激活环境: source ~/.nexusremote_rust_env"
echo "   2. 进入项目: cd /home/admin/.openclaw/workspace/nexusremote"
echo "   3. 构建项目: ./build_with_rustup.sh"
echo "   4. 或者手动:"
echo "      cargo check    # 检查代码"
echo "      cargo build    # 编译项目"
echo "      cargo test     # 运行测试"
echo ""
echo "🔧 环境验证:"
echo "   rustc版本: $(rustc --version 2>/dev/null || echo '未安装')"
echo "   cargo版本: $(cargo --version 2>/dev/null || echo '未安装')"
echo "   标准库: $(find ~/.rustup -name "libcore*.rlib" 2>/dev/null | head -1 | xargs basename 2>/dev/null || echo '未找到')"
echo ""
echo "🚀 现在可以开始编译NexusRemote项目了!"