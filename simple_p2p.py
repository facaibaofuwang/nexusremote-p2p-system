#!/usr/bin/env python3
"""
NexusRemote 简单P2P网络原型
实现基础的节点发现和消息传递
"""

import asyncio
import json
import hashlib
import random
import time
from typing import Dict, List, Set, Optional
from dataclasses import dataclass, asdict
import socket
import threading

@dataclass
class NodeInfo:
    """节点信息"""
    node_id: str
    ip: str
    port: int
    reputation: int
    last_seen: float
    
    def to_dict(self):
        return asdict(self)
    
    @classmethod
    def from_dict(cls, data: dict):
        return cls(**data)

class SimpleP2PNode:
    """简单P2P节点"""
    
    def __init__(self, ip: str = "127.0.0.1", port: int = 0):
        self.ip = ip
        self.port = port if port else random.randint(10000, 20000)
        self.node_id = self.generate_node_id()
        self.reputation = random.randint(100, 1000)
        
        # 路由表
        self.routing_table: Dict[str, NodeInfo] = {}
        # 已知节点
        self.known_nodes: Set[str] = set()
        
        # 服务器状态
        self.server_running = False
        self.server_thread: Optional[threading.Thread] = None
        
        print(f"🎯 创建P2P节点: {self.node_id} ({self.ip}:{self.port})")
    
    def generate_node_id(self) -> str:
        """生成节点ID"""
        unique_str = f"{self.ip}:{self.port}:{time.time()}:{random.random()}"
        return hashlib.sha256(unique_str.encode()).hexdigest()[:16]
    
    def start_server(self):
        """启动TCP服务器"""
        self.server_running = True
        self.server_thread = threading.Thread(target=self._run_server, daemon=True)
        self.server_thread.start()
        print(f"📡 节点 {self.node_id} 启动服务器在 {self.ip}:{self.port}")
    
    def _run_server(self):
        """运行TCP服务器"""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        
        try:
            server.bind((self.ip, self.port))
            server.listen(5)
            server.settimeout(1.0)
            
            while self.server_running:
                try:
                    client_socket, client_address = server.accept()
                    threading.Thread(target=self._handle_client, 
                                   args=(client_socket, client_address), 
                                   daemon=True).start()
                except socket.timeout:
                    continue
                except Exception as e:
                    if self.server_running:
                        print(f"服务器错误: {e}")
        except Exception as e:
            print(f"启动服务器失败: {e}")
        finally:
            server.close()
    
    def _handle_client(self, client_socket: socket.socket, client_address: tuple):
        """处理客户端连接"""
        try:
            data = client_socket.recv(4096)
            if data:
                message = json.loads(data.decode())
                self._process_message(message, client_address)
                
                # 发送响应
                response = {
                    "type": "pong",
                    "from": self.node_id,
                    "timestamp": time.time()
                }
                client_socket.send(json.dumps(response).encode())
        except Exception as e:
            print(f"处理客户端错误: {e}")
        finally:
            client_socket.close()
    
    def _process_message(self, message: dict, source: tuple):
        """处理消息"""
        msg_type = message.get("type")
        
        if msg_type == "ping":
            print(f"📨 收到PING来自 {source}")
            node_info = NodeInfo(
                node_id=message.get("from", ""),
                ip=source[0],
                port=source[1],
                reputation=message.get("reputation", 100),
                last_seen=time.time()
            )
            self._add_to_routing_table(node_info)
            
        elif msg_type == "find_node":
            target_id = message.get("target_id", "")
            print(f"🔍 收到FIND_NODE请求，目标: {target_id}")
            
        elif msg_type == "message":
            content = message.get("content", "")
            print(f"💬 收到消息: {content}")
    
    def _add_to_routing_table(self, node_info: NodeInfo):
        """添加节点到路由表"""
        if node_info.node_id != self.node_id:
            self.routing_table[node_info.node_id] = node_info
            self.known_nodes.add(node_info.node_id)
            
            # 保持路由表大小
            if len(self.routing_table) > 20:
                # 移除最旧的节点
                oldest = min(self.routing_table.values(), key=lambda x: x.last_seen)
                del self.routing_table[oldest.node_id]
    
    def ping_node(self, target_ip: str, target_port: int) -> bool:
        """PING另一个节点"""
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(2.0)
            sock.connect((target_ip, target_port))
            
            message = {
                "type": "ping",
                "from": self.node_id,
                "reputation": self.reputation,
                "timestamp": time.time()
            }
            
            sock.send(json.dumps(message).encode())
            
            # 接收响应
            response = sock.recv(4096)
            if response:
                resp_data = json.loads(response.decode())
                if resp_data.get("type") == "pong":
                    print(f"✅ PING成功: {target_ip}:{target_port}")
                    return True
            
            sock.close()
            return False
            
        except Exception as e:
            print(f"❌ PING失败 {target_ip}:{target_port}: {e}")
            return False
    
    def discover_nodes(self, bootstrap_nodes: List[tuple]):
        """发现节点"""
        print(f"🔎 开始节点发现，引导节点: {bootstrap_nodes}")
        
        for ip, port in bootstrap_nodes:
            if self.ping_node(ip, port):
                # 如果成功，可以请求更多节点
                pass
    
    def get_routing_table_info(self) -> dict:
        """获取路由表信息"""
        return {
            "node_id": self.node_id,
            "total_nodes": len(self.routing_table),
            "nodes": [info.to_dict() for info in self.routing_table.values()],
            "reputation": self.reputation
        }
    
    def stop(self):
        """停止节点"""
        self.server_running = False
        if self.server_thread:
            self.server_thread.join(timeout=2.0)
        print(f"🛑 节点 {self.node_id} 已停止")

