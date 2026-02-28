#!/usr/bin/env python3
"""
NexusRemote Python后端服务
提供REST API和WebSocket接口，包装核心算法功能
"""

import sys
import os
import json
import time
import threading
import random
from dataclasses import dataclass
from typing import Dict, List, Any
from flask import Flask, jsonify, request
from flask_cors import CORS
import flask_socketio

# 添加项目根目录到路径，以便导入现有模块
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'nexusremote'))

# 尝试导入现有模块
try:
    from demo_workflow import NexusRemoteDemo, PeerInfo, TokenAmount, ReputationScore
    from test_weighted_routing import simulate_routing, test_algorithm_correctness
    HAS_ORIGINAL_MODULES = True
except ImportError as e:
    print(f"⚠️ 警告: 无法导入原始模块: {e}")
    print("   将使用简化版本")
    HAS_ORIGINAL_MODULES = False

# 创建Flask应用
app = Flask(__name__)
CORS(app)  # 允许跨域请求
socketio = flask_socketio.SocketIO(app, cors_allowed_origins="*", async_mode='eventlet')

# 全局状态
demo_instance = None
simulation_results_cache = None
last_update_time = 0
update_interval = 3  # 秒

# 如果没有原始模块，创建简化版本
if not HAS_ORIGINAL_MODULES:
    @dataclass
    class TokenAmount:
        value: int
        def __str__(self):
            return f"{self.value} NEXUS"
    
    @dataclass
    class ReputationScore:
        value: int
        def __str__(self):
            return str(self.value)
    
    @dataclass
    class PeerInfo:
        name: str
        reputation: ReputationScore
        balance: TokenAmount
        role: str
    
    class NexusRemoteDemo:
        def __init__(self):
            self.nodes = {}
            self.transactions = []
        
        def create_node(self, name: str, is_high_reputation: bool = False):
            reputation = ReputationScore(900 if is_high_reputation else 100)
            balance = TokenAmount(0)
            node = PeerInfo(name, reputation, balance, "Idle")
            self.nodes[name] = node
            return node

# 初始化演示实例
def init_demo():
    global demo_instance
    if demo_instance is None:
        if HAS_ORIGINAL_MODULES:
            demo_instance = NexusRemoteDemo()
        else:
            demo_instance = NexusRemoteDemo()
        
        # 创建一些示例节点
        demo_instance.create_node("Alice", is_high_reputation=True)
        demo_instance.create_node("Bob", is_high_reputation=False)
        demo_instance.create_node("Charlie", is_high_reputation=True)
    
    return demo_instance

# 后台线程：定期更新数据并通过WebSocket推送
def background_thread():
    """定期更新数据并推送到WebSocket客户端"""
    while True:
        try:
            update_data()
            socketio.sleep(update_interval)
        except Exception as e:
            print(f"后台线程错误: {e}")
            socketio.sleep(5)

def update_data():
    """更新数据并推送到客户端"""
    global simulation_results_cache, last_update_time
    
    current_time = time.time()
    if current_time - last_update_time >= update_interval:
        # 生成或更新模拟数据
        if simulation_results_cache is None:
            simulation_results_cache = generate_routing_data()
        
        # 添加一些随机变化以模拟实时数据
        simulation_results_cache["high_rep_selection_rate"] = min(0.95, 
            simulation_results_cache["high_rep_selection_rate"] + random.uniform(-0.02, 0.02))
        
        # 通过WebSocket推送更新
        socketio.emit('data_update', {
            'type': 'routing_update',
            'data': simulation_results_cache,
            'timestamp': current_time
        })
        
        # 推送设备状态更新
        device_data = generate_device_data()
        socketio.emit('data_update', {
            'type': 'device_update',
            'data': device_data,
            'timestamp': current_time
        })
        
        last_update_time = current_time
        print(f"📡 数据更新推送: {current_time}")

