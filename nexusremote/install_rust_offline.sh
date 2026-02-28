#!/bin/bash
# Rust离线安装脚本
# 解决系统依赖问题的手动安装方法

set -e

echo "🚀 Rust离线安装解决方案"
echo "========================"

# 创建安装目录
RUST_DIR="$HOME/.rustup"
CARGO_DIR="$HOME/.cargo"
INSTALL_DIR="$HOME/rust-local"

echo "安装目录: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

# 方法1: 使用预编译的rustup
install_rustup_binary() {
    echo "方法1: 下载rustup二进制..."
    
    # 下载rustup-init
    if [ ! -f "$INSTALL_DIR/rustup-init" ]; then
        echo "下载rustup-init..."
        # 尝试多个镜像
        for mirror in \
            "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
            "https://mirrors.ustc.edu.cn/rust-static/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
            "https://mirrors.tuna.tsinghua.edu.cn/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"
        do
            echo "尝试从 $mirror 下载..."
            if wget -q "$mirror" -O "$INSTALL_DIR/rustup-init"; then
                echo "下载成功"
                chmod +x "$INSTALL_DIR/rustup-init"
                break
            fi
        done
    fi
    
    if [ ! -f "$INSTALL_DIR/rustup-init" ]; then
        echo "❌ 无法下载rustup-init"
        return 1
    fi
    
    # 安装
    echo "运行rustup-init..."
    RUSTUP_HOME="$RUST_DIR" CARGO_HOME="$CARGO_DIR" "$INSTALL_DIR/rustup-init" -y --no-modify-path
    
    # 添加到PATH
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
    source "$HOME/.bashrc"
    
    return 0
}

# 方法2: 使用预编译的Rust工具链
install_rust_toolchain() {
    echo "方法2: 下载预编译工具链..."
    
    # 下载Rust工具链
    RUST_VERSION="1.75.0"
    RUST_ARCH="x86_64-unknown-linux-gnu"
    RUST_PACKAGE="rust-$RUST_VERSION-$RUST_ARCH.tar.gz"
    
    if [ ! -f "$INSTALL_DIR/$RUST_PACKAGE" ]; then
        echo "下载 $RUST_PACKAGE..."
        # 尝试多个源
        for url in \
            "https://static.rust-lang.org/dist/$RUST_PACKAGE" \
            "https://mirrors.ustc.edu.cn/rust-static/dist/$RUST_PACKAGE" \
            "https://mirrors.tuna.tsinghua.edu.cn/rust-static/dist/$RUST_PACKAGE"
        do
            echo "尝试从 $url 下载..."
            if wget -q "$url" -O "$INSTALL_DIR/$RUST_PACKAGE"; then
                echo "下载成功"
                break
            fi
        done
    fi
    
    if [ ! -f "$INSTALL_DIR/$RUST_PACKAGE" ]; then
        echo "❌ 无法下载Rust工具链"
        return 1
    fi
    
    # 解压
    echo "解压工具链..."
    tar -xzf "$INSTALL_DIR/$RUST_PACKAGE" -C "$INSTALL_DIR"
    
    # 安装
    echo "安装工具链..."
    cd "$INSTALL_DIR/rust-$RUST_VERSION-$RUST_ARCH"
    sudo ./install.sh
    
    return 0
}

# 方法3: 使用miniconda安装rust (如果conda可用)
install_via_conda() {
    echo "方法3: 使用conda安装..."
    
    if command -v conda &> /dev/null; then
        echo "发现conda，安装rust..."
        conda install -c conda-forge rust -y
        return 0
    else
        echo "❌ conda未安装"
        return 1
    fi
}

