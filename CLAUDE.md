# ArgForge 项目上下文

> **项目定位**：可视化命令行参数配置工具
> 技术栈：Tauri v2 + Vue 3 + TypeScript + Rust

---

## 一、项目概要

ArgForge 是一个桌面应用，让用户通过可视化表单界面配置命令行参数，支持多种参数风格（Argparse/Hydra/Positional），并实时预览生成的命令。

**核心功能**：
- 项目管理（创建/编辑/删除/复制）
- 表单配置（命令模板 + 参数项）
- 参数风格切换（`--key value` / `key=value` / `value`）
- 下拉选择模式（参数值预设选项）
- 拖拽排序（表单项重排）
- 实时命令预览

---

## 二、架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend (Vue 3)                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Components  │  │   Stores    │  │      API Layer      │  │
│  │  (UI View)  │─→│  (Pinia)    │─→│  (IPC 封装)         │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            ↓ Tauri IPC
┌─────────────────────────────────────────────────────────────┐
│                         Backend (Rust)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │  Commands   │  │  Services   │  │      Models         │  │
│  │ (API 接口)  │─→│ (业务逻辑)  │─→│    (数据结构)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            ↓
                    ┌───────────────┐
                    │ File Storage  │
                    │   (JSON)      │
                    └───────────────┘
```

### 后端模块

| 模块 | 职责 | 关键文件 |
|------|------|---------|
| **Models** | 数据结构定义 | `models/project.rs` (Project/Form/FormItem/ParamStyle) |
| **Services** | 业务逻辑 | `services/storage.rs` (文件存储)<br>`services/command.rs` (命令生成) |
| **Commands** | Tauri API | `commands/project.rs` (项目 CRUD)<br>`commands/form.rs` (表单 CRUD)<br>`commands/command_gen.rs` (命令生成) |
| **Error** | 统一错误 | `error.rs` (thiserror 定义 AppError) |

### 前端模块

| 模块 | 职责 | 关键文件 |
|------|------|---------|
| **Types** | 类型定义 | `types/bindings.ts` (需与 Rust models 同步) |
| **API** | IPC 调用封装 | `api/project.ts`, `api/form.ts`, `api/command.ts` |
| **Stores** | 状态管理 | `stores/project.ts` (projects/currentProject/currentForm) |
| **Components** | UI 组件 | `components/features/` (业务组件)<br>`components/layout/` (布局组件) |
| **UI** | 基础组件 | `components/ui/` (Shadcn-vue) |

---

## 三、核心数据模型

```rust
// src-tauri/src/models/project.rs

Project {
    id: UUID,
    name: String,
    description: String,
    created_at: DateTime,
    updated_at: DateTime,
    forms: Vec<Form>
}

Form {
    id: UUID,
    project_id: UUID,
    name: String,
    command_template: String,  // 支持 {params} 占位符
    sort_order: i32,
    items: Vec<FormItem>
}

FormItem {
    id: UUID,
    param_name: String,        // 参数名
    param_value: String,       // 参数值
    param_style: ParamStyle,   // Argparse/Hydra/Positional
    enabled: bool,
    use_dropdown: bool,        // 是否使用下拉选择
    dropdown_options: Vec<String>
}

enum ParamStyle {
    Argparse,    // --key value
    Hydra,       // key=value
    Positional   // value
}
```

---

## 四、Tauri Commands 总览

### 项目管理 (6个)
```rust
create_project(ProjectConfig) -> Project
list_projects() -> Vec<Project>
get_project(id: Uuid) -> Project
update_project(Project) -> Project
delete_project(id: Uuid) -> ()
duplicate_project(id: Uuid) -> Project  // 深拷贝
```

### 表单管理 (9个)
```rust
create_form(FormConfig) -> Form
update_form(Form) -> Form
delete_form(id: Uuid) -> ()
add_form_item(form_id: Uuid, item: FormItem) -> FormItem
delete_form_item(item_id: Uuid) -> ()
update_form_item(FormItem) -> FormItem
reorder_form_items(form_id: Uuid, item_ids: Vec<Uuid>) -> ()
update_dropdown_options(item_id: Uuid, options: Vec<String>) -> ()
toggle_dropdown_mode(item_id: Uuid, use_dropdown: bool) -> ()
```

### 命令生成 (1个)
```rust
generate_command(form_id: Uuid) -> String
```

---

## 五、前端关键模式

### 1. IPC 调用封装

```typescript
// ✅ 正确：通过 API 层封装
// src/api/project.ts
import { invoke } from "@/lib/tauri";

