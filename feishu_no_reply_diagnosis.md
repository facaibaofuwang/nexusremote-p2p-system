# 🔍 飞书消息无回复问题诊断报告

## 问题总结

用户报告：已经配置了飞书 IP 白名单和权限，并通过飞书发送了消息，但是没有收到回复。

---

## ✅ 已确认正常的项目

### 1. Gateway 运行状态
- ✅ Gateway 正在运行 (PID: 206608)
- ✅ 监听端口: 18789 (loopback)
- ✅ CPU 使用率: 1.0%
- ✅ 内存使用率: 5.5%

### 2. 飞书应用配置
- ✅ App ID: cli_a90a058843785cc1
- ✅ App Secret: 已配置
- ✅ Domain: feishu (国内版)
- ✅ IP 白名单: 8.139.210.212 ✅
- ✅ Bot Open ID: ou_30a8cb6d8c096d34ce1ed5f9102d2064
- ✅ Bot 名称: AI助手
- ✅ 激活状态: 2 (已激活)

### 3. 配对状态
- ✅ 用户已配对: ou_d4e975c35ebcabdbf3b9a3765335373e
- ✅ 配对代码: 4H3WFBTK
- ✅ 用户名称: 赵文博
- ✅ 最后活跃: 2026-02-26T06:34:31.098Z

### 4. API 访问测试
- ✅ Tenant Access Token: 获取成功
- ✅ Bot Info API: 访问成功
- ✅ IP 白名单: 已配置 8.139.210.212

---

## ❌ 发现的问题

### 🔴 关键问题：缺少必要的加密配置

当前飞书配置：

```json
{
  "appId": "cli_a90a058843785cc1",
  "appSecret": "Je7jlqJtGaC3hMAnvlucBhxmTuRhTzQp",
  "domain": "feishu",
  "enabled": true
}
```

**缺少的配置**：
- ❌ `encryptKey` - 消息加密密钥
- ❌ `verificationToken` - 验证令牌
- ❌ `connectionMode` - 连接模式

### 为什么这会导致问题？

从飞书插件源码分析，`createEventDispatcher` 函数需要这些参数：

```typescript
export function createEventDispatcher(account: ResolvedFeishuAccount): Lark.EventDispatcher {
  return new Lark.EventDispatcher({
    encryptKey: account.encryptKey,        // ❌ 未配置
    verificationToken: account.verificationToken,  // ❌ 未配置
  });
}
```

如果没有这些参数：
1. **WebSocket 模式**: 无法验证飞书服务器发送的事件
2. **Webhook 模式**: 无法验证飞书发送的请求
3. **消息处理**: 事件分发器无法正常工作
4. **结果**: 收到消息但无法处理和回复

### 🟡 次要问题：Gateway 绑定模式

当前配置：
- **绑定模式**: `loopback` (127.0.0.1)
- **影响**: 只能从本地访问

如果使用 Webhook 模式，需要：
- 绑定到 `0.0.0.0` (所有接口)
- 开放防火墙端口

---

## 🔧 解决方案

### 方式 1: 使用 WebSocket 模式（推荐）⭐

**步骤 1: 获取加密配置**

访问飞书开放平台：
```
https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1
```

获取以下信息：
- **Encrypt Key** - 消息加密密钥
- **Verification Token** - 验证令牌

**步骤 2: 更新配置**

运行以下命令：

```bash
# 设置连接模式为 WebSocket
openclaw config set channels.feishu.accounts.default.connectionMode websocket

# 设置 Encrypt Key（替换为实际值）
openclaw config set channels.feishu.accounts.default.encryptKey "YOUR_ENCRYPT_KEY"

# 设置 Verification Token（替换为实际值）
openclaw config set channels.feishu.accounts.default.verificationToken "YOUR_VERIFICATION_TOKEN"

# 重启 Gateway
openclaw gateway restart
```

或者手动编辑配置文件 `/home/admin/.openclaw/openclaw.json`：

```json
{
  "channels": {
    "feishu": {
      "accounts": {
        "default": {
          "appId": "cli_a90a058843785cc1",
          "appSecret": "Je7jlqJtGaC3hMAnvlucBhxmTuRhTzQp",
          "domain": "feishu",
          "connectionMode": "websocket",
          "encryptKey": "YOUR_ENCRYPT_KEY",
          "verificationToken": "YOUR_VERIFICATION_TOKEN",
          "enabled": true
        }
      }
    }
  }
}
```

**步骤 3: 重启 Gateway**

```bash
openclaw gateway restart
```

### 方式 2: 使用 Webhook 模式

**步骤 1: 获取加密配置**

同方式 1

**步骤 2: 更新 Gateway 绑定**

```bash
# 绑定到所有接口
openclaw config set gateway.bind 0.0.0.0

# 开放防火墙端口
sudo ufw allow 18789/tcp
```

**步骤 3: 配置飞书事件订阅**