# 方法4: 从源码编译 (最后手段)
install_from_source() {
    echo "方法4: 从源码编译..."
    
    echo "⚠️ 警告: 从源码编译可能需要很长时间"
    echo "这应该是最后的选择"
    
    # 下载源码
    if [ ! -f "$INSTALL_DIR/rustc-1.75.0-src.tar.gz" ]; then
        echo "下载Rust源码..."
        wget https://static.rust-lang.org/dist/rustc-1.75.0-src.tar.gz -O "$INSTALL_DIR/rustc-1.75.0-src.tar.gz"
    fi
    
    # 解压
    tar -xzf "$INSTALL_DIR/rustc-1.75.0-src.tar.gz" -C "$INSTALL_DIR"
    
    # 编译
    cd "$INSTALL_DIR/rustc-1.75.0-src"
    ./configure --prefix="$INSTALL_DIR/rust-install"
    make -j$(nproc)
    make install
    
    # 添加到PATH
    echo "export PATH=\"$INSTALL_DIR/rust-install/bin:\$PATH\"" >> "$HOME/.bashrc"
    
    return 0
}

# 验证安装
verify_installation() {
    echo "验证安装..."
    
    if command -v rustc &> /dev/null; then
        echo "✅ rustc: $(rustc --version)"
    else
        echo "❌ rustc未找到"
        return 1
    fi
    
    if command -v cargo &> /dev/null; then
        echo "✅ cargo: $(cargo --version)"
    else
        echo "❌ cargo未找到"
        return 1
    fi
    
    return 0
}

# 测试NexusRemote项目
test_nexusremote() {
    echo "测试NexusRemote项目..."
    
    cd /home/admin/.openclaw/workspace/nexusremote
    
    # 检查Cargo.toml
    if [ ! -f "Cargo.toml" ]; then
        echo "❌ Cargo.toml未找到"
        return 1
    fi
    
    # 尝试编译
    echo "尝试编译..."
    if cargo check; then
        echo "✅ 项目检查通过"
    else
        echo "⚠️ 项目检查失败，但继续..."
    fi
    
    # 尝试构建
    echo "尝试构建..."
    if cargo build --release; then
        echo "✅ 项目构建成功"
        echo "二进制位置: target/release/nexusremote"
    else
        echo "❌ 项目构建失败"
        return 1
    fi
    
    return 0
}

# 主函数
main() {
    echo "开始解决Rust安装问题..."
    echo "系统问题: dpkg版本冲突，使用离线安装绕过"
    
    # 尝试方法1
    echo ""
    echo "=== 尝试方法1: rustup二进制 ==="
    if install_rustup_binary; then
        echo "✅ 方法1成功"
    else
        echo "❌ 方法1失败，尝试方法2"
        
        # 尝试方法2
        echo ""
        echo "=== 尝试方法2: 预编译工具链 ==="
        if install_rust_toolchain; then
            echo "✅ 方法2成功"
        else
            echo "❌ 方法2失败，尝试方法3"
            
            # 尝试方法3
            echo ""
            echo "=== 尝试方法3: conda安装 ==="
            if install_via_conda; then
                echo "✅ 方法3成功"
            else
                echo "❌ 方法3失败，尝试方法4"
                
                # 尝试方法4 (最后手段)
                echo ""
                echo "=== 尝试方法4: 源码编译 ==="
                if install_from_source; then
                    echo "✅ 方法4成功"
                else
                    echo "❌ 所有方法都失败"
                    exit 1
                fi
            fi
        fi
    fi
    
    # 验证安装
    echo ""
    echo "=== 验证安装 ==="
    if verify_installation; then
        echo "🎉 Rust安装成功!"
    else
        echo "❌ Rust安装验证失败"
        exit 1
    fi
    
    # 测试项目
    echo ""
    echo "=== 测试NexusRemote项目 ==="
    if test_nexusremote; then
        echo "🎉 NexusRemote项目编译成功!"
        echo ""
        echo "📋 下一步:"
        echo "   1. 运行测试: cargo test"
        echo "   2. 运行模拟: cargo run -- simulate"
        echo "   3. 开发继续: cargo run -- test-routing"
    else
        echo "⚠️ 项目测试有问题，但Rust已安装"
        echo "可以手动调试项目"
    fi
    
    echo ""
    echo "🔧 环境配置:"
    echo "   Rust已添加到PATH"
    echo "   重启终端或运行: source ~/.bashrc"
    echo "   验证: rustc --version"
    
    exit 0
}

# 运行主函数
main "$@"
