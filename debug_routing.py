#!/usr/bin/env python3
"""
调试加权路由算法
"""

import hashlib
import random
from dataclasses import dataclass
from typing import List, Tuple

@dataclass(frozen=True)
class NodeID:
    bytes: bytes
    
    @classmethod
    def from_string(cls, s: str) -> 'NodeID':
        return cls(hashlib.sha256(s.encode()).digest()[:16])
    
    def xor_distance(self, other: 'NodeID') -> int:
        distance = 0
        for i in range(len(self.bytes)):
            distance = (distance << 8) | (self.bytes[i] ^ other.bytes[i])
        return distance
    
    def __str__(self) -> str:
        return self.bytes.hex()[:8]
    
    def __hash__(self):
        return hash(self.bytes)
    
    def __eq__(self, other):
        return isinstance(other, NodeID) and self.bytes == other.bytes

@dataclass
class ReputationScore:
    value: int
    
    def __post_init__(self):
        self.value = max(0, min(1000, self.value))
    
    def __str__(self) -> str:
        return str(self.value)

def weighted_distance(node_id: NodeID, target_id: NodeID, reputation: ReputationScore) -> float:
    """原始公式：2000/(reputation+1000)"""
    xor_dist = node_id.xor_distance(target_id)
    weight_factor = 2000.0 / (reputation.value + 1000.0)
    return xor_dist * weight_factor

def test_single_case():
    """测试单个案例"""
    print("=== 测试单个路由案例 ===")
    
    # 创建测试节点
    source = NodeID.from_string("source")
    target = NodeID.from_string("target")
    
    # 创建候选节点：2个高信誉，3个低信誉
    candidates = []
    
    # 高信誉节点
    for i in range(2):
        node_id = NodeID.from_string(f"high-rep-{i}")
        reputation = ReputationScore(900)
        candidates.append((node_id, reputation))
        print(f"高信誉节点 {node_id}: 信誉={reputation}")
    
    # 低信誉节点
    for i in range(3):
        node_id = NodeID.from_string(f"low-rep-{i}")
        reputation = ReputationScore(100)
        candidates.append((node_id, reputation))
        print(f"低信誉节点 {node_id}: 信誉={reputation}")
    
    print(f"\n目标节点: {target}")
    
    # 计算距离
    print("\n计算加权距离:")
    best_node = None
    best_distance = float('inf')
    
    for node_id, reputation in candidates:
        distance = weighted_distance(node_id, target, reputation)
        node_type = "高信誉" if reputation.value >= 700 else "低信誉"
        print(f"  {node_type}节点 {node_id}: 距离={distance:.2f}")
        
        if distance < best_distance:
            best_distance = distance
            best_node = (node_id, reputation)
    
    print(f"\n最佳节点: {best_node[0]} (信誉={best_node[1]})")
    print(f"最佳距离: {best_distance:.2f}")
    
    # 检查是否选择了高信誉节点
    if best_node[1].value >= 700:
        print("✅ 选择了高信誉节点！")
    else:
        print("❌ 选择了低信誉节点")

def analyze_formula():
    """分析公式"""
    print("\n=== 分析加权公式 ===")
    
    # 高信誉节点
    high_rep = 900
    weight_high = 2000.0 / (high_rep + 1000.0)
    print(f"高信誉({high_rep}): 权重因子 = 2000/({high_rep}+1000) = {weight_high:.3f}")
    
    # 低信誉节点
    low_rep = 100
    weight_low = 2000.0 / (low_rep + 1000.0)
    print(f"低信誉({low_rep}): 权重因子 = 2000/({low_rep}+1000) = {weight_low:.3f}")
    
    advantage = weight_low / weight_high
    print(f"优势比: {weight_low:.3f} / {weight_high:.3f} = {advantage:.2f}x")
    
    if advantage >= 1.5:
        print(f"✅ 公式提供 {advantage:.2f}x 优势，达到目标！")
    else:
        print(f"⚠️ 公式只提供 {advantage:.2f}x 优势，未达到1.5x目标")

if __name__ == "__main__":
    test_single_case()
    analyze_formula()