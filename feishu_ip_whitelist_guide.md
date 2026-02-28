# 飞书 IP 白名单配置指南

## 🌐 服务器 IP 信息

### 公网 IP
- **IP 地址**: `8.139.210.212`
- **来源**: ifconfig.co

### 内网 IP
- **IP 地址**: `10.0.136.53/24`
- **接口**: eth0

### Gateway 当前配置
- **绑定模式**: `loopback` (127.0.0.1)
- **监听端口**: `18789`
- **访问方式**: 仅本地访问

---

## ⚠️ 问题分析

### 当前问题

1. **Gateway 绑定到 loopback**
   - 只能从本地访问 (127.0.0.1)
   - 飞书 Webhook 无法访问
   - 需要改为 `0.0.0.0` 或 `lan`

2. **飞书 Webhook 需要**
   - 可访问的公网 IP 或域名
   - 端口开放（防火墙规则）
   - IP 白名单配置

---

## 🔧 解决方案

### 方式 1: 使用 WebSocket 模式（推荐）

**优点**：
- ✅ 无需公网 IP
- ✅ 无需配置防火墙
- ✅ 无需配置 IP 白名单
- ✅ 连接更稳定

**配置步骤**：

1. **更新 Gateway 配置**：
   ```json
   {
     "gateway": {
       "bind": "loopback",
       "mode": "local"
     },
     "channels": {
       "feishu": {
         "accounts": {
           "default": {
             "connectionMode": "websocket",
             "encryptKey": "your_encrypt_key",
             "verificationToken": "your_verification_token"
           }
         }
       }
     }
   }
   ```

2. **重启 Gateway**：
   ```bash
   openclaw gateway restart
   ```

### 方式 2: 使用 Webhook 模式（需要公网）

**步骤 1: 更新 Gateway 绑定**

将 Gateway 绑定到所有接口：

```json
{
  "gateway": {
    "bind": "0.0.0.0",
    "port": 18789
  }
}
```

**步骤 2: 配置防火墙**

开放端口 18789：

```bash
# 使用 iptables
sudo iptables -A INPUT -p tcp --dport 18789 -j ACCEPT

# 或使用 ufw
sudo ufw allow 18789/tcp

# 或使用 firewalld
sudo firewall-cmd --permanent --add-port=18789/tcp
sudo firewall-cmd --reload
```

**步骤 3: 配置飞书 IP 白名单**

#### 访问飞书开放平台

```
https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=event
```

#### 配置事件订阅

1. **事件类型**: `im.message.receive_v1`
2. **请求方式**: HTTP POST
3. **请求地址**: `http://8.139.210.212:18789/webhook/feishu`
4. **IP 白名单**: 添加 `8.139.210.212`

#### IP 白名单配置

在飞书开放平台的"事件订阅"页面：

1. 找到"IP 白名单"设置
2. 添加服务器 IP: `8.139.210.212`
3. 保存配置

**步骤 4: 配置 Webhook 模式**

```json
{
  "channels": {
    "feishu": {
      "accounts": {
        "default": {
          "connectionMode": "webhook",
          "webhookPort": 18789,
          "encryptKey": "your_encrypt_keyFrom Feishu",
          "verificationToken": "your_verification_token�From Feishu"
        }
      }
    }
  }
}
```

**步骤 5: 重启 Gateway**

```bash
openclaw gateway restart
```

---

## 📋 IP 白名单配置详情

### 飞书 IP 白名单要求

飞书要求在 IP 白名单中配置以下 IP：

- **服务器公网 IP**: `8.139.210.212`

如果使用内网 IP（如 NAT 环境），需要配置：
- **服务器内网 IP**: `10.0.!36.53`

### 多 IP 配置

如果服务器有多个 IP，全部添加：

```
8.139.210.212
10.0.136.53
```

### IP 段配置

如果 IP 可能变化，可以配置 IP 段：

```
8.139.210.0/24
10.0.136.0/24
```

---

## 🔍 验证配置

### 1. 检查 Gateway 绑定

```bash
netstat -tlnp | grep 18789
```

期望输出：
```
tcp  0  0 0.0.0.0:18789  0.0.0.0:*  LISTEN  <pid>/clawdbot-gateway
```

### 2. 测试 Webhook 访问

从外部测试：

```bash
curl -X POST http://8.139.210.212:18789/webhook/feishu \
  -H "Content-Type: application/json" \
  -d '{"test": "connection"}'
```

### 3. 检查防火墙

```bash
# 检查 iptables
sudo iptables -L -n | grep 18789

# 检查 ufw
sudo ufw status numbered

# 检查 firewalld
sudo firewall-cmd --list-ports
```

---

## �!️ 安全建议

### 1. 使用 HTTPS!**

如果可能，配置 HTTPS：

- 使用 Nginx 反向代理
- 配置 SSL 证书
- 使用 Let's Encrypt 免费证书

### 2. 限制访问

- 只允许飞书 IP 访问
- 使用防火墙规则限制来源 IP
- 配置速率限制

### !3. 使用 Token 认证

确保 Gateway 启用了 Token 认证：

```json
{
  "gateway": {
    "auth": {
      "mode": "token",
      "token": "your_secure_token"
    }
  }
}
```

---

## 🚀 快速配置脚本

### Webhook 模式配置

```bash
#!/bin/bash

# 更新 Gateway 绑定
openclaw config set gateway.bind 0.0.0.0

# 开放防火墙端口
sudo ufw allow 18789/tcp

# 重启 Gateway
openclaw gateway restart

echo "✅ Gateway 已配置为 Webhook 模式"
echo "📋 请在飞书开放平台配置："
echo "   - 事件订阅: http://8.139.210.212:18789/webhook/feishu"
echo "   - IP 白名单: 8.139.210.212"
```

### WebSocket 模式配置

```bash
#!/bin/bash

# 更新飞书连接模式
openclaw config set channels.feishu.accounts.default.connectionMode websocket

# 重启 Gateway
openclaw gateway restart

echo "✅ Gateway 已配置为 WebSocket 模式"
echo "📋 请在飞书开放平台获取："
echo "   - Encrypt Key"
echo "   - Verification Token"
```

---

## 📞 相关链接

- **飞书开放平台**: https://open.feishu.cn
- **应用信息**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1
- **事件订阅**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=event
- **OpenClaw 文档**: https://docs.openclaw.ai/channels/feishu

---

## 🎯 推荐方案

**强烈推荐使用 WebSocket 模式**，因为：

1. ✅ 无需公网 IP
2. ✅ 无需配置防火墙
3. ✅ 无需配置 IP 白名单
4. ✅ 连接更稳定
5. ✅ 配置更简单

---

**服务器 IP**: 8.139.210.212  
**配置状态**: ⚠️ 需要完成配置  
**推荐模式**: WebSocket
