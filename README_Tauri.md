# Excel Splitter - Tauri GUI 版本

使用 Tauri 框架构建的 GUI 版本，具有原生性能和极小的体积。

## 特点

- ✅ **超小体积**：仅 5-8 MB（含 GUI）
- ✅ **原生性能**：Rust 后端，极速处理
- ✅ **现代界面**：美观的 Web 技术 GUI
- ✅ **跨平台**：Windows、macOS、Linux
- ✅ **零依赖**：单文件可执行

## 界面预览

```
┌─────────────────────────────────────────────────────────────┐
│  📊 Excel Splitter - 高性能Excel拆分工具                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  📁 选择文件                                                  │
│  ┌──────────────────────────────────────┐  ┌──────────┐    │
│  │ C:\data\sales.xlsx                   │  │ 浏览...  │    │
│  └──────────────────────────────────────┘  └──────────┘    │
│                                                             │
│  📋 选择工作表                                                │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ ☑ Sheet1    │ │ ☐ Sheet2    │ │ ☐ Sheet3    │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
│                                                             │
│  ⚙️ 拆分配置                                                  │
│  表头行号: [1]    拆分依据列: [▼ 区域  ]                     │
│  空值处理: [▼ 归为"空值"组]   文件名: [{base}_{group}.xlsx]  │
│                                                             │
│  👁️ 数据预览                                                  │
│  ┌──────┬────────┬────────┬────────┐                       │
│  │ 区域  │ 销售额  │ 成本   │ 利润   │                       │
│  ├──────┼────────┼────────┼────────┤                       │
│  │ 华东  │ 100000 │ 60000  │ 40000  │                       │
│  │ 华北  │ 150000 │ 90000  │ 60000  │                       │
│  └──────┴────────┴────────┴────────┘                       │
│                                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    🚀 开始拆分                           ││
│  └─────────────────────────────────────────────────────────┘│
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 快速开始

### 方式 1：下载预编译版本

1. 访问 GitHub Releases 页面
2. 下载对应平台的安装包
3. 安装并运行

### 方式 2：从源码编译

#### 1. 安装依赖

**Windows:**
```bash
# 安装 Rust
# https://win.rustup.rs/x86_64

# 安装 Visual Studio 2022 或 Build Tools
# https://visualstudio.microsoft.com/downloads/
```

**macOS:**
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装系统依赖
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

#### 2. 克隆项目

```bash
git clone <repository-url>
cd rust_excel_splitter
```

#### 3. 编译

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 开发模式（带热重载）
cargo tauri dev

# 发布版本
cargo tauri build
```

#### 4. 运行

```bash
# Windows
target\release\excel-splitter.exe

# macOS/Linux
./target/release/excel-splitter
```

## 使用说明

### 1. 选择文件

- 点击"浏览..."按钮
- 选择要拆分的 Excel 文件（.xlsx 格式）

### 2. 选择工作表

- **拆分 Sheet**：勾选需要按列拆分的工作表
- **公共 Sheet**：勾选需要完整复制到所有输出文件的工作表

### 3. 配置拆分选项

- **表头行号**：数据表头所在的行（从 1 开始）
- **拆分依据列**：选择用于分组的列名
- **空值处理**：
  - 归为"空值"组：将空值归为一组
  - 跳过空值行：忽略空值行
  - 遇到空值停止：报错并停止
- **文件名模板**：使用 `{base}` 和 `{group}` 占位符

### 4. 预览数据

- 自动显示前 20 行数据
- 确认表头和数据正确

### 5. 开始拆分

- 点击"🚀 开始拆分"按钮
- 等待处理完成
- 查看输出文件

## 功能特性

### 核心功能
- ✅ 多 Sheet 同时拆分
- ✅ 按列分组
- ✅ 公共 Sheet 复制
- ✅ 表头行配置
- ✅ 空值处理
- ✅ 文件名模板
- ✅ 数据预览
- ✅ 进度显示

### 界面特性
- ✅ 现代化 UI 设计
- ✅ 实时数据预览
- ✅ 进度条显示
- ✅ 结果展示
- ✅ 一键打开输出目录

## 编译配置

### 优化选项

编辑 `Cargo.toml`：

```toml
[profile.release]
opt-level = 3        # 最高优化
lto = true           # 链接时优化
codegen-units = 1    # 单代码生成单元
panic = "abort"      # 简化 panic
strip = true         # 移除符号
```

### 平台特定配置

#### Windows
- 使用 MSVC 工具链
- 支持 WebView2
- 可生成 MSI 安装包

#### macOS
- 使用 Clang 工具链
- 支持原生菜单
- 可生成 DMG 安装包

#### Linux
- 使用 GCC 工具链
- 支持 WebKit2GTK
- 可生成 DEB 和 AppImage

## 常见问题

### Q: 编译失败，提示缺少 WebView2

**A:** 
- Windows 10/11：已内置，更新系统即可
- Windows 7/8：下载安装 [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Q: macOS 编译失败

**A:**
```bash
# 确保 Xcode Command Line Tools 已安装
xcode-select --install

# 如果已安装但有问题
sudo xcode-select --reset
```

### Q: Linux 编译失败，缺少依赖

**A:**
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev

# Fedora
sudo dnf install webkit2gtk3-devel gtk3-devel libappindicator-gtk3-devel
```

### Q: 如何减小编译后的体积？

**A:**
1. 使用 Release 模式编译
2. 启用 LTO 和 strip
3. 使用 UPX 压缩（可选）

### Q: 如何调试？

**A:**
```bash
# 开发模式（带热重载和调试工具）
cargo tauri dev

# 查看详细日志
RUST_LOG=debug cargo tauri dev
```

## 开发计划

- [x] 基本 GUI 界面
- [x] 文件选择
- [x] Sheet 选择
- [x] 配置面板
- [x] 数据预览
- [x] 进度显示
- [x] 结果展示
- [ ] 完整格式保留
- [ ] 多线程处理
- [ ] 配置文件支持
- [ ] 自动更新

## 技术栈

- **后端**：Rust + Tauri
- **前端**：HTML + CSS + JavaScript
- **Excel 读取**：calamine
- **Excel 写入**：rust_xlsxwriter
- **构建工具**：Cargo + Tauri CLI

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**感谢使用 Excel Splitter！**
