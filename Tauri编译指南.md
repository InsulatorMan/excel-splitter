# Tauri GUI 版本编译指南

## 项目结构

```
rust_excel_splitter/
├── Cargo.toml              # Rust 项目配置
├── tauri.conf.json         # Tauri 配置文件
├── build.rs                # 构建脚本
├── src/
│   ├── main.rs             # Tauri 主程序
│   └── format.rs           # 格式处理模块
└── ui/
    └── index.html          # GUI 界面
```

## 系统要求

### Windows
- Windows 7 或更高版本
- WebView2 运行时（Windows 10/11 已内置）
- Visual Studio 2022 或 Build Tools（需要 C++ 工具链）

### macOS
- macOS 10.13 或更高版本
- Xcode Command Line Tools

### Linux
- 各种现代 Linux 发行版
- 需要安装 WebKit2GTK

## 编译步骤

### 1. 安装 Rust

```bash
# Windows: 下载并运行
# https://win.rustup.rs/x86_64

# macOS/Linux:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 2. 安装系统依赖

#### Windows

**选项 A: 安装 Visual Studio 2022（推荐）**
1. 下载 https://visualstudio.microsoft.com/downloads/
2. 安装 "使用 C++ 的桌面开发" 工作负载
3. 包含 Windows 10/11 SDK

**选项 B: 安装 Build Tools（轻量）**
1. 下载 https://aka.ms/vs/17/release/vs_BuildTools.exe
2. 安装 "使用 C++ 的桌面开发"

**验证 WebView2**
- Windows 10/11: 已内置
- Windows 7/8: 下载 https://developer.microsoft.com/en-us/microsoft-edge/webview2/

#### macOS

```bash
# 安装 Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)

```bash
# 安装依赖
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Linux (Fedora)

```bash
sudo dnf install webkit2gtk3-devel \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

### 3. 安装 Tauri CLI

```bash
cargo install tauri-cli
```

### 4. 克隆/准备项目

```bash
# 如果是从 Git 仓库克隆
git clone <repository-url>
cd rust_excel_splitter

# 或者解压项目压缩包
cd rust_excel_splitter
```

### 5. 编译开发版本

```bash
# 使用 Tauri CLI
cargo tauri dev

# 这会启动开发服务器，自动打开 GUI 窗口
```

### 6. 编译发布版本

```bash
# 使用 Tauri CLI 构建发布版本
cargo tauri build

# 或者使用 cargo 直接构建
cargo build --release
```

**输出位置：**
- Windows: `src-tauri/target/release/excel-splitter.exe`
- macOS: `src-tauri/target/release/excel-splitter`
- Linux: `src-tauri/target/release/excel-splitter`

### 7. 打包分发

```bash
# Tauri 会自动打包
cargo tauri build

# 输出目录：
# Windows: src-tauri/target/release/bundle/msi/*.msi
# macOS: src-tauri/target/release/bundle/dmg/*.dmg
# Linux: src-tauri/target/release/bundle/deb/*.deb
```

## 编译优化

### 减小体积

编辑 `Cargo.toml`：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### UPX 压缩（可选）

```bash
# 下载 UPX: https://github.com/upx/upx/releases

# 压缩可执行文件
upx --best target/release/excel-splitter.exe
```

## 常见问题

### Q: 编译失败，提示找不到 WebView2

**A:** 
- Windows 10/11: 更新系统或安装 Edge
- Windows 7/8: 下载安装 WebView2 Runtime

### Q: 编译失败，提示缺少 MSVC

**A:**
- 安装 Visual Studio 或 Build Tools
- 确保安装了 "使用 C++ 的桌面开发" 工作负载

### Q: macOS 编译失败

**A:**
```bash
# 确保 Xcode Command Line Tools 已安装
xcode-select --install

# 如果已安装但有问题，重置
sudo xcode-select --reset
```

### Q: Linux 编译失败，缺少 webkit2gtk

**A:**
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk3-devel
```

### Q: 编译后的文件很大

**A:**
- 使用 `strip` 移除调试符号
- 使用 UPX 压缩
- 检查是否启用了 LTO

### Q: 运行时提示缺少 DLL

**A:**
- Windows: 确保 WebView2 Runtime 已安装
- Linux: 确保 WebKit2GTK 已安装
- 使用静态链接（如果支持）

## 跨平台编译

### 从 macOS 编译 Windows 版本

```bash
# 安装交叉编译工具
rustup target add x86_64-pc-windows-msvc

# 需要安装 mingw-w64
brew install mingw-w64

# 编译（可能有限制）
cargo build --release --target x86_64-pc-windows-gnu
```

### 从 Linux 编译 Windows 版本

```bash
# 安装 mingw
sudo apt install mingw-w64

# 添加目标
rustup target add x86_64-pc-windows-gnu

# 编译
cargo build --release --target x86_64-pc-windows-gnu
```

## 自动化编译（GitHub Actions）

### 配置 GitHub Actions

创建 `.github/workflows/build.yml`：

```yaml
name: Build Tauri App

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
      
      - name: Install Tauri CLI
        run: cargo install tauri-cli
      
      - name: Build Tauri App
        run: cargo tauri build
      
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: windows-installer
          path: src-tauri/target/release/bundle/msi/*.msi

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
      
      - name: Install Tauri CLI
        run: cargo install tauri-cli
      
      - name: Build Tauri App
        run: cargo tauri build
      
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: macos-installer
          path: src-tauri/target/release/bundle/dmg/*.dmg
```

## 性能优化

### 启动速度优化

1. **使用 Release 模式编译**
   ```bash
   cargo tauri build --release
   ```

2. **启用 LTO**
   ```toml
   [profile.release]
   lto = true
   ```

3. **代码分割**
   ```toml
   [profile.release]
   codegen-units = 1
   ```

### 内存优化

1. **使用 jemalloc（可选）**
   ```toml
   [dependencies]
   jemallocator = "0.5"
   ```

2. **优化数据结构**
   - 使用 `Vec` 而不是 `HashMap` 当顺序访问时
   - 预分配容量避免重新分配

## 调试技巧

### 开发模式调试

```bash
# 启动开发服务器，带热重载
cargo tauri dev

# 查看详细日志
RUST_LOG=debug cargo tauri dev
```

### 前端调试

1. 右键点击应用窗口
2. 选择 "检查" 或 "Inspect"
3. 使用 Chrome DevTools 调试

### 后端调试

```bash
# 打印日志
RUST_LOG=info cargo tauri dev

# 详细日志
RUST_LOG=debug cargo tauri dev
```

## 分发说明

### Windows

**安装包（推荐）**
- MSI 安装程序：`*.msi`
- 自动创建快捷方式
- 支持卸载

**便携版**
- 单个 EXE 文件
- 无需安装
- 直接运行

### macOS

**DMG 安装包**
- 拖拽安装
- 自动签名（需要 Apple Developer 账号）

### Linux

**DEB 包（Debian/Ubuntu）**
```bash
sudo dpkg -i excel-splitter_1.0.0_amd64.deb
```

**AppImage**
- 无需安装
- 直接运行

## 版本更新

### 自动更新（可选）

配置 Tauri Updater：

```json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": ["https://your-server.com/updates"],
      "dialog": true
    }
  }
}
```

---

**编译完成后，您将获得一个独立的 GUI 应用程序，可以在没有 Rust 环境的电脑上运行！**
