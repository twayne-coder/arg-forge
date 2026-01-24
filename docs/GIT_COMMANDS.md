# Git 命令大全

> 常用 Git 命令速查手册

---

## 一、基础操作

### 初始化与配置

```bash
# 初始化仓库
git init

# 克隆仓库
git clone <url>

# 配置用户信息
git config --global user.name "Your Name"
git config --global user.email "your@email.com"

# 查看配置
git config --list
```

### 基础增删改

```bash
# 查看当前状态
git status

# 添加文件到暂存区
git add <file>           # 添加单个文件
git add .                # 添加所有文件
git add *.txt            # 添加所有 txt 文件

# 提交更改
git commit -m "message"
git commit -am "message" # 添加并提交已跟踪文件

### 查看差异
git diff                 # 工作区与暂存区差异
git diff --staged        # 暂存区与上次提交差异
git diff HEAD            # 工作区与上次提交差异
```

---

## 二、分支操作

### 分支查看与创建

```bash
# 查看所有分支
git branch               # 本地分支
git branch -r            # 远程分支
git branch -a            # 所有分支

# 创建分支
git branch <branch-name>

# 切换分支
git checkout <branch-name>
git switch <branch-name> # Git 2.23+

# 创建并切换分支
git checkout -b <branch-name>
git switch -c <branch-name>

# 删除分支
git branch -d <branch-name>     # 删除已合并分支
git branch -D <branch-name>     # 强制删除分支
```

### 分支合并

```bash
# 合并分支
git merge <branch-name>

# 变基操作
git rebase <branch-name>

# 终止变基
git rebase --abort

# 继续变基
git rebase --continue
```

---

## 三、远程操作

### 远程仓库管理

```bash
# 查看远程仓库
git remote
git remote -v            # 显示详细信息

# 添加远程仓库
git remote add <name> <url>

# 删除远程仓库
git remote remove <name>

# 更新远程仓库信息
git remote update
git fetch --all
```

### 推送与拉取

```bash
# 推送到远程
git push
git push -u origin <branch>    # 首次推送并设置上游
git push -f                     # 强制推送（谨慎使用）

# 拉取远程更新
git pull
git pull --rebase               # 使用变基方式拉取

# 获取远程更新但不合并
git fetch
```

---

## 四、撤销操作

### 撤销工作区更改

```bash
# 撤销工作区文件修改
git restore <file>              # Git 2.23+
git checkout -- <file>          # 旧版命令

# 撤销所有工作区更改
git restore .
git checkout -- .
```

### 撤销暂存区

```bash
# 取消暂存
git restore --staged <file>
git reset HEAD <file>

# 取消所有暂存
git restore --staged .
git reset HEAD
```

### 撤销提交

```bash
# 撤销最后一次提交（保留更改）
git reset --soft HEAD~1

# 撤销最后一次提交（不保留更改）
git reset --hard HEAD~1

# 撤销指定提交（创建新提交）
git revert <commit-id>

# 回退到指定提交
git reset --hard <commit-id>
```

---

## 五、查看操作

### 查看提交历史

```bash
# 查看提交历史
git log
git log --oneline              # 单行显示
git log --graph --oneline      # 图形化显示
git log --grep "keyword"       # 搜索提交信息
git log --author="name"        # 按作者筛选

# 查看文件修改历史
git log -p <file>

# 查看某次提交详情
git show <commit-id>
```

### 查看文件状态

```bash
# 查看文件内容
git show <commit-id>:<file>

# 查看文件历史版本
git log --follow <file>

# 查看文件修改者
git blame <file>
```

---

## 六、标签操作

```bash
# 查看标签
git tag

# 创建标签
git tag v1.0.0
git tag -a v1.0.0 -m "version 1.0.0"

# 删除标签
git tag -d v1.0.0

# 推送标签到远程
git push origin v1.0.0
git push origin --tags

# 删除远程标签
git push origin --delete v1.0.0
```

---

## 七、暂存操作

```bash
# 暂存当前工作
git stash
git stash save "message"

# 查看暂存列表
git stash list

# 应用暂存
git stash apply
git stash pop                 # 应用后删除

# 删除暂存
git stash drop
git stash clear               # 清空所有暂存

# 应用指定暂存
git stash apply stash@{0}
```

---

## 八、高级操作

### 交互式变基

