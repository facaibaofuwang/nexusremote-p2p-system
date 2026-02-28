#!/bin/bash
# 最简单的Rust安装脚本
# 完全绕过系统包管理器

set -e

echo "🔧 最简单Rust安装方案"
echo "====================="

# 创建本地目录
LOCAL_RUST="$HOME/rust-local"
mkdir -p "$LOCAL_RUST"
cd "$LOCAL_RUST"

echo "安装目录: $LOCAL_RUST"

# 方法1: 下载预编译的rustc和cargo
install_precompiled() {
    echo "下载预编译的Rust组件..."
    
    # Rust版本
    RUST_VERSION="1.75.0"
    
    # 下载rustc
    echo "下载rustc..."
    if ! wget -q "https://static.rust-lang.org/dist/rustc-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz"; then
        echo "尝试镜像..."
        wget -q "https://mirrors.ustc.edu.cn/rust-static/dist/rustc-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz" || return 1
    fi
    
    # 下载cargo
    echo "下载cargo..."
    if ! wget -q "https://static.rust-lang.org/dist/cargo-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz"; then
        echo "尝试镜像..."
        wget -q "https://mirrors.ustc.edu.cn/rust-static/dist/cargo-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz" || return 1
    fi
    
    # 解压
    echo "解压..."
    tar -xzf "rustc-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
    tar -xzf "cargo-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz"
    
    # 安装到本地目录
    echo "安装到本地目录..."
    cp -r "rustc-$RUST_VERSION-x86_64-unknown-linux-gnu/"* "$LOCAL_RUST/"
    cp -r "cargo-$RUST_VERSION-x86_64-unknown-linux-gnu/"* "$LOCAL_RUST/"
    
    # 创建符号链接
    ln -sf "$LOCAL_RUST/bin/rustc" "$HOME/.local/bin/rustc" 2>/dev/null || true
    ln -sf "$LOCAL_RUST/bin/cargo" "$HOME/.local/bin/cargo" 2>/dev/null || true
    
    return 0
}

# 方法2: 使用已存在的二进制（如果有）
check_existing() {
    echo "检查现有Rust安装..."
    
    # 检查~/.cargo/bin
    if [ -f "$HOME/.cargo/bin/rustc" ]; then
        echo "发现 ~/.cargo/bin/rustc"
        ln -sf "$HOME/.cargo/bin/rustc" "$HOME/.local/bin/rustc" 2>/dev/null || true
        ln -sf "$HOME/.cargo/bin/cargo" "$HOME/.local/bin/cargo" 2>/dev/null || true
        return 0
    fi
    
    # 检查/usr/local/bin
    if [ -f "/usr/local/bin/rustc" ]; then
        echo "发现 /usr/local/bin/rustc"
        return 0
    fi
    
    return 1
}

# 方法3: 使用Docker作为备用
use_docker_fallback() {
    echo "设置Docker备用方案..."
    
    # 创建Docker构建脚本
    cat > "$LOCAL_RUST/docker-build.sh" << 'EOF'
#!/bin/bash
# Docker构建脚本

# 构建项目
docker run --rm -v "$(pwd):/app" -w /app rust:latest \
    sh -c "cargo build --release && cp target/release/nexusremote /app/nexusremote-bin"

echo "✅ 使用Docker构建完成"
echo "二进制: ./nexusremote-bin"
EOF
    
    chmod +x "$LOCAL_RUST/docker-build.sh"
    
    echo "创建了Docker构建脚本: $LOCAL_RUST/docker-build.sh"
    echo "运行: ./docker-build.sh 来构建项目"
    
    return 0
}

