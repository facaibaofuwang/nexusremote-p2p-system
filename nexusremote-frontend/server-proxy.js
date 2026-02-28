const express = require('express');
const cors = require('cors');
const http = require('http');
const path = require('path');
const WebSocket = require('ws');

const app = express();
const PORT = process.env.PORT || 3000;
const PYTHON_BACKEND = 'http://localhost:5000';

// 中间件
app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname)));

// 简单的HTTP请求函数
function proxyRequest(url, options = {}) {
    return new Promise((resolve, reject) => {
        const req = http.request(url, options, (res) => {
            let data = '';
            res.on('data', (chunk) => data += chunk);
            res.on('end', () => {
                try {
                    const parsed = JSON.parse(data);
                    resolve({
                        statusCode: res.statusCode,
                        headers: res.headers,
                        data: parsed
                    });
                } catch (e) {
                    resolve({
                        statusCode: res.statusCode,
                        headers: res.headers,
                        data: data
                    });
                }
            });
        });
        
        req.on('error', reject);
        
        if (options.body) {
            req.write(JSON.stringify(options.body));
        }
        
        req.end();
    });
}

// 代理API端点到Python后端
app.get('/api/*', async (req, res) => {
    try {
        const path = req.path.replace('/api', '');
        const url = `${PYTHON_BACKEND}${path}`;
        
        console.log(`🔗 代理请求: ${req.path} -> ${url}`);
        
        const response = await proxyRequest(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        });
        
        // 转发响应
        res.status(response.statusCode).json(response.data);
    } catch (error) {
        console.error('代理请求失败:', error);
        res.status(500).json({
            success: false,
            error: '后端服务不可用',
            message: error.message
        });
    }
});

