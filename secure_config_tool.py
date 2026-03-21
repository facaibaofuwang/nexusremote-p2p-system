#!/usr/bin/env python3
"""
NexusRemote 加密配置工具
用于安全地管理敏感信息（如GitHub Token）
"""

import os
import json
import base64
import hashlib
from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import getpass

class SecureConfigManager:
    """安全配置管理器"""
    
    def __init__(self, config_file=".secure_config"):
        self.config_file = config_file
        self.key = None
        self.cipher = None
        self.config = {}
    
    def derive_key(self, password: str, salt: bytes) -> bytes:
        """从密码派生加密密钥"""
        kdf = PBKDF2HMAC(
            algorithm=hashes.SHA256(),
            length=32,
            salt=salt,
            iterations=100000,
        )
        return kdf.derive(password.encode('utf-8'))
    
    def initialize(self, password: str):
        """初始化加密配置"""
        # 生成随机salt
        salt = os.urandom(16)
        
        # 派生密钥
        self.key = self.derive_key(password, salt)
        
        # 创建加密器
        self.cipher = Fernet(base64.urlsafe_b64encode(self.key))
        
        # 保存salt到配置文件
        self.config = {
            '_salt': base64.b64encode(salt).decode('utf-8'),
            '_version': '1.0',
            '_encrypted': {}
        }
        
        self.save()
        print("✅ 安全配置已初始化")
    
    def load(self, password: str):
        """加载加密配置"""
        if not os.path.exists(self.config_file):
            print("❌ 配置文件不存在，需要先初始化")
            return False
        
        # 读取配置
        with open(self.config_file, 'r', encoding='utf-8') as f:
            self.config = json.load(f)
        
        # 获取salt
        salt = base64.b64decode(self.config['_salt'].encode('utf-8'))
        
        # 派生密钥
        self.key = self.derive_key(password, salt)
        
        # 创建加密器
        self.cipher = Fernet(base64.urlsafe_b64encode(self.key))
        
        print("✅ 安全配置已加载")
        return True
    
    def save(self):
        """保存配置"""
        with open(self.config_file, 'w', encoding='utf-8') as f:
            json.dump(self.config, f, indent=2, ensure_ascii=False)
    
    def set(self, key: str, value: str):
        """设置配置值（加密存储）"""
        if not self.cipher:
            print("❌ 配置未初始化或未加载")
            return False
        
        # 加密值
        encrypted_value = self.cipher.encrypt(value.encode('utf-8'))
        encrypted_b64 = base64.b64encode(encrypted_value).decode('utf-8')
        
        # 保存到配置
        self.config['_encrypted'][key] = encrypted_b64
        self.save()
        
        print(f"✅ 配置 '{key}' 已加密保存")
        return True
    
    def get(self, key: str) -> str:
        """获取配置值（解密）"""
        if not self.cipher:
            print("❌ 配置未初始化或未加载")
            return None
        
        # 检查键是否存在
        if key not in self.config['_encrypted']:
            print(f"❌ 配置 '{key}' 不存在")
            return None
        
        # 解密值
        encrypted_b64 = self.config['_encrypted'][key].encode('utf-8')
        encrypted_value = base64.b64decode(encrypted_b64)
        decrypted_value = self.cipher.decrypt(encrypted_value)
        
        return decrypted_value.decode('utf-8')
    
    def list_keys(self):
        """列出所有配置键（不显示值）"""
        if '_encrypted' not in self.config:
            print("❌ 配置为空")
            return
        
        keys = list(self.config['_encrypted'].keys())
        if not keys:
            print("📭 没有存储的配置项")
        else:
            print(f"📋 已存储的配置项 ({len(keys)}个):")
            for i, key in enumerate(keys, 1):
                print(f"  {i}. {key}")
    
    def delete_key(self, key: str):
        """删除配置键"""
        if '_encrypted' not in self.config:
            print("❌配置为空")
            return False
        
        if key not in self.config['_encrypted']:
            print(f"❌ 配置 '{key}' 不存在")
            return False
        
        del self.config['_encrypted'][key]
        self.save()
        print(f"✅ 配置 '{key}' 已删除")
        return True
    
    def export_env(self, output_file='.env'):
        """导出为环境变量文件（不加密，仅用于开发）"""
        if '_encrypted' not in self.config:
            print("❌ 配置为空")
            return False
        
        env_lines = []
        for key in self.config['_encrypted'].keys():
            value = self.get(key)
            env_lines.append(f"{key.upper()}={value}")
        
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write('\n'.join(env_lines))
        
        print(f"✅ 环境变量已导出到: {output_file}")
        print(f"   ⚠️  警告: 此文件包含敏感信息，请妥善保管")
        return True

def main():
    """主函数"""
    import sys
    
    if len(sys.argv) < 2:
        print("NexusRemote 安全配置工具")
        print("=" * 60)
        print("用法:")
        print("  python secure_config_tool.py init           # 初始化配置")
        print("  python secure_config_tool.py set KEY VALUE # 设置配置")
        print("  python secure_config_tool.py get KEY       # 获取配置")
        print("  python secure_config_tool.py list          # 列出所有键")
        print("  python secure_config_tool.py delete KEY    # 删除配置")
        print("  python secure_config_tool.py export       # 导出为.env文件")
        print("")
        print("示例:")
        print("  python secure_config_tool.py set github_token YOUR_TOKEN")
        print("  python secure_config_tool.py get github_token")
        return 1
    
    manager = SecureConfigManager()
    command = sys.argv[1]
    
    # 对于需要密码的命令，提示输入
    if command in ['set', 'get', 'list', 'delete', 'export']:
        # 检查配置文件是否存在
        if os.path.exists(manager.config_file):
            print("🔑 输入配置密码:")
            password = getpass.getpass("> ")
            
            if not manager.load(password):
                return 1
        else:
            print("❌ 配置文件不存在，请先使用 'init' 命令初始化")
            return 1
    
    if command == 'init':
        print("🔑 创建配置密码:")
        password = getpass.getpass("> ")
        confirm_password = getpass.getpass("确认密码: ")
        
        if password != confirm_password:
            print("❌ 密码不匹配")
            return 1
        
        if len(password) < 8:
            print("❌ 密码太短，至少需要8个字符")
            return 1
        
        manager.initialize(password)
        
        # 设置GitHub Token（如果提供了）
        if len(sys.argv) >= 4 and sys.argv[2] == 'github_token':
            token = sys.argv[3]
            manager.set('github_token', token)
        
    elif command == 'set':
        if len(sys.argv) < 4:
            print("❌ 用法: python secure_config_tool.py set KEY VALUE")
            return 1
        
        key = sys.argv[2]
        value = sys.argv[3]
        manager.set(key, value)
    
    elif command == 'get':
        if len(sys.argv) < 3:
            print("❌ 用法: python secure_config_tool.py get KEY")
            return 1
        
        key = sys.argv[2]
        value = manager.get(key)
        
        if value:
            print(f"🔓 {key}: {value[:10]}...{value[-4:]}")  # 只显示部分
    
    elif command == 'list':
        manager.list_keys()
    
    elif command == 'delete':
        if len(sys.argv) < 3:
            print("❌ 用法: python secure_config_tool.py delete KEY")
            return 1
        
        key = sys.argv[2]
        manager.delete_key(key)
    
    elif command == 'export':
        manager.export_env()
    
    else:
        print(f"❌ 未知命令: {command}")
        return 1
    
    return 0

if __name__ == "__main__":
    import sys
    try:
        import cryptography
        sys.exit(main())
    except ImportError:
        print("❌ 需要安装cryptography库:")
        print("   pip install cryptography")
        sys.exit(1)