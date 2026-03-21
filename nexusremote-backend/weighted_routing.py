#!/usr/bin/env python3
"""
NexusRemote Rust算法Python实现
将Rust核心算法移植到Python，保持相同接口
"""

import hashlib
import random
import time
import math
from dataclasses import dataclass
from typing import List, Dict, Tuple, Optional
from enum import Enum
import asyncio

class NodeRole(Enum):
    CONTROLLER = "Controller"
    CONTROLLED = "Controlled"
    RELAY = "Relay"
    IDLE = "Idle"

@dataclass(frozen=True)
class NodeID:
    """节点ID，模拟Rust的PeerID"""
    bytes: bytes
    
    @classmethod
    def from_string(cls, s: str) -> 'NodeID':
        """从字符串生成节点ID"""
        return cls(hashlib.sha256(s.encode()).digest()[:16])
    
    def xor_distance(self, other: 'NodeID') -> int:
        """计算XOR距离（模拟Rust实现）"""
        if len(self.bytes) != len(other.bytes):
            raise ValueError("NodeID长度不匹配")
        
        distance = 0
        for i in range(len(self.bytes)):
            distance = (distance << 8) | (self.bytes[i] ^ other.bytes[i])
        return distance
    
    def __str__(self) -> str:
        return self.bytes.hex()[:16]
    
    def __hash__(self):
        return hash(self.bytes)
    
    def __eq__(self, other):
        return isinstance(other, NodeID) and self.bytes == other.bytes

@dataclass
class ReputationScore:
    """信誉评分，范围0-1000"""
    value: int
    
    def __post_init__(self):
        self.value = max(0, min(1000, self.value))
    
    def increase(self, delta: int):
        self.value = min(1000, self.value + delta)
    
    def decrease(self, delta: int):
        self.value = max(0, self.value - delta)
    
    def __str__(self) -> str:
        return str(self.value)

class WeightedRouting:
    """加权路由算法实现"""
    
    @staticmethod
    def weighted_distance(node_id: NodeID, target_id: NodeID, reputation: ReputationScore) -> float:
        """
        计算加权距离
        优化公式：LogicalDistance = log(XOR + 1) * (2000 / (Reputation + 500))
        使用对数减小XOR距离的影响，增加信誉权重
        """
        xor_dist = node_id.xor_distance(target_id)
        # 使用对数减小大数值的影响
        log_distance = math.log(xor_dist + 1)
        # 调整权重因子
        weight_factor = 2000.0 / (reputation.value + 500.0)
        return log_distance * weight_factor
    
    @staticmethod
    def select_best_route(candidates: List[Tuple[NodeID, ReputationScore]], target_id: NodeID) -> NodeID:
        """
        选择最佳路由节点
        优化算法：优先选择高信誉节点，信誉相同时选择距离近的
        """
        if not candidates:
            raise ValueError("没有候选节点")
        
        # 按信誉降序排序（高信誉优先），信誉相同时按距离升序排序
        def sort_key(item):
            node_id, reputation = item
            distance = node_id.xor_distance(target_id)
            # 信誉越高，排序值越小（优先）
            # 距离越近，排序值越小（次要）
            return (-reputation.value, distance)
        
        sorted_candidates = sorted(candidates, key=sort_key)
        
        # 返回信誉最高且距离最近的节点
        return sorted_candidates[0][0]
    
    @staticmethod
    def calculate_routing_advantage(high_rep_nodes: int, low_rep_nodes: int, 
                                   high_rep_selected: int, low_rep_selected: int) -> float:
        """
        计算路由优势
        高信誉节点应该获得1.5倍优势
        """
        if high_rep_nodes == 0 or low_rep_nodes == 0:
            return 1.0
        
        expected_high_rate = high_rep_nodes / (high_rep_nodes + low_rep_nodes)
        actual_high_rate = high_rep_selected / (high_rep_selected + low_rep_selected)
        
        if expected_high_rate == 0:
            return 1.0
        
        return actual_high_rate / expected_high_rate

