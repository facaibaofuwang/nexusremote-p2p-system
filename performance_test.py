#!/usr/bin/env python3
"""
NexusRemote性能测试
分析系统性能瓶颈
"""

import time
import subprocess
import statistics
import json
import os
from datetime import datetime

class PerformanceTester:
    """性能测试器"""
    
    def __init__(self):
        self.results = {}
        self.rust_binary = os.path.join(
            os.path.dirname(__file__),
            "nexusremote/target/release/main"
        )
    
    def test_routing_algorithm_speed(self, num_tests: int = 10):
        """测试路由算法速度"""
        print("🔍 测试加权路由算法速度...")
        
        if not os.path.exists(self.rust_binary):
            print("❌ Rust二进制文件不存在")
            return False
        
        times = []
        
        for i in range(num_tests):
            start_time = time.time()
            
            try:
                result = subprocess.run(
                    [self.rust_binary, "simulate", "--nodes", "50", "--lookups", "100"],
                    capture_output=True,
                    text=True,
                    timeout=30
                )
                
                if result.returncode == 0:
                    elapsed = time.time() - start_time
                    times.append(elapsed)
                    print(f"  测试 {i+1}/{num_tests}: {elapsed:.2f}秒")
                else:
                    print(f"  测试 {i+1} 失败")
                    
            except subprocess.TimeoutExpired:
                print(f"  测试 {i+1} 超时")
            except Exception as e:
                print(f"  测试 {i+1} 异常: {e}")
        
        if times:
            avg_time = statistics.mean(times)
            min_time = min(times)
            max_time = max(times)
            std_dev = statistics.stdev(times) if len(times) > 1 else 0
            
            self.results['routing_speed'] = {
                'average_ms': avg_time * 1000,
                'min_ms': min_time * 1000,
                'max_ms': max_time * 1000,
                'std_dev_ms': std_dev * 1000,
                'tests': len(times),
                'throughput_nodes_per_sec': 50 / avg_time if avg_time > 0 else 0,
                'throughput_lookups_per_sec': 100 / avg_time if avg_time > 0 else 0
            }
            
            print(f"✅ 路由算法速度测试完成:")
            print(f"  平均时间: {avg_time:.2f}秒 ({avg_time*1000:.0f}ms)")
            print(f"  最快: {min_time:.2f}秒, 最慢: {max_time:.2f}秒")
            print(f"  吞吐量: {50/avg_time:.1f} 节点/秒, {100/avg_time:.1f} 查找/秒")
            
            return True
        else:
            print("❌ 所有测试都失败")
            return False
    
    def test_websocket_latency(self, num_messages: int = 20):
        """测试WebSocket延迟"""
        print("\n🔍 测试WebSocket延迟...")
        
        try:
            import websocket
            import threading
            import queue
            
            messages_received = queue.Queue()
            latencies = []
            
            def on_message(ws, message):
                receive_time = time.time()
                try:
                    data = json.loads(message)
                    if 'send_time' in data:
                        send_time = data['send_time']
                        latency = (receive_time - send_time) * 1000  # 转换为毫秒
                        latencies.append(latency)
                        messages_received.put(message)
                except:
                    pass
            
            def on_error(ws, error):
                print(f"  WebSocket错误: {error}")
            
            def on_close(ws, close_status_code, close_msg):
                print("  WebSocket连接关闭")
            
            def on_open(ws):
                print("  WebSocket连接建立，发送测试消息...")
                
                for i in range(num_messages):
                    test_msg = {
                        "type": "performance_test",
                        "message_id": i,
                        "send_time": time.time(),
                        "data": "x" * 100  # 100字节测试数据
                    }
                    ws.send(json.dumps(test_msg))
                    time.sleep(0.05)  # 50ms间隔
                
                # 等待所有消息返回
                time.sleep(1)
                ws.close()
            
            # 连接WebSocket
            ws = websocket.WebSocketApp(
                "ws://127.0.0.1:8081",
                on_open=on_open,
                on_message=on_message,
                on_error=on_error,
                on_close=on_close
            )
            
            # 运行WebSocket客户端
            thread = threading.Thread(target=ws.run_forever)
            thread.daemon = True
            thread.start()
            
            # 等待测试完成
            thread.join(timeout=5)
            
            if latencies:
                avg_latency = statistics.mean(latencies)
                min_latency = min(latencies)
                max_latency = max(latencies)
                
                self.results['websocket_latency'] = {
                    'average_ms': avg_latency,
                    'min_ms': min_latency,
                    'max_ms': max_latency,
                    'messages_sent': num_messages,
                    'messages_received': len(latencies),
                    'success_rate': len(latencies) / num_messages * 100
                }
                
                print(f"✅ WebSocket延迟测试完成:")
                print(f"  平均延迟: {avg_latency:.1f}ms")
                print(f"  最低延迟: {min_latency:.1f}ms, 最高延迟: {max_latency:.1f}ms")
                print(f"  成功率: {len(latencies)}/{num_messages} ({len(latencies)/num_messages*100:.1f}%)")
                
                return True
            else:
                print("❌ 未收到任何响应消息")
                return False
                
        except ImportError:
            print("⚠️ 缺少websocket-client库，跳过WebSocket测试")
            return True  # 不视为失败
        except Exception as e:
            print(f"❌ WebSocket测试失败: {e}")
            return False
    
    def test_memory_usage(self):
        """测试内存使用"""
        print("\n🔍 测试内存使用...")
        
        try:
            import psutil
            import os
            
            # 获取当前进程内存
            process = psutil.Process(os.getpid())
            memory_info = process.memory_info()
            
            # 获取系统内存信息
            system_memory = psutil.virtual_memory()
            
            self.results['memory_usage'] = {
                'process_rss_mb': memory_info.rss / 1024 / 1024,
                'process_vms_mb': memory_info.vms / 1024 / 1024,
                'system_total_mb': system_memory.total / 1024 / 1024,
                'system_available_mb': system_memory.available / 1024 / 1024,
                'system_used_percent': system_memory.percent
            }
            
            print(f"✅ 内存使用测试完成:")
            print(f"  进程RSS: {memory_info.rss/1024/1024:.1f} MB")
            print(f"  进程VMS: {memory_info.vms/1024/1024:.1f} MB")
            print(f"  系统内存: {system_memory.percent}% 已使用")
            
            return True
            
        except ImportError:
            print("⚠️ 缺少psutil库，跳过内存测试")
            return True
        except Exception as e:
            print(f"❌ 内存测试失败: {e}")
            return False
    
    def analyze_bottlenecks(self):
        """分析性能瓶颈"""
        print("\n🔍 分析性能瓶颈...")
        
        bottlenecks = []
        recommendations = []
        
        # 分析路由算法性能
        if 'routing_speed' in self.results:
            speed = self.results['routing_speed']
            
            if speed['average_ms'] > 1000:  # 超过1秒
                bottlenecks.append("路由算法计算时间过长")
                recommendations.append("优化距离计算算法，考虑缓存计算结果")
            
            if speed['std_dev_ms'] > speed['average_ms'] * 0.5:  # 标准差过大
                bottlenecks.append("路由算法性能不稳定")
                recommendations.append("检查随机数生成和网络模拟的稳定性")
            
            throughput = speed['throughput_lookups_per_sec']
            if throughput < 10:  # 每秒查找数过低
                bottlenecks.append("路由查找吞吐量低")
                recommendations.append("优化数据结构，使用更高效的查找算法")
        
        # 分析WebSocket延迟
        if 'websocket_latency' in self.results:
            latency = self.results['websocket_latency']
            
            if latency['average_ms'] > 100:  # 超过100ms
                bottlenecks.append("WebSocket延迟过高")
                recommendations.append("优化网络通信，减少序列化开销")
            
            if latency['success_rate'] < 90:  # 成功率低于90%
                bottlenecks.append("WebSocket通信可靠性不足")
                recommendations.append("增加重试机制和错误处理")
        
        # 分析内存使用
        if 'memory_usage' in self.results:
            memory = self.results['memory_usage']
            
            if memory['process_rss_mb'] > 500:  # 超过500MB
                bottlenecks.append("内存使用过高")
                recommendations.append("优化数据结构，减少内存分配")
        
        # 生成报告
        self.results['bottlenecks'] = bottlenecks
        self.results['recommendations'] = recommendations
        
        print(f"✅ 性能瓶颈分析完成:")
        
        if bottlenecks:
            print(f"  发现 {len(bottlenecks)} 个性能瓶颈:")
            for i, bottleneck in enumerate(bottlenecks, 1):
                print(f"    {i}. {bottleneck}")
            
            print(f"\n  优化建议:")
            for i, recommendation in enumerate(recommendations, 1):
                print(f"    {i}. {recommendation}")
        else:
            print("  未发现明显性能瓶颈，系统性能良好")
        
        return len(bottlenecks) == 0
    
    def save_results(self):
        """保存测试结果"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"performance_results_{timestamp}.json"
        
        results_file = os.path.join(os.path.dirname(__file__), filename)
        
        with open(results_file, 'w', encoding='utf-8') as f:
            json.dump(self.results, f, indent=2, ensure_ascii=False)
        
        print(f"\n📊 测试结果已保存到: {filename}")
        return results_file
    
    def run_all_tests(self):
        """运行所有性能测试"""
        print("=" * 70)
        print("NexusRemote 性能测试套件")
        print("=" * 70)
        
        tests_passed = 0
        total_tests = 3
        
        # 运行测试
        if self.test_routing_algorithm_speed():
            tests_passed += 1
        
        if self.test_websocket_latency():
            tests_passed += 1
        
        if self.test_memory_usage():
            tests_passed += 1
        
        # 分析瓶颈
        no_bottlenecks = self.analyze_bottlenecks()
        
        # 保存结果
        results_file = self.save_results()
        
        # 生成总结
        print("\n" + "=" * 70)
        print("性能测试总结")
        print("=" * 70)
        
        print(f"测试通过率: {tests_passed}/{total_tests}")
        
        if tests_passed == total_tests and no_bottlenecks:
            print("🎉 性能测试全部通过，系统性能良好!")
        elif tests_passed == total_tests:
            print("⚠️ 测试通过但发现性能瓶颈，需要优化")
        else:
            print("❌ 部分测试未通过，需要修复")
        
        print(f"\n详细结果: {results_file}")
        
        return tests_passed == total_tests

def main():
    """主函数"""
    tester = PerformanceTester()
    success = tester.run_all_tests()
    
    return success

if __name__ == "__main__":
    # 检查依赖
    try:
        import websocket
    except ImportError:
        print("注意: 缺少websocket-client，WebSocket测试将跳过")
    
    try:
        import psutil
    except ImportError:
        print("注意: 缺少psutil，内存测试将跳过")
    
    success = main()
    exit(0 if success else 1)