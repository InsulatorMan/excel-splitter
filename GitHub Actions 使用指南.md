# GitHub Actions 自动编译指南

## 概述

使用 GitHub Actions 自动编译 Rust 项目，无需本地安装 Rust 环境。

## 步骤

### 1. 创建 GitHub 仓库

1. 访问 https://github.com/new
2. 创建新仓库，例如 `excel-splitter-rust`
3. 记住仓库地址：`https://github.com/yourname/excel-splitter-rust`

### 2. 推送代码到 GitHub

```bash
# 在项目目录中初始化 git
cd rust_excel_splitter
git init

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit"

# 添加远程仓库（替换 yourname 为您的 GitHub 用户名）
git remote add origin https://github.com/yourname/excel-splitter-rust.git

# 推送代码
git push -u origin main
```

### 3. 触发自动编译

#### 方式 1：自动触发（推荐）

每次推送代码到 main 分支时，GitHub Actions 会自动编译。

#### 方式 2：手动触发

1. 访问 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 选择 "Build Excel Splitter" 工作流
4. 点击 "Run workflow" 按钮

### 4. 下载编译结果

1. 等待编译完成（约 5-10 分钟）
2. 点击 "Actions" 标签查看最新运行记录
3. 点击进入最新的 workflow run
4. 在 "Artifacts" 部分下载编译结果：
   - `excel-splitter-windows` - Windows 版本
   - `excel-splitter-linux` - Linux 版本
   - `excel-splitter-macos` - macOS 版本

### 5. 创建 Release（可选）

1. 在 GitHub 仓库页面点击 "Releases"
2. 点击 "Create a new release"
3. 输入版本号，例如 `v1.0.0`
4. GitHub Actions 会自动将编译结果附加到 Release

## 文件说明

### .github/workflows/build.yml

自动编译配置文件，支持：
- Windows 编译
- Linux 编译（musl 静态链接）
- macOS 编译
- 自动上传 Artifact
- 自动创建 Release

## 编译输出

编译完成后，您可以在以下位置找到可执行文件：

### Windows
- 文件名：`excel-splitter.exe`
- 大小：约 3-5 MB
- 依赖：无（纯静态链接）

### Linux
- 文件名：`excel-splitter`
- 大小：约 3-5 MB
- 依赖：无（musl 静态链接）

### macOS
- 文件名：`excel-splitter`
- 大小：约 3-5 MB
- 依赖：无

## 常见问题

### Q: 编译失败怎么办？

**A:** 
1. 检查 GitHub Actions 日志
2. 确保 Cargo.toml 配置正确
3. 检查是否有未提交的更改

### Q: 如何更新代码后重新编译？

**A:** 
1. 修改代码
2. 提交并推送：`git add . && git commit -m "update" && git push`
3. GitHub Actions 会自动重新编译

### Q: 可以只编译 Windows 版本吗？

**A:** 可以，编辑 `.github/workflows/build.yml`，删除不需要的 job。

### Q: 编译后的文件在哪里下载？

**A:** 
1. 访问 GitHub 仓库
2. 点击 Actions 标签
3. 选择最新的 workflow run
4. 在 Artifacts 部分下载

## 优势

- ✅ 无需本地 Rust 环境
- ✅ 自动编译，一键下载
- ✅ 支持多平台（Windows/Linux/macOS）
- ✅ 每次推送自动更新
- ✅ 免费使用（GitHub Actions 免费额度）

## 替代方案

如果 GitHub Actions 不方便，还可以使用：

1. **GitLab CI/CD** - 类似功能
2. **Azure Pipelines** - 微软提供
3. **Travis CI** - 老牌 CI 服务
4. **CircleCI** - 快速构建

## 联系方式

如有问题，欢迎提交 Issue。
