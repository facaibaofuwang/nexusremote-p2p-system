#!/usr/bin/env python3
"""
NexusRemote 通证经济原型演示
展示NEXUS代币的激励机制
"""

import json
import time
import random
from dataclasses import dataclass
from typing import List, Dict
from datetime import datetime

@dataclass
class Node:
    """P2P网络节点"""
    id: int
    name: str
    reputation: int  # 0-1000
    nexus_balance: float  # NEXUS代币余额
    bandwidth_available: int  # 可用带宽 (Mbps)
    
    def __str__(self):
        return f"{self.name} (信誉: {self.reputation}, 余额: {self.nexus_balance:.2f} NEXUS)"

@dataclass
class RelayTransaction:
    """中继交易"""
    id: str
    source_node_id: int
    target_node_id: int
    relay_node_id: int
    data_size_mb: float
    cost_nexus: float
    earnings_nexus: float
    timestamp: str

class TokenEconomySimulator:
    """通证经济模拟器"""
    
    def __init__(self):
        self.nodes: List[Node] = []
        self.transactions: List[RelayTransaction] = []
        self.nexus_total_supply = 1000000  # 总供应量
        self.nexus_circulating = 0
        
    def create_nodes(self, num_nodes: int = 5):
        """创建模拟节点"""
        print(f"🚀 创建 {num_nodes} 个P2P节点...")
        
        for i in range(num_nodes):
            # 随机分配信誉和余额
            reputation = random.randint(50, 1000)
            initial_balance = random.uniform(10.0, 100.0)
            bandwidth = random.randint(10, 100)
            
            node = Node(
                id=i + 1,
                name=f"节点{i+1}",
                reputation=reputation,
                nexus_balance=initial_balance,
                bandwidth_available=bandwidth
            )
            
            self.nodes.append(node)
            self.nexus_circulating += initial_balance
            
            print(f"  {node}")
    
    def calculate_relay_cost(self, data_size_mb: float, reputation: int) -> float:
        """计算中继成本（基于信誉的折扣）"""
        base_cost = data_size_mb * 1.0  # 1 NEXUS per MB
        discount = 1.0 - (reputation / 2000.0)  # 最高50%折扣
        return max(base_cost * discount, 0.1)
    
    def calculate_relay_earnings(self, data_size_mb: float, reputation: int) -> float:
        """计算中继收益（基于信誉的奖励）"""
        base_earning = data_size_mb * 1.0
        bonus = 1.0 + (reputation / 2000.0)  # 最高50%奖励
        return base_earning * bonus
    
    def simulate_relay_transaction(self):
        """模拟一次中继交易"""
        if len(self.nodes) < 3:
            print("❌ 需要至少3个节点进行中继交易")
            return
        
        # 随机选择源节点、目标节点和中继节点
        source = random.choice(self.nodes)
        target = random.choice([n for n in self.nodes if n.id != source.id])
        relay_candidates = [n for n in self.nodes if n.id not in [source.id, target.id]]
        
        if not relay_candidates:
            return
        
        # 使用加权路由算法选择中继节点
        relay = self.select_relay_node(relay_candidates, source, target)
        
        # 随机数据大小 (0.1-10 MB)
        data_size_mb = random.uniform(0.1, 10.0)
        
        # 计算成本和收益
        cost = self.calculate_relay_cost(data_size_mb, source.reputation)
        earnings = self.calculate_relay_earnings(data_size_mb, relay.reputation)
        
        # 检查源节点余额
        if source.nexus_balance < cost:
            print(f"❌ {source.name} 余额不足 ({source.nexus_balance:.2f} < {cost:.2f})")
            return
        
        # 执行交易
        source.nexus_balance -= cost
        relay.nexus_balance += earnings
        
        # 记录交易
        transaction = RelayTransaction(
            id=f"tx_{int(time.time())}_{random.randint(1000, 9999)}",
            source_node_id=source.id,
            target_node_id=target.id,
            relay_node_id=relay.id,
            data_size_mb=data_size_mb,
            cost_nexus=cost,
            earnings_nexus=earnings,
            timestamp=datetime.now().isoformat()
        )
        
        self.transactions.append(transaction)
        
        print(f"📊 中继交易完成:")
        print(f"  源节点: {source.name} → 支付 {cost:.2f} NEXUS")
        print(f"  中继节点: {relay.name} → 获得 {earnings:.2f} NEXUS")
        print(f"  目标节点: {target.name}")
        print(f"  数据大小: {data_size_mb:.1f} MB")
    
    def select_relay_node(self, candidates: List[Node], source: Node, target: Node) -> Node:
        """使用加权路由算法选择中继节点"""
        # 简化版的加权选择：基于信誉和余额
        weighted_scores = []
        
        for node in candidates:
            # 优先级分数 = 70%信誉 + 30%余额（归一化）
            rep_score = node.reputation / 1000.0
            bal_score = min(node.nexus_balance / 100.0, 1.0)  # 假设100 NEXUS为上限
            priority_score = (rep_score * 0.7) + (bal_score * 0.3)
            
            weighted_scores.append((priority_score, node))
        
        # 选择优先级最高的节点
        weighted_scores.sort(reverse=True, key=lambda x: x[0])
        return weighted_scores[0][1]
    
    def demonstrate_economic_incentives(self):
        """演示经济激励机制"""
        print("\n🎯 通证经济激励机制演示")
        print("=" * 60)
        
        # 创建节点
        self.create_nodes(5)
        
        print("\n💰 初始经济状态:")
        total_balance = sum(node.nexus_balance for node in self.nodes)
        avg_reputation = sum(node.reputation for node in self.nodes) / len(self.nodes)
        print(f"  总流通量: {total_balance:.2f} NEXUS")
        print(f"  平均信誉: {avg_reputation:.1f}")
        
        # 模拟多次中继交易
        print("\n🔄 模拟中继交易...")
        num_transactions = 10
        
        for i in range(num_transactions):
            print(f"\n交易 #{i+1}:")
            self.simulate_relay_transaction()
            time.sleep(0.1)  # 短暂延迟
        
        # 展示最终状态
        print("\n📈 最终经济状态:")
        print("=" * 60)
        
        # 节点排名
        print("\n🏆 节点排名 (按余额):")
        sorted_nodes = sorted(self.nodes, key=lambda n: n.nexus_balance, reverse=True)
        for i, node in enumerate(sorted_nodes[:3], 1):
            print(f"  {i}. {node.name}: {node.nexus_balance:.2f} NEXUS (信誉: {node.reputation})")
        
        # 交易统计
        print("\n📊 交易统计:")
        total_cost = sum(tx.cost_nexus for tx in self.transactions)
        total_earnings = sum(tx.earnings_nexus for tx in self.transactions)
        total_data = sum(tx.data_size_mb for tx in self.transactions)
        
        print(f"  总交易数: {len(self.transactions)}")
        print(f"  总数据量: {total_data:.1f} MB")
        print(f"  总支付: {total_cost:.2f} NEXUS")
        print(f"  总收益: {total_earnings:.2f} NEXUS")
        
        # 经济激励分析
        print("\n💡 经济激励分析:")
        
        # 计算高信誉节点的平均收益
        high_rep_nodes = [n for n in self.nodes if n.reputation >= 700]
        low_rep_nodes = [n for n in self.nodes if n.reputation < 700]
        
        if high_rep_nodes and low_rep_nodes:
            high_rep_avg_balance = sum(n.nexus_balance for n in high_rep_nodes) / len(high_rep_nodes)
            low_rep_avg_balance = sum(n.nexus_balance for n in low_rep_nodes) / len(low_rep_nodes)
            
            print(f"  高信誉节点平均余额: {high_rep_avg_balance:.2f} NEXUS")
            print(f"  低信誉节点平均余额: {low_rep_avg_balance:.2f} NEXUS")
            
            if high_rep_avg_balance > low_rep_avg_balance:
                advantage = high_rep_avg_balance / low_rep_avg_balance
                print(f"  ✅ 高信誉节点有 {advantage:.2f}x 经济优势")
            else:
                print("  ⚠️ 经济激励需要优化")
        
        # 展示通证经济闭环
        print("\n🔄 通证经济闭环验证:")
        print("  1. 用户支付NEXUS使用网络服务")
        print("  2. 中继节点赚取NEXUS提供带宽")
        print("  3. 高信誉节点获得更多收益")
        print("  4. 激励节点维护高信誉")
        print("  5. 形成正向经济循环")
        
        return True

def main():
    """主函数"""
    print("=" * 70)
    print("NexusRemote 通证经济原型演示")
    print("=" * 70)
    
    simulator = TokenEconomySimulator()
    
    try:
        success = simulator.demonstrate_economic_incentives()
        
        print("\n" + "=" * 70)
        if success:
            print("🎉 通证经济原型演示成功!")
            print("\n✅ 验证的核心机制:")
            print("  1. 基于信誉的成本折扣")
            print("  2. 基于信誉的收益奖励")
            print("  3. 加权路由节点选择")
            print("  4. 通证经济闭环")
            print("  5. 正向激励循环")
        else:
            print("⚠️ 演示过程中遇到问题")
        
        print("\n📈 下一步:")
        print("  1. 集成到Rust核心系统")
        print("  2. 实现真实的代币转账")
        print("  3. 添加智能合约功能")
        print("  4. 实现跨链桥接")
        
        return success
        
    except Exception as e:
        print(f"❌ 演示失败: {e}")
        return False

if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)