# GitHub 上传和 Actions 编译指南

## 步骤 1: 创建 GitHub 仓库

### 1.1 在 GitHub 上创建新仓库

1. 访问 https://github.com/new
2. 填写仓库信息：
   - **Repository name**: `excel-splitter`
   - **Description**: `High-performance Excel file splitter with Rust and Tauri`
   - **Visibility**: Public（或 Private）
   - **Initialize**: 不要勾选任何选项（我们已有代码）
3. 点击 **Create repository**

### 1.2 获取仓库地址

创建后，GitHub 会显示仓库地址：
```
https://github.com/YOUR_USERNAME/excel-splitter.git
```

## 步骤 2: 初始化本地 Git 仓库

在项目目录中执行：

```bash
# 进入项目目录
cd rust_excel_splitter

# 初始化 Git 仓库
git init

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit: Excel Splitter with Tauri GUI"

# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/excel-splitter.git

# 推送到 GitHub
git push -u origin main
```

如果遇到 `main` 分支问题：
```bash
# 查看当前分支
git branch

# 如果是 master，重命名
git branch -M main

# 再推送
git push -u origin main
```

## 步骤 3: 验证上传

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter`
2. 确认所有文件都已上传

## 步骤 4: GitHub Actions 自动编译

### 4.1 配置已自动完成

项目已包含 `.github/workflows/build.yml`，会自动：
- ✅ 在 Windows 上编译 EXE 和 MSI
- ✅ 在 Linux 上编译 DEB 和 AppImage
- ✅ 在 macOS 上编译 APP 和 DMG
- ✅ 创建 GitHub Release（当推送 tag 时）

### 4.2 触发编译

#### 方式 1: 推送代码自动触发

每次推送到 `main` 分支，Actions 会自动编译：

```bash
# 修改代码后
git add .
git commit -m "Update features"
git push
```

#### 方式 2: 手动触发

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/actions`
2. 点击 **Build Excel Splitter**
3. 点击 **Run workflow**
4. 选择分支，点击 **Run workflow**

#### 方式 3: 创建 Release（推荐）

```bash
# 创建标签
git tag -a v1.0.0 -m "Release version 1.0.0"

# 推送标签
git push origin v1.0.0
```

推送标签后，Actions 会自动：
1. 编译所有平台版本
2. 创建 GitHub Release
3. 上传所有编译文件

## 步骤 5: 下载编译结果

### 5.1 从 Actions 下载

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/actions`
2. 点击最新的工作流运行
3. 在 **Artifacts** 部分下载：
   - `excel-splitter-windows-exe`
   - `excel-splitter-windows-msi`
   - `excel-splitter-linux-deb`
   - `excel-splitter-linux-appimage`
   - `excel-splitter-macos-app`
   - `excel-splitter-macos-dmg`

### 5.2 从 Release 下载（推荐）

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/releases`
2. 点击最新的 Release
3. 下载对应平台的文件：
   - Windows: `excel-splitter.exe` 或 `.msi`
   - Linux: `.deb` 或 `.AppImage`
   - macOS: `.app` 或 `.dmg`

## 完整操作流程

```bash
# 1. 进入项目目录
cd rust_excel_splitter

# 2. 初始化 Git
git init

# 3. 添加文件
git add .

# 4. 提交
git commit -m "Initial commit"

# 5. 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/excel-splitter.git

# 6. 推送
git push -u origin main

# 7. 创建标签触发 Release
git tag -a v1.0.0 -m "First release"
git push origin v1.0.0

# 8. 等待 Actions 完成（约 10-20 分钟）
# 9. 访问 Releases 页面下载
```

## 常见问题

### Q: 推送失败，提示权限错误

**A:**
```bash
# 使用 HTTPS 令牌
# 或配置 SSH
git remote set-url origin git@github.com:YOUR_USERNAME/excel-splitter.git

# 生成 SSH 密钥
ssh-keygen -t ed25519 -C "your@email.com"

# 添加公钥到 GitHub
# https://github.com/settings/keys
```

### Q: Actions 编译失败

**A:**
1. 访问 Actions 页面查看详细日志
2. 常见原因：
   - 依赖版本问题
   - 代码编译错误
   - 配置问题

### Q: 如何更新代码后重新编译

**A:**
```bash
# 修改代码
git add .
git commit -m "Update"
git push

# Actions 会自动重新编译
```

### Q: 如何只编译 Windows 版本

**A:**
编辑 `.github/workflows/build.yml`，注释掉不需要的 job：
```yaml
# 注释掉 build-linux 和 build-macos
```

## 下一步

1. **创建 GitHub 仓库**
2. **推送代码**
3. **等待 Actions 编译**
4. **下载 EXE 文件**

需要我帮您配置 GitHub 账号或解决其他问题吗？