app.post('/api/*', async (req, res) => {
    try {
        const path = req.path.replace('/api', '');
        const url = `${PYTHON_BACKEND}${path}`;
        
        console.log(`🔗 代理POST请求: ${req.path} -> ${url}`);
        
        const response = await proxyRequest(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: req.body
        });
        
        // 转发响应
        res.status(response.statusCode).json(response.data);
    } catch (error) {
        console.error('代理POST请求失败:', error);
        res.status(500).json({
            success: false,
            error: '后端服务不可用',
            message: error.message
        });
    }
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

app.get('/simple-test.html', (req, res) => {
    res.sendFile(path.join(__dirname, 'simple-test.html'));
});

// 启动服务器
const server = app.listen(PORT, () => {
    console.log(`🚀 NexusRemote前端代理服务器运行在 http://localhost:${PORT}`);
    console.log(`📊 Python后端地址: ${PYTHON_BACKEND}`);
    console.log(`🔗 代理路径: /api/* -> ${PYTHON_BACKEND}/api/*`);
    console.log(`🌐 静态文件服务: /, /index.html, /connect.html`);
});

// WebSocket服务器 - 转发到Python后端WebSocket
const wss = new WebSocket.Server({ server });

// Python后端WebSocket连接池
const pythonWSConnections = new Map();

function connectToPythonWebSocket(clientId) {
    return new Promise((resolve, reject) => {
        const pythonWS = new WebSocket('ws://localhost:5000');
        
        pythonWS.on('open', () => {
            console.log(`🔗 连接到Python后端WebSocket: ${clientId}`);
            pythonWSConnections.set(clientId, pythonWS);
            resolve(pythonWS);
        });
        
        pythonWS.on('error', (error) => {
            console.error(`❌ Python WebSocket连接错误: ${error.message}`);
            pythonWSConnections.delete(clientId);
            reject(error);
        });
        
        pythonWS.on('close', () => {
            console.log(`🔌 Python WebSocket连接关闭: ${clientId}`);
            pythonWSConnections.delete(clientId);
        });
    });
}

wss.on('connection', (clientWS) => {
    const clientId = `client_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    console.log(`🔌 新的前端WebSocket连接: ${clientId}`);
    
    // 连接到Python后端WebSocket
    connectToPythonWebSocket(clientId)
        .then((pythonWS) => {
            // 转发消息：前端 -> Python后端
            clientWS.on('message', (message) => {
                try {
                    console.log(`📨 前端 -> Python: ${message.toString().substring(0, 100)}...`);
                    pythonWS.send(message);
                } catch (error) {
                    console.error('转发消息到Python失败:', error);
                }
            });
            
            // 转发消息：Python后端 -> 前端
            pythonWS.on('message', (message) => {
                try {
                    console.log(`📨 Python -> 前端: ${message.toString().substring(0, 100)}...`);
                    if (clientWS.readyState === WebSocket.OPEN) {
                        clientWS.send(message);
                    }
                } catch (error) {
                    console.error('转发消息到前端失败:', error);
                }
            });
            
            // 前端连接关闭
            clientWS.on('close', () => {
                console.log(`🔌 前端WebSocket连接关闭: ${clientId}`);
                if (pythonWS.readyState === WebSocket.OPEN) {
                    pythonWS.close();
                }
                pythonWSConnections.delete(clientId);
            });
            
            // Python后端连接关闭
            pythonWS.on('close', () => {
                console.log(`🔌 Python后端WebSocket关闭，通知前端: ${clientId}`);
                if (clientWS.readyState === WebSocket.OPEN) {
                    clientWS.send(JSON.stringify({
                        type: 'system',
                        message: '后端连接断开，正在重连...',
                        timestamp: new Date().toISOString()
                    }));
                }
                pythonWSConnections.delete(clientId);
                
                // 尝试重连
                setTimeout(() => {
                    console.log(`🔄 尝试重连到Python后端: ${clientId}`);
                    connectToPythonWebSocket(clientId).catch(() => {
                        console.error(`❌ 重连失败: ${clientId}`);
                    });
                }, 5000);
            });
            
            // 发送连接成功消息
            clientWS.send(JSON.stringify({
                type: 'connected',
                message: 'WebSocket连接成功，已连接到Python后端',
                clientId,
                timestamp: new Date().toISOString()
            }));
        })
        .catch((error) => {
            console.error(`❌ 无法连接到Python后端WebSocket: ${error.message}`);
            clientWS.send(JSON.stringify({
                type: 'error',
                message: '无法连接到后端服务',
                error: error.message,
                timestamp: new Date().toISOString()
            }));
            clientWS.close();
        });
    
    // 错误处理
    clientWS.on('error', (error) => {
        console.error(`❌ 前端WebSocket错误: ${error.message}`);
    });
});

// 优雅关闭
process.on('SIGTERM', () => {
    console.log('收到SIGTERM信号，正在关闭服务器...');
    // 关闭所有WebSocket连接
    pythonWSConnections.forEach((ws) => {
        if (ws.readyState === WebSocket.OPEN) {
            ws.close();
        }
    });
    server.close(() => {
        console.log('服务器已关闭');
        process.exit(0);
    });
});

process.on('SIGINT', () => {
    console.log('收到SIGINT信号，正在关闭服务器...');
    // 关闭所有WebSocket连接
    pythonWSConnections.forEach((ws) => {
        if (ws.readyState === WebSocket.OPEN) {
            ws.close();
        }
    });
    server.close(() => {
        console.log('服务器已关闭');
        process.exit(0);
    });
});

console.log(`\n🎯 代理服务器已启动配置:`);
console.log(`  前端地址: http://localhost:${PORT}`);
console.log(`  Python后端: ${PYTHON_BACKEND}`);
console.log(`  API代理: 所有 /api/* 请求转发到Python后端`);
console.log(`  WebSocket代理: 前端WS -> Python后端WS`);
console.log(`\n📊 健康检查:`);
console.log(`  前端: http://localhost:${PORT}/`);
console.log(`  Python后端: ${PYTHON_BACKEND}/api/health`);
console.log(`\n🚀 集成测试:`);
console.log(`  curl http://localhost:${PORT}/api/health`);
console.log(`  curl http://localhost:${PORT}/api/routing/algorithm`);
console.log(`  curl http://localhost:${PORT}/api/devices`);