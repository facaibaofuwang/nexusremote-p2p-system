# 飞书插件配置诊断报告

## 🔍 问题诊断

### 当前配置状态

**Gateway 状态**: ✅ 运行中 (PID: 206608)  
**监听端口**: ✅ 18789  
**飞书插件**: ✅ 已启用

### 配置信息

```json
{
  "appId": "cli_a90a058843785cc1",
  "appSecret": "Je7jlqJtGaC3hMAnvlucBhxmTuRhTzQp",
  "domain": "feishu",
  "enabled": true
}
```

### ⚠️ 发现的问题

1. **缺少必要配置**:
   - ❌ `encrypt_key` - 消息加密密钥
   - ❌ `verification_token` - 验证令牌
   - ❌ `connectionMode` - 连接模式

2. **可能的原因**:
   - 飞书应用未正确配置事件订阅
   - Webhook/WebSocket 连接未建立
   - 消息加密验证失败

## 🔧 解决方案

### 方式 1: 使用配置向导（推荐）

```bash
openclaw configure
```

### 方式 2: 手动获取配置信息

#### 1. 访问飞书开放平台

```
https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1
```

#### 2. 获取必要信息

- **Encrypt Key** - 消息加密密钥
- **Verification Token** - 验证令牌

#### 3. 配置事件订阅

在飞书开放平台配置事件订阅：

- **事件类型**: `im.message.receive_v1`
- **请求方式**: 
  - Webhook: HTTP POST
  - WebSocket: WS 连接

### 方式 3: 使用 WebSocket 模式

如果不想配置 Webhook，可以使用 WebSocket 模式。

## 📝 配置示例

### WebSocket 模式配置

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
          "encryptKey": "your_encrypt_key",
          "verificationToken": "your_verification_token",
          "enabled": true
        }
      }
    }
  }
}
```

### Webhook 模式配置

```json
{
  "channels": {
    "feishu": {
      "accounts": {
        "default": {
          "appId": "cli_a90a058843785cc1",
          "appSecret": "Je7jlqJtGaC3hMAnvlucBhxmTuRhTzQp",
          "domain": "feishu",
          "connectionMode": "webhook",
          "webhookPort": 8189,
          "encryptKey": "your_encrypt_key",
          "verificationToken": "your_verification_token",
          "enabled": true
        }
      }
    }
  }
}
```

## 🚀 下一步

1. **获取配置信息**:
   - 访问飞书开放平台
   - 获取 Encrypt Key 和 Verification Token

2. **选择连接模式**:
   - WebSocket（推荐，无需公网）
   - Webhook（需要公网 IP 或域名）

3. **更新配置**:
   - 运行 `openclaw configure`
   - 或手动编辑配置文件

4. **重启 Gateway**:
   ```bash
   openclaw gateway restart
   ```

## 📞 相关文档

- [飞书开放平台文档](https://open.feishu.cn/document)
- [OpenClaw 飞书插件文档](https://docs.openclaw.ai/channels/feishu)
- [飞书机器人开发指南](https://open.feishu.cn/document/ukTMukTMukTM/uEjNwUjN2YzMjM0)

## 🔗 飞书开放平台链接

- **应用信息**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1
- **事件订阅**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=event
- **权限管理**: https://open.feishu.cn/open-apis/bot/v2/info?app_id=cli_a90a058843785cc1&tab=permission

---

**状态**: ⚠️ 需要完成配置  
**下一步**: 获取飞书应用配置信息并更新
