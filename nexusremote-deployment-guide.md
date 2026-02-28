# NexusRemote 部署指南
# 三服务架构一键部署说明

## 📋 部署概览

### 系统要求
- **操作系统**: Linux (Ubuntu 20.04+ 或 CentOS 7+)
- **内存**: 最小 2GB RAM
- **存储**: 最小 10GB 可用空间
- **网络**: 需要开放端口 3000, 5000, 8081

### 服务端口
```
🖥️ 前端服务: 端口 3000 (HTTP)
🐍 Python后端: 端口 5000 (HTTP)
🔧 Rust WebSocket: 端口 8081 (WebSocket)
```

## 🚀 快速开始

### 一键部署脚本
```bash
#!/bin/bash
# nexusremote-quick-deploy.sh

echo "🚀 开始部署 NexusRemote 三服务架构..."

# 1. 克隆项目代码
git clone https://github.com/facaibaofuwang/nexusremote-p2p-system.git
cd nexusremote-p2p-system

# 2. 安装系统依赖
echo "📦 安装系统依赖..."
sudo apt-get update
sudo apt-get install -y python3 python3-pip nodejs npm curl

# 3. 安装Rust (如果未安装)
if ! command -v rustc &> /dev/null; then
    echo "🔧 安装Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 4. 部署Python后端
echo "🐍 部署Python后端..."
cd python-backend
pip3 install -r requirements.txt
python3 main.py &

# 5. 部署Rust WebSocket服务器
echo "🔧 部署Rust WebSocket服务器..."
cd ../rust-backend
cargo build --release
./target/release/websocket_server &

# 6. 部署前端服务
echo "🖥️ 部署前端服务..."
cd ../frontend
npm install
npm start &

echo "✅ 部署完成！"
echo "🔗 访问地址: http://localhost:3000"
echo "📊 健康检查: http://localhost:5000/api/health"
```

## 📁 项目结构

```
nexusremote-p2p-system/
├── README.md                    # 项目说明
├── LICENSE                      # MIT许可证
├── rust-backend/                # Rust后端
│   ├── Cargo.toml              # Rust依赖配置
│   ├── src/                    # 源代码
│   │   ├── bin/               # 可执行文件
│   │   │   └── websocket_server.rs
│   │   ├── core/              # 核心模块
│   │   ├── network/           # 网络模块
│   │   ├── simulator/         # 模拟器
│   │   └── wallet/            # 钱包模块
│   └── target/                # 编译输出
├── python-backend/             # Python后端
│   ├── main.py                # 主程序
│   ├── requirements.txt       # Python依赖
│   └── start-simple.sh       # 启动脚本
├── frontend/                   # 前端界面
│   ├── package.json           # Node.js配置
│   ├── server.js              # 前端服务器
│   ├── index.html             # 主界面
│   ├── js/                    # JavaScript文件
│   │   └── rust-websocket-client.js
│   └── test-integration.html  # 集成测试页面
└── deployment/                # 部署相关
    ├── docker-compose.yml     # Docker编排
    ├── Dockerfile.frontend    # 前端Dockerfile
    ├── Dockerfile.python      # Python后端Dockerfile
    └── Dockerfile.rust        # Rust后端Dockerfile
```

## 🐳 Docker部署

### Docker Compose配置
```yaml
# docker-compose.yml
version: '3.8'

services:
  frontend:
    build: ./deployment/Dockerfile.frontend
    ports:
      - "3000:3000"
    depends_on:
      - python-backend
      - rust-websocket
    environment:
      - PYTHON_BACKEND_URL=http://python-backend:5000
      - RUST_WEBSOCKET_URL=ws://rust-websocket:8081

  python-backend:
    build: ./deployment/Dockerfile.python
    ports:
      - "5000:5000"
    environment:
      - PORT=5000

  rust-websocket:
    build: ./deployment/Dockerfile.rust
    ports:
      - "8081:8081"
    environment:
      - PORT=8081
```

### 启动Docker服务
```bash
# 构建并启动所有服务
docker-compose up -d

# 查看服务状态
docker-compose ps

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

## 🔧 手动部署步骤

### 步骤1: 环境准备
```bash
# 更新系统
sudo apt-get update
sudo apt-get upgrade -y

# 安装基础工具
sudo apt-get install -y git curl wget

# 安装Node.js (前端需要)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# 安装Python3
sudo apt-get install -y python3 python3-pip python3-venv

# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 步骤2: 获取项目代码
```bash
# 克隆项目
git clone https://github.com/facaibaofuwang/nexusremote-p2p-system.git
cd nexusremote-p2p-system

# 或下载发布版本
# wget https://github.com/facaibaofuwang/nexusremote-p2p-system/releases/latest/download/nexusremote-release.tar.gz
# tar -xzf nexusremote-release.tar.gz
# cd nexusremote-release
```

### 步骤3: 部署Python后端
```bash
cd python-backend

# 创建虚拟环境
python3 -m venv venv
source venv/bin/activate

# 安装依赖
pip install -r requirements.txt

# 启动服务 (后台运行)
nohup python main.py > backend.log 2>&1 &

# 验证服务
curl http://localhost:5000/api/health
```

### 步骤4: 部署Rust WebSocket服务器
```bash
cd ../rust-backend

# 编译发布版本
cargo build --release

# 启动服务 (后台运行)
nohup ./target/release/websocket_server > websocket.log 2>&1 &

# 验证服务
# 使用WebSocket客户端测试连接
```