export async function createProject(
  name: string,
  description: string
): Promise<Project> {
  return await invoke("create_project", {
    config: { name, description },
  });
}
```

```typescript
// ❌ 错误：组件内直接调用 invoke
import { invoke } from "@tauri-apps/api/core";
invoke("create_project", { config });
```

### 2. 组件开发模式

```vue
<script setup lang="ts">
import { ref, computed } from "vue";
import { useProjectStore } from "@/stores/project";
import { createProject } from "@/api/project";

const store = useProjectStore();
const name = ref("");
// ...
</script>

<template>
  <!-- 使用 Shadcn 组件 + Tailwind -->
  <div class="flex flex-col gap-4">
    <Input v-model="name" />
  </div>
</template>
```

### 3. 状态管理流程

```typescript
// stores/project.ts
export const useProjectStore = defineStore("project", () => {
  const projects = ref<Project[]>([]);
  const currentProject = ref<Project | null>(null);

  const fetchProjects = async () => {
    projects.value = await listProjects();
  };

  return { projects, currentProject, fetchProjects };
});
```

---

## 六、开发规范

### Rust 后端

| 规范 | 说明 |
|------|------|
| **命令返回** | 必须返回 `Result<T, String>` 或自定义 Error |
| **参数传递** | 使用 `serde::Deserialize` 结构体，禁止散参 |
| **状态管理** | 使用 `tauri::State<T>` + `Mutex/RwLock` |
| **错误处理** | 使用 `thiserror` 定义统一错误类型 |
| **类型同步** | 修改 Struct 后同步更新 `src/types/bindings.ts` |

### Vue 前端

| 规范 | 说明 |
|------|------|
| **Script Setup** | 必须使用 `<script setup lang="ts">` |
| **样式优先** | 优先使用 Tailwind，避免 `<style>` |
| **IPC 封装** | 所有调用封装在 `src/api/` 中 |
| **类型安全** | 严格 TypeScript，禁止 `any` |
| **Win11 风格** | 自定义标题栏，Segoe UI 字体，圆角设计 |

---

## 七、关键依赖

### Cargo.toml (Rust)
```
tauri v2           # 框架核心
serde/serde_json   # 序列化
thiserror/anyhow   # 错误处理
uuid               # ID 生成
chrono             # 时间处理
tokio              # 异步运行时
tauri-specta       # 类型同步 (TS 生成)
```

### package.json (Frontend)
```
vue@3              # 前端框架
typescript         # 类型系统
pinia              # 状态管理
shadcn-vue         # UI 组件库
tailwindcss        # 样式框架
vue-router         # 路由
vueuse             # 工具函数
sortablejs         # 拖拽排序
lucide-vue-next    # 图标库
```

---

## 八、开发指令

```bash
# 开发模式
pnpm tauri dev

# 构建
pnpm build

# 前端开发服务器
pnpm dev
```

---

## 九、特色实现

### 自动备份机制
```rust
// services/storage.rs
// 每次保存前创建 .bak 备份文件
fs::copy(&current_path, &backup_path)?;
```

### 命令生成逻辑
```rust
// services/command.rs
// 1. 过滤禁用和空值参数
// 2. 按 param_style 格式化
// 3. 替换 {params} 占位符
```

### 拖拽排序
```typescript
// 前端使用 SortableJS
// 后端提供 reorder_form_items() command
```

---

**参考文件**：`CLAUDE-RULE.md`（编码规范详情）
