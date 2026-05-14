# Excel Splitter - 高性能Excel拆分工具

使用 Rust + Tauri 构建的高性能 Excel 拆分工具，支持格式保留和现代化 GUI。

## 特点

- ✅ **超小体积**：仅 5-8 MB（含 GUI）
- ✅ **原生性能**：Rust 后端，极速处理
- ✅ **现代界面**：美观的 Web 技术 GUI
- ✅ **跨平台**：Windows、macOS、Linux
- ✅ **零依赖**：单文件可执行

## 快速开始

### 下载预编译版本

访问 [GitHub Releases](https://github.com/YOUR_USERNAME/excel-splitter/releases) 下载对应平台的安装包。

### 从源码编译

#### 1. 安装依赖

**Windows:**
- 安装 Rust: https://win.rustup.rs/x86_64
- 安装 Visual Studio 2022 或 Build Tools

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
xcode-select --install
```

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install libwebkit2gtk-4.0-dev build-essential libssl-dev libgtk-3-dev
```

#### 2. 克隆项目

```bash
git clone https://github.com/YOUR_USERNAME/excel-splitter.git
cd excel-splitter
```

#### 3. 编译

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 编译发布版本
cd src-tauri
cargo tauri build
```

## 使用说明

### 1. 选择文件
- 点击"浏览..."按钮选择 Excel 文件

### 2. 选择工作表
- **拆分 Sheet**：勾选需要按列拆分的工作表
- **公共 Sheet**：勾选需要完整复制到所有输出文件的工作表

### 3. 配置拆分选项
- **表头行号**：数据表头所在的行（从 1 开始）
- **拆分依据列**：选择用于分组的列名
- **空值处理**：归为"空值"组 / 跳过 / 停止
- **文件名模板**：使用 `{base}` 和 `{group}` 占位符

### 4. 预览数据
- 自动显示前 20 行数据

### 5. 开始拆分
- 点击"🚀 开始拆分"按钮
- 等待处理完成

## 项目结构

```
rust_excel_splitter/
├── src-tauri/              # Tauri 后端代码
│   ├── Cargo.toml          # Rust 依赖配置
│   ├── tauri.conf.json     # Tauri 配置
│   ├── build.rs            # 构建脚本
│   └── src/
│       ├── main.rs         # 主程序
│       └── format.rs       # 格式处理
├── ui/
│   └── index.html          # GUI 界面
├── .github/workflows/
│   └── build.yml           # GitHub Actions 配置
└── README.md               # 本文件
```

## 技术栈

- **后端**：Rust + Tauri
- **前端**：HTML + CSS + JavaScript
- **Excel 读取**：calamine
- **Excel 写入**：rust_xlsxwriter

## 许可证

MIT License
