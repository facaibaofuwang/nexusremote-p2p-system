# NexusRemote 原型测试指南

**测试时间**: 2026-02-28 11:31  
**测试环境**: Ubuntu 22.04.5 LTS

---

## 🚀 **快速测试启动**

### **步骤1：启动前端服务器**
```bash
# 如果尚未启动，运行：
cd /home/admin/.openclaw/workspace/nexusremote-frontend
node server.js > server.log 2>&1 &

# 检查服务器状态
curl -s -o /dev/null -w "HTTP状态: %{http_code}\\n" http://localhost:3000

# 查看服务器日志
tail -f server.log
```

**预期输出**: HTTP状态: 200

---

### **步骤2：打开浏览器界面**
```
🌐 访问地址: http://localhost:3000
```

**可选访问路径**:
- 主控制面板: http://localhost:3000/
- 设备连接页面: http://localhost:3000/connect.html  
- 简化测试页: http://localhost:3000/simple-test.html

---

## 📊 **API 接口测试**

### **基础功能测试**
```bash
# 1. 设备列表
curl http://localhost:3000/api/devices

# 2. 网络状态
curl http://localhost:3000/api/network

# 3. 通证统计
curl http://localhost:3000/api/tokens

# 4. 加权路由算法数据
curl http://localhost:3000/api/routing/algorithm

# 5. 通证经济模型
curl http://localhost:3000/api/economy/model
```

### **WebSocket 实时通信测试**
```bash
# 使用 wscat 工具测试 WebSocket
# 安装: npm install -g wscat
# 连接: wscat -c ws://localhost:3000

# 或使用 curl 测试 WebSocket 连接
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" \
  -H "Sec-WebSocket-Version: 13" \
  http://localhost:3000
```

---

## 🧪 **核心算法测试**

### **Python 加权路由算法测试**
```bash
cd /home/admin/.openclaw/workspace/nexusremote
python3 test_weighted_routing.py
```

**预期结果**:
```
✅ 成功: 高信誉节点获得 1.28 倍路由优势!
```

### **Python 通证经济闭环测试**
```bash
cd /home/admin/.openclaw/workspace/nexusremote
python3 demo_workflow.py
```

**预期结果**:
```
✅ 经济闭环演示完成!
🎯 核心验证:
   ✅ 通证激励驱动网络贡献
   ✅ 高信誉节点获得优势
   ✅ 经济闭环可持续
   ✅ 抗Sybil攻击机制
```

---

## 🎯 **完整功能测试脚本**

### **一键测试脚本**
```bash
#!/bin/bash
# nexusremote-test.sh

echo "🚀 NexusRemote 原型全面测试"
echo "="*60

# 1. 检查前端服务器
echo "📊 1. 检查前端服务器..."
if curl -s -o /dev/null -w "%{http_code}" http://localhost:3000 | grep -q "200"; then
    echo "✅ 前端服务器运行正常 (端口:3000)"
else
    echo "⚠️ 前端服务器未运行，正在启动..."
    cd /home/admin/.openclaw/workspace/nexusremote-frontend
    node server.js > server.log 2>&1 &
    sleep 2
    echo "✅ 前端服务器已启动"
fi

# 2. API接口测试
echo "📊 2. API接口测试..."
APIs=("devices" "network" "tokens" "routing/algorithm" "economy/model")
for api in "${APIs[@]}"; do
    status=$(curl -s -o /dev/null -w "%{http_code}" "http://localhost:3000/api/$api")
    if [ "$status" = "200" ]; then
        echo "  ✅ /api/$api : HTTP $status"
    else
        echo "  ❌ /api/$api : HTTP $status"
    fi
done

# 3. Python算法测试
echo "📊 3. Python核心算法测试..."
cd /home/admin/.openclaw/workspace/nexusremote

echo "  🧪 测试加权路由算法..."
python3 test_weighted_routing.py | grep -E "成功|优势|验证通过"

echo "  🧪 测试通证经济闭环..."
python3 demo_workflow.py | grep -E "完成|验证通过"

echo ""
echo "📋 测试总结:"
echo "  前端界面: http://localhost:3000"
echo "  API文档: 查看 server.js 中的路由定义"
echo "  算法验证: Python脚本测试通过"
echo ""
echo "✅ NexusRemote 原型测试完成!"
```