### 步骤5: 部署前端服务
```bash
cd ../frontend

# 安装依赖
npm install

# 启动服务 (后台运行)
nohup npm start > frontend.log 2>&1 &

# 验证服务
curl http://localhost:3000/api/devices
```

## ⚙️ 配置说明

### 环境变量配置
```bash
# 前端服务配置
export PYTHON_BACKEND_URL="http://localhost:5000"
export RUST_WEBSOCKET_URL="ws://localhost:8081"
export FRONTEND_PORT=3000

# Python后端配置
export PYTHON_PORT=5000
export LOG_LEVEL="INFO"

# Rust后端配置
export RUST_PORT=8081
export RUST_LOG="info"
```

### 配置文件
```yaml
# config.yaml (可选)
services:
  frontend:
    port: 3000
    api_timeout: 30
    websocket_reconnect: true
    
  python_backend:
    port: 5000
    algorithm:
      advantage_target: 1.5
      simulation_nodes: 100
      
  rust_websocket:
    port: 8081
    max_connections: 1000
    heartbeat_interval: 30
```

## 📊 监控和维护

### 服务健康检查
```bash
# 检查所有服务状态
./scripts/health-check.sh

# 输出示例：
# ✅ 前端服务: http://localhost:3000 (200 OK)
# ✅ Python后端: http://localhost:5000/api/health (healthy)
# ✅ Rust WebSocket: ws://localhost:8081 (connected)
```

### 日志管理
```bash
# 查看实时日志
tail -f python-backend/backend.log
tail -f rust-backend/websocket.log
tail -f frontend/frontend.log

# 日志轮转配置
# 在 /etc/logrotate.d/nexusremote 中添加：
# /path/to/nexusremote/*.log {
#   daily
#   rotate 7
#   compress
#   missingok
#   notifempty
# }
```

### 性能监控
```bash
# 监控系统资源
./scripts/monitor-resources.sh

# 监控API响应时间
./scripts/monitor-api.sh

# 监控WebSocket连接数
./scripts/monitor-websocket.sh
```

## 🔒 安全配置

### 防火墙设置
```bash
# 只开放必要端口
sudo ufw allow 3000/tcp  # 前端
sudo ufw allow 5000/tcp  # Python后端
sudo ufw allow 8081/tcp  # Rust WebSocket
sudo ufw enable
```

### HTTPS配置 (生产环境)
```bash
# 使用Nginx反向代理
sudo apt-get install -y nginx
sudo cp deployment/nginx.conf /etc/nginx/sites-available/nexusremote
sudo ln -s /etc/nginx/sites-available/nexusremote /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx

# 配置SSL证书 (Let's Encrypt)
sudo apt-get install -y certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

## 🚨 故障排除

### 常见问题

#### 问题1: 端口被占用
```bash
# 检查端口占用
sudo lsof -i :3000
sudo lsof -i :5000
sudo lsof -i :8081

# 释放端口
sudo kill -9 <PID>
```

#### 问题2: 服务启动失败
```bash
# 检查日志
tail -100 python-backend/backend.log
tail -100 rust-backend/websocket.log
tail -100 frontend/frontend.log

# 重新启动服务
./scripts/restart-services.sh
```

#### 问题3: 依赖安装失败
```bash
# 清理并重新安装
cd frontend && rm -rf node_modules && npm cache clean --force && npm install
cd ../python-backend && rm -rf venv && python3 -m venv venv && pip install -r requirements.txt
cd ../rust-backend && cargo clean && cargo build --release
```

#### 问题4: 内存不足
```bash
# 检查内存使用
free -h

# 优化Rust编译 (使用更少内存)
cd rust-backend
CARGO_BUILD_JOBS=1 cargo build --release
```

### 诊断脚本
```bash
# 运行完整诊断
./scripts/diagnose.sh

# 输出诊断报告到文件
./scripts/diagnose.sh > diagnosis-report.txt
```

## 📈 扩展和优化

### 水平扩展
```bash
# 使用负载均衡器
# 配置多个前端实例
# 配置多个Python后端实例
# 配置多个Rust WebSocket实例
```

### 性能优化
```bash
# 优化Rust编译参数
export RUSTFLAGS="-C target-cpu=native"

# 优化Python性能
pip install gunicorn
gunicorn -w 4 -b 0.0.0.0:5000 main:app

# 优化前端性能
npm run build
serve -s build -l 3000
```

### 监控集成
```bash
# 集成Prometheus监控
./scripts/setup-monitoring.sh

# 集成Grafana仪表板
./scripts/setup-grafana.sh

# 集成日志聚合
./scripts/setup-log-aggregation.sh
```

## 📞 支持与联系

### 获取帮助
- **GitHub Issues**: https://github.com/facaibaofuwang/nexusremote-p2p-system/issues
- **文档网站**: 项目README.md
- **社区支持**: GitHub Discussions

### 报告问题
```bash
# 收集诊断信息
./scripts/collect-debug-info.sh

# 提交问题报告时包含：
# 1. 操作系统版本
# 2. 软件版本 (Node.js, Python, Rust)
# 3. 错误日志
# 4. 复现步骤
```

### 更新和升级
```bash
# 检查更新
git pull origin main

# 重新部署
./scripts/redeploy.sh

# 回滚到上一个版本
git checkout <previous-commit>
./scripts/redeploy.sh
```

---

*部署指南版本: 1.0.0*
*最后更新: 2026-02-28*
*适用于: NexusRemote v1.0 发布版本*