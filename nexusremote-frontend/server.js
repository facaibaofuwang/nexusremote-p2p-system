const express = require('express');
const cors = require('cors');
const WebSocket = require('ws');
const path = require('path');
const fs = require('fs');

const app = express();
const PORT = process.env.PORT || 3000;

// 中间件
app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname)));

// 模拟数据存储
const mockData = {
  devices: [
    {
      id: 1,
      name: '办公室电脑',
      ip: '192.168.1.100',
      status: 'online',
      type: '控制端',
      lastActive: '刚刚',
      cpuUsage: 45,
      memoryUsage: 62,
      tokenBalance: 1248.5
    },
    {
      id: 2,
      name: '家用笔记本',
      ip: '192.168.1.105',
      status: 'online',
      type: '被控端',
      lastActive: '5分钟前',
      cpuUsage: 32,
      memoryUsage: 48,
      tokenBalance: 856.2
    },
    {
      id: 3,
      name: '测试服务器',
      ip: '192.168.1.110',
      status: 'offline',
      type: '服务器',
      lastActive: '2小时前',
      cpuUsage: 0,
      memoryUsage: 0,
      tokenBalance: 2100.0
    },
    {
      id: 4,
      name: '移动设备',
      ip: '192.168.1.115',
      status: 'online',
      type: '移动端',
      lastActive: '10分钟前',
      cpuUsage: 28,
      memoryUsage: 74,
      tokenBalance: 125.5
    }
  ],
  networkStats: {
    latency: 12,
    bandwidth: 50,
    packetLoss: 0.1,
    nodes: 127,
    health: 92
  },
  tokenStats: {
    balance: 1248.5,
    change: 12.5,
    transactions: [
      { id: 1, type: 'receive', amount: 12.5, from: '节点A', time: '10分钟前' },
      { id: 2, type: 'send', amount: 5.0, to: '节点B', time: '1小时前' },
      { id: 3, type: 'receive', amount: 8.2, from: '节点C', time: '3小时前' }
    ]
  }
};

// API路由

// 获取设备列表
app.get('/api/devices', (req, res) => {
  res.json({
    success: true,
    data: mockData.devices,
    timestamp: new Date().toISOString()
  });
});

// 获取网络状态
app.get('/api/network', (req, res) => {
  res.json({
    success: true,
    data: mockData.networkStats,
    timestamp: new Date().toISOString()
  });
});

// 获取通证统计
app.get('/api/tokens', (req, res) => {
  res.json({
    success: true,
    data: mockData.tokenStats,
    timestamp: new Date().toISOString()
  });
});

// 连接设备
app.post('/api/devices/connect', (req, res) => {
  const { deviceId } = req.body;
  const device = mockData.devices.find(d => d.id === parseInt(deviceId));
  
  if (device) {
    res.json({
      success: true,
      message: `正在连接设备: ${device.name}`,
      device,
      connectionId: `conn_${Date.now()}`
    });
  } else {
    res.status(404).json({
      success: false,
      message: '设备未找到'
    });
  }
});

// 发送远程命令
app.post('/api/devices/command', (req, res) => {
  const { deviceId, command, params } = req.body;
  
  res.json({
    success: true,
    message: `命令已发送: ${command}`,
    commandId: `cmd_${Date.now()}`,
    timestamp: new Date().toISOString()
  });
});

// 获取设备详情
app.get('/api/devices/:id', (req, res) => {
  const deviceId = parseInt(req.params.id);
  const device = mockData.devices.find(d => d.id === deviceId);
  
  if (device) {
    res.json({
      success: true,
      data: device
    });
  } else {
    res.status(404).json({
      success: false,
      message: '设备未找到'
    });
  }
});

// 模拟加权路由算法数据
const { exec } = require('child_process');

