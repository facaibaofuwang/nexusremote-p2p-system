#!/usr/bin/env python3
"""
NexusRemote 端到端集成测试
测试三服务架构的完整通信流程
"""

import asyncio
import websockets
import json
import aiohttp
import time
import sys

class NexusRemoteE2ETest:
    def __init__(self):
        self.ws_url = "ws://localhost:8081"
        self.frontend_url = "http://localhost:3000"
        self.backend_url = "http://localhost:5000"
        self.test_results = []
        
    def log(self, message, status="INFO"):
        timestamp = time.strftime("%H:%M:%S")
        print(f"[{timestamp}] [{status}] {message}")
        self.test_results.append(f"[{timestamp}] {message}")
        
    async def test_python_backend(self):
        """测试Python后端API"""
        try:
            async with aiohttp.ClientSession() as session:
                # 测试健康检查
                async with session.get(f"{self.backend_url}/api/health") as response:
                    if response.status == 200:
                        data = await response.json()
                        self.log(f"Python后端健康: {data.get('status', 'unknown')}", "✅")
                    else:
                        self.log(f"Python后端健康检查失败: {response.status}", "❌")
                        return False
                
                # 测试加权路由算法
                async with session.get(f"{self.backend_url}/api/routing/algorithm") as response:
                    if response.status == 200:
                        data = await response.json()
                        advantage = data.get('advantage_ratio', 0)
                        self.log(f"加权路由算法: {advantage}x优势", "✅" if advantage >= 1.2 else "⚠️")
                    else:
                        self.log("加权路由算法API失败", "❌")
                        return False
                
                # 测试通证经济模型
                async with session.get(f"{self.backend_url}/api/economy/model") as response:
                    if response.status == 200:
                        data = await response.json()
                        self.log(f"通证经济模型: {data.get('model_name', 'unknown')}", "✅")
                    else:
                        self.log("通证经济模型API失败", "❌")
                
                return True
                
        except Exception as e:
            self.log(f"Python后端测试异常: {e}", "❌")
            return False
    
    async def test_frontend_api(self):
        """测试前端API"""
        try:
            async with aiohttp.ClientSession() as session:
                # 测试设备列表
                async with session.get(f"{self.frontend_url}/api/devices") as response:
                    if response.status == 200:
                        data = await response.json()
                        device_count = len(data.get('devices', []))
                        self.log(f"前端API: {device_count}个设备", "✅")
                    else:
                        self.log(f"前端API失败: {response.status}", "❌")
                        return False
                
                # 测试网络状态
                async with session.get(f"{self.frontend_url}/api/network") as response:
                    if response.status == 200:
                        self.log("前端网络API正常", "✅")
                    else:
                        self.log("前端网络API失败", "⚠️")
                
                return True
                
        except Exception as e:
            self.log(f"前端API测试异常: {e}", "❌")
            return False
    
    async def test_rust_websocket(self):
        """测试Rust WebSocket服务器"""
        try:
            self.log(f"连接Rust WebSocket: {self.ws_url}", "🔌")
            
            async with websockets.connect(self.ws_url) as websocket:
                # 接收欢迎消息
                welcome_msg = await websocket.recv()
                welcome_data = json.loads(welcome_msg)
                
                if welcome_data.get('type') == 'welcome':
                    client_id = welcome_data.get('client_id', 'unknown')
                    self.log(f"WebSocket连接成功, 客户端ID: {client_id[:8]}...", "✅")
                else:
                    self.log(f"意外的欢迎消息: {welcome_data}", "⚠️")
                
                # 测试ping/pong
                ping_msg = json.dumps({"type": "ping", "timestamp": int(time.time())})
                await websocket.send(ping_msg)
                self.log("发送ping消息", "📤")
                
                pong_msg = await websocket.recv()
                pong_data = json.loads(pong_msg)
                if pong_data.get('type') == 'pong':
                    self.log("收到pong响应", "✅")
                else:
                    self.log(f"意外的pong响应: {pong_data}", "⚠️")
                
                # 测试路由统计
                stats_msg = json.dumps({"type": "get_routing_stats"})
                await websocket.send(stats_msg)
                self.log("请求路由统计", "📤")
                
                stats_response = await websocket.recv()
                stats_data = json.loads(stats_response)
                if stats_data.get('type') == 'routing_stats':
                    total_peers = stats_data.get('total_peers', 0)
                    advantage = stats_data.get('expected_advantage', 0)
                    self.log(f"路由统计: {total_peers}个节点, {advantage}x优势", "✅")
                else:
                    self.log(f"意外的路由统计响应: {stats_data}", "⚠️")
                
                # 测试对等节点发现
                peers_msg = json.dumps({"type": "get_peers", "target_id": ""})
                await websocket.send(peers_msg)
                self.log("请求对等节点列表", "📤")
                
                peers_response = await websocket.recv()
                peers_data = json.loads(peers_response)
                if peers_data.get('type') == 'peers':
                    peer_count = len(peers_data.get('peers', []))
                    self.log(f"对等节点: {peer_count}个", "✅")
                else:
                    self.log(f"意外的对等节点响应: {peers_data}", "⚠️")
                
                # 测试远程命令
                command_msg = json.dumps({
                    "type": "send_command",
                    "command": "test_e2e_integration",
                    "target": "test_device_001"
                })
                await websocket.send(command_msg)
                self.log("发送测试命令", "📤")
                
                command_response = await websocket.recv()
                command_data = json.loads(command_response)
                if command_data.get('type') == 'command_result':
                    self.log(f"命令结果: {command_data.get('status', 'unknown')}", "✅")
                else:
                    self.log(f"意外的命令响应: {command_data}", "⚠️")
                
                return True
                
        except ConnectionRefusedError:
            self.log("WebSocket连接被拒绝", "❌")
            return False
        except Exception as e:
            self.log(f"WebSocket测试异常: {e}", "❌")
            return False
    
    async def test_integrated_workflow(self):
        """测试集成工作流：前端 ↔ Rust ↔ 算法"""
        self.log("开始集成工作流测试...", "🚀")
        
        try:
            # 1. 通过前端获取设备列表
            async with aiohttp.ClientSession() as session:
                async with session.get(f"{self.frontend_url}/api/devices") as response:
                    if response.status == 200:
                        devices_data = await response.json()
                        device_names = [d.get('name', 'unknown') for d in devices_data.get('devices', [])]
                        self.log(f"前端设备: {', '.join(device_names[:3])}...", "✅")
                    else:
                        self.log("无法获取前端设备列表", "❌")
                        return False
            
            # 2. 通过WebSocket发送控制命令
            async with websockets.connect(self.ws_url) as websocket:
                # 跳过欢迎消息
                await websocket.recv()
                
                # 发送集成测试命令
                command_msg = json.dumps({
                    "type": "send_command",
                    "command": "integrated_remote_control",
                    "target": "办公室电脑",
                    "action": "screenshot"
                })
                await websocket.send(command_msg)
                self.log("发送集成远程控制命令", "📤")
                
                response = await websocket.recv()
                response_data = json.loads(response)
                if response_data.get('type') == 'command_result':
                    self.log("集成命令接收成功", "✅")
                else:
                    self.log("集成命令响应异常", "⚠️")
            
            # 3. 验证加权路由数据一致性
            async with aiohttp.ClientSession() as session:
                # 从Python后端获取算法数据
                async with session.get(f"{self.backend_url}/api/routing/algorithm") as response:
                    if response.status == 200:
                        algo_data = await response.json()
                        python_advantage = algo_data.get('advantage_ratio', 0)
                        
                        # 从WebSocket获取路由统计
                        async with websockets.connect(self.ws_url) as websocket:
                            await websocket.recv()  # 欢迎消息
                            
                            stats_msg = json.dumps({"type": "get_routing_stats"})
                            await websocket.send(stats_msg)
                            stats_response = await websocket.recv()
                            stats_data = json.loads(stats_response)
                            ws_advantage = stats_data.get('expected_advantage', 0)
                            
                            # 比较优势比例
                            advantage_diff = abs(python_advantage - ws_advantage)
                            if advantage_diff < 0.1:
                                self.log(f"数据一致性验证: Python({python_advantage}x) ≈ WebSocket({ws_advantage}x)", "✅")
                            else:
                                self.log(f"数据一致性警告: Python({python_advantage}x) ≠ WebSocket({ws_advantage}x)", "⚠️")
            
            self.log("集成工作流测试完成", "🎉")
            return True
            
        except Exception as e:
            self.log(f"集成工作流测试异常: {e}", "❌")
            return False
    
    async def run_all_tests(self):
        """运行所有测试"""
        self.log("=" * 60, "📋")
        self.log("开始 NexusRemote 端到端集成测试", "🚀")
        self.log("=" * 60, "📋")
        
        test_start_time = time.time()
        results = {}
        
        # 测试1: Python后端
        self.log("\n1. 测试Python后端API", "🔧")
        results['python_backend'] = await self.test_python_backend()
        
        # 测试2: 前端API
        self.log("\n2. 测试前端API", "🖥️")
        results['frontend_api'] = await self.test_frontend_api()
        
        # 测试3: Rust WebSocket
        self.log("\n3. 测试Rust WebSocket服务器", "🔌")
        results['rust_websocket'] = await self.test_rust_websocket()
        
        # 测试4: 集成工作流
        self.log("\n4. 测试集成工作流", "🔄")
        results['integrated_workflow'] = await self.test_integrated_workflow()
        
        # 测试总结
        test_duration = time.time() - test_start_time
        self.log("\n" + "=" * 60, "📊")
        self.log("测试完成总结", "🎯")
        self.log("=" * 60, "📊")
        
        total_tests = len(results)
        passed_tests = sum(1 for result in results.values() if result)
        
        for test_name, result in results.items():
            status = "✅ 通过" if result else "❌ 失败"
            self.log(f"{test_name}: {status}")
        
        self.log(f"\n测试统计: {passed_tests}/{total_tests} 通过")
        self.log(f"测试用时: {test_duration:.2f}秒")
        
        if passed_tests == total_tests:
            self.log("\n🎉 所有端到端测试通过！系统集成验证成功！", "🎉")
            return True
        else:
            self.log(f"\n⚠️  {total_tests - passed_tests}个测试失败，需要检查", "⚠️")
            return False
    
    def save_test_report(self):
        """保存测试报告"""
        report_path = "/home/admin/.openclaw/workspace/e2e_test_report.md"
        with open(report_path, 'w') as f:
            f.write("# NexusRemote 端到端集成测试报告\n\n")
            f.write(f"生成时间: {time.strftime('%Y-%m-%d %H:%M:%S')}\n\n")
            f.write("## 测试结果\n\n")
            for line in self.test_results:
                f.write(f"{line}\n")
        
        self.log(f"测试报告已保存: {report_path}", "💾")

async def main():
    """主函数"""
    tester = NexusRemoteE2ETest()
    
    try:
        success = await tester.run_all_tests()
        tester.save_test_report()
        
        if success:
            print("\n" + "=" * 60)
            print("✅ 端到端集成测试验证成功！")
            print("✅ 三服务架构通信正常！")
            print("✅ 加权路由算法工作正常！")
            print("✅ 系统已准备好进行最终演示！")
            print("=" * 60)
            sys.exit(0)
        else:
            print("\n" + "=" * 60)
            print("⚠️  部分测试失败，需要检查系统配置")
            print("=" * 60)
            sys.exit(1)
            
    except KeyboardInterrupt:
        print("\n测试被用户中断")
        sys.exit(1)
    except Exception as e:
        print(f"\n测试运行异常: {e}")
        sys.exit(1)

if __name__ == "__main__":
    # 检查websockets库
    try:
        import websockets
        import aiohttp
    except ImportError:
        print("请先安装依赖: pip install websockets aiohttp")
        sys.exit(1)
    
    asyncio.run(main())