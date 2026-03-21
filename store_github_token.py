#!/usr/bin/env python3
"""
简单的GitHub Token存储工具
使用Python keyring或环境变量
"""

import os
import json
import base64

class GitHubTokenManager:
    """GitHub Token管理器"""
    
    def __init__(self, config_file=".github_token_config"):
        self.config_file = config_file
        self.config = {}
        self.load()
    
    def load(self):
        """加载配置"""
        if os.path.exists(self.config_file):
            try:
                with open(self.config_file, 'r', encoding='utf-8') as f:
                    self.config = json.load(f)
            except:
                self.config = {}
    
    def save(self):
        """保存配置"""
        with open(self.config_file, 'w', encoding='utf-8') as f:
            json.dump(self.config, f, indent=2)
    
    def store_token(self, token: str, name: str = "default"):
        """存储Token（简单编码，生产环境应使用真正的加密）"""
        # 简单base64编码（仅用于演示，生产环境应使用Fernet等）
        encoded = base64.b64encode(token.encode('utf-8')).decode('utf-8')
        
        self.config[name] = {
            'token': encoded,
            'stored_at': __import__('datetime').datetime.now().isoformat(),
            'description': 'GitHub Personal Access Token'
        }
        
        self.save()
        print(f"✅ GitHub Token已存储 (名称: {name})")
        print(f"   只显示: {token[:10]}...{token[-4:]}")
    
    def get_token(self, name: str = "default") -> str:
        """获取Token"""
        if name not in self.config:
            print(f"❌ 未找到名为 '{name}' 的Token")
            return None
        
        encoded = self.config[name]['token']
        token = base64.b64decode(encoded.encode('utf-8')).decode('utf-8')
        
        return token
    
    def export_to_git(self, name: str = "default"):
        """导出为Git配置"""
        token = self.get_token(name)
        if not token:
            return False
        
        # 生成Git远程URL
        git_url = f"https://facaibaofuwang:{token}@github.com/facaibaofuwang/nexusremote-p2p-system.git"
        
        print(f"🔗 Git远程URL已生成:")
        print(f"   {git_url[:50]}...")
        print(f"   {git_url[-50:]}")
        
        # 保存到临时文件
        with open('.git_remote_url.txt', 'w', encoding='utf-8') as f:
            f.write(git_url)
        
        print(f"\n✅ 已保存到: .git_remote_url.txt")
        print(f"   使用: git remote set-url origin $(cat .git_remote_url.txt)")
        
        return True
    
    def list_tokens(self):
        """列出所有存储的Token"""
        if not self.config:
            print("📭 没有存储的Token")
            return
        
        print(f"📋 已存储的Token ({len(self.config)}个):")
        for i, (name, info) in enumerate(self.config.items(), 1):
            print(f"  {i}. {name}")
            print(f"     存储时间: {info['stored_at']}")
            print(f"     描述: {info['description']}")

def main():
    """主函数"""
    import sys
    
    if len(sys.argv) < 2:
        print("GitHub Token管理工具")
        print("=" * 60)
        print("用法:")
        print("  python store_github_token.py store TOKEN [NAME]")
        print("  python store_github_token.py get [NAME]")
        print("  python store_github_token.py export [NAME]")
        print("  python store_github_token.py list")
        print("")
        print("示例:")
        print("  python store_github_token.py store YOUR_TOKEN")
        print("  python store_github_token.py export")
        return 1
    
    manager = GitHubTokenManager()
    command = sys.argv[1]
    
    if command == 'store':
        if len(sys.argv) < 3:
            print("❌ 用法: python store_github_token.py store TOKEN [NAME]")
            return 1
        
        token = sys.argv[2]
        name = sys.argv[3] if len(sys.argv) > 3 else "default"
        
        manager.store_token(token, name)
    
    elif command == 'get':
        name = sys.argv[2] if len(sys.argv) > 2 else "default"
        token = manager.get_token(name)
        
        if token:
            print(f"🔓 Token ({name}): {token[:10]}...{token[-4:]}")
    
    elif command == 'export':
        name = sys.argv[2] if len(sys.argv) > 2 else "default"
        manager.export_to_git(name)
    
    elif command == 'list':
        manager.list_tokens()
    
    else:
        print(f"❌ 未知命令: {command}")
        return 1
    
    return 0

if __name__ == "__main__":
    import sys
    sys.exit(main())