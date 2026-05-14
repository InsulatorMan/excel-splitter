# Linux 交叉编译 Windows EXE 指南

## 概述

在 Linux 服务器上编译 Windows 可执行文件（EXE），无需 Windows 环境。

## 方案对比

| 方案 | 复杂度 | 成功率 | 推荐度 |
|------|--------|--------|--------|
| **MinGW 交叉编译** | 中 | 高 | ⭐⭐⭐⭐⭐ |
| **Cross 工具** | 低 | 高 | ⭐⭐⭐⭐⭐ |
| **Docker 容器** | 低 | 高 | ⭐⭐⭐⭐ |
| **GitHub Actions** | 低 | 最高 | ⭐⭐⭐⭐⭐ |

---

## 方案 1: MinGW 交叉编译（推荐）

### 1. 安装 MinGW 工具链

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y mingw-w64
```

**CentOS/RHEL/Fedora:**
```bash
# Fedora
sudo dnf install mingw64-gcc mingw64-gcc-c++

# CentOS/RHEL
sudo yum install mingw64-gcc mingw64-gcc-c++
```

**Arch Linux:**
```bash
sudo pacman -S mingw-w64-gcc
```

### 2. 添加 Rust 交叉编译目标

```bash
# 添加 Windows 64位目标
rustup target add x86_64-pc-windows-gnu

# 可选：添加 Windows 32位目标
rustup target add i686-pc-windows-gnu
```

### 3. 配置 Cargo

创建/编辑 `~/.cargo/config.toml`：

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[target.i686-pc-windows-gnu]
linker = "i686-w64-mingw32-gcc"
ar = "i686-w64-mingw32-ar"
```

### 4. 安装 Windows 依赖库

Tauri 需要 Windows 特定的库，下载并设置：

```bash
# 创建工作目录
mkdir -p ~/windows-deps
cd ~/windows-deps

# 下载 WebView2 运行时（可选，用于运行时）
# 下载 Windows API 库
sudo apt install mingw-w64-x86-64-dev

# 对于 Tauri，需要额外的 Windows 库
# 下载并解压 Windows SDK 部分文件
```

### 5. 修改项目配置

编辑 `src-tauri/Cargo.toml`，确保没有平台特定依赖：

```toml
[dependencies]
# 确保所有依赖都支持跨平台
tauri = { version = "1.6", features = ["shell-open", "dialog-open", "dialog-save"] }
calamine = "0.24"
rust_xlsxwriter = "0.70"
anyhow = "1.0"
log = "0.4"
env_logger = "0.11"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
walkdir = "2.5"
```

### 6. 编译

```bash
cd rust_excel_splitter/src-tauri

# 编译 Windows 版本
cargo build --release --target x86_64-pc-windows-gnu
```

**输出位置：**
```
target/x86_64-pc-windows-gnu/release/excel-splitter.exe
```

---

## 方案 2: 使用 Cross 工具（更简单）

### 1. 安装 Cross

```bash
cargo install cross
```

### 2. 使用 Docker 交叉编译

```bash
cd rust_excel_splitter/src-tauri

# 使用 cross 编译（自动使用 Docker 容器）
cross build --release --target x86_64-pc-windows-gnu
```

**Cross 会自动：**
- 下载配置好的 Docker 镜像
- 包含所有必要的依赖
- 编译 Windows 可执行文件

---

## 方案 3: 使用 Docker 容器

### 1. 创建 Dockerfile

```dockerfile
FROM rust:latest

# 安装 MinGW
RUN apt-get update && apt-get install -y \
    mingw-w64 \
    mingw-w64-x86-64-dev \
    && rm -rf /var/lib/apt/lists/*

# 添加 Windows 目标
RUN rustup target add x86_64-pc-windows-gnu

# 配置 Cargo
RUN mkdir -p /root/.cargo
RUN echo '[target.x86_64-pc-windows-gnu]\nlinker = "x86_64-w64-mingw32-gcc"\nar = "x86_64-w64-mingw32-ar"' > /root/.cargo/config.toml

WORKDIR /app
```

