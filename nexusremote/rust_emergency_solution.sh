#!/bin/bash
# Rust紧急解决方案脚本

set -e

echo "🚨 Rust工具链紧急解决方案"
echo "=============================="

# 检查当前状态
check_rust() {
    echo "检查Rust安装状态..."
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        echo "✅ Rust已安装:"
        echo "   rustc: $(rustc --version 2>/dev/null || echo '未找到')"
        echo "   cargo: $(cargo --version 2>/dev/null || echo '未找到')"
        return 0
    else
        echo "❌ Rust未安装"
        return 1
    fi
}

# 方法1: 使用apt安装
install_via_apt() {
    echo "尝试方法1: apt安装..."
    sudo apt-get update
    sudo apt-get install -y rustc cargo
}

# 方法2: 使用snap安装
install_via_snap() {
    echo "尝试方法2: snap安装..."
    sudo snap install rustup --classic
    rustup default stable
}

# 方法3: 使用curl安装（备用）
install_via_curl() {
    echo "尝试方法3: curl安装..."
    # 使用国内镜像
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source $HOME/.cargo/env
}

# 方法4: 使用预编译包
install_via_precompiled() {
    echo "尝试方法4: 预编译包..."
    # 下载预编译包
    wget https://static.rust-lang.org/dist/rust-1.75.0-x86_64-unknown-linux-gnu.tar.gz -O /tmp/rust.tar.gz
    tar -xzf /tmp/rust.tar.gz -C /tmp
    cd /tmp/rust-1.75.0-x86_64-unknown-linux-gnu
    sudo ./install.sh
}

# 方法5: 使用Docker（如果可用）
install_via_docker() {
    echo "尝试方法5: Docker容器..."
    if command -v docker &> /dev/null; then
        # 创建Docker构建环境
        docker run -v $(pwd):/app -w /app rust:latest cargo build --release
        echo "✅ 使用Docker构建成功"
        return 0
    else
        echo "❌ Docker未安装"
        return 1
    fi
}

# 方法6: 使用在线构建服务
setup_ci_cd() {
    echo "设置方法6: CI/CD构建..."
    cat > .github/workflows/build.yml << 'EOF'
name: Build NexusRemote

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Install Rust
      run: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
    - name: Check format
      run: cargo fmt -- --check
    - name: Clippy
      run: cargo clippy -- -D warnings
EOF
    echo "✅ GitHub Actions配置已创建"
    echo "   推送代码到GitHub后会自动构建"
}

# 主函数
main() {
    echo "NexusRemote项目Rust工具链解决方案"
    echo "======================================"
    
    # 首先检查是否已安装
    if check_rust; then
        echo "🎉 Rust已安装，开始构建项目..."
        ./build.sh
        exit 0
    fi
    
    echo "Rust未安装，尝试多种安装方法..."
    echo ""
    
    # 尝试方法1: apt
    echo "1. 尝试apt安装..."
    if install_via_apt; then
        echo "✅ apt安装成功"
        check_rust && ./build.sh
        exit 0
    fi
    
    # 尝试方法2: snap
    echo ""
    echo "2. 尝试snap安装..."
    if install_via_snap; then
        echo "✅ snap安装成功"
        check_rust && ./build.sh
        exit 0
    fi
    
    # 尝试方法3: curl（使用镜像）
    echo ""
    echo "3. 尝试curl安装（使用国内镜像）..."
    if install_via_curl; then
        echo "✅ curl安装成功"
        check_rust && ./build.sh
        exit 0
    fi
    
    # 尝试方法5: Docker
    echo ""
    echo "4. 尝试Docker构建..."
    if install_via_docker; then
        exit 0
    fi
    
    # 最后方法: 设置CI/CD
    echo ""
    echo "5. 设置CI/CD自动构建..."
    setup_ci_cd
    
    echo ""
    echo "⚠️ 所有安装方法都失败了"
    echo ""
    echo "📋 建议的下一步:"
    echo "   1. 手动安装Rust: https://www.rust-lang.org/tools/install"
    echo "   2. 使用Docker: docker run -v $(pwd):/app -w /app rust:latest cargo build"
    echo "   3. 使用GitHub Actions自动构建（已配置）"
    echo ""
    echo "🔧 临时解决方案:"
    echo "   继续使用Python验证逻辑，同时解决Rust安装问题"
    
    exit 1
}

# 运行主函数
main "$@"