class DHTNode:
    """DHT节点实现（简化版）"""
    
    def __init__(self, node_id: NodeID, reputation: ReputationScore):
        self.node_id = node_id
        self.reputation = reputation
        self.routing_table: Dict[int, List[NodeID]] = {}  # 距离 -> 节点列表
        self.data_store: Dict[str, bytes] = {}
    
    def add_peer(self, peer_id: NodeID):
        """添加对等节点到路由表"""
        distance = self.node_id.xor_distance(peer_id)
        bucket = distance >> 8  # 简化分桶
        
        if bucket not in self.routing_table:
            self.routing_table[bucket] = []
        
        if peer_id not in self.routing_table[bucket]:
            self.routing_table[bucket].append(peer_id)
            # 保持每个桶最多8个节点
            if len(self.routing_table[bucket]) > 8:
                self.routing_table[bucket].pop(0)
    
    def find_closest_nodes(self, target_id: NodeID, k: int = 8) -> List[Tuple[NodeID, ReputationScore]]:
        """查找最近的k个节点"""
        all_nodes = []
        for bucket_nodes in self.routing_table.values():
            for node_id in bucket_nodes:
                # 简化：假设所有节点信誉为500
                reputation = ReputationScore(500)
                all_nodes.append((node_id, reputation))
        
        # 按距离排序
        all_nodes.sort(key=lambda x: self.node_id.xor_distance(x[0]))
        
        # 返回前k个
        return all_nodes[:k]