访问飞书开放平台：
```
https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=event
```

配置：
- **事件类型**: `im.message.receive_v1`
- **请求方式**: HTTP POST
- **请求地址**: `http://8.139.210.212:18789/feishu/events`
- **IP 白名单**: 8.139.210.212

**步骤 4: 更新飞书配置**

```bash
# 设置连接模式为 Webhook
openclaw config set channels.feishu.accounts.default.connectionMode webhook

# 设置 Webhook 端口
openclaw config set channels.feishu.accounts.default.webhookPort 18789

# 设置 Encrypt Key（替换为实际值）
openclaw config set channels.feishu.accounts.default.encryptKey "YOUR_ENCRYPT_KEY"

# 设置 Verification Token（替换为实际值）
openclaw config set channels.feishu.accounts.default.verificationToken "YOUR_VERIFICATION_TOKEN"

# 重启 Gateway
openclaw gateway restart
```

---

## 📋 配置检查清单

完成以下检查：

- [ ] 从飞书开放平台获取 Encrypt Key
- [ ] 从飞书开放平台获取 Verification Token
- [ ] 选择连接模式（WebSocket 或 Webhook）
- [ ] 更新 openclaw.json 配置文件
- [ ] 如果使用 Webhook，配置防火墙规则
- [ ] 如果使用 Webhook，配置飞书事件订阅
- [ ] 重启 Gateway
- [ ] 测试发送消息

---

## 🔍 验证步骤

### 1. 检查配置

```bash
cat /home/admin/.openclaw/openclaw.json | grep -A 20 '"feishu"'
```

### 2. 检查 Gateway 状态

```bash
ps aux | grep clawdbot-gateway
netstat -tlnp | grep 18789
```

### 3. 检查网络连接

```bash
lsof -p $(pgrep clawdbot-gateway) | grep -E 'TCP|UDP'
```

### 4. 测试消息发送

在飞书中发送测试消息，观察是否有回复。

---

## 🚀 快速配置脚本

### WebSocket 模式配置脚本

```bash
#!/bin/bash

echo "🔧 配置飞书 WebSocket 模式..."

# 提示用户输入加密配置
read -p "请输入 Encrypt Key: " ENCRYPT_KEY
read -p "请输入 Verification Token: " VERIFICATION_TOKEN

# 更新配置
openclaw config set channels.feishu.accounts.default.connectionMode websocket
openclaw config set channels.feishu.accounts.default.encryptKey "$ENCRYPT_KEY"
openclaw config set channels.feishu.accounts.default.verificationToken "$VERIFICATION_TOKEN"

# 重启 Gateway
openclaw gateway restart

echo "✅ 配置完成！请测试发送消息。"
```

### Webhook 模式配置脚本

```bash
#!/bin/bash

echo "🔧 配置飞书 Webhook 模式..."

# 提示用户输入加密配置
read -p "请输入 Encrypt Key: " ENCRYPT_KEY
read -p "请输入 Verification Token: " VERIFICATION_TOKEN

# 更新 Gateway 绑定
openclaw config set gateway.bind 0.0.0.0

# 开放防火墙端口
sudo ufw allow 18789/tcp

# 更新飞书配置
openclaw config set channels.feishu.accounts.default.connectionMode webhook
openclaw config set channels.feishu.accounts.default.webhookPort 18789
openclaw config set channels.feishu.accounts.default.encryptKey "$ENCRYPT_KEY"
openclaw config set channels.feishu.accounts.default.verificationToken "$VERIFICATION_TOKEN"

# 重启 Gateway
openclaw gateway restart

echo "✅ 配置完成！"
echo "📋 请在飞书开放平台配置事件订阅："
echo "   事件类型: im.message.receive_v1"
echo "   请求地址: http://8.139.210.212:18789/feishu/events"
echo "   IP 白名单: 8.139.210.212"
```

---

## 📞 相关链接

- **飞书开放平台**: https://open.feishu.cn

- **应用信息**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1
- **事件订阅**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=event
- **权限管理**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=permission
- **OpenClaw 文档**: https://docs.openclaw.ai/channels/feishu

---

## 🎯 推荐方案

**强烈推荐使用 WebSocket 模式**，因为：

1. ✅ 无需公网 IP
2. ✅ 无需配置防火墙
3. ✅ 无需配置 IP 白名单（已配置）
4. ✅ 连接更稳定
5. ✅ 配置更简单

---

## 📝 总结

**问题根源**: 缺少 `encryptKey` 和 `verificationToken` 配置

**解决方案**:
1. 从飞书开放平台获取加密配置
2. 更新 openclaw.json 配置文件
3. 重启 Gateway

**推荐模式**: WebSocket

**预计解决时间**: 5-10 分钟

---

**诊断完成时间**: 2026-02-26 14:40  
**问题状态**: 🔴 需要配置加密参数  
**下一步**: 获取并配置 Encrypt Key 和 Verification Token
