/**
 * NexusRemote Rust WebSocket 客户端
 * 连接Rust后端WebSocket服务器 (ws://localhost:8081)
 */

class RustWebSocketClient {
    constructor() {
        this.ws = null;
        this.clientId = null;
        this.isConnected = false;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000; // 1秒
        
        // 事件监听器
        this.listeners = {
            'connect': [],
            'disconnect': [],
            'message': [],
            'error': [],
            'peers': [],
            'routingStats': [],
            'commandResult': []
        };
    }
    
    /**
     * 连接到Rust WebSocket服务器
     */
    connect() {
        if (this.isConnected) {
            console.log('已经连接到Rust WebSocket服务器');
            return;
        }
        
        console.log('正在连接到Rust WebSocket服务器...');
        this.ws = new WebSocket('ws://localhost:8081');
        
        this.ws.onopen = () => {
            console.log('✅ 已连接到Rust WebSocket服务器');
            this.isConnected = true;
            this.reconnectAttempts = 0;
            this.emit('connect', { timestamp: new Date().toISOString() });
        };
        
        this.ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                console.log('收到Rust服务器消息:', data);
                this.handleMessage(data);
            } catch (error) {
                console.error('解析消息失败:', error, event.data);
            }
        };
        
        this.ws.onclose = (event) => {
            console.log('Rust WebSocket连接关闭:', event.code, event.reason);
            this.isConnected = false;
            this.emit('disconnect', { 
                code: event.code, 
                reason: event.reason,
                timestamp: new Date().toISOString() 
            });
            
            // 尝试重新连接
            this.attemptReconnect();
        };
        
        this.ws.onerror = (error) => {
            console.error('Rust WebSocket错误:', error);
            this.emit('error', { 
                error: error, 
                timestamp: new Date().toISOString() 
            });
        };
    }
    
    /**
     * 处理来自服务器的消息
     */
    handleMessage(data) {
        this.emit('message', data);
        
        switch (data.type) {
            case 'welcome':
                this.clientId = data.client_id;
                console.log('欢迎消息，客户端ID:', this.clientId);
                break;
                
            case 'peers':
                console.log('收到对等节点列表:', data.peers.length, '个节点');
                this.emit('peers', data.peers);
                break;
                
            case 'routing_stats':
                console.log('收到路由统计:', data);
                this.emit('routingStats', data);
                break;
                
            case 'command_result':
                console.log('命令结果:', data);
                this.emit('commandResult', data);
                break;
                
            case 'pong':
                console.log('收到Pong响应');
                break;
                
            case 'error':
                console.error('服务器错误:', data.message);
                break;
                
            default:
                console.log('未知消息类型:', data.type);
        }
    }
    
    /**
     * 尝试重新连接
     */
    attemptReconnect() {
        if (this.reconnectAttempts >= this.maxReconnectAttempts) {
            console.log('已达到最大重连次数，停止重连');
            return;
        }
        
        this.reconnectAttempts++;
        console.log(`尝试重新连接 (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
        
        setTimeout(() => {
            this.connect();
        }, this.reconnectDelay * this.reconnectAttempts);
    }
    
    /**
     * 断开连接
     */
    disconnect() {
        if (this.ws) {
            this.ws.close(1000, '客户端主动断开');
            this.ws = null;
        }
        this.isConnected = false;
    }
    
    /**
     * 发送消息到服务器
     */
    sendMessage(type, data = {}) {
        if (!this.isConnected || !this.ws) {
            console.error('未连接到服务器，无法发送消息');
            return false;
        }
        
        const message = {
            type: type,
            ...data,
            client_id: this.clientId,
            timestamp: new Date().toISOString()
        };
        
        try {
            this.ws.send(JSON.stringify(message));
            console.log('发送消息:', message);
            return true;
        } catch (error) {
            console.error('发送消息失败:', error);
            return false;
        }
    }
    
    /**
     * 发送Ping消息
     */
    ping() {
        return this.sendMessage('ping');
    }
    
    /**
     * 获取对等节点列表
     */
    getPeers(targetId = '') {
        return this.sendMessage('get_peers', { target_id: targetId });
    }
    
    /**
     * 发送远程命令
     */
    sendCommand(command, target = '') {
        return this.sendMessage('send_command', { 
            command: command, 
            target: target 
        });
    }
    
    /**
     * 获取路由统计
     */
    getRoutingStats() {
        return this.sendMessage('get_routing_stats');
    }
    
    /**
     * 添加事件监听器
     */
    on(event, callback) {
        if (this.listeners[event]) {
            this.listeners[event].push(callback);
        }
    }
    
    /**
     * 移除事件监听器
     */
    off(event, callback) {
        if (this.listeners[event]) {
            this.listeners[event] = this.listeners[event].filter(cb => cb !== callback);
        }
    }
    
    /**
     * 触发事件
     */
    emit(event, data) {
        if (this.listeners[event]) {
            this.listeners[event].forEach(callback => {
                try {
                    callback(data);
                } catch (error) {
                    console.error(`事件监听器错误 (${event}):`, error);
                }
            });
        }
    }
    
    /**
     * 获取连接状态
     */
    getStatus() {
        return {
            connected: this.isConnected,
            clientId: this.clientId,
            reconnectAttempts: this.reconnectAttempts,
            maxReconnectAttempts: this.maxReconnectAttempts
        };
    }
}

// 创建全局实例
window.RustWebSocketClient = RustWebSocketClient;

// 自动连接（可选）
document.addEventListener('DOMContentLoaded', function() {
    // 检查页面是否需要自动连接
    if (document.body.hasAttribute('data-rust-websocket-auto-connect')) {
        const client = new RustWebSocketClient();
        window.rustWebSocketClient = client;
        client.connect();
        
        // 添加一些示例事件监听器
        client.on('connect', function(data) {
            console.log('连接成功事件:', data);
            // 连接成功后获取路由统计
            setTimeout(() => {
                client.getRoutingStats();
                client.getPeers();
            }, 1000);
        });
        
        client.on('peers', function(peers) {
            console.log('收到对等节点:', peers);
            // 可以在这里更新UI显示对等节点
            updatePeerListUI(peers);
        });
        
        client.on('routingStats', function(stats) {
            console.log('路由统计:', stats);
            // 可以在这里更新UI显示路由统计
            updateRoutingStatsUI(stats);
        });
    }
});

// 示例UI更新函数
function updatePeerListUI(peers) {
    const container = document.getElementById('rust-peers-container');
    if (!container) return;
    
    container.innerHTML = `
        <h3>Rust后端对等节点 (${peers.length})</h3>
        <div class="peer-list">
            ${peers.map(peer => `
                <div class="peer-item">
                    <div class="peer-name">${peer.peer_id}</div>
                    <div class="peer-info">
                        <span class="peer-reputation">信誉: ${peer.reputation}</span>
                        <span class="peer-role">角色: ${peer.role}</span>
                    </div>
                    <div class="peer-address">${peer.addresses.join(', ')}</div>
                </div>
            `).join('')}
        </div>
    `;
}

function updateRoutingStatsUI(stats) {
    const container = document.getElementById('rust-routing-stats-container');
    if (!container) return;
    
    container.innerHTML = `
        <h3>加权路由统计</h3>
        <div class="stats-grid">
            <div class="stat-item">
                <div class="stat-label">总节点数</div>
                <div class="stat-value">${stats.total_peers}</div>
            </div>
            <div class="stat-item">
                <div class="stat-label">高信誉节点</div>
                <div class="stat-value">${stats.high_reputation_peers}</div>
            </div>
            <div class="stat-item">
                <div class="stat-label">低信誉节点</div>
                <div class="stat-value">${stats.low_reputation_peers}</div>
            </div>
            <div class="stat-item">
                <div class="stat-label">预期优势</div>
                <div class="stat-value">${stats.expected_advantage}x</div>
            </div>
        </div>
        <div class="status-indicator ${stats.weighted_routing_enabled ? 'enabled' : 'disabled'}">
            加权路由: ${stats.weighted_routing_enabled ? '已启用' : '已禁用'}
        </div>
    `;
}

console.log('Rust WebSocket客户端已加载');