app.get('/api/routing/algorithm', (req, res) => {
  // 使用Rust CLI工具运行真实的加权路由算法模拟
  const rustBinaryPath = path.join(__dirname, '../nexusremote/target/release/main');
  
  exec(`${rustBinaryPath} simulate --nodes 100 --lookups 500`, { timeout: 10000 }, (error, stdout, stderr) => {
    if (error) {
      console.error('运行Rust模拟失败:', error);
      // 回退到模拟数据
      const fallbackData = {
        name: '加权路由算法',
        description: '基于节点信誉的智能路由选择',
        formula: 'LogicalDistance = XOR(NodeID, TargetID) * (3000 / (Reputation + 500))',
        advantage: 1.58,
        simulation: {
          highRepNodes: 33,
          lowRepNodes: 67,
          highRepSelected: 520, // 51.98% of 1000
          totalSelections: 1000,
          advantageRatio: 1.58
        },
        algorithm_version: 'optimized_v2',
        source: 'fallback_simulation'
      };
      
      return res.json({
        success: true,
        data: fallbackData,
        warning: '使用回退数据，Rust模拟失败'
      });
    }
    
    try {
      // 从Rust输出中提取数据
      let advantage = 1.58; // 默认值
      let highRepNodes = 33;
      let lowRepNodes = 67;
      let highRepSelected = 520;
      
      // 解析输出
      const lines = stdout.split('\n');
      for (const line of lines) {
        if (line.includes('High reputation nodes:')) {
          const match = line.match(/High reputation nodes:\s*(\d+)/);
          if (match) highRepNodes = parseInt(match[1]);
        }
        if (line.includes('Low reputation nodes:')) {
          const match = line.match(/Low reputation nodes:\s*(\d+)/);
          if (match) lowRepNodes = parseInt(match[1]);
        }
        if (line.includes('High reputation selection rate:')) {
          const match = line.match(/High reputation selection rate:\s*([\d.]+)%/);
          if (match) {
            const rate = parseFloat(match[1]) / 100;
            highRepSelected = Math.round(rate * 500); // 基于500次查找
          }
        }
        if (line.includes('routing advantage')) {
          const match = line.match(/have\s+([\d.]+)x\s+routing advantage/);
          if (match) advantage = parseFloat(match[1]);
        }
      }
      
      const algorithmData = {
        name: '加权路由算法',
        description: '基于节点信誉的智能路由选择',
        formula: 'LogicalDistance = XOR(NodeID, TargetID) * (3000 / (Reputation + 500))',
        advantage: advantage,
        simulation: {
          highRepNodes: highRepNodes,
          lowRepNodes: lowRepNodes,
          highRepSelected: highRepSelected,
          totalSelections: 500,
          advantageRatio: advantage
        },
        algorithm_version: 'optimized_v2',
        source: 'rust_simulation',
        raw_output: stdout.substring(0, 500) // 限制输出长度
      };
      
      res.json({
        success: true,
        data: algorithmData
      });
      
    } catch (parseError) {
      console.error('解析Rust输出失败:', parseError);
      
      // 回退数据
      const fallbackData = {
        name: '加权路由算法',
        description: '基于节点信誉的智能路由选择',
        formula: 'LogicalDistance = XOR(NodeID, TargetID) * (3000 / (Reputation + 500))',
        advantage: 1.58,
        simulation: {
          highRepNodes: 33,
          lowRepNodes: 67,
          highRepSelected: 520,
          totalSelections: 1000,
          advantageRatio: 1.58
        },
        algorithm_version: 'optimized_v2',
        source: 'fallback_parse_error'
      };
      
      res.json({
        success: true,
        data: fallbackData,
        warning: '解析失败，使用回退数据'
      });
    }
  });
});

// 模拟通证经济模型数据
app.get('/api/economy/model', (req, res) => {
  const economyData = {
    name: '通证经济闭环模型',
    description: '贡献即收益，消费需支付的可持续经济模型',
    components: [
      {
        name: '入场机制',
        description: 'PoW挖矿获得初始代币',
        tokens: 100
      },
      {
        name: '消费机制',
        description: '支付中继服务费用',
        tokens: -50
      },
      {
        name: '危机处理',
        description: '余额不足时的分级方案',
        tokens: 20
      },
      {
        name: '循环机制',
        description: '闲置时提供中继服务赚取代币',
        tokens: 30
      }
    ],
    cycle: {
      startBalance: 100,
      steps: [
        { action: 'PoW挖矿', balance: 200, change: 100 },
        { action: '使用服务', balance: 150, change: -50 },
        { action: '提供中继', balance: 180, change: 30 },
        { action: '网络奖励', balance: 200, change: 20 }
      ],
      finalBalance: 200,
      profit: 100
    }
  };
  
  res.json({
    success: true,
    data: economyData
  });
});

// 静态文件服务 - 主页面
app.get('/', (req, res) => {
  res.sendFile(path.join(__dirname, 'index_component.html'));
});

app.get('/index.html', (req, res) => {
  res.sendFile(path.join(__dirname, 'index.html'));
});

app.get('/connect.html', (req, res) => {
  res.sendFile(path.join(__dirname, 'connect.html'));
});

// 健康检查端点
app.get('/api/health', (req, res) => {
  res.json({
    status: 'healthy',
    service: 'NexusRemote Frontend',
    version: '1.0.0',
    timestamp: new Date().toISOString(),
    rust_backend_connected: rustWebSocket && rustWebSocket.readyState === WebSocket.OPEN,
    uptime: process.uptime()
  });
});

// WebSocket服务器
const server = app.listen(PORT, () => {
  console.log(`🚀 NexusRemote前端服务器运行在 http://localhost:${PORT}`);
  console.log(`📊 API接口可用:`);
  console.log(`   GET  /api/devices          # 获取设备列表`);
  console.log(`   GET  /api/network          # 获取网络状态`);
  console.log(`   GET  /api/tokens           # 获取通证统计`);
  console.log(`   GET  /api/routing/algorithm # 加权路由算法数据`);
  console.log(`   GET  /api/economy/model    # 通证经济模型数据`);
  console.log(`   POST /api/devices/connect  # 连接设备`);
  console.log(`   POST /api/devices/command  # 发送远程命令`);
  console.log(`🔗 连接到Rust后端: ws://127.0.0.1:8081`);
});

