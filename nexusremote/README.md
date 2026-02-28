# NexusRemote v3.0

> 基于通证激励的去中心化 P2P 远程控制系统

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

## 项目愿景

构建一个完全去中心化、自我演进、经济激励驱动的点对点远程控制网络。

**核心理念**："人人皆节点，贡献即收益"。打破传统远控软件对中心服务器的依赖，利用区块链思维解决资源分配问题，实现真正的隐私安全与网络自由。

## 核心特性

### 🏗️ 三合一架构
单一应用程序集成控制端、被控端、中继节点功能，根据网络状态动态切换角色。

### 🚫 零服务器依赖
除初始引导节点外，无中心服务器。信令交换、数据存储、流量转发均由社区节点共同维护。

### 💰 通证经济驱动
引入原生通证 NEXUS，通过"贡献带宽获币、消耗代币加速"的闭环，解决 P2P 网络的"公地悲剧"。

### 🎯 加权路由优化
持有代币越多、信誉越高的用户，在 DHT 网络中拥有更高的路由权重，享受更短链路和更低延迟。

### ⚡ 极致体验
- IPv6 优先直连，低延迟 (<30ms)
- H.265 视频编码，硬件加速
- QUIC 传输，抗丢包
- Noise 协议端到端加密

## 技术栈

| 组件 | 技术选型 | 理由 |
|------|----------|------|
| 核心语言 | Rust | 内存安全、高性能、并发强、区块链生态完善 |
| P2P 网络 | libp2p (rust-libp2p) | 模块化，内置 Kademlia, Relay, AutoNAT, Identify |
| 传输协议 | QUIC (via libp2p-quic) | 基于 UDP，低延迟，抗丢包，天然穿透 NAT |
| 加密协议 | Noise Protocol (XX pattern) | 轻量级，端到端加密，防中间人 |
| 账本/通证 | Custom DAG + State Channels | 极轻量，无需全局共识，支持微支付 |
| 视频编码 | H.265 / HEVC | 高压缩率，节省带宽，硬件加速支持好 |
| UI 框架 | Tauri v2 | Rust 后端 + Web 前端，体积极小 |

## 快速开始

### 前置要求

- Rust 1.70+
- Cargo

### 构建

```bash
# Clone the repository
git clone https://github.com/nexusremote/nexusremote.git
cd nexusremote

# Build
cargo build --release

# Run tests
cargo test
```

### 运行

```bash
# Show version
cargo run -- version

# Run network simulation
cargo run -- simulate

# Test weighted routing
cargo run -- test-routing

# Mine initial tokens (PoW)
cargo run -- mine
```

## 项目结构

```
nexusremote/
├── src/
│   ├── core/              # Core types and utilities
│   │   ├── types.rs       # Fundamental type definitions
│   │   ├── crypto.rs      # Cryptographic utilities
│   │   ├── distance.rs    # Weighted distance calculation
│   │   └── state.rs       # Node state management
│   ├── network/           # P2P networking
│   │   ├── dht.rs         # Distributed Hash Table
│   │   ├── transport.rs   # QUIC transport
│   │   ├── relay.rs       # Relay node functionality
│   │   └── discovery.rs   # Peer discovery
│   ├── wallet/            # Token economics
│   │   ├── wallet.rs      # Wallet implementation
│   │   ├── token.rs       # Token calculations
│   │   ├── channel.rs     # Payment channels
│   │   └── mining.rs      # PoW mining
│   ├── simulator/         # Network simulation
│   │   ├── network.rs     # Network simulator
│   │   └── stats.rs       # Statistics and analysis
│   ├── ui/                # Tauri UI (coming soon)
│   ├── lib.rs             # Library entry point
│   └── bin/
│       └── main.rs        # CLI entry point
├── Cargo.toml
└── README.md
```

## 通证经济模型

### 入场
- 新用户安装 → PoW 挖矿 (得 10 币) 或 担保 (得 5 币)

### 消费
- 发起远控 → 若需中继 → 微支付通道扣费

### 危机
- 余额归零 → 高信誉? → 赊账模式 (继续服务)
- 低信誉? → 降级直连 或 即时任务 (开 5 分钟中继赚币)

### 循环
- 用户闲置时自动开启 中继节点 → 赚取代币 + 提升信誉 → 获得更高权重与透支额度

## 加权路由算法

核心公式：
```
LogicalDistance = XOR(NodeID, TargetID) * (2000 / (Reputation + 1000))
```

这使得高信誉节点在 DHT 路由中拥有更高的优先级，享受更短的路径。

## 开发阶段

### Phase 1 (Current) - 骨架搭建 ✅
- [x] 初始化 Rust 项目
- [x] 配置 libp2p 依赖
- [x] 实现核心类型定义
- [x] 实现加权路由算法
- [x] 实现钱包原型
- [x] 实现网络模拟器
- [x] 编写单元测试

### Phase 2 - 网络集成
- [ ] 集成 libp2p (kad, quic, noise, relay)
- [ ] 实现 DHT 节点发现
- [ ] 实现 NAT 穿透
- [ ] 实现端到端加密

### Phase 3 - 远程控制
- [ ] 实现视频捕获和编码
- [ ] 实现输入注入
- [ ] 实现自适应码率
- [ ] 实现低延迟传输

### Phase 4 - UI and Polish
- [ ] Tauri UI 开发
- [ ] 性能优化
- [ ] 文档完善
- [ ] 安全审计

## 测试

### 运行所有测试

```bash
cargo test
```

### 运行网络模拟

```bash
cargo run -- simulate --nodes 100 --lookups 1000
```

### 测试加权路由

```bash
cargo run -- test-routing
```

## 贡献

欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 联系方式

- GitHub: https://github.com/nexusremote
- Discord: https://discord.gg/nexusremote
- Twitter: @NexusRemote

---

**"人人皆节点，贡献即收益"** 🚀
