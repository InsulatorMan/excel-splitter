#!/bin/bash

# Excel Splitter - Linux 交叉编译 Windows EXE 脚本
# 使用方法: chmod +x Linux一键编译脚本.sh && ./Linux一键编译脚本.sh

set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     Excel Splitter - Linux 交叉编译 Windows EXE              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否以 root 运行
if [ "$EUID" -eq 0 ]; then 
   echo -e "${RED}[警告] 不建议以 root 用户运行${NC}"
fi

# 1. 检查 Rust 安装
echo "[1/6] 检查 Rust 环境..."
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}[错误] Rust 未安装${NC}"
    echo "请先安装 Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "    Rust 版本: $(rustc --version)"
echo "    Cargo 版本: $(cargo --version)"

# 2. 安装 MinGW（如果未安装）
echo
echo "[2/6] 检查 MinGW 交叉编译工具链..."

if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "    MinGW 未安装，尝试自动安装..."
    
    # 检测发行版并安装
    if command -v apt &> /dev/null; then
        # Debian/Ubuntu
        sudo apt update
        sudo apt install -y mingw-w64 mingw-w64-x86-64-dev
    elif command -v dnf &> /dev/null; then
        # Fedora
        sudo dnf install -y mingw64-gcc mingw64-gcc-c++
    elif command -v yum &> /dev/null; then
        # CentOS/RHEL
        sudo yum install -y mingw64-gcc mingw64-gcc-c++
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        sudo pacman -S mingw-w64-gcc
    else
        echo -e "${RED}[错误] 无法自动安装 MinGW，请手动安装${NC}"
        exit 1
    fi
else
    echo "    MinGW 已安装: $(x86_64-w64-mingw32-gcc --version | head -n1)"
fi

# 3. 添加 Rust 目标
echo
echo "[3/6] 添加 Windows 交叉编译目标..."
rustup target add x86_64-pc-windows-gnu
echo -e "    ${GREEN}✓ 目标已添加${NC}"

# 4. 配置 Cargo
echo
echo "[4/6] 配置 Cargo 交叉编译..."

mkdir -p ~/.cargo

if [ ! -f ~/.cargo/config.toml ]; then
    cat > ~/.cargo/config.toml << 'EOF'
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"

[target.i686-pc-windows-gnu]
linker = "i686-w64-mingw32-gcc"
ar = "i686-w64-mingw32-ar"
EOF
    echo "    已创建 ~/.cargo/config.toml"
else
    echo "    ~/.cargo/config.toml 已存在，跳过"
fi

# 5. 编译项目
echo
echo "[5/6] 开始编译 Windows EXE..."
echo "    这可能需要 10-30 分钟，请耐心等待..."
echo

# 进入项目目录
if [ -d "src-tauri" ]; then
    cd src-tauri
elif [ -d "../src-tauri" ]; then
    cd ../src-tauri
fi

# 编译（使用静态链接减少依赖）
RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-pc-windows-gnu

# 检查编译结果
if [ -f "target/x86_64-pc-windows-gnu/release/excel-splitter.exe" ]; then
    echo
    echo -e "    ${GREEN}✓ 编译成功！${NC}"
else
    echo
    echo -e "    ${RED}✗ 编译失败${NC}"
    exit 1
fi

# 6. 完成
echo
echo "[6/6] 编译完成！"
echo
echo "═══════════════════════════════════════════════════════════════"
echo "  输出文件:"
echo "  $(pwd)/target/x86_64-pc-windows-gnu/release/excel-splitter.exe"
echo
echo "  文件信息:"
file "target/x86_64-pc-windows-gnu/release/excel-splitter.exe"
echo
echo "  文件大小:"
ls -lh "target/x86_64-pc-windows-gnu/release/excel-splitter.exe" | awk '{print $5}'
echo
echo "═══════════════════════════════════════════════════════════════"
echo
echo "使用方法:"
echo "  1. 将 EXE 文件复制到 Windows 电脑"
echo "  2. 直接双击运行，无需安装任何依赖"
echo
echo "注意:"
echo "  - Windows 10/11 已内置 WebView2"
echo "  - Windows 7/8 可能需要安装 WebView2 Runtime"
echo

# 询问是否复制到指定目录
read -p "是否复制 EXE 到当前目录? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    cp "target/x86_64-pc-windows-gnu/release/excel-splitter.exe" ../excel-splitter.exe
    echo -e "${GREEN}已复制到: $(pwd)/../excel-splitter.exe${NC}"
fi

echo
echo -e "${GREEN}完成！${NC}"
