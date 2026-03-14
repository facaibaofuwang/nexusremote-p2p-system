# NexusRemote Windows 安装包构建指南

## 📦 概述

NexusRemote Windows安装包让您能够一键安装NexusRemote P2P远程控制系统到Windows桌面。

## 🚀 快速开始

### 方式1: 在Windows上直接构建（推荐）

**要求**:
- Windows 10/11
- Node.js 18+
- npm 9+
- Visual Studio Build Tools（用于Rust编译）
- Rust 1.75+（或使用预构建版本）

**步骤**:

1. **克隆项目**
   ```bash
   git clone https://github.com/facaibaofuwang/nexusremote-p2p-system.git
   cd nexusremote-p2p-system
   ```

2. **运行Windows打包脚本**
   ```cmd
   cd nexusremote-app-client
   build-windows.bat
   ```

3. **等待构建完成**
   - 安装依赖
   - 编译Rust后端
   - 生成NSIS安装程序
   - 创建便携版本

4. **使用生成的安装包**
   - 位置: `nexusremote-app-client/dist/`
   - 安装程序: `NexusRemote-Setup-x.x.x.exe`
   - 便携版本: `NexusRemote-x.x.x-portable.exe`

### 方式2: 跨平台交叉编译（Linux/macOS → Windows）

**要求**:
- Linux或macOS开发环境
- Rust 1.75+
- Node.js 18+
- Docker（可选，用于简化流程）

**步骤**:

1. **克隆项目**
   ```bash
   git clone https://github.com/facaibaofuwang/nexusremote-p2p-system.git
   cd nexusremote-p2p-system
   ```

2. **运行交叉编译脚本**
   ```bash
   cd nexusremote-app-client
   chmod +x build-windows-cross-platform.sh
   ./build-windows-cross-platform.sh
   ```

3. **脚本会自动完成以下操作**:
   - 添加Windows编译目标
   - 交叉编译Rust后端为Windows
   - 安装Electron依赖
   - 生成Windows安装包

### 方式3: 使用CI/CD自动构建

在GitHub上配置自动化构建（推荐用于生产发布）

**GitHub Actions示例**:
```yaml
name: Build Windows Installer

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: x86_64-pc-windows-msvc
          override: true
          
      - name: Install dependencies
        run: npm install
        working-directory: ./nexusremote-app-client
        
      - name: Build Rust backend
        run: cargo build --release --target x86_64-pc-windows-msvc
        working-directory: ./nexusremote
        
      - name: Build Electron app
        run: npm run build:win
        working-directory: ./nexusremote-app-client
        
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: windows-installer
          path: nexusremote-app-client/dist/*.exe
```

## 📦 生成的包类型

### 1. NSIS安装程序（.exe）

**文件名**: `NexusRemote-Setup-x.x.x.exe`

**特性**:
- ✅ 专业的安装向导
- ✅ 自定义安装路径选择
- ✅ 桌面快捷方式创建
- ✅ 开始菜单项注册
- ✅ Windows服务安装
- ✅ 系统托盘自动启动
- ✅ 自动更新支持
- ✅ 完整卸载支持

**用户体验**:
```
┌─────────────────────────────────────────┐
│  NexusRemote 安装向导        │
├─────────────────────────────────────────┤
│                                   │
│  ☑ 主程序                     │
│  ☑ 后台服务（推荐）         │
│  ☑ 系统托盘（推荐）         │
│  ☑ 文档                       │
│                                   │
│  安装位置：                     │
│  C:\Program Files\NexusRemote  │
│                                   │
│    [上一步]   [取消]            │
└─────────────────────────────────────────┘
```

### 2. 便携版本（.zip）

**文件名**: `NexusRemote-x.x.x-portable.zip`

**特性**:
- ✅ 无需安装，解压即用
- ✅ 不写注册表
- ✅ 可在U盘上运行
- ✅ 适合临时使用
- ✅ 更新不残留文件

**用户体验**:
```bash
# 解压
unzip NexusRemote-1.0.0-portable.zip -d NexusRemote

# 运行
cd NexusRemote
.\nexusremote.exe
```

## 🔧 自定义配置

### 修改应用程序信息

编辑 `nexusremote-app-client/package.json`:
```json
{
  "name": "nexusremote-client",
  "version": "1.0.0",
  "description": "NexusRemote 桌面客户端应用程序",
  "author": "NexusRemote Team",
  "license": "MIT"
}
```

### 修改打包配置

编辑 `nexusremote-app-client/electron-builder.json`:
```json
{
  "appId": "com.nexusremote.client",
  "productName": "NexusRemote",
  "icon": "assets/icons/icon.png",
  "win": {
    "icon": "assets/icons/icon.ico",
    "target": ["nsis", "portable"]
  }
}
```

### 自定义NSIS安装脚本