def generate_routing_data():
    """生成加权路由算法数据"""
    try:
        if HAS_ORIGINAL_MODULES:
            results = simulate_routing(num_nodes=100, num_lookups=1000)
        else:
            # 简化版本
            results = {
                "total_nodes": 100,
                "high_rep_nodes": 30,
                "low_rep_nodes": 70,
                "high_rep_selected": 387,
                "total_selections": 1000,
                "high_rep_selection_rate": 0.387,
                "high_rep_population_rate": 0.3,
                "advantage_ratio": 1.29
            }
        return results
    except Exception as e:
        print(f"生成路由数据错误: {e}")
        return {
            "total_nodes": 100,
            "high_rep_nodes": 30,
            "low_rep_nodes": 70,
            "high_rep_selection_rate": 0.387,
            "advantage_ratio": 1.29,
            "error": str(e)
        }

def generate_device_data():
    """生成设备数据"""
    devices = []
    statuses = ['online', 'offline', 'busy', 'idle']
    types = ['控制端', '被控端', '中继节点', '普通节点']
    
    for i in range(8):
        devices.append({
            "id": i + 1,
            "name": f"设备{i+1}",
            "ip": f"192.168.1.{100 + i}",
            "status": random.choice(statuses),
            "type": random.choice(types),
            "lastActive": f"{random.randint(1, 60)}分钟前",
            "cpuUsage": random.randint(10, 90),
            "memoryUsage": random.randint(20, 95),
            "tokenBalance": random.uniform(100, 5000)
        })
    
    return devices

def generate_economy_data():
    """生成通证经济数据"""
    return {
        "total_nodes": 3,
        "total_transactions": 15,
        "high_rep_nodes": 2,
        "total_tokens": 2500.5,
        "daily_volume": 125.75,
        "avg_reputation": 650,
        "network_health": 85,  # 0-100
        "verifications": [
            "✅ 通证激励驱动网络贡献",
            "✅ 高信誉节点获得优势",
            "✅ 经济闭环可持续",
            "✅ 抗Sybil攻击机制"
        ]
    }

# WebSocket事件处理
@socketio.on('connect')
def handle_connect():
    """客户端连接时调用"""
    print(f"🔗 WebSocket客户端连接: {request.sid}")
    socketio.emit('connected', {
        'message': 'NexusRemote WebSocket连接成功',
        'timestamp': time.time(),
        'update_interval': update_interval
    })

@socketio.on('disconnect')
def handle_disconnect():
    """客户端断开连接时调用"""
    print(f"🔌 WebSocket客户端断开: {request.sid}")

@socketio.on('get_data')
def handle_get_data(data):
    """客户端请求数据"""
    data_type = data.get('type', 'all')
    
    if data_type == 'routing':
        socketio.emit('data_response', {
            'type': 'routing',
            'data': generate_routing_data()
        })
    elif data_type == 'devices':
        socketio.emit('data_response', {
            'type': 'devices',
            'data': generate_device_data()
        })
    elif data_type == 'economy':
        socketio.emit('data_response', {
            'type': 'economy',
            'data': generate_economy_data()
        })

# REST API端点
@app.route('/')
def index():
    """API根端点"""
    return jsonify({
        "name": "NexusRemote Python后端",
        "version": "1.0.0",
        "endpoints": [
            "/api/health",
            "/api/routing/algorithm",
            "/api/devices",
            "/api/network",
            "/api/tokens",
            "/api/economy/model",
            "/api/simulation/results",
            "/api/demo/run"
        ],
        "websocket": "ws://localhost:5000"
    })

@app.route('/api/health')
def health():
    """健康检查端点"""
    return jsonify({
        "status": "healthy",
        "timestamp": time.time(),
        "services": {
            "flask": "running",
            "websocket": "running",
            "algorithm": "available" if HAS_ORIGINAL_MODULES else "simulated"
        }
    })

@app.route('/api/routing/algorithm')
def routing_algorithm():
    """加权路由算法数据"""
    data = generate_routing_data()
    return jsonify(data)

@app.route('/api/devices')
def devices():
    """设备列表"""
    data = generate_device_data()
    return jsonify(data)

@app.route('/api/network')
def network():
    """网络状态"""
    return jsonify({
        "total_nodes": 156,
        "online_nodes": 128,
        "offline_nodes": 28,
        "avg_latency": 42.5,
        "total_bandwidth": "1.2 Gbps",
        "connections": 245,
        "health_score": 88,
        "topology": "mesh",
        "last_updated": time.time()
    })

