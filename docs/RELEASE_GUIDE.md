# 版本发布指南

> 本文档详细说明如何将 `develop` 分支合并到 `main` 分支，以及如何发布新版本。

---

## 目录

- [分支策略](#分支策略)
- [方式一：通过 PR 合并（推荐）](#方式一通过-pr-合并推荐)
- [方式二：本地 Merge 合并](#方式二本地-merge-合并)
- [版本发布流程](#版本发布流程)
- [常见问题](#常见问题)

---

## 分支策略

```
main          ← 生产分支（稳定版本）
  ↑
  └── merge  ← 通过 PR 或本地 merge
develop       ← 开发分支（最新功能）
```

- **main**: 生产分支，只包含稳定版本
- **develop**: 开发分支，包含最新功能和修复

---

## 方式一：通过 PR 合并（推荐）

### 优点

- 代码审查更规范
- 保留合并历史记录
- 团队协作更友好
- 可在 Web 界面操作

### 具体步骤

#### 1. 确保 develop 分支代码就绪

```bash
# 切换到 develop 分支
git checkout develop

# 拉取最新代码
git pull origin develop

# 运行代码检查
pnpm build          # TypeScript 类型检查
cd src-tauri
cargo clippy        # Rust 代码检查
```

#### 2. 将 develop 推送到远程

```bash
# 确保所有改动已提交
git status

# 推送到远程（如果尚未推送）
git push origin develop
```

#### 3. 创建 Pull Request

**方式 A：通过 Web 界面**

1. 访问 GitHub 仓库页面
2. 点击 **"Pull requests"** → **"New pull request"**
3. 选择：
   - base: `main`
   - compare: `develop`
4. 填写 PR 信息：
   - **标题**: `chore: 合并 develop 到 main (发布版本 1.1.5)`
   - **描述**: 包含本次改动要点
5. 点击 **"Create pull request"**

**方式 B：通过 GitHub CLI**

```bash
# 安装 gh CLI 后执行
gh pr create \
  --base main \
  --head develop \
  --title "chore: 合并 develop 到 main (发布版本 1.1.5)" \
  --body "本次更新内容：- 新增功能 X- 修复问题 Y"
```

#### 4. 审查并合并 PR

1. 等待 CI 检查通过（绿色✅）
2. 点击 **"Merge pull request"**
3. 选择合并方式：
   - **Merge commit**（保留完整历史）
   - **Squash and merge**（压缩为单条提交）
4. 点击 **"Confirm merge"**

#### 5. 删除 develop 分支（可选）

合并后可以选择删除远程分支（不影响本地）：

```bash
# 在 PR 页面点击 "Delete branch" 按钮
# 或通过命令删除
git push origin --delete develop
```

---

## 方式二：本地 Merge 合并

### 优点

- 操作更快速
- 无需切换到 Web 界面
- 适合单人项目或紧急发布

### 具体步骤

#### 1. 确保 develop 分支最新

```bash
git checkout develop
git pull origin develop
```

#### 2. 运行代码检查

```bash
pnpm build
cd src-tauri
cargo clippy
```

#### 3. 切换到 main 分支

```bash
git checkout main
git pull origin main
```

#### 4. 合并 develop

```bash
git merge develop
```

如果有冲突，解决冲突后：

```bash
# 查看冲突文件
git status

# 解决冲突后标记为已解决
git add <冲突文件>

# 完成合并
git commit -m "chore: 合并 develop 到 main"
```

#### 5. 推送到远程

```bash
git push origin main
```

---

## 版本发布流程

### 当前版本号

查看当前版本：

```bash
# 查看 package.json
cat package.json | grep version

# 查看 Cargo.toml
cat src-tauri/Cargo.toml | grep version
```

当前版本：**1.1.4**

### 1. 确定新版本号

根据改动类型选择：

| 改动类型 | 版本示例 | 说明 |
|---------|---------|------|
| Bug 修复 | `1.1.5` | 补丁版本（最后一位） |
| 新功能 | `1.2.0` | 次版本（中间一位） |
| 破坏性变更 | `2.0.0` | 主版本（第一位） |

### 2. 更新版本号

项目提供了版本同步脚本：

```bash
# 更新到新版本（例如 1.1.5）
pnpm version 1.1.5
```

此命令会自动同步：
- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

### 3. 提交版本更新

```bash
# 查看改动
git status

# 提交版本更新
git add .
git commit -m "chore: 发布版本 1.1.5"
```

### 4. 打版本标签

```bash
# 创建带注释的标签
git tag -a v1.1.5 -m "发布版本 1.1.5"
```

### 5. 推送到远程

```bash
# 推送 main 分支
git push origin main

# 推送标签
git push origin v1.1.5
```

### 6. 触发 GitHub Actions

推送标签后，GitHub Actions 会自动：
- ✅ 构建前端和后端
- ✅ 打包 Windows 安装程序（NSIS）
- ✅ 创建 GitHub Release（草稿状态）
- ✅ 上传构建产物

等待约 **5-10 分钟**，构建完成后查看：
- GitHub 仓库 → **Releases** 页面

### 7. 发布 Release

1. 找到最新的草稿 Release
2. 检查构建产物是否正确
3. 点击 **"Publish release"** 按钮

---

## 完整命令清单

### 通过 PR 方式发布

```bash
# 1. 准备 develop 分支
git checkout develop
git pull origin develop
pnpm build
cd src-tauri && cargo clippy && cd ..

# 2. 推送代码
git push origin develop

# 3. 在 GitHub 创建 PR（main ← develop）
# 4. 等待 CI 通过后合并 PR

# 5. 在本地更新版本号
git checkout main
git pull origin main
git merge develop
pnpm version 1.1.5
git add .
git commit -m "chore: 发布版本 1.1.5"

# 6. 打标签并推送
git tag -a v1.1.5 -m "发布版本 1.1.5"
git push origin main
git push origin v1.1.5

# 7. 等待 GitHub Actions 构建完成
# 8. 在 GitHub Releases 页面发布草稿
```

### 通过本地 Merge 方式发布

```bash
# 1. 准备 develop 分支
git checkout develop
git pull origin develop
pnpm build
cd src-tauri && cargo clippy && cd ..

# 2. 合并到 main
git checkout main
git pull origin main
git merge develop

# 3. 解决冲突（如果有）
# git add .
# git commit -m "chore: 合并 develop 到 main"

# 4. 更新版本号
pnpm version 1.1.5
git add .
git commit -m "chore: 发布版本 1.1.5"

# 5. 打标签并推送
git tag -a v1.1.5 -m "发布版本 1.1.5"
git push origin main
git push origin v1.1.5

# 6. 等待 GitHub Actions 构建完成
# 7. 在 GitHub Releases 页面发布草稿
```

---

## 常见问题

### Q1: 推送标签后没有触发 GitHub Actions？

**A**: 检查标签格式是否正确，必须以 `v` 开头：

```bash
# 正确
git tag -a v1.1.5 -m "发布版本 1.1.5"

# 错误（不会触发）
git tag -a 1.1.5 -m "发布版本 1.1.5"
```

### Q2: 如何删除错误的标签？

```bash
# 删除本地标签
git tag -d v1.1.5

# 删除远程标签
git push origin :refs/tags/v1.1.5
```

### Q3: 如何查看所有标签？

```bash
# 列出所有标签
git tag

# 查看标签详情
git show v1.1.5
```

### Q4: 版本号同步失败怎么办？

手动修改三个文件：

```bash
# 1. 修改 package.json
# 2. 修改 src-tauri/Cargo.toml
# 3. 修改 src-tauri/tauri.conf.json

# 确保三个文件版本号一致后提交
git add .
git commit -m "chore: 手动更新版本号到 1.1.5"
```

### Q5: GitHub Actions 构建失败怎么办？

1. 查看 GitHub Actions 日志
2. 常见原因：
   - TypeScript 类型错误 → 运行 `pnpm build` 检查
   - Rust Clippy 错误 → 运行 `cargo clippy` 检查
   - 依赖安装失败 → 检查 `pnpm-lock.yaml` 是否更新
3. 修复后重新推送标签：

```bash
# 删除旧标签
git tag -d v1.1.5
git push origin :refs/tags/v1.1.5

# 修复代码后重新打标签
git tag -a v1.1.5 -m "发布版本 1.1.5"
git push origin v1.1.5
```

---

## 附录：GitHub CLI 快捷命令

安装 GitHub CLI 后可使用以下命令简化操作：

```bash
# 创建 PR
gh pr create --base main --head develop --title "chore: 合并 develop" --body "发布新版本"

# 查看 PR 状态
gh pr status

# 查看 Release
gh release list

# 创建 Release（手动方式，不推荐）
gh release create v1.1.5 --title "版本 1.1.5" --notes "本次更新内容"
```

---

## 相关文档

- [Git 规范文档](./GIT_COMMANDS.md)
- [CI/CD 教程](./CICD_TUTORIAL.md)
- [项目编码规范](../CLAUDE-RULE.md)
