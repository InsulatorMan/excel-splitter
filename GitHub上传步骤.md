# GitHub 上传步骤

## 本地仓库已初始化完成

项目已成功提交到本地 Git 仓库。现在需要推送到 GitHub。

## 步骤 1: 在 GitHub 创建仓库

1. 访问 https://github.com/new
2. 填写信息：
   - **Repository name**: `excel-splitter`
   - **Description**: `High-performance Excel file splitter with Rust and Tauri`
   - **Visibility**: Public（推荐，免费使用 Actions）
   - **不要勾选** "Initialize this repository with a README"
3. 点击 **Create repository**

## 步骤 2: 获取仓库地址

创建后，GitHub 会显示：
```
https://github.com/YOUR_USERNAME/excel-splitter.git
```

## 步骤 3: 推送到 GitHub

在 PowerShell 中执行（替换 YOUR_USERNAME）：

```powershell
cd "C:\Users\Administrator\.qclaw\workspace\rust_excel_splitter"

# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/excel-splitter.git

# 推送代码
git push -u origin master
```

### 如果遇到认证问题：

**方式 1: 使用 HTTPS + Personal Access Token**

1. 访问 https://github.com/settings/tokens
2. 点击 **Generate new token (classic)**
3. 勾选 `repo` 权限
4. 生成 token
5. 使用 token 推送：

```powershell
# 使用 token 作为密码
git push -u origin master
# 用户名: YOUR_USERNAME
# 密码: ghp_xxxxxxxxxxxx (你的 token)
```

**方式 2: 使用 SSH（推荐）**

```powershell
# 生成 SSH 密钥
ssh-keygen -t ed25519 -C "your@email.com"

# 查看公钥
cat $env:USERPROFILE\.ssh\id_ed25519.pub

# 复制公钥内容，添加到 GitHub:
# https://github.com/settings/keys
# 点击 New SSH key，粘贴公钥

# 修改远程地址为 SSH
git remote set-url origin git@github.com:YOUR_USERNAME/excel-splitter.git

# 推送
git push -u origin master
```

## 步骤 4: 验证上传

访问 `https://github.com/YOUR_USERNAME/excel-splitter`

确认所有文件都已上传。

## 步骤 5: 触发 GitHub Actions 编译

### 方式 1: 创建标签触发 Release

```powershell
cd "C:\Users\Administrator\.qclaw\workspace\rust_excel_splitter"

# 创建标签
git tag -a v1.0.0 -m "Release version 1.0.0"

# 推送标签
git push origin v1.0.0
```

推送标签后，Actions 会自动：
1. 在 Windows 编译 EXE 和 MSI
2. 在 Linux 编译 DEB 和 AppImage
3. 在 macOS 编译 APP 和 DMG
4. 创建 GitHub Release 并上传所有文件

### 方式 2: 手动触发

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/actions`
2. 点击 **Build Excel Splitter**
3. 点击 **Run workflow**
4. 选择分支 `master`，点击 **Run workflow**

## 步骤 6: 下载编译结果

### 从 Actions 下载

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/actions`
2. 点击最新的工作流运行
3. 在 **Artifacts** 部分下载文件

### 从 Release 下载（推荐）

1. 访问 `https://github.com/YOUR_USERNAME/excel-splitter/releases`
2. 点击最新的 Release
3. 下载对应平台的文件

## 完整命令汇总

```powershell
# 1. 进入目录
cd "C:\Users\Administrator\.qclaw\workspace\rust_excel_splitter"

# 2. 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/excel-splitter.git

# 3. 推送代码
git push -u origin master

# 4. 创建标签触发编译
git tag -a v1.0.0 -m "First release"
git push origin v1.0.0

# 5. 等待 10-20 分钟

# 6. 访问 Releases 页面下载
# https://github.com/YOUR_USERNAME/excel-splitter/releases
```

## 注意事项

1. **GitHub Actions 免费额度**：
   - 公共仓库：无限使用
   - 私有仓库：每月 2000 分钟

2. **编译时间**：
   - 首次编译：约 15-25 分钟
   - 后续编译：约 10-15 分钟（有缓存）

3. **文件大小限制**：
   - Release 附件：每个 2 GB
   - Artifacts：每个 500 MB

## 下一步

1. 在 GitHub 创建仓库
2. 推送代码
3. 创建标签 `v1.0.0`
4. 等待 Actions 完成
5. 下载 Windows EXE 文件

需要我帮您配置 GitHub 账号或其他设置吗？