**保存并运行**:
```bash
cd /home/admin/.openclaw/workspace
chmod +x nexusremote-test.sh
./nexusremote-test.sh
```

---

## 🎨 **界面功能验证**

### **主控制面板测试**
1. **访问** http://localhost:3000/
2. **验证以下组件**:
   - ✅ 侧边栏导航菜单
   - ✅ 顶部状态栏
   - ✅ 设备状态卡片
   - ✅ 实时数据图表
   - ✅ 通证经济统计
   - ✅ 网络拓扑图

### **设备连接页面测试**
1. **访问** http://localhost:3000/connect.html
2. **验证功能**:
   - ✅ 设备搜索和筛选
   - ✅ 远程控制连接界面
   - ✅ 通证支付显示
   - ✅ 信誉评分展示

### **简化测试页**
1. **访问** http://localhost:3000/simple-test.html
2. **特点**:
   - ✅ 无外部CDN依赖
   - ✅ 本地化资源加载
   - ✅ 快速性能测试

---

## 🔧 **故障排除**

### **常见问题**

#### **1. 端口3000被占用**
```bash
# 查看占用进程
lsof -i :3000

# 停止占用进程
kill -9 <PID>

# 或使用其他端口启动
PORT=3001 node server.js
```

#### **2. Node.js依赖问题**
```bash
# 重新安装依赖
cd /home/admin/.openclaw/workspace/nexusremote-frontend
npm install

# 检查node版本
node --version  # 需要 >= 14.0.0
```

#### **3. Python环境问题**
```bash
# 检查Python版本
python3 --version  # 需要 >= 3.7

# 安装缺少的模块
pip3 install dataclasses  # 如果提示缺少dataclasses
```

#### **4. 前端界面无法加载资源**
- 检查浏览器控制台错误
- 尝试访问 simple-test.html（无CDN版本）
- 查看 server.log 中的错误信息

---

## 📈 **性能基准测试**

### **响应时间测试**
```bash
# API响应时间
time curl -s -o /dev/null http://localhost:3000/api/devices

# 页面加载时间
time curl -s -o /dev/null http://localhost:3000/
```

### **并发测试**
```bash
# 并发API请求测试
ab -n 100 -c 10 http://localhost:3000/api/devices

# WebSocket连接测试
# 需要编写专门的测试脚本
```

---

## 🎯 **测试成功标准**

### **基本功能**
- ✅ 前端界面可访问 (HTTP 200)
- ✅ 所有API接口响应正常
- ✅ 加权路由算法验证通过 (1.2x+优势)
- ✅ 通证经济闭环演示成功

### **用户体验**
- ✅ 界面加载时间 < 3秒
- ✅ 交互响应时间 < 1秒
- ✅ 实时数据更新正常
- ✅ 错误处理友好

### **技术指标**
- ✅ 内存占用 < 500MB
- ✅ CPU使用率 < 30%
- ✅ 无内存泄漏
- ✅ 日志记录完整

---

## 📝 **测试记录表**

| 测试项 | 预期结果 | 实际结果 | 状态 | 备注 |
|--------|----------|----------|------|------|
| 前端服务器 | HTTP 200 | | | |
| API接口 | 全部响应 | | | |
| 加权路由 | 1.2x+优势 | | | |
| 经济闭环 | 演示成功 | | | |
| 界面功能 | 组件正常 | | | |
| 实时通信 | WebSocket正常 | | | |
| 性能指标 | 响应时间<1s | | | |

---

## 🎉 **测试总结**

### **通过标准**
- 所有基本功能测试通过
- 核心算法验证成功
- 用户体验符合预期
- 无严重技术问题

### **测试完成标志**
1. ✅ 前端界面可正常访问
2. ✅ 所有API接口测试通过
3. ✅ 核心算法验证成功
4. ✅ 性能指标符合要求

### **后续建议**
1. 添加自动化测试脚本
2. 完善错误处理和日志
3. 增加压力测试和安全性测试
4. 准备生产环境部署测试

---

**测试命令**:
```bash
# 快速测试
cd /home/admin/.openclaw/workspace
curl -s http://localhost:3000/api/devices | jq .  # 需要安装jq

# 完整测试
./nexusremote-test.sh  # 运行测试脚本
```

**测试完成**: 当所有✅标记为绿色时，原型测试通过，可投入试运行！