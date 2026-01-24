# ArgForge 项目说明

> 可视化命令行参数配置工具 | Tauri v2 + Vue 3 + TypeScript + Rust

---

## 一、项目概要

ArgForge 是一个桌面应用，通过可视化表单界面配置命令行参数，支持多种参数风格，实时预览生成的命令。

**核心功能**：
- 项目管理（创建/编辑/删除/复制）
- 表单配置（表单项管理）
- 参数风格切换（`--key value` / `key=value` / `value`）
- 下拉选择模式（预设选项）
- 拖拽排序（完全自由重排）
- 实时命令预览

---

## 二、架构设计

```
Frontend (Vue 3)
  Components -> Stores (Pinia) -> API Layer (IPC 封装)
                     Tauri IPC
Backend (Rust)
  Commands -> Services -> Models
                     File Storage (JSON)
```

**模块职责**：
- **Commands** (Rust): Tauri API 接口层
- **Services** (Rust): 业务逻辑层
- **Models** (Rust): 数据结构定义
- **API** (TS): IPC 调用封装
- **Stores** (TS): Pinia 状态管理
- **Components** (TS): UI 组件

---

## 三、核心数据模型

```rust
Project {
    id: String,              // UUID
    name: String,
    description: String,
    forms: Vec<Form>
}

Form {
    id: String,
    name: String,
    items: Vec<FormItem>
}

FormItem {
    id: String,
    item_type: ItemType,           // Command | Parameter
    content: String,                // 统一内容字段
    param_name: String,             // Parameter 类型使用
    param_style: ParamStyle,        // KeyValue | EqualValue | ValueOnly
    use_dropdown: bool,
    dropdown_options: Vec<String>
}
```

---

## 四、编译和运行

### 开发模式
```bash
# 安装依赖
pnpm install

# 启动 Tauri 开发模式（推荐）
pnpm start

# 仅启动前端开发服务器
pnpm dev
```

### 构建
```bash
# 构建前端
pnpm build

# 构建桌面应用
pnpm tauri build

# 构建发布版本
pnpm tauri build -- --release
```

### 代码检查
```bash
# TypeScript 类型检查
pnpm build          # 包含 vue-tsc 检查

# Rust 代码检查
cd src-tauri
cargo clippy
cargo fmt --check
```

---

## 五、Git 提交规范

遵循 **Conventional Commits** 规范：

```
<type>(<scope>): <subject>
```

**类型**：
- `feat`: 新功能
- `fix`: Bug 修复
- `refactor`: 重构
- `docs`: 文档
- `test`: 测试
- `chore`: 构建/工具

**示例**：
```
feat(project): 添加项目复制功能
fix(form): 修复拖拽排序后顺序未保存问题
refactor(frontend): 重构表单状态管理
```

---

## 六、关键文件索引

```
src-tauri/src/
  commands/project.rs        # 项目 CRUD Commands
  commands/form.rs           # 表单 CRUD Commands
  services/storage.rs        # 文件存储服务
  services/command.rs        # 命令生成逻辑
  models/project.rs          # 数据模型
  error.rs                   # 统一错误定义
  lib.rs                     # Commands 注册

src/
  api/project.ts             # 项目 API 封装
  api/form.ts                # 表单 API 封装
  stores/project.ts          # 项目状态管理
  types/bindings.ts          # TypeScript 类型定义
  components/features/       # 业务组件
  components/ui/             # UI 组件
```

---

## 参考文档

编码规范
@CLAUDE-RULE.md
