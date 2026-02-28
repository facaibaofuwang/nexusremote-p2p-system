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

// 代理API端点到Python后端 - 修正版本
app.all('/api/*', async (req, res) => {
    try {
        // 保持原始路径，直接转发到Python后端
        const pythonUrl = `${PYTHON_BACKEND}${req.path}`;
        
        console.log(`🔗 代理请求: ${req.method} ${req.path} -> ${pythonUrl}`);
        
        const response = await proxyRequest(pythonUrl, {
            method: req.method,
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: req.method !== 'GET' ? req.body : undefined
        });
        
        // 转发响应
        res.status(response.statusCode).json(response.data);
    } catch (error) {
        console.error('代理请求失败:', error);
        res.status(502).json({
            success: false,
            error: '后端服务不可用',
            message: error.message,
            timestamp: new Date().toISOString()
        });
    }
});

// 静态文件服务
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
    console.log(`🚀 NexusRemote前端代理服务器 (修正版) 运行在 http://localhost:${PORT}`);
    console.log(`📊 Python后端地址: ${PYTHON_BACKEND}`);
    console.log(`🔗 代理路径: ${PYTHON_BACKEND}/api/*`);
    console.log(`🌐 静态文件服务: /, /index.html, /connect.html, /simple-test.html`);
});

// 简单的WebSocket代理
const wss = new WebSocket.Server({ server });

wss.on('connection', (clientWS) => {
    const clientId = `client_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    console.log(`🔌 新的前端WebSocket连接: ${clientId}`);
    
    // 尝试连接到Python后端WebSocket
    const pythonWS = new WebSocket('ws://localhost:5000');
    
    pythonWS.on('open', () => {
        console.log(`🔗 连接到Python后端WebSocket成功: ${clientId}`);
        
        // 转发消息：前端 -> Python后端
        clientWS.on('message', (message) => {
            try {
                pythonWS.send(message);
            } catch (error) {
                console.error('转发消息到Python失败:', error);
            }
        });
        
        // 转发消息：Python后端 -> 前端
        pythonWS.on('message', (message) => {
            try {
                if (clientWS.readyState === WebSocket.OPEN) {
                    clientWS.send(message);
                }
            } catch (error) {
                console.error('转发消息到前端失败:', error);
            }
        });
        
        // 发送连接成功消息
        clientWS.send(JSON.stringify({
            type: 'connected',
            message: 'WebSocket连接成功，已连接到Python后端',
            clientId,
            timestamp: new Date().toISOString()
        }));
    });
    
    pythonWS.on('error', (error) => {
        console.error(`❌ Python WebSocket连接错误: ${error.message}`);
        clientWS.send(JSON.stringify({
            type: 'error',
            message: '无法连接到后端WebSocket',
            error: error.message,
            timestamp: new Date().toISOString()
        }));
    });
    
    // 前端连接关闭
    clientWS.on('close', () => {
        console.log(`🔌 前端WebSocket连接关闭: ${clientId}`);
        if (pythonWS.readyState === WebSocket.OPEN) {
            pythonWS.close();
        }
    });
    
    // 错误处理
    clientWS.on('error', (error) => {
        console.error(`❌ 前端WebSocket错误: ${error.message}`);
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

console.log(`\n🎯 代理服务器配置:`);
console.log(`  前端地址: http://localhost:${PORT}`);
console.log(`  Python后端: ${PYTHON_BACKEND}`);
console.log(`  API代理: 所有 /api/* 请求保持路径转发`);
console.log(`  WebSocket代理: 前端WS <-> Python后端WS`);