# Linux 编译 Windows EXE - 命令速查

## 快速开始（复制粘贴即可）

### 1. 安装依赖（一次性）

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y mingw-w64 mingw-w64-x86-64-dev

# Fedora
sudo dnf install -y mingw64-gcc mingw64-gcc-c++

# CentOS/RHEL
sudo yum install -y mingw64-gcc mingw64-gcc-c++

# Arch Linux
sudo pacman -S mingw-w64-gcc
```

### 2. 添加 Rust 目标

```bash
rustup target add x86_64-pc-windows-gnu
```

### 3. 配置 Cargo

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
EOF
```

### 4. 编译

```bash
cd rust_excel_splitter/src-tauri

# 基础编译
cargo build --release --target x86_64-pc-windows-gnu

# 静态链接（推荐，减少依赖）
RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-pc-windows-gnu
```

### 5. 获取结果

```bash
# 文件位置
ls -lh target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 复制到方便的位置
cp target/x86_64-pc-windows-gnu/release/excel-splitter.exe ~/excel-splitter.exe
```

---

## 使用一键脚本

```bash
# 给脚本执行权限
chmod +x Linux一键编译脚本.sh

# 运行
./Linux一键编译脚本.sh
```

脚本会自动：
1. 检查 Rust 环境
2. 安装 MinGW（如果未安装）
3. 添加 Windows 目标
4. 配置 Cargo
5. 编译项目
6. 显示结果

---

## 常见问题速查

### 编译失败：找不到 `x86_64-w64-mingw32-gcc`

```bash
# 检查是否安装
which x86_64-w64-mingw32-gcc

# 重新安装
sudo apt install --reinstall mingw-w64
```

### 编译失败：链接错误

```bash
# 安装完整的 MinGW 库
sudo apt install mingw-w64-x86-64-dev

# 或者使用动态链接（文件更小，但需要 DLL）
cargo build --release --target x86_64-pc-windows-gnu
```

### 文件太大

```bash
# 使用 strip 移除符号
strip target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 使用 UPX 压缩（如果可用）
upx --best target/x86_64-pc-windows-gnu/release/excel-splitter.exe
```

### 运行时缺少 DLL

```bash
# 使用静态链接重新编译
RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-pc-windows-gnu
```

---

## 高级选项

### 编译 32位 Windows 版本

```bash
# 添加 32位目标
rustup target add i686-pc-windows-gnu

# 编译
cargo build --release --target i686-pc-windows-gnu
```

### 使用 Docker 编译

```bash
# 使用预配置的 Docker 镜像
docker run --rm -v $(pwd):/app \
    -w /app/src-tauri \
    rust:latest \
    bash -c "
        apt-get update && apt-get install -y mingw-w64 && \
        rustup target add x86_64-pc-windows-gnu && \
        cargo build --release --target x86_64-pc-windows-gnu
    "
```

### 使用 Cross 工具

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

# 预期输出：
# PE32+ executable (GUI) x86-64, for MS Windows

# 检查依赖的 DLL
objdump -p target/x86_64-pc-windows-gnu/release/excel-splitter.exe | grep "DLL Name"

# 静态链接应该只显示：
# KERNEL32.dll
# ntdll.dll
# 等系统 DLL
```

---

## 传输到 Windows

```bash
# 方法 1: SCP
scp target/x86_64-pc-windows-gnu/release/excel-splitter.exe \
    user@windows-host:/c:/Users/Public/Downloads/

# 方法 2: 下载到本地（如果有 sz 命令）
sz target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 方法 3: 复制到 Web 目录
sudo cp target/x86_64-pc-windows-gnu/release/excel-splitter.exe \
    /var/www/html/downloads/

# 方法 4: 使用 Python 简易 HTTP 服务器
cd target/x86_64-pc-windows-gnu/release/
python3 -m http.server 8080
# 然后在 Windows 浏览器访问：http://linux-server-ip:8080/
```

---

## 完整示例

```bash
# 1. 进入项目目录
cd ~/rust_excel_splitter

# 2. 运行一键编译
chmod +x Linux一键编译脚本.sh
./Linux一键编译脚本.sh

# 3. 等待编译完成（10-30分钟）

# 4. 获取结果
ls -lh src-tauri/target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 5. 复制到 Windows（根据你的环境选择）
scp src-tauri/target/x86_64-pc-windows-gnu/release/excel-splitter.exe \
    admin@192.168.1.100:C:/Users/admin/Desktop/
```

---

## 性能优化编译

```bash
# 最高优化 + 静态链接 + strip
cd src-tauri

RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-pc-windows-gnu

# 移除符号表
strip target/x86_64-pc-windows-gnu/release/excel-splitter.exe

# 查看最终大小
ls -lh target/x86_64-pc-windows-gnu/release/excel-splitter.exe
```

---

## 故障排除

### 问题 1: `error: linker x86_64-w64-mingw32-gcc not found`

解决：
```bash
sudo apt install mingw-w64
which x86_64-w64-mingw32-gcc
```

### 问题 2: `error: could not find native static library`

解决：
```bash
# 安装完整的开发库
sudo apt install mingw-w64-x86-64-dev

# 或者使用动态链接
cargo build --release --target x86_64-pc-windows-gnu
```

### 问题 3: Tauri 特定的编译错误

解决：
```bash
# Tauri 在 Linux 交叉编译可能有额外依赖
# 建议简化：不使用 Tauri，使用纯命令行版本

# 或者使用 GitHub Actions 在真实 Windows 环境编译
```

---

**提示：** 如果交叉编译遇到问题，最简单的方法是使用 GitHub Actions，在真实的 Windows 环境中编译。
