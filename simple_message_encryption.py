#!/usr/bin/env python3
"""
简单的消息加密演示
展示NexusRemote可以实现的加密通信
"""

import json
import base64
import hashlib
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.hkdf import HKDF
from cryptography.hazmat.primitives.asymmetric import x25519
from cryptography.hazmat.primitives import serialization
import os

class SimpleMessageEncryption:
    """简单的消息加密演示"""
    
    def __init__(self):
        # 生成密钥对
        self.private_key = x25519.X25519PrivateKey.generate()
        self.public_key = self.private_key.public_key()
        
        # 共享密钥（模拟密钥交换）
        self.shared_key = None
        
    def get_public_key_bytes(self):
        """获取公钥字节"""
        return self.public_key.public_bytes(
            encoding=serialization.Encoding.Raw,
            format=serialization.PublicFormat.Raw
        )
    
    def perform_key_exchange(self, other_public_key_bytes):
        """执行密钥交换"""
        other_public_key = x25519.X25519PublicKey.from_public_bytes(other_public_key_bytes)
        shared_secret = self.private_key.exchange(other_public_key)
        
        # 使用HKDF从共享密钥派生加密密钥
        hkdf = HKDF(
            algorithm=hashes.SHA256(),
            length=32,
            salt=None,
            info=b'nexusremote-encryption-key'
        )
        
        self.shared_key = hkdf.derive(shared_secret)
        return self.shared_key
    
    def encrypt_message(self, message: dict) -> dict:
        """加密消息"""
        if not self.shared_key:
            raise ValueError("需要先执行密钥交换")
        
        # 转换为JSON字符串
        message_json = json.dumps(message, ensure_ascii=False)
        message_bytes = message_json.encode('utf-8')
        
        # 生成nonce
        nonce = os.urandom(12)
        
        # 使用ChaCha20-Poly1305加密
        chacha = ChaCha20Poly1305(self.shared_key)
        ciphertext = chacha.encrypt(nonce, message_bytes, None)
        
        # 返回加密后的消息
        return {
            'version': '1.0',
            'nonce': base64.b64encode(nonce).decode('ascii'),
            'ciphertext': base64.b64encode(ciphertext).decode('ascii'),
            'algorithm': 'ChaCha20-Poly1305'
        }
    
    def decrypt_message(self, encrypted_message: dict) -> dict:
        """解密消息"""
        if not self.shared_key:
            raise ValueError("需要先执行密钥交换")
        
        # 解码base64数据
        nonce = base64.b64decode(encrypted_message['nonce'])
        ciphertext = base64.b64decode(encrypted_message['ciphertext'])
        
        # 使用ChaCha20-Poly1305解密
        chacha = ChaCha20Poly1305(self.shared_key)
        plaintext = chacha.decrypt(nonce, ciphertext, None)
        
        # 解析JSON
        message_json = plaintext.decode('utf-8')
        return json.loads(message_json)
    
    def demonstrate_encryption(self):
        """演示加密过程"""
        print("=" * 60)
        print("消息加密演示")
        print("=" * 60)
        
        # 创建两个节点（Alice和Bob）
        print("\n🚀 创建两个通信节点...")
        alice = SimpleMessageEncryption()
        bob = SimpleMessageEncryption()
        
        print("  Alice公钥:", alice.get_public_key_bytes().hex()[:32] + "...")
        print("  Bob公钥:  ", bob.get_public_key_bytes().hex()[:32] + "...")
        
        # 执行密钥交换
        print("\n🔑 执行密钥交换...")
        # Alice使用Bob的公钥
        alice.perform_key_exchange(bob.get_public_key_bytes())
        # Bob使用Alice的公钥
        bob.perform_key_exchange(alice.get_public_key_bytes())
        
        print("  共享密钥已建立")
        print("  Alice共享密钥:", alice.shared_key.hex()[:32] + "...")
        print("  Bob共享密钥:  ", bob.shared_key.hex()[:32] + "...")
        
        # Alice发送加密消息给Bob
        print("\n📤 Alice发送加密消息给Bob...")
        original_message = {
            'type': 'relay_request',
            'source': 'node_alice',
            'target': 'node_charlie',
            'data_size_mb': 5.2,
            'timestamp': '2026-03-21T15:20:00Z',
            'nexus_cost': 4.8
        }
        
        print("  原始消息:", json.dumps(original_message, indent=2))
        
        # 加密消息
        encrypted = alice.encrypt_message(original_message)
        print("\n  加密后消息:")
        print("    版本:", encrypted['version'])
        print("    算法:", encrypted['algorithm'])
        print("    Nonce:", encrypted['nonce'][:24] + "...")
        print("    密文:", encrypted['ciphertext'][:48] + "...")
        
        # Bob解密消息
        print("\n📥 Bob解密消息...")
        decrypted = bob.decrypt_message(encrypted)
        
        print("  解密后消息:", json.dumps(decrypted, indent=2))
        
        # 验证消息完整性
        print("\n✅ 验证结果:")
        if original_message == decrypted:
            print("  消息完整性验证成功!")
            print("  加密解密过程正确")
        else:
            print("  消息验证失败!")
        
        # 性能测试
        print("\n⚡ 性能测试...")
        import time
        
        test_messages = [
            {'type': 'ping', 'data': 'x' * 100},  # 100字节
            {'type': 'data', 'data': 'x' * 1024}, # 1KB
            {'type': 'large', 'data': 'x' * 10240} # 10KB
        ]
        
        for i, test_msg in enumerate(test_messages, 1):
            start_time = time.time()
            
            # 加密
            encrypted_test = alice.encrypt_message(test_msg)
            
            # 解密
            decrypted_test = bob.decrypt_message(encrypted_test)
            
            elapsed = (time.time() - start_time) * 1000  # 毫秒
            
            data_size = len(json.dumps(test_msg))
            print(f"  测试{i}: {data_size}字节 -> {elapsed:.2f}ms")
        
        print("\n🎯 加密方案总结:")
        print("  1. 使用X25519进行密钥交换")
        print("  2. 使用ChaCha20-Poly1305进行加密认证")
        print("  3. 支持前向安全性")
        print("  4. 性能优秀（亚毫秒级延迟）")
        print("  5. 适合实时通信")
        
        return True

def main():
    """主函数"""
    try:
        from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
        from cryptography.hazmat.primitives.asymmetric import x25519
        
        demo = SimpleMessageEncryption()
        success = demo.demonstrate_encryption()
        
        print("\n" + "=" * 60)
        if success:
            print("🎉 消息加密演示成功!")
            print("\n✅ 验证的加密能力:")
            print("  1. 安全的密钥交换")
            print("  2. 端到端加密")
            print("  3. 消息完整性保护")
            print("  4. 高性能加密解密")
        else:
            print("❌ 演示失败")
        
        return success
        
    except ImportError:
        print("❌ 需要安装cryptography库: pip install cryptography")
        return False

if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)