```bash
# 交互式变基（修改历史提交）
git rebase -i HEAD~n          # n 为要修改的提交数
```

操作选项：
- `pick`：保留该提交
- `reword`：修改提交信息
- `edit`：修改提交内容
- `squash`：合并到前一个提交
- `drop`：删除该提交

### Cherry-pick

```bash
# 应用指定提交到当前分支
git cherry-pick <commit-id>

# 应用多个提交
git cherry-pick <commit-id1> <commit-id2>
```

### 清理操作

```bash
# 清理未跟踪文件
git clean -f

# 清理未跟踪文件和目录
git clean -fd

# 预览要清理的文件
git clean -n
```

---

## 九、常见场景

### 场景 1：修复上一个提交

```bash
# 修改文件后
git add <file>
git commit --amend            # 追加到上一个提交
git commit --amend -m "新信息" # 修改提交信息
```

### 场景 2：合并多个提交为一个

```bash
# 交互式变基最近 3 个提交
git rebase -i HEAD~3

# 将要合并的提交标记为 squash
# 保存并编辑合并后的提交信息
```

### 场景 3：临时切换分支

```bash
# 暂存当前工作
git stash

# 切换到其他分支
git switch other-branch

# 完成工作后切回
git switch -

# 恢复暂存
git stash pop
```

### 场景 4：同步远程分支

```bash
# 拉取远程更新
git fetch origin

# 变基本地分支
git rebase origin/main

# 或者直接 pull --rebase
git pull --rebase
```

### 场景 5：回退远程分支

```bash
# 回退本地分支
git reset --hard <commit-id>

# 强制推送到远程
git push -f origin <branch>
```

---

## 十、常见问题

### 1. 冲突解决

```bash
# 合并冲突后
git status                   # 查看冲突文件

# 手动编辑文件解决冲突
# 然后标记为已解决
git add <conflict-file>

# 继续合并
git commit
```

### 2. 撤销已推送的提交

```bash
# 方案一：revert（推荐）
git revert <commit-id>
git push

# 方案二：reset + force push（谨慎）
git reset --hard <commit-id>
git push -f
```

### 3. 查看分支关系

```bash
# 图形化查看分支历史
git log --graph --oneline --all

# 查看分支包含的提交
git log master..feature      # feature 有而 master 没有的提交
```

### 4. 大文件处理

```bash
# 从历史中移除大文件
git filter-branch --tree-filter 'rm -f path/to/large/file' HEAD

# 或使用 BFG Repo-Cleaner（更快）
# bfg --delete-files large-file.jar
```

---

## 十一、.gitignore 规范

```bash
# 常见忽略规则

# 依赖目录
node_modules/
vendor/
target/

# 编译产物
dist/
build/
*.exe
*.dll

# IDE 配置
.vscode/
.idea/
*.swp
*.swo

# 日志文件
*.log
logs/

# 环境配置
.env
.env.local

# 操作系统文件
.DS_Store
Thumbs.db
```

---

## 十二、最佳实践

### 分支命名规范

```
main/master        - 主分支（生产环境）
develop            - 开发分支
feature/*          - 功能分支
feature/user-auth
fix/*              - 修复分支
fix/login-bug
hotfix/*           - 紧急修复分支
release/*          - 发布分支
release/v1.0.0
```

### 提交信息规范

遵循 Conventional Commits：

```
<type>(<scope>): <subject>

类型：
feat     - 新功能
fix      - Bug 修复
docs     - 文档更新
style    - 代码格式（不影响功能）
refactor - 重构
test     - 测试相关
chore    - 构建/工具配置

示例：
feat(auth): add login function
fix(ui): resolve button alignment issue
docs(readme): update installation guide
```

### 工作流建议

1. **功能开发**：从 `develop` 创建 `feature` 分支，完成后合并回 `develop`
2. **发布准备**：从 `develop` 创建 `release` 分支，测试后合并到 `main` 和 `develop`
3. **紧急修复**：从 `main` 创建 `hotfix` 分支，完成后合并到 `main` 和 `develop`
4. **保持历史整洁**：使用 `git rebase` 而非 `git merge` 整理提交历史
5. **及时推送**：完成阶段性工作后及时推送到远程备份

---

## 参考资料

- [Git 官方文档](https://git-scm.com/doc)
- [Pro Git 中文版](https://git-scm.com/book/zh/v2)
- [Conventional Commits](https://www.conventionalcommits.org/)
