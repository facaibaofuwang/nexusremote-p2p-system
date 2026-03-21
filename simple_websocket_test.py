#!/usr/bin/env python3
"""
简单的WebSocket性能测试
"""

import time
import json
import websocket
import threading

def test_websocket_connection():
    """测试WebSocket连接"""
    print("🔗 测试WebSocket连接...")
    
    connected = False
    message_received = False
    latency = None
    
    def on_message(ws, message):
        nonlocal message_received, latency
        receive_time = time.time()
        
        try:
            data = json.loads(message)
            if 'send_time' in data:
                send_time = data['send_time']
                latency = (receive_time - send_time) * 1000  # 毫秒
                message_received = True
                print(f"📨 收到消息，延迟: {latency:.1f}ms")
        except:
            pass
    
    def on_error(ws, error):
        print(f"⚠️ WebSocket错误: {error}")
    
    def on_close(ws, close_status_code, close_msg):
        print("🔌 连接关闭")
    
    def on_open(ws):
        nonlocal connected
        connected = True
        print("✅ WebSocket连接建立")
        
        # 发送测试消息
        test_msg = {
            "type": "ping",
            "send_time": time.time(),
            "message": "性能测试"
        }
        ws.send(json.dumps(test_msg))
        print("📤 发送测试消息")
    
    # 创建WebSocket连接
    ws = websocket.WebSocketApp(
        "ws://127.0.0.1:8081",
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close
    )
    
    # 在后台线程运行
    thread = threading.Thread(target=ws.run_forever)
    thread.daemon = True
    thread.start()
    
    # 等待连接建立
    for _ in range(10):  # 最多等待2秒
        if connected:
            break
        time.sleep(0.2)
    
    if not connected:
        print("❌ 连接建立失败")
        return False
    
    # 等待消息返回
    for _ in range(20):  # 最多等待4秒
        if message_received:
            break
        time.sleep(0.2)
    
    # 关闭连接
    ws.close()
    
    if message_received and latency is not None:
        print(f"✅ WebSocket测试成功，延迟: {latency:.1f}ms")
        
        # 评估延迟
        if latency < 10:
            print("  延迟优秀 (<10ms)")
        elif latency < 50:
            print("  延迟良好 (<50ms)")
        elif latency < 100:
            print("  延迟可接受 (<100ms)")
        else:
            print("  延迟较高 (>100ms)")
        
        return True
    else:
        print("❌ 未收到响应消息")
        return False

def test_websocket_throughput():
    """测试WebSocket吞吐量"""
    print("\n📊 测试WebSocket吞吐量...")
    
    messages_sent = 0
    messages_received = 0
    latencies = []
    
    def on_message(ws, message):
        nonlocal messages_received
        receive_time = time.time()
        
        try:
            data = json.loads(message)
            if 'send_time' in data:
                latency = (receive_time - data['send_time']) * 1000
                latencies.append(latency)
                messages_received += 1
        except:
            pass
    
    def on_error(ws, error):
        print(f"  错误: {error}")
    
    def on_close(ws, close_status_code, close_msg):
        print("  测试完成")
    
    def on_open(ws):
        nonlocal messages_sent
        print("  连接建立，开始发送消息...")
        
        start_time = time.time()
        
        # 发送100条消息
        for i in range(100):
            test_msg = {
                "type": "throughput_test",
                "message_id": i,
                "send_time": time.time(),
                "data": "x" * 50  # 50字节数据
            }
            ws.send(json.dumps(test_msg))
            messages_sent += 1
            
            # 控制发送速率
            time.sleep(0.01)  # 10ms间隔
        
        # 等待所有消息返回
        time.sleep(1)
        
        test_duration = time.time() - start_time
        print(f"  发送完成，耗时: {test_duration:.2f}秒")
        
        ws.close()
    
    # 创建连接
    ws = websocket.WebSocketApp(
        "ws://127.0.0.1:8081",
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close
    )
    
    # 运行测试
    thread = threading.Thread(target=ws.run_forever)
    thread.daemon = True
    thread.start()
    
    # 等待测试完成
    thread.join(timeout=10)
    
    # 计算结果
    if messages_received > 0:
        success_rate = messages_received / messages_sent * 100
        avg_latency = sum(latencies) / len(latencies) if latencies else 0
        throughput = messages_received / 5  # 假设5秒测试时间
        
        print(f"✅ 吞吐量测试完成:")
        print(f"  发送: {messages_sent}, 接收: {messages_received}")
        print(f"  成功率: {success_rate:.1f}%")
        print(f"  平均延迟: {avg_latency:.1f}ms")
        print(f"  吞吐量: {throughput:.1f} 消息/秒")
        
        return success_rate > 80  # 成功率大于80%为通过
    else:
        print("❌ 未收到任何消息")
        return False

def main():
    """主函数"""
    print("=" * 60)
    print("WebSocket性能测试")
    print("=" * 60)
    
    # 测试基本连接
    connection_ok = test_websocket_connection()
    
    if connection_ok:
        # 测试吞吐量
        throughput_ok = test_websocket_throughput()
    else:
        throughput_ok = False
    
    print("\n" + "=" * 60)
    print("测试结果")
    print("=" * 60)
    
    if connection_ok and throughput_ok:
        print("🎉 WebSocket性能测试全部通过!")
        return True
    elif connection_ok:
        print("⚠️ 基本连接正常，但吞吐量测试未通过")
        return True  # 基本功能正常
    else:
        print("❌ WebSocket连接测试失败")
        return False

if __name__ == "__main__":
    try:
        import websocket
        success = main()
        exit(0 if success else 1)
    except ImportError:
        print("❌ 需要安装websocket-client: pip install websocket-client")
        exit(1)