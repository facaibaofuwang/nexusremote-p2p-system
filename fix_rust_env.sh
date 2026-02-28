#!/bin/bash
echo "🛠️ Rust环境手动修复脚本"
echo "======================"

# 创建必要的目录结构
echo "1. 创建目录结构..."
mkdir -p ~/rust-local/rustc/lib/rustlib/x86_64-unknown-linux-gnu/lib
mkdir -p ~/rust-fix

cd ~/rust-fix

# 尝试下载完整安装包
echo "2. 尝试下载Rust完整安装包..."
echo "   尝试清华镜像..."

# 方法1: 尝试下载rustup-init
if command -v wget &> /dev/null; then
    echo "   下载rustup-init..."
    wget -q --show-progress https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/dist/x86_64-unknown-linux-gnu/rustup-init || \
    wget -q https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
fi

if [ -f "rustup-init" ]; then
    echo "✅ rustup-init下载成功"
    chmod +x rustup-init
    echo "   运行rustup-init..."
    # 设置环境变量使用国内镜像
    export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
    export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
    ./rustup-init -y --no-modify-path || echo "⚠️ rustup-init运行失败"
else
    echo "⚠️ 无法下载rustup-init"
fi

# 方法2: 尝试直接下载标准库组件
echo "3. 尝试直接下载标准库组件..."
echo "   这需要从完整安装包中提取..."

# 创建测试文件验证修复
echo "4. 创建测试验证..."
cat > ~/rust-fix/test_simple.rs << 'EOF'
// 测试无标准库编译
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
EOF

echo "5. 测试当前环境..."
if [ -f ~/rust-local/rustc/bin/rustc ]; then
    echo "   测试rustc编译无标准库程序..."
    ~/rust-local/rustc/bin/rustc --target x86_64-unknown-linux-gnu \
        -C linker=ld ~/rust-fix/test_simple.rs \
        --crate-type staticlib 2>&1 | grep -i "error\|success" || true
fi

# 创建环境激活脚本
echo "6. 创建环境激活脚本..."
cat > ~/.rust_fixed_env << 'EOF'
#!/bin/bash
# 修复后的Rust环境配置
export RUSTC_PATH="$HOME/rust-local/rustc/bin/rustc"
export CARGO_PATH="$HOME/.cargo/bin/cargo"

# 添加到PATH
if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# 设置库路径
export LD_LIBRARY_PATH="$HOME/rust-local/rustc/lib:$LD_LIBRARY_PATH"
export RUSTLIB_PATH="$HOME/rust-local/rustc/lib/rustlib/x86_64-unknown-linux-gnu/lib"

echo "✅ Rust修复环境已激活"
echo "   rustc: $(which rustc 2>/dev/null || echo '未找到')"
echo "   cargo: $(which cargo 2>/dev/null || echo '未找到')"
EOF

chmod +x ~/.rust_fixed_env

echo ""
echo "📋 修复完成摘要:"
echo "   1. 目录结构已创建"
echo "   2. 尝试下载了rustup-init"
echo "   3. 测试文件已创建"
echo "   4. 环境激活脚本: ~/.rust_fixed_env"
echo ""
echo "🔧 下一步手动操作建议:"
echo "   1. 运行: source ~/.rust_fixed_env"
echo "   2. 如果rustup-init下载成功，运行: ./rustup-init -y"
echo "   3. 或者手动从其他机器复制标准库文件到:"
echo "      ~/rust-local/rustc/lib/rustlib/x86_64-unknown-linux-gnu/lib/"
echo ""
echo "💡 替代方案: 使用Docker进行编译"
echo "   docker run --rm -v \$(pwd):/app -w /app rust:1.75 cargo build"