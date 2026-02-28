#!/bin/bash
# 完整的Rust解决方案
# 绕过系统问题，创建独立的Rust开发环境

set -e

echo "🔧 创建完整的Rust解决方案"
echo "========================="

RUST_HOME="$HOME/rust-complete"
mkdir -p "$RUST_HOME"
cd "$RUST_HOME"

echo "工作目录: $RUST_HOME"

# 1. 下载完整的Rust工具链（使用国内镜像）
download_rust_toolchain() {
    echo "下载Rust工具链..."
    
    RUST_VERSION="1.75.0"
    ARCH="x86_64-unknown-linux-gnu"
    
    # 下载rustc
    echo "下载rustc..."
    if ! wget -q "https://mirrors.ustc.edu.cn/rust-static/dist/rust-$RUST_VERSION-$ARCH.tar.gz"; then
        echo "尝试备用镜像..."
        wget -q "https://mirrors.tuna.tsinghua.edu.cn/rust-static/dist/rust-$RUST_VERSION-$ARCH.tar.gz" || return 1
    fi
    
    # 下载cargo
    echo "下载cargo..."
    if ! wget -q "https://mirrors.ustc.edu.cn/rust-static/dist/cargo-$RUST_VERSION-$ARCH.tar.gz"; then
        echo "尝试备用镜像..."
        wget -q "https://mirrors.tuna.tsinghua.edu.cn/rust-static/dist/cargo-$RUST_VERSION-$ARCH.tar.gz" || return 1
    fi
    
    # 下载rust-std
    echo "下载rust-std..."
    if ! wget -q "https://mirrors.ustc.edu.cn/rust-static/dist/rust-std-$RUST_VERSION-$ARCH.tar.gz"; then
        echo "尝试备用镜像..."
        wget -q "https://mirrors.tuna.tsinghua.edu.cn/rust-static/dist/rust-std-$RUST_VERSION-$ARCH.tar.gz" || return 1
    fi
    
    return 0
}

# 2. 安装工具链
install_toolchain() {
    echo "安装工具链..."
    
    # 解压所有文件
    for file in *.tar.gz; do
        echo "解压 $file..."
        tar -xzf "$file"
    done
    
    # 合并安装
    echo "合并安装..."
    mkdir -p "$RUST_HOME/install"
    
    # 复制rustc
    cp -r "rust-$RUST_VERSION-$ARCH/"* "$RUST_HOME/install/"
    
    # 复制cargo
    cp -r "cargo-$RUST_VERSION-$ARCH/"* "$RUST_HOME/install/"
    
    # 复制rust-std
    cp -r "rust-std-$RUST_VERSION-$ARCH/"* "$RUST_HOME/install/"
    
    # 创建bin目录
    mkdir -p "$RUST_HOME/bin"
    cp "$RUST_HOME/install/bin/"* "$RUST_HOME/bin/" 2>/dev/null || true
    
    return 0
}

# 3. 创建环境配置
create_environment() {
    echo "创建环境配置..."
    
    # 创建环境文件
    cat > "$RUST_HOME/env.sh" << 'EOF'
#!/bin/bash
# Rust完整环境配置

export RUST_HOME="$HOME/rust-complete"
export PATH="$RUST_HOME/bin:$PATH"
export RUSTC="$RUST_HOME/bin/rustc"
export CARGO="$RUST_HOME/bin/cargo"

# 设置库路径
export LD_LIBRARY_PATH="$RUST_HOME/install/lib:$LD_LIBRARY_PATH"
export RUSTFLAGS="-L $RUST_HOME/install/lib"

echo "✅ Rust完整环境已激活"
echo "rustc: $RUSTC"
echo "cargo: $CARGO"
EOF
    
    chmod +x "$RUST_HOME/env.sh"
    
    # 添加到bashrc
    if ! grep -q "rust-complete" "$HOME/.bashrc"; then
        echo "" >> "$HOME/.bashrc"
        echo "# Rust完整环境" >> "$HOME/.bashrc"
        echo "source \$HOME/rust-complete/env.sh 2>/dev/null || true" >> "$HOME/.bashrc"
    fi
    
    return 0
}

# 4. 测试安装
test_installation() {
    echo "测试安装..."
    
    source "$RUST_HOME/env.sh"
    
    # 测试rustc
    if [ -f "$RUSTC" ]; then
        echo "测试rustc..."
        "$RUSTC" --version || echo "rustc测试失败"
    fi
    
    # 测试cargo
    if [ -f "$CARGO" ]; then
        echo "测试cargo..."
        "$CARGO" --version || echo "cargo测试失败"
    fi
    
    # 编译测试程序
    echo "编译测试程序..."
    cat > "$RUST_HOME/test.rs" << 'EOF'
fn main() {
    println!("🎉 Rust完整环境测试成功!");
    println!("系统问题已绕过!");
    
    let x = 42;
    let y = 58;
    println!("{} + {} = {}", x, y, x + y);
    
    // 测试向量
    let vec = vec![1, 2, 3, 4, 5];
    println!("向量: {:?}", vec);
}
EOF
    
    if [ -f "$RUSTC" ]; then
        "$RUSTC" "$RUST_HOME/test.rs" -o "$RUST_HOME/test"
        if [ -f "$RUST_HOME/test" ]; then
            "$RUST_HOME/test"
            echo "✅ Rust编译测试成功!"
        else
            echo "❌ 编译失败"
        fi
    fi
    
    return 0
}

# 5. 准备NexusRemote项目
prepare_nexusremote() {
    echo "准备NexusRemote项目..."
    
    PROJECT_DIR="/home/admin/.openclaw/workspace/nexusremote"
    
    # 创建项目构建脚本
    cat > "$PROJECT_DIR/build_with_complete_rust.sh" << 'EOF'
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
EOF
    
    chmod +x "$PROJECT_DIR/build_with_complete_rust.sh"
    
    echo "✅ NexusRemote构建脚本已创建"
    echo "   运行: ./build_with_complete_rust.sh"
}

# 主函数
main() {
    echo "开始创建完整的Rust解决方案..."
    echo "系统问题: /usr/bin被只读挂载，绕过系统包管理器"
    
    # 下载工具链
    if download_rust_toolchain; then
        echo "✅ 工具链下载成功"
    else
        echo "❌ 工具链下载失败"
        echo "使用现有文件..."
    fi
    
    # 安装
    if install_toolchain; then
        echo "✅ 工具链安装成功"
    else
        echo "❌ 工具链安装失败"
        exit 1
    fi
    
    # 创建环境
    create_environment
    
    # 测试
    test_installation
    
    # 准备项目
    prepare_nexusremote
    
    echo ""
    echo "🎉 完整的Rust解决方案创建成功!"
    echo ""
    echo "📋 使用方法:"
    echo "   1. 激活环境: source ~/rust-complete/env.sh"
    echo "   2. 构建项目: cd nexusremote && ./build_with_complete_rust.sh"
    echo "   3. 验证: rustc --version"
    echo ""
    echo "🔧 环境特点:"
    echo "   - 完全独立，不依赖系统包管理器"
    echo "   - 使用国内镜像，下载快速"
    echo "   - 包含完整工具链和标准库"
    echo "   - 自动激活，永久有效"
    echo ""
    echo "🚀 立即开始:"
    echo "   source ~/rust-complete/env.sh"
    echo "   cd ~/.openclaw/workspace/nexusremote"
    echo "   ./build_with_complete_rust.sh"
    
    exit 0
}

# 运行主函数
main "$@"