### 2. 构建并运行容器

```bash
# 构建镜像
docker build -t rust-windows-builder .

# 运行编译
docker run -v $(pwd):/app rust-windows-builder \
    cargo build --release --target x86_64-pc-windows-gnu
```

---

## 方案 4: GitHub Actions（最简单）

创建 `.github/workflows/build.yml`：

```yaml
name: Build Windows EXE

on:
  push:
    branches: [ main ]
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: dtolnay/rust-action@stable
    
    - name: Install Tauri CLI
      run: cargo install tauri-cli
    
    - name: Build Windows EXE
      run: cargo tauri build
    
    - name: Upload EXE
      uses: actions/upload-artifact@v4
      with:
        name: excel-splitter-windows
        path: src-tauri/target/release/excel-splitter.exe
    
    - name: Upload MSI
      uses: actions/upload-artifact@v4
      with:
        name: excel-splitter-msi
        path: src-tauri/target/release/bundle/msi/*.msi
```

---

## 常见问题

### Q: 编译失败，提示找不到 `windows.h`

**A:**
```bash
# 安装完整的 MinGW 开发库
sudo apt install mingw-w64-x86-64-dev

# 或者
sudo apt install mingw-w64-tools
```

### Q: 链接错误，缺少 Windows 库

**A:**
```bash
# 下载 Windows API 库
wget https://github.com/mirror/mingw-w64/archive/master.zip
unzip master.zip

# 或者使用 vcpkg
```

### Q: Tauri 特定错误

**A:** Tauri 在 Linux 上交叉编译 Windows 可能有额外依赖。建议：
1. 使用 GitHub Actions（在真实 Windows 环境编译）
2. 或使用 Docker + Windows 容器

### Q: 编译后的 EXE 在 Windows 上运行报错

**A:**
1. 确保目标 Windows 系统有 WebView2 运行时
2. 可能需要静态链接运行时库
3. 尝试添加 `-C target-feature=+crt-static`

---

## 推荐的编译命令

### 基础编译

```bash
# 进入项目目录
cd rust_excel_splitter/src-tauri

# 编译 Windows 64位版本
cargo build --release --target x86_64-pc-windows-gnu

# 编译 Windows 32位版本
cargo build --release --target i686-pc-windows-gnu
```

### 静态链接（推荐）

```bash
# 静态链接 C 运行时，减少依赖
RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-pc-windows-gnu
```

### 使用 Cross

```bash
# 安装 cross
cargo install cross

# 编译（自动使用 Docker）
cross build --release --target x86_64-pc-windows-gnu
```

---

## 验证编译结果

```bash
# 检查文件类型
file target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 输出应该是：
# PE32+ executable (GUI) x86-64, for MS Windows

# 检查依赖
dependencies target/x86_64-pc-windows-gnu/release/excel-splitter.exe
# 或
objdump -p target/x86_64-pc-windows-gnu/release/excel-splitter.exe | grep DLL
```

---

## 文件传输到 Windows

```bash
# 使用 scp
scp target/x86_64-pc-windows-gnu/release/excel-splitter.exe user@windows-host:C:/

# 或使用 rsync
rsync -avz target/x86_64-pc-windows-gnu/release/excel-splitter.exe user@windows-host:/c:/

# 或下载到本地
sz target/x86_64-pc-windows-gnu/release/excel-splitter.exe
```

---

## 总结

| 方法 | 命令 | 难度 |
|------|------|------|
| MinGW | `cargo build --target x86_64-pc-windows-gnu` | 中 |
| Cross | `cross build --target x86_64-pc-windows-gnu` | 低 |
| Docker | `docker run ... cargo build` | 低 |
| GitHub Actions | 推送代码自动编译 | 最低 |

**推荐：** 如果 Linux 服务器配置正确，使用 MinGW 交叉编译；如果配置复杂，使用 GitHub Actions 最简单。