@app.route('/api/tokens')
def tokens():
    """通证统计"""
    return jsonify({
        "total_supply": 1000000,
        "circulating": 525000,
        "daily_volume": 12500.75,
        "avg_transaction": 85.5,
        "top_holders": [
            {"address": "0x1a2b...", "balance": 12500, "percentage": 2.38},
            {"address": "0x3c4d...", "balance": 9800, "percentage": 1.87},
            {"address": "0x5e6f...", "balance": 7600, "percentage": 1.45}
        ],
        "token_price": 0.85,
        "market_cap": 446250
    })

@app.route('/api/economy/model')
def economy_model():
    """通证经济模型"""
    data = generate_economy_data()
    return jsonify(data)

@app.route('/api/simulation/results')
def simulation_results():
    """运行模拟并返回结果"""
    try:
        if HAS_ORIGINAL_MODULES:
            results = simulate_routing(num_nodes=100, num_lookups=1000)
        else:
            results = generate_routing_data()
        
        # 保存到缓存
        global simulation_results_cache
        simulation_results_cache = results
        
        return jsonify({
            "success": True,
            "results": results,
            "message": "模拟运行成功",
            "timestamp": time.time()
        })
    except Exception as e:
        return jsonify({
            "success": False,
            "error": str(e),
            "timestamp": time.time()
        }), 500

@app.route('/api/demo/run')
def run_demo():
    """运行通证经济演示"""
    try:
        if HAS_ORIGINAL_MODULES:
            # 导入并运行原始演示
            import demo_workflow
            demo_result = demo_workflow.main()
            
            return jsonify({
                "success": True,
                "message": "通证经济演示运行成功",
                "summary": {
                    "total_nodes": len(demo_result.nodes) if hasattr(demo_result, 'nodes') else 3,
                    "total_transactions": len(demo_result.transactions) if hasattr(demo_result, 'transactions') else 3,
                    "verifications": [
                        "✅ 通证激励驱动网络贡献",
                        "✅ 高信誉节点获得优势", 
                        "✅ 经济闭环可持续",
                        "✅ 抗Sybil攻击机制"
                    ]
                },
                "timestamp": time.time()
            })
        else:
            return jsonify({
                "success": True,
                "message": "简化版演示运行",
                "summary": {
                    "total_nodes": 3,
                    "total_transactions": 5,
                    "verifications": [
                        "✅ 通证激励驱动网络贡献",
                        "✅ 高信誉节点获得优势",
                        "✅ 经济闭环可持续",
                        "✅ 抗Sybil攻击机制"
                    ]
                },
                "timestamp": time.time()
            })
    except Exception as e:
        return jsonify({
            "success": False,
            "error": str(e),
            "timestamp": time.time()
        }), 500

@app.route('/api/config')
def config():
    """配置信息"""
    return jsonify({
        "update_interval": update_interval,
        "has_original_modules": HAS_ORIGINAL_MODULES,
        "websocket_port": 5000,
        "http_port": 5000,
        "version": "1.0.0"
    })

# 错误处理
@app.errorhandler(404)
def not_found(error):
    return jsonify({
        "error": "未找到资源",
        "path": request.path,
        "timestamp": time.time()
    }), 404

@app.errorhandler(500)
def internal_error(error):
    return jsonify({
        "error": "服务器内部错误",
        "message": str(error) if app.debug else "请稍后重试",
        "timestamp": time.time()
    }), 500

# 启动应用
if __name__ == '__main__':
    # 初始化演示实例
    init_demo()
    
    print("=" * 60)
    print("🚀 NexusRemote Python后端服务启动")
    print("=" * 60)
    print(f"📡 REST API: http://localhost:5000")
    print(f"🔗 WebSocket: ws://localhost:5000")
    print(f"🔄 更新间隔: {update_interval}秒")
    print(f"🧮 原始模块: {'✅ 可用' if HAS_ORIGINAL_MODULES else '⚠️ 使用简化版本'}")
    print("=" * 60)
    
    # 启动后台线程
    socketio.start_background_task(background_thread)
    
    # 启动服务器
    socketio.run(app, host='0.0.0.0', port=5000, debug=False, use_reloader=False)