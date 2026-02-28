#!/bin/bash
# NexusRemote Python后端启动脚本

set -e

echo "🚀 启动 NexusRemote Python后端服务"
echo "========================================"

# 检查Python版本
python3 --version

# 检查并安装依赖
echo "📦 检查Python依赖..."
if [ ! -f ".venv/bin/activate" ]; then
    echo "  创建虚拟环境..."
    python3 -m venv .venv
fi

echo "  激活虚拟环境并安装依赖..."
source .venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt

echo ""
echo "🎯 服务信息:"
echo "  REST API: http://localhost:5000"
echo "  WebSocket: ws://localhost:5000"
echo "  健康检查: http://localhost:5000/api/health"
echo ""

echo "📡 启动服务..."
echo "  按 Ctrl+C 停止服务"
echo "========================================"

# 启动服务
python3 main.py