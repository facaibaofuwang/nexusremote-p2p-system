#!/bin/bash
# NexusRemote Windows 打包脚本 (Linux/macOS环境)
# 用于在Linux/macOS上交叉编译Windows版本

set -e

echo "===================================="
echo "NexusRemote Windows 交叉编译脚本"
echo "===================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# 检查系统
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "${GREEN}[系统] Linux环境${NC}"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "${GREEN}[系统] macOS环境${NC}"
else
    echo -e "${RED}[错误] 未知系统: $OSTYPE${NC}"
    exit 1
fi

# 检查Rust工具链
echo -e "${YELLOW}[检查] Rust工具链...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}[错误] 未找到Rust，请先安装Rust${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[错误] 未找到Cargo${NC}"
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo -e "${GREEN}[信息] Rust版本: $RUST_VERSION${NC}"

# 检查Windows目标编译器
echo -e "${YELLOW}[检查] Windows交叉编译工具...${NC}"
if ! rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
    echo -e "${YELLOW}[安装] 添加Windows目标...${NC}"
    rustup target add x86_64-pc-windows-msvc
fi

# 进入Rust项目目录
cd nexusremote
if [ ! -d "nexusremote" ]; then
    echo -e "${RED}[错误] nexusremote目录不存在${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}[步骤1] 交叉编译Rust后端为Windows...${NC}"
echo "目标: x86_64-pc-windows-msvc"
echo ""

# 交叉编译Rust项目
cargo build --release --target x86_64-pc-windows-msvc

if [ $? -ne 0 ]; then
    echo -e "${RED}[错误] Rust交叉编译失败${NC}"
    exit 1
fi

echo -e "${GREEN}[完成] Rust交叉编译成功${NC}"
echo ""

# 检查Node.js
echo -e "${YELLOW}[检查] Node.js环境...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}[安装] 未找到Node.js，使用Docker打包...${NC}"
    
    # 使用Docker构建Windows包
    echo -e "${GREEN}[步骤2] 使用Docker构建Windows Electron包...${NC}"
    
    cd nexusremote-app-client
    
    # 创建Docker构建脚本
    cat > docker-build-windows.sh << 'EOF'
#!/bin/bash
docker run --rm -v \$(pwd):/workspace -w /workspace \
    node:18-windowsservercore \
    bash -c "cd /workspace && npm install && npm run build:win"
EOF
    
    chmod +x docker-build-windows.sh
    ./docker-build-windows.sh
    
else
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}[信息] Node.js版本: $NODE_VERSION${NC}"
    
    # 检查npm
    if ! command -v npm &> /dev/null; then
        echo -e "${RED}[错误] 未找到npm${NC}"
        exit 1
    fi
    
    NPM_VERSION=$(npm --version)
    echo -e "${GREEN}[信息] npm版本: $NPM_VERSION${NC}"
    
    echo ""
    echo -e "${GREEN}[步骤2] 安装Electron依赖...${NC}"
    
    cd nexusremote-app-client
    
    # 安装依赖
    npm install
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}[错误] npm依赖安装失败${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}[完成] 依赖安装完成${NC}"
    echo ""
    
    echo -e "${GREEN}[步骤3] 构建Windows Electron包...${NC}"
    
    # 复制交叉编译的Rust二进制
    echo "复制交叉编译的nexusremote.exe..."
    if [ -f "../nexusremote/target/x86_64-pc-windows-msvc/release/nexusremote.exe" ]; then
        cp "../nexusremote/target/x86_64-pc-windows-msvc/release/nexusremote.exe" "nexusremote.exe"
        echo -e "${GREEN}[完成] Rust二进制文件复制完成${NC}"
    else
        echo -e "${YELLOW}[警告] 未找到交叉编译的Rust二进制${NC}"
    fi
    
    # 构建Windows包
    npm run build:win
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}[错误] Windows打包失败${NC}"
        exit 1
    fi
fi

echo ""
echo "===================================="
echo -e "${GREEN}[成功] Windows打包完成！${NC}"
echo "===================================="
echo ""

# 显示生成的文件
if [ -d "nexusremote-app-client/dist" ]; then
    echo "生成的文件："
    ls -lh nexusremote-app-client/dist/
    echo ""
    echo "文件说明："
    echo "  - .exe文件: NSIS安装程序（标准安装）"
    echo "  - .zip文件: 便携版本（免安装）"
fi

echo ""
echo "使用方法："
echo "1. 将.exe或.zip文件传输到Windows电脑"
echo "2. 双击.exe文件运行安装向导"
echo "3. 或解压.zip文件运行便携版本"