// 连接到Rust WebSocket后端
let rustWebSocket = null;

function connectToRustBackend() {
  try {
    rustWebSocket = new WebSocket('ws://127.0.0.1:8081');
    
    rustWebSocket.on('open', () => {
      console.log('✅ 成功连接到Rust WebSocket后端');
    });
    
    rustWebSocket.on('message', (data) => {
      console.log('📨 从Rust后端收到消息:', data.toString());
      // 可以在这里将消息转发给前端客户端
    });
    
    rustWebSocket.on('error', (error) => {
      console.error('❌ Rust WebSocket连接错误:', error);
    });
    
    rustWebSocket.on('close', () => {
      console.log('🔌 Rust WebSocket连接关闭，5秒后重连...');
      setTimeout(connectToRustBackend, 5000);
    });
    
  } catch (error) {
    console.error('❌ 连接Rust后端失败:', error);
    setTimeout(connectToRustBackend, 5000);
  }
}

// 初始连接
connectToRustBackend();

// 创建前端WebSocket服务器（用于浏览器连接）
const wss = new WebSocket.Server({ server });

wss.on('connection', (ws) => {
  console.log('🔌 新的前端WebSocket连接已建立');
  
  // 发送欢迎消息
  ws.send(JSON.stringify({
    type: 'welcome',
    message: '连接到NexusRemote实时数据服务',
    timestamp: new Date().toISOString(),
    rust_backend_connected: rustWebSocket && rustWebSocket.readyState === WebSocket.OPEN
  }));
  
  // 定期发送实时数据（从Rust后端或模拟数据）
  const interval = setInterval(() => {
    if (ws.readyState === WebSocket.OPEN) {
      const liveData = {
        type: 'liveData',
        timestamp: new Date().toISOString(),
        network: {
          latency: Math.max(5, Math.min(50, mockData.networkStats.latency + (Math.random() - 0.5) * 5)),
          bandwidth: Math.max(10, Math.min(100, mockData.networkStats.bandwidth + (Math.random() - 0.5) * 10)),
          packetLoss: Math.max(0, Math.min(2, mockData.networkStats.packetLoss + (Math.random() - 0.5) * 0.2)),
          nodes: mockData.networkStats.nodes + Math.floor(Math.random() * 3 - 1)
        },
        devices: mockData.devices.map(device => ({
          id: device.id,
          name: device.name,
          cpuUsage: device.status === 'online' 
            ? Math.max(10, Math.min(90, device.cpuUsage + (Math.random() - 0.5) * 10))
            : 0,
          memoryUsage: device.status === 'online'
            ? Math.max(20, Math.min(90, device.memoryUsage + (Math.random() - 0.5) * 5))
            : 0,
          status: device.status
        }))
      };
      
      ws.send(JSON.stringify(liveData));
    }
  }, 3000); // 每3秒发送一次
  
  // 处理消息
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message);
      console.log('收到消息:', data);
      
      // 根据消息类型处理
      switch (data.type) {
        case 'command':
          // 处理远程命令
          ws.send(JSON.stringify({
            type: 'commandResponse',
            commandId: data.commandId,
            status: 'executing',
            timestamp: new Date().toISOString()
          }));
          break;
          
        case 'subscribe':
          // 处理订阅请求
          ws.send(JSON.stringify({
            type: 'subscription',
            channels: data.channels,
            subscribed: true,
            timestamp: new Date().toISOString()
          }));
          break;
          
        default:
          ws.send(JSON.stringify({
            type: 'error',
            message: '未知的消息类型',
            timestamp: new Date().toISOString()
          }));
      }
    } catch (error) {
      console.error('消息处理错误:', error);
      ws.send(JSON.stringify({
        type: 'error',
        message: '消息格式错误',
        timestamp: new Date().toISOString()
      }));
    }
  });
  
  // 连接关闭
  ws.on('close', () => {
    console.log('🔌 WebSocket连接已关闭');
    clearInterval(interval);
  });
  
  // 错误处理
  ws.on('error', (error) => {
    console.error('WebSocket错误:', error);
    clearInterval(interval);
  });
});

// 优雅关闭
process.on('SIGTERM', () => {
  console.log('收到SIGTERM信号，正在关闭服务器...');
  server.close(() => {
    console.log('服务器已关闭');
    process.exit(0);
  });
});

process.on('SIGINT', () => {
  console.log('收到SIGINT信号，正在关闭服务器...');
  server.close(() => {
    console.log('服务器已关闭');
    process.exit(0);
  });
});