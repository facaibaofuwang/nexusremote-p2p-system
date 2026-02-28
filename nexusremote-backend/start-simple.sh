#!/bin/bash
# NexusRemote Python后端启动脚本（简化版）

set -e

echo "🚀 启动 NexusRemote Python后端服务"
echo "========================================"

# 检查Python版本
python3 --version

# 检查并安装依赖
echo "📦 检查Python依赖..."
pip3 install --user --upgrade pip 2>/dev/null || true

# 检查Flask是否已安装
if ! python3 -c "import flask" 2>/dev/null; then
    echo "  安装Flask及相关依赖..."
    pip3 install --user flask flask-cors flask-socketio python-socketio eventlet
fi

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