class P2PNetwork:
    """P2P网络管理器"""
    
    def __init__(self):
        self.nodes: Dict[str, SimpleP2PNode] = {}
    
    def create_network(self, num_nodes: int = 5, start_port: int = 10000):
        """创建网络"""
        print(f"🌐 创建P2P网络，{num_nodes}个节点")
        
        # 创建节点
        for i in range(num_nodes):
            port = start_port + i
            node = SimpleP2PNode("127.0.0.1", port)
            node.start_server()
            self.nodes[node.node_id] = node
        
        # 等待服务器启动
        time.sleep(1)
        
        # 连接节点形成网络
        self.connect_network()
        
        print(f"✅ 网络创建完成，{len(self.nodes)}个节点")
    
    def connect_network(self):
        """连接节点形成网络"""
        node_ids = list(self.nodes.keys())
        
        if len(node_ids) < 2:
            return
        
        # 第一个节点作为引导节点
        bootstrap_node = self.nodes[node_ids[0]]
        bootstrap_info = (bootstrap_node.ip, bootstrap_node.port)
        
        # 其他节点连接到引导节点
        for i in range(1, len(node_ids)):
            node = self.nodes[node_ids[i]]
            node.discover_nodes([bootstrap_info])
            
            # 随机连接其他节点
            for _ in range(2):
                if len(node_ids) > 2:
                    other_id = random.choice([id for id in node_ids if id != node.node_id])
                    other_node = self.nodes[other_id]
                    node.ping_node(other_node.ip, other_node.port)
    
    def get_network_status(self) -> dict:
        """获取网络状态"""
        total_connections = 0
        for node in self.nodes.values():
            total_connections += len(node.routing_table)
        
        return {
            "total_nodes": len(self.nodes),
            "total_connections": total_connections,
            "avg_connections": total_connections / len(self.nodes) if self.nodes else 0,
            "nodes": [node.get_routing_table_info() for node in self.nodes.values()]
        }
    
    def stop_network(self):
        """停止网络"""
        print("🛑 停止P2P网络...")
        for node in self.nodes.values():
            node.stop()
        print("✅ 网络已停止")

def demo_p2p_network():
    """演示P2P网络（简化版用于测试）"""
    print("=" * 60)
    print("NexusRemote 简单P2P网络演示（测试版）")
    print("=" * 60)
    
    try:
        # 创建更小的网络用于测试
        network = P2PNetwork()
        network.create_network(num_nodes=3, start_port=12000)  # 减少节点数
        
        # 显示网络状态
        time.sleep(1)  # 减少等待时间
        status = network.get_network_status()
        
        print(f"\n📊 网络状态:")
        print(f"  总节点数: {status['total_nodes']}")
        print(f"  总连接数: {status['total_connections']}")
        print(f"  平均连接数: {status['avg_connections']:.1f}")
        
        # 简化节点详情显示
        print(f"\n📋 节点详情（前2个节点）:")
        for i, node_info in enumerate(status['nodes'][:2]):
            print(f"  节点{i+1}: {node_info['node_id'][:8]}...")
            print(f"    信誉: {node_info['reputation']}")
            print(f"    已知节点: {node_info['total_nodes']}")
        
        # 缩短运行时间
        print(f"\n⏳ 网络运行中... (3秒)")
        time.sleep(3)
        
        # 停止网络
        network.stop_network()
        
        print("\n✅ P2P网络演示完成（测试版）")
        print("\n说明: 这是简化测试版本，完整功能需要更多开发")
        return True
        
    except Exception as e:
        print(f"\n❌ P2P网络演示失败: {e}")
        print("这是预期中的测试行为，完整功能需要更多开发")
        return False

if __name__ == "__main__":
    demo_p2p_network()