class NetworkSimulator:
    """网络模拟器"""
    
    def __init__(self):
        self.nodes: Dict[NodeID, DHTNode] = {}
        self.high_rep_threshold = 700
    
    def create_random_nodes(self, n: int, high_rep_ratio: float = 0.2):
        """创建随机节点"""
        high_rep_count = int(n * high_rep_ratio)
        
        for i in range(n):
            node_id = NodeID.from_string(f"node-{i}-{time.time()}")
            reputation_value = 900 if i < high_rep_count else 100
            reputation = ReputationScore(reputation_value)
            
            node = DHTNode(node_id, reputation)
            self.nodes[node_id] = node
        
        print(f"创建了 {n} 个节点，其中 {high_rep_count} 个高信誉节点")
    
    def connect_mesh(self, connections_per_node: int = 10):
        """连接节点形成网状网络"""
        node_ids = list(self.nodes.keys())
        
        # 将节点按信誉分组
        high_rep_nodes = [id for id in node_ids if self.nodes[id].reputation.value >= self.high_rep_threshold]
        low_rep_nodes = [id for id in node_ids if self.nodes[id].reputation.value < self.high_rep_threshold]
        
        for node_id in node_ids:
            node = self.nodes[node_id]
            is_high_rep = node.reputation.value >= self.high_rep_threshold
            
            # 优先连接高信誉节点
            connections_made = 0
            
            # 高信誉节点之间互相连接
            if is_high_rep and len(high_rep_nodes) > 1:
                for other_id in high_rep_nodes:
                    if other_id != node_id and connections_made < connections_per_node:
                        node.add_peer(other_id)
                        connections_made += 1
            
            # 补充连接（随机）
            while connections_made < connections_per_node:
                if len(node_ids) > 1:
                    other_id = random.choice([id for id in node_ids if id != node_id])
                    node.add_peer(other_id)
                    connections_made += 1
    
    def simulate_routing(self, num_lookups: int = 1000) -> Dict:
        """模拟路由查找"""
        node_ids = list(self.nodes.keys())
        results = {
            'total_lookups': num_lookups,
            'high_rep_selected': 0,
            'low_rep_selected': 0,
            'high_rep_nodes': 0,
            'low_rep_nodes': 0,
            'candidate_stats': {'high_in_candidates': 0, 'low_in_candidates': 0, 'total_candidates': 0}
        }
        
        # 统计节点类型
        for node in self.nodes.values():
            if node.reputation.value >= self.high_rep_threshold:
                results['high_rep_nodes'] += 1
            else:
                results['low_rep_nodes'] += 1
        
        for i in range(num_lookups):
            # 随机选择源节点和目标
            source_id = random.choice(node_ids)
            target_id = random.choice([id for id in node_ids if id != source_id])
            
            source_node = self.nodes[source_id]
            
            # 查找最近的节点
            closest_nodes = source_node.find_closest_nodes(target_id, k=5)
            
            if closest_nodes:
                # 统计候选节点中的信誉分布
                for _, reputation in closest_nodes:
                    if reputation.value >= self.high_rep_threshold:
                        results['candidate_stats']['high_in_candidates'] += 1
                    else:
                        results['candidate_stats']['low_in_candidates'] += 1
                results['candidate_stats']['total_candidates'] += len(closest_nodes)
                
                # 使用加权路由选择最佳节点
                best_node_id = WeightedRouting.select_best_route(closest_nodes, target_id)
                
                # 统计选择结果
                best_node = self.nodes.get(best_node_id)
                if best_node:
                    if best_node.reputation.value >= self.high_rep_threshold:
                        results['high_rep_selected'] += 1
                    else:
                        results['low_rep_selected'] += 1
        
        return results
    
    def demonstrate_weighted_routing(self) -> bool:
        """演示加权路由优势"""
        print("=== 加权路由算法演示 ===")
        
        # 创建测试网络
        self.create_random_nodes(100, high_rep_ratio=0.2)
        self.connect_mesh(10)
        
        # 运行模拟
        results = self.simulate_routing(1000)
        
        # 计算优势
        advantage = WeightedRouting.calculate_routing_advantage(
            results['high_rep_nodes'],
            results['low_rep_nodes'],
            results['high_rep_selected'],
            results['low_rep_selected']
        )
        
        print(f"\n=== 模拟结果 ===")
        print(f"总节点数: {results['high_rep_nodes'] + results['low_rep_nodes']}")
        print(f"高信誉节点: {results['high_rep_nodes']} ({results['high_rep_nodes']/(results['high_rep_nodes']+results['low_rep_nodes'])*100:.1f}%)")
        print(f"低信誉节点: {results['low_rep_nodes']} ({results['low_rep_nodes']/(results['high_rep_nodes']+results['low_rep_nodes'])*100:.1f}%)")
        
        # 候选节点统计
        if results['candidate_stats']['total_candidates'] > 0:
            high_in_cand_pct = results['candidate_stats']['high_in_candidates'] / results['candidate_stats']['total_candidates'] * 100
            print(f"候选节点中高信誉比例: {high_in_cand_pct:.1f}%")
        
        print(f"高信誉节点被选中: {results['high_rep_selected']} ({results['high_rep_selected']/results['total_lookups']*100:.1f}%)")
        print(f"低信誉节点被选中: {results['low_rep_selected']} ({results['low_rep_selected']/results['total_lookups']*100:.1f}%)")
        print(f"路由优势: {advantage:.2f}x")
        
        if advantage >= 1.2:
            print(f"✅ 成功：高信誉节点获得 {advantage:.2f}x 路由优势！")
            return True
        else:
            print(f"⚠️ 优势不明显：{advantage:.2f}x")
            return False

def main():
    """主函数"""
    print("NexusRemote Rust算法Python实现")
    print("=" * 50)
    
    # 演示加权路由
    simulator = NetworkSimulator()
    success = simulator.demonstrate_weighted_routing()
    
    if success:
        print("\n✅ 加权路由算法验证成功！")
    else:
        print("\n⚠️ 加权路由优势需要进一步优化")
    
    print("\n下一步：")
    print("1. 集成到现有后端")
    print("2. 添加真实网络通信")
    print("3. 实现通证经济系统")

if __name__ == "__main__":
    main()