编辑 `nexusremote-app-client/build/installer.nsh` 来自定义：
- 安装组件
- 许可协议
- 卸载逻辑
- 自定义UI元素

## 🎨 添加图标和资源

### 准备图标文件

需要准备以下尺寸的图标：

| 类型 | 尺寸 | 文件名 | 用途 |
|------|------|--------|------|
| ICO | 256x256 | icon.ico | Windows图标 |
| PNG | 512x512 | icon.png | Linux图标 |
| BMP | 150x50 | bitmap.bmp | 安装界面位图 |

### 图标生成工具

1. **在线生成器**
   - https://icoconvert.com/
   - https://www.favicon-generator.org/

2. **命令行工具**
   - ImageMagick: `convert icon.png -resize 256x256 icon.ico`
   - GIMP: 导出为ICO格式

3. **资源替换**
   ```bash
   # 复制备好的图标
   cp your-icon.png nexusremote-app-client/assets/icons/icon.png
   cp your-icon.ico nexusremote-app-client/assets/icons/icon.ico
   cp your-banner.bmp nexusremote-app-client/assets/icons/bitmap.bmp
   ```

## 📝 分发和发布

### 本地分发

```bash
# 构建安装包
cd nexusremote-app-client
npm run build:win

# 复制到分发目录
cp dist/*.exe ~/Desktop/NexusRemote-Installer/

# 或创建ZIP
zip -r nexusremote-installer.zip dist/*.exe
```

### GitHub自动发布

1. **创建GitHub发布**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **GitHub Actions自动构建**
   - 自动生成Windows安装包
   - 上传到Release资产

3. **用户下载安装**
   - 访问: https://github.com/facaibaofuwang/nexusremote-p2p-system/releases/latest
   - 下载: `NexusRemote-Setup-x.x.x.exe`

### 数字签名（推荐）

为Windows安装包添加数字签名以避免安全警告：

```bash
# 使用signtool进行签名
signtool sign /f dist/NexusRemote-Setup-x.x.x.exe \
  /t "NexusRemote P2P Remote Control" \
  /d "NexusRemote Team"

# 或使用其他工具
# - Authenticode
# - SignTool
# - OSS Code Signing Service
```

## 🔍 故障排除

### 常见问题

**问题1: "未找到node命令"**
```cmd
# 解决：添加Node.js到PATH或重新安装
node --version
```

**问题2: "Rust编译失败"**
```bash
# 解决：安装Visual Studio Build Tools
# 或使用预构建版本
cargo build --release --skip-compile
```

**问题3: "electron-builder找不到"**
```bash
# 解决：全局安装electron-builder
npm install -g electron-builder
# 或确保在devDependencies中
```

**问题4: "图标缺失警告"**
```bash
# 解决：提供至少一个占位符图标
echo. > "assets/icons/icon.ico"
```

### 调试构建

启用详细输出来调试问题：
```bash
# 调试模式
DEBUG=electron-builder npm run build:win

# 或使用VS详细输出
npm run build:win -- --verbose
```

## 📊 构建时间估算

| 环境 | 依赖安装 | Rust编译 | 打包 | 总计 |
|------|----------|----------|------|------|
| **Windows本地** | 3-5分钟 | 10-15分钟 | 2-3分钟 | 15-25分钟 |
| **Linux跨编译** | 3-5分钟 | 15-25分钟 | 2-3分钟 | 20-35分钟 |
| **CI/CD** | 1-2分钟 | 8-12分钟 | 1-2分钟 | 10-20分钟 |

## ✅ 质量检查清单

在发布前验证：

- [ ] 所有依赖正确安装
- [ ] Rust编译无错误和警告
- [ ] 图标资源正确包含
- [ ] NSIS安装脚本语法正确
- [ ] 安装程序在干净的Windows上测试
- [ ] 便携版本可以正常运行
- [ ] 数字签名已添加（推荐）
- [ ] 文件大小合理（<100MB）
- [ ] 版本号正确更新
- [ ] Release Notes已准备

## 🎯 发布流程

1. **更新版本号**
   ```bash
   # 更新package.json
   npm version patch  # 1.0.0 -> 1.0.1
   
   # 或手动编辑
   "version": "1.1.0"
   ```

2. **提交更改**
   ```bash
   git add .
   git commit -m "Release v1.0.0"
   ```

3. **创建标签**
   ```bash
   git tag -a v1.0.0 -m "NexusRemote v1.0.0"
   git push origin v1.0.0
   ```

4. **等待CI/CD构建完成**
   - GitHub Actions自动运行
   - 检查构建状态
   - 等待安装包生成

5. **验证Release**
   - 下载生成的安装包
   - 在干净的Windows上测试安装
   - 验证所有功能正常

6. **发布公告**
   - 更新CHANGELOG.md
   - 发布博客文章
   - 通知社区

---

**NexusRemote Windows安装包让用户能够一键安装到Windows桌面！** 🚀
