# NexusRemote开发工作流

## 环境设置
```bash
# 激活Rust环境
source ~/.rust_env

# 验证环境
rustc --version
cargo --version
```

## 日常开发
```bash
# 1. 检查代码
cargo check

# 2. 运行测试
cargo test

# 3. 构建项目
cargo build

# 4. 运行CLI
cargo run -- help
cargo run -- simulate
cargo run -- test-routing
```

## 代码质量
```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 文档生成
cargo doc --open
```

## 发布构建
```bash
# 发布构建
cargo build --release

# 运行性能测试
cargo bench
```

## 模块开发
1. **core模块**: 基础类型和算法
2. **network模块**: P2P网络功能
3. **wallet模块**: 通证经济系统
4. **simulator模块**: 网络模拟器
5. **ui模块**: 用户界面
```

## 故障排除
1. **依赖问题**: `cargo update`
2. **编译错误**: `cargo clean && cargo build`
3. **测试失败**: `cargo test --verbose`
4. **性能问题**: `cargo build --release`