# 验证安装
verify() {
    echo "验证安装..."
    
    # 检查PATH
    export PATH="$HOME/.local/bin:$PATH"
    export PATH="$LOCAL_RUST/bin:$PATH"
    
    # 检查rustc
    if command -v rustc >/dev/null 2>&1; then
        echo "✅ rustc: $(rustc --version 2>/dev/null || echo '找到但无法运行')"
    else
        # 尝试直接路径
        if [ -f "$LOCAL_RUST/bin/rustc" ]; then
            echo "✅ rustc: $LOCAL_RUST/bin/rustc (需要直接调用)"
        else
            echo "❌ rustc未找到"
            return 1
        fi
    fi
    
    # 检查cargo
    if command -v cargo >/dev/null 2>&1; then
        echo "✅ cargo: $(cargo --version 2>/dev/null || echo '找到但无法运行')"
    else
        if [ -f "$LOCAL_RUST/bin/cargo" ]; then
            echo "✅ cargo: $LOCAL_RUST/bin/cargo (需要直接调用)"
        else
            echo "❌ cargo未找到"
            return 1
        fi
    fi
    
    return 0
}

# 测试简单Rust程序
test_simple_rust() {
    echo "测试简单Rust程序..."
    
    cat > "$LOCAL_RUST/test.rs" << 'EOF'
fn main() {
    println!("Hello from Rust!");
    let x = 42;
    let y = 58;
    println!("{} + {} = {}", x, y, x + y);
}
EOF
    
    # 尝试编译
    if command -v rustc >/dev/null 2>&1; then
        rustc "$LOCAL_RUST/test.rs" -o "$LOCAL_RUST/test"
        if [ -f "$LOCAL_RUST/test" ]; then
            echo "✅ 编译成功"
            "$LOCAL_RUST/test"
            return 0
        fi
    elif [ -f "$LOCAL_RUST/bin/rustc" ]; then
        "$LOCAL_RUST/bin/rustc" "$LOCAL_RUST/test.rs" -o "$LOCAL_RUST/test"
        if [ -f "$LOCAL_RUST/test" ]; then
            echo "✅ 编译成功 (使用直接路径)"
            "$LOCAL_RUST/test"
            return 0
        fi
    fi
    
    echo "❌ 编译测试失败"
    return 1
}

# 主函数
main() {
    echo "开始安装Rust..."
    
    # 首先检查是否已有
    if check_existing; then
        echo "发现现有Rust安装"
    else
        echo "未发现现有安装，尝试下载..."
        
        # 尝试安装预编译版本
        if install_precompiled; then
            echo "✅ 预编译版本安装成功"
        else
            echo "❌ 预编译安装失败，设置备用方案"
            use_docker_fallback
        fi
    fi
    
    # 验证
    echo ""
    if verify; then
        echo "🎉 Rust验证通过"
        
        # 测试
        if test_simple_rust; then
            echo "✅ Rust工作正常"
        else
            echo "⚠️ Rust编译测试失败，但可能仍可使用"
        fi
    else
        echo "⚠️ Rust验证失败，使用备用方案"
        use_docker_fallback
    fi
    
    # 创建使用脚本
    echo ""
    echo "📋 使用说明:"
    echo ""
    
    if [ -f "$LOCAL_RUST/bin/rustc" ]; then
        cat > "$LOCAL_RUST/use-rust.sh" << 'EOF'
#!/bin/bash
# 使用本地Rust环境

export RUSTUP_HOME="$HOME/rust-local"
export CARGO_HOME="$HOME/rust-local"
export PATH="$HOME/rust-local/bin:$PATH"

echo "Rust环境已设置"
echo "rustc: $(rustc --version 2>/dev/null || echo '未找到')"
echo "cargo: $(cargo --version 2>/dev/null || echo '未找到')"

# 执行命令
exec "$@"
EOF
        chmod +x "$LOCAL_RUST/use-rust.sh"
        
        echo "使用本地Rust:"
        echo "  $LOCAL_RUST/use-rust.sh cargo build"
    fi
    
    if [ -f "$LOCAL_RUST/docker-build.sh" ]; then
        echo ""
        echo "使用Docker构建:"
        echo "  $LOCAL_RUST/docker-build.sh"
    fi
    
    echo ""
    echo "🔧 下一步:"
    echo "  1. 尝试编译NexusRemote项目"
    echo "  2. 如果失败，使用Docker方案"
    echo "  3. 或者继续修复系统依赖"
    
    exit 0
}

# 运行
main "$@"
