# ArgForge CI/CD 发布教程

> 本文档面向新手，详细介绍如何使用 ArgForge 项目的自动化发布系统。

---

## 目录

- [什么是 CI/CD？](#什么是-cicd)
- [本项目发布系统概览](#本项目发布系统概览)
- [发布前准备](#发布前准备)
- [发布流程详解](#发布流程详解)
- [常见问题](#常见问题)
- [最佳实践](#最佳实践)

---

## 什么是 CI/CD？

**CI/CD** 是 **Continuous Integration（持续集成）** 和 **Continuous Deployment（持续部署）** 的缩写。

简单来说：
- **CI（持续集成）**：每次代码提交后，自动进行构建、测试
- **CD（持续部署）**：测试通过后，自动发布到生产环境

### 本项目的 CI/CD

ArgForge 使用 **GitHub Actions** 实现 CI/CD：
- 当你推送一个版本标签（tag）时
- GitHub 自动在云端构建 Windows 安装包
- 构建完成后自动发布到 GitHub Releases

**整个过程自动化，无需手动操作！**

---

## 本项目发布系统概览

### 技术栈

| 组件 | 技术 | 作用 |
|------|------|------|
| **版本管理** | Git + Semantic Versioning | 管理版本号和代码历史 |
| **自动化工具** | GitHub Actions | 监听标签推送，自动构建发布 |
| **构建工具** | Tauri CLI | 打包 Windows 安装包 |
| **安装包格式** | NSIS | 生成 Windows 标准安装程序 |

### 工作流程

```
┌─────────────────┐
│  开发者本地     │
└────────┬────────┘
         │ 1. 修改版本号
         │ 2. 创建 Git Tag
         ▼
┌─────────────────┐
│  GitHub 仓库    │
└────────┬────────┘
         │ Tag 推送触发
         ▼
┌─────────────────┐
│ GitHub Actions  │ ← 自动化构建
│  (Windows)      │
└────────┬────────┘
         │ 构建完成
         ▼
┌─────────────────┐
│ GitHub Releases │ ← 用户下载
└─────────────────┘
```

---

## 发布前准备

### 1. 检查 GitHub Actions 权限

首次发布前，必须确保 GitHub Actions 有写入权限：

1. 访问仓库设置页面：
   ```
   https://github.com/twayne-coder/arg-forge/settings/actions
   ```

2. 找到 **Workflow permissions** 部分

3. 选择 **Read and write permissions**

4. 点击 **Save** 保存

> ⚠️ **如果不设置此权限，Actions 无法创建 Release，构建会失败！**

### 2. 安装开发工具

确保本地已安装以下工具：

```bash
# 检查 Node.js 版本（需要 v20+）
node --version

# 检查 pnpm 版本（需要 v9+）
pnpm --version

# 检查 Git 版本
git --version
```

如果未安装，请访问：
- [Node.js 官网](https://nodejs.org/)
- [pnpm 安装指南](https://pnpm.io/installation)

### 3. 本地测试构建（推荐）

在正式发布前，建议先测试本地构建：

```bash
# 1. 安装依赖
pnpm install

pnpm start

# 2. 构建前端
pnpm build

# 3. 构建 Tauri 应用（约 5-10 分钟）
pnpm tauri build
```

构建成功后，检查输出目录：
```
src-tauri/target/release/
├── ArgForge_0.1.0_x64-setup.exe  # NSIS 安装包
└── ArgForge_0.1.0_x64.zip        # 便携版
```

如果本地构建成功，说明配置无误，可以继续发布流程。

---

## 发布流程详解

### 版本号规范

本项目遵循 **Semantic Versioning 2.0.0** 规范：

版本号格式：`主版本号.次版本号.修订号`

| 版本号类型 | 示例 | 说明 | 使用场景 |
|-----------|------|------|---------|
| **主版本号** (Major) | `1.0.0` → `2.0.0` | 不兼容的 API 修改 | 重大架构变更、删除旧功能 |
| **次版本号** (Minor) | `1.0.0` → `1.1.0` | 向下兼容的新增功能 | 新增特性、功能增强 |
| **修订号** (Patch) | `1.0.0` → `1.0.1` | 向下兼容的问题修正 | Bug 修复、小改进 |

### 快速发布（3 步）

#### 步骤 1：切换到主分支

```bash
# 确保在 main 分支
git checkout main

# 拉取最新代码
git pull origin main
```

#### 步骤 2：更新版本号

根据你的更改类型，选择以下命令之一：

```bash
# Bug 修复（修订号 +1）
npm version patch
# 示例：0.1.0 → 0.1.1

# 新增功能（次版本号 +1）
npm version minor
# 示例：0.1.1 → 0.2.0

# 重大更新（主版本号 +1）
npm version major
# 示例：0.2.0 → 1.0.0
```

**这个命令会自动完成：**
- ✅ 更新 `package.json` 的版本号
- ✅ 同步版本号到 `src-tauri/tauri.conf.json`
- ✅ 同步版本号到 `src-tauri/Cargo.toml`
- ✅ 创建一个 Git commit（提交信息：`v0.1.1`）
- ✅ 创建一个 Git tag（标签名：`v0.1.1`）

#### 步骤 3：推送到 GitHub

```bash
# 推送代码和标签
git push origin main
git push origin v0.1.1
```

推送标签后，GitHub Actions 会自动触发构建流程！

### 手动指定版本号

如果你想指定特定版本号（而不是递增）：

```bash
# 指定版本号
npm version 1.2.3

# 推送
git push origin main
git push origin v1.2.3
```

### 监控构建进度

1. **访问 Actions 页面**
   ```
   https://github.com/twayne-coder/arg-forge/actions
   ```

2. **查看最新的 Workflow 运行**
   - 找到以 `Release` 开头的工作流
   - 点击查看详细日志
   - 状态指示：
     - 🟡 **Queued** - 排队中
     - 🟠 **In progress** - 构建中
     - ✅ **Success** - 构建成功
     - ❌ **Failed** - 构建失败

3. **等待构建完成**
   - 通常需要 **5-10 分钟**
   - 构建时间取决于 GitHub 的负载情况

### 下载发布产物

构建成功后，访问 Releases 页面：

```
https://github.com/twayne-coder/arg-forge/releases
```

你会看到以下文件：

| 文件名 | 说明 | 推荐给 |
|--------|------|--------|
| `ArgForge_0.1.1_x64-setup.exe` | NSIS 安装包 | 普通用户（推荐） |
| `ArgForge_0.1.1_x64.zip` | 便携版（免安装） | 高级用户 |
| `SHA256SUMS.txt` | 文件校验和 | 需要验证完整性的用户 |

**下载和安装：**
1. 点击 `ArgForge_0.1.1_x64-setup.exe` 下载
2. 运行安装程序
3. 按照安装向导完成安装

---

## 常见问题

### Q1: 构建失败，提示 "Permission denied"

**原因**：GitHub Actions 没有写入权限。

**解决方案**：
1. 访问仓库设置页面
2. 启用 **Read and write permissions**
3. 重新推送 tag：
   ```bash
   git tag -d v0.1.1
   git push origin :refs/tags/v0.1.1
   npm version patch
   git push origin main
   git push origin v0.1.1
   ```

### Q2: 版本号不同步怎么办？

**原因**：`scripts/sync-version.js` 脚本未执行。

**解决方案**：手动同步版本号
```bash
# 执行同步脚本
node scripts/sync-version.js

# 检查版本号是否一致
grep "0.1.1" package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml

# 提交更改
git add src-tauri/tauri.conf.json src-tauri/Cargo.toml
git commit -m "chore: 同步版本号"
```

### Q3: 如何撤销已发布的 Release？

**步骤**：
1. 访问 Releases 页面
2. 找到要撤销的版本
3. 点击 **Delete release** 删除发布
4. 删除对应的 Git tag：
   ```bash
   git tag -d v0.1.1
   git push origin :refs/tags/v0.1.1
   ```

> ⚠️ **注意**：已下载安装包的用户不受影响，但无法再从 GitHub 下载。

### Q4: 构建时间过长怎么办？

**正常情况**：首次构建可能需要 10-15 分钟（需要下载 Rust 依赖）。

**后续构建**：通常 5-10 分钟。

**如果超过 30 分钟**：
1. 检查 Actions 日志，查看是否有错误
2. 确认是否有依赖下载失败
3. 尝试取消并重新构建

### Q5: 如何预发布测试版本？

创建 **pre-release** 版本：

```bash
# 使用 prerelease 标识
npm version 0.1.1-beta.1
git push origin main
git push origin v0.1.1-beta.1
```

然后在 GitHub Releases 页面将其标记为 **Pre-release**。

---

## 最佳实践

### 1. 发布前检查清单

在发布新版本前，请确认：

- [ ] 所有代码更改已提交到 `main` 分支
- [ ] 本地构建测试通过（`pnpm tauri build`）
- [ ] 版本号符合语义化规范
- [ ] 更新了 CHANGELOG.md（如果有）
- [ ] 重要的 Bug 已修复
- [ ] 新功能已充分测试

### 2. 版本号递增建议

| 场景 | 推荐命令 | 示例 |
|------|---------|------|
| 修复 Bug | `npm version patch` | `0.1.0` → `0.1.1` |
| 新增功能 | `npm version minor` | `0.1.0` → `0.2.0` |
| 破坏性变更 | `npm version major` | `0.1.0` → `1.0.0` |
| 紧急热修复 | `npm version patch` | `0.1.1` → `0.1.2` |

### 3. 保持 `main` 分支干净

- ✅ **推荐**：在 `develop` 分支开发，完成后合并到 `main`
- ❌ **不推荐**：直接在 `main` 分支开发新功能

### 4. 定期更新依赖

```bash
# 检查过时的依赖
pnpm outdated

# 更新依赖
pnpm update

# 本地测试
pnpm tauri build
```

### 5. 保护重要版本

对于稳定版本（如 v1.0.0），可以创建 Git 分支进行维护：

```bash
# 创建维护分支
git checkout -b v1.0.x

# 推送到远程
git push origin v1.0.x
```

---

## 进阶：自定义发布配置

### 修改 NSIS 安装包配置

编辑 `src-tauri/tauri.conf.json` 文件：

```json
{
  "bundle": {
    "nsis": {
      "installMode": "perUser",        // 安装模式：perUser / allUsers
      "createDesktopIcon": true,       // 创建桌面快捷方式
      "createStartMenuShortcut": true, // 创建开始菜单快捷方式
      "allowDowngrades": true,         // 允许降级安装
      "languages": ["SimpChinese"]     // 安装界面语言
    }
  }
}
```

### 添加代码签名（推荐）

为了避免 Windows SmartScreen 警告，建议对安装包进行数字签名。

**步骤**：
1. 购买代码签名证书（如 [DigiCert](https://www.digicert.com/)）
2. 将证书上传到 GitHub Secrets
3. 修改 `.github/workflows/release.yml` 添加签名配置

详细教程：[Tauri 代码签名指南](https://v2.tauri.app/distribute/sign/)

---

## 总结

通过本教程，你应该已经掌握：

✅ **CI/CD 的基本概念**
✅ **本项目的发布系统架构**
✅ **完整的发布流程**
✅ **常见问题的解决方法**
✅ **发布最佳实践**

### 快速参考

```bash
# 标准发布流程（3 步）
git checkout main && git pull origin main
npm version patch  # 或 minor / major
git push origin main && git push origin v0.1.1
```

### 链接汇总

- [GitHub Actions](https://github.com/twayne-coder/arg-forge/actions)
- [GitHub Releases](https://github.com/twayne-coder/arg-forge/releases)
- [仓库设置](https://github.com/twayne-coder/arg-forge/settings)
- [Semantic Versioning](https://semver.org/lang/zh-CN/)
- [Tauri 发布文档](https://v2.tauri.app/distribute/publish/)

---

## 获取帮助

如果遇到问题：

1. **查看文档**
   - [Tauri 官方文档](https://v2.tauri.app/)
   - [GitHub Actions 文档](https://docs.github.com/cn/actions)

2. **搜索 Issue**
   - [GitHub Issues](https://github.com/twayne-coder/arg-forge/issues)

3. **提问**
   - 创建新的 Issue
   - 描述问题和复现步骤

---

**祝你发布顺利！🎉**
