# ArgForge Tauri 重构进度记录

## 项目概述

将 Python 版的 ArgForge（可视化终端命令生成器）完整重构成 Tauri 版本。

**核心功能**：
- 可视化配置 shell 命令参数
- 支持多种参数风格（Argparse: `--lr 0.001`, Hydra: `lr=0.001`, 位置参数）
- 项目管理（创建、编辑多个命令项目）
- 实时预览和一键复制
- 本地 JSON 存储
- 主题系统（动态主题切换）
- 拖拽排序
- 键盘快捷键

**技术栈**：
- 后端：Rust + Tauri v2
- 前端：Vue 3 + TypeScript + Vite
- UI 框架：Shadcn-vue + Tailwind CSS v3.4
- 状态管理：Pinia
- 工具库：VueUse, SortableJS, lucide-vue-next

---

## 总体进度：75% (6/8 阶段完成)

| 阶段 | 状态 | 完成度 |
|-----|-----|-------|
| ✅ 阶段 1: 项目重构与基础设施 | 完成 | 100% |
| ✅ 阶段 2: 数据模型与后端核心 | 完成 | 100% |
| ✅ 阶段 3: API 层与 Pinia 状态 | 完成 | 100% |
| ✅ 阶段 4: 项目管理 UI 组件 | 完成 | 100% |
| ✅ 阶段 5: 表单管理 | 完成 | 100% |
| ✅ 阶段 6: 参数编辑器 | 完成 | 100% |
| ⏳ 阶段 7: 主题系统 | 待开始 | 0% |
| ⏳ 阶段 8: 打磨与优化 | 待开始 | 0% |

---

## ✅ 阶段 1: 项目重构与基础设施（已完成）

### 完成时间
预计 2-3 天，实际完成

### 主要工作

#### 1. 目录结构创建
```
src-tauri/src/
├── commands/       # Tauri 命令层
│   ├── project.rs
│   ├── form.rs
│   └── command_gen.rs
├── services/       # 纯 Rust 业务逻辑
│   ├── storage.rs
│   └── command.rs
├── models/         # 数据模型
│   └── project.rs
├── error.rs        # 错误处理
├── lib.rs          # 插件注册
└── main.rs

src/
├── api/            # IPC 调用封装
├── components/
│   ├── ui/         # Shadcn 组件
│   ├── layout/     # 布局组件
│   └── features/   # 功能组件
├── stores/         # Pinia 状态
├── types/          # TS 类型定义
├── lib/            # 工具函数
└── App.vue
```

#### 2. 依赖安装
**Rust 依赖**：
- `tauri-specta = "2.0.0-rc.20"` - 类型同步
- `specta = "2.0.0-rc.22"` - 类型导出
- `thiserror = "1.0"` - 错误处理
- `anyhow = "1.0"` - 顶层错误
- `uuid = { version = "1", features = ["v4", "serde"] }` - UUID 生成
- `chrono = { version = "0.4", features = ["serde"] }` - 时间处理
- `tokio = { version = "1", features = ["fs"] }` - 异步 IO
- `tempfile = "3"` - 测试用临时文件

**前端依赖**：
- `tailwindcss@3.4.19` - 样式框架（降级以兼容 Shadcn）
- `@vueuse/core` - Vue 工具库
- `sortablejs` - 拖拽排序
- `lucide-vue-next` - 图标库
- `shadcn-vue` - UI 组件
- `class-variance-authority` - 类名变体
- `clsx` + `tailwind-merge` - 类名合并

#### 3. 配置文件
- ✅ `components.json` - Shadcn-vue 配置
- ✅ `tailwind.config.js` - Tailwind 主题配置
- ✅ `postcss.config.js` - PostCSS 配置
- ✅ `src/styles.css` - 全局样式和 CSS 变量

#### 4. 自定义标题栏
**文件**：`src/components/layout/TitleBar.vue`

**功能**：
- Win11 风格标题栏
- 窗口控制按钮（最小化/最大化/关闭）
- 拖拽区域
- Mica 材质背景支持

---

## ✅ 阶段 2: 数据模型与后端核心（已完成）

### 完成时间
预计 3-4 天，实际完成

### 主要工作

#### 1. 数据模型 (`src-tauri/src/models/project.rs`)

**ParamStyle 枚举**：
```rust
pub enum ParamStyle {
    Argparse,   // --key value
    Hydra,      // key=value
    Positional, // value only
}
```

**FormItem 结构**：
- id: String
- param_name: String
- param_value: String
- enabled: bool
- param_style: ParamStyle
- use_dropdown: bool
- dropdown_options: Vec<String>

**Form 结构**：
- id: String
- name: String
- description: String
- sort_order: i32
- updated_at: String
- command_template: String
- items: Vec<FormItem>

**Project 结构**：
- id: String
- name: String
- description: String
- created_at: String
- updated_at: String
- forms: Vec<Form>

#### 2. 错误处理 (`src-tauri/src/error.rs`)

使用 `thiserror` 定义错误类型：
- `Io` - IO 错误
- `Json` - JSON 序列化错误
- `ProjectNotFound` - 项目未找到
- `FormNotFound` - 表单未找到
- `FormItemNotFound` - 表单项未找到
- `InvalidArgument` - 无效参数
- `CommandGeneration` - 命令生成失败
- `BackupRestore` - 备份恢复失败

#### 3. 存储服务 (`src-tauri/src/services/storage.rs`)

**核心功能**：
- `save_project()` - 保存项目到 JSON 文件
- `load_project()` - 从 JSON 文件加载项目
- `delete_project()` - 删除项目文件
- `list_projects()` - 列出所有项目
- `project_exists()` - 检查项目是否存在
- `clean_backups()` - 清理备份文件

**特色功能**：
- 自动备份机制（保存前先创建 .bak 文件）
- 备份恢复逻辑（加载失败时尝试从备份恢复）
- 跨平台数据目录支持

**单元测试**：6 个测试全部通过
- ✅ `test_create_storage`
- ✅ `test_save_and_load_project`
- ✅ `test_project_exists`
- ✅ `test_delete_project`
- ✅ `test_list_projects`
- ✅ `test_backup_creation`

#### 4. 命令生成服务 (`src-tauri/src/services/command.rs`)

**核心功能**：
- `generate_command()` - 生成完整命令字符串
- `generate_params_string()` - 生成参数部分
- `format_item()` - 格式化单个参数项

**支持 3 种参数风格**：
- Argparse: `--lr 0.001`
- Hydra: `lr=0.001`
- Positional: `0.001`

**单元测试**：7 个测试全部通过
- ✅ `test_generate_command_argparse`
- ✅ `test_generate_command_hydra`
- ✅ `test_generate_command_positional`
- ✅ `test_generate_command_mixed`
- ✅ `test_disabled_item_excluded`
- ✅ `test_empty_value_excluded`
- ✅ 文档测试

#### 5. Tauri Commands

**project.rs**（6 个命令）：
- ✅ `create_project` - 创建新项目
- ✅ `list_projects` - 获取所有项目
- ✅ `get_project` - 获取单个项目
- ✅ `update_project` - 更新项目信息
- ✅ `delete_project` - 删除项目
- ✅ `duplicate_project` - 深拷贝项目

**form.rs**（8 个命令）：
- ✅ `create_form` - 创建表单
- ✅ `update_form` - 更新表单信息
- ✅ `delete_form` - 删除表单
- ✅ `update_form_item` - 更新表单项字段
- ✅ `add_form_item` - 添加新表单项
- ✅ `delete_form_item` - 删除表单项
- ✅ `reorder_form_items` - 拖拽排序
- ✅ `update_dropdown_options` - 更新下拉选项
- ✅ `toggle_dropdown_mode` - 切换下拉/手动模式

**command_gen.rs**（1 个命令）：
- ✅ `generate_command` - 生成命令字符串

#### 6. 类型绑定

由于 tauri-specta RC 版本 API 不稳定，采用手动方式创建类型定义：
- ✅ `src/types/bindings.ts` - 完整的 TypeScript 类型定义

#### 7. 命令注册

在 `src-tauri/src/lib.rs` 中注册所有 15 个命令，并初始化 StorageService。

---

## ✅ 阶段 3: API 层与 Pinia 状态（已完成）

### 完成时间
预计 2-3 天，实际完成

### 主要工作

#### 1. API 层封装

**`src/api/project.ts`**（6 个函数）：
- ✅ `createProject()` - 创建项目
- ✅ `listProjects()` - 列出所有项目
- ✅ `getProject()` - 获取单个项目
- ✅ `updateProject()` - 更新项目
- ✅ `deleteProject()` - 删除项目
- ✅ `duplicateProject()` - 复制项目

**`src/api/form.ts`**（9 个函数）：
- ✅ `createForm()` - 创建表单
- ✅ `updateForm()` - 更新表单
- ✅ `deleteForm()` - 删除表单
- ✅ `updateFormItem()` - 更新表单项
- ✅ `addFormItem()` - 添加表单项
- ✅ `deleteFormItem()` - 删除表单项
- ✅ `reorderFormItems()` - 重新排序
- ✅ `updateDropdownOptions()` - 更新下拉选项
- ✅ `toggleDropdownMode()` - 切换下拉模式

**`src/api/command.ts`**（1 个函数）：
- ✅ `generateCommand()` - 生成命令字符串

#### 2. Pinia 状态管理

**`src/stores/project.ts`**（项目状态）：
- ✅ 状态：`projects`, `currentProject`, `currentForm`
- ✅ 计算属性：`currentForms`, `hasProjects`
- ✅ Actions：
  - `loadProjects()` - 加载项目列表
  - `createProject()` - 创建项目
  - `updateProject()` - 更新项目
  - `deleteProject()` - 删除项目
  - `duplicateProject()` - 复制项目
  - `setCurrentProject()` - 设置当前项目
  - `createForm()` - 创建表单
  - `updateForm()` - 更新表单
  - `deleteForm()` - 删除表单
  - `setCurrentForm()` - 设置当前表单
  - `$reset()` - 重置状态

**`src/stores/ui.ts`**（UI 状态）：
- ✅ 状态：`sidebarCollapsed`, `isReorderMode`, `saveStatus`, `saveMessage`, `isLoading`, `showDialog`, `dialogType`
- ✅ Actions：
  - `toggleSidebar()` / `setSidebarCollapsed()`
  - `toggleReorderMode()` / `setReorderMode()`
  - `setSaveStatus()` - 带自动重置
  - `setLoading()`
  - `openDialog()` / `closeDialog()`
  - `$reset()` - 重置状态

---

## ✅ 阶段 4: 项目管理 UI 组件（已完成）

### 完成时间
预计 4-5 天，实际完成

### 主要工作

#### 1. Shadcn-vue 组件安装

已安装的组件（30+ 个文件）：
- ✅ button
- ✅ input
- ✅ textarea
- ✅ card（Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter）
- ✅ dialog（Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogTrigger, DialogClose, DialogScrollContent）
- ✅ scroll-area（ScrollArea, ScrollBar）
- ✅ separator
- ✅ switch
- ✅ label
- ✅ dropdown-menu（15 个子组件）

#### 2. 功能组件创建

**`src/components/features/ProjectCard.vue`**：
- ✅ 显示项目名称、描述、表单数量
- ✅ Hover 效果（阴影、边框高亮）
- ✅ 点击事件
- ✅ 集成下拉菜单

**`src/components/features/ProjectCardDropdown.vue`**：
- ✅ 编辑项目
- ✅ 复制项目
- ✅ 删除项目（红色警告）

**`src/components/features/CreateProjectDialog.vue`**：
- ✅ 项目名称输入（必填）
- ✅ 项目描述输入（可选）
- ✅ 表单验证
- ✅ 加载状态
- ✅ 成功后自动关闭

**`src/components/features/EditProjectDialog.vue`**：
- ✅ 编辑项目信息
- ✅ 显示创建时间和表单数量
- ✅ 复制项目按钮
- ✅ 删除项目按钮（带二次确认对话框）
- ✅ 加载状态

**`src/components/features/ProjectList.vue`**：
- ✅ 头部（标题 + 新建按钮）
- ✅ 空状态提示（暂无项目时显示）
- ✅ 网格布局（响应式：1/2/3 列）
- ✅ 项目卡片渲染
- ✅ 整合所有子组件
- ✅ 集成 Pinia store

#### 3. 主应用集成

**`src/App.vue`** 更新：
- ✅ 集成 TitleBar 组件
- ✅ 集成 ProjectList 组件
- ✅ 移除占位内容

### 测试状态

🎉 **Tauri 开发服务器正在运行**：http://localhost:1420

可测试功能：
- ✅ 创建新项目
- ✅ 查看项目列表
- ✅ 编辑项目信息
- ✅ 复制项目
- ✅ 删除项目（带确认）
- ✅ 空状态显示

---

## ✅ 阶段 5: 表单管理（已完成）

### 完成时间
2026-01-23

### 主要工作

#### 1. 路由系统实现

**依赖安装**：
- ✅ `vue-router@4` - Vue 官方路由管理器

**路由配置** (`src/router/index.ts`)：
- ✅ `/` - 项目列表页（ProjectList）
- ✅ `/projects/:id` - 项目详情页（ProjectDetail）
- ✅ 404 重定向到首页

**路由集成**：
- ✅ `src/main.ts` - 注册 router
- ✅ `src/App.vue` - 使用 `<router-view>`
- ✅ `src/components/features/ProjectList.vue` - 集成路由导航

#### 2. 功能组件创建

**`src/components/features/FormListItem.vue`**（102 行）：
- ✅ 显示表单名称、描述、更新时间
- ✅ 激活状态高亮样式
- ✅ Hover 效果
- ✅ 智能时间格式化（今天/昨天/N天前）

**`src/components/features/CreateFormDialog.vue`**（130 行）：
- ✅ 表单名称输入（必填）
- ✅ 表单描述输入（可选）
- ✅ 命令模板输入（可选，支持 `{params}` 占位符说明）
- ✅ 表单验证
- ✅ 加载状态

**`src/components/features/EditFormDialog.vue`**（130 行）：
- ✅ 编辑表单信息
- ✅ 预填充表单数据
- ✅ 支持修改命令模板
- ✅ 加载状态

**`src/components/features/FormDetailPreview.vue`**（150 行）：
- ✅ 表单头部信息展示
- ✅ 命令模板显示（代码块）
- ✅ 表单项列表只读展示
  - 参数名、参数值
  - 参数风格标识（Badge）
  - 启用/禁用状态（Switch）
  - 下拉选项提示
- ✅ 编辑按钮

**`src/components/features/ProjectDetail.vue`**（220 行）：
- ✅ 顶部导航栏
  - 返回按钮
  - 项目名称和表单数量
  - 编辑项目按钮
- ✅ 左右分割布局（Tailwind Flexbox）
  - 左侧：表单列表（30%）
  - 右侧：表单详情（70%）
- ✅ 表单列表区
  - 新建表单按钮
  - 滚动区域
  - 空状态提示
- ✅ 集成所有子组件

#### 3. 状态管理更新

**`src/stores/project.ts`** 更新：
- ✅ 新增 `loadProject(projectId)` - 加载单个项目的完整数据
- ✅ 重构 `createForm(data)` - 支持命令模板参数
- ✅ 重构 `updateForm(formId, data)` - 使用对象参数
- ✅ 导出 `loadProject` 方法

#### 4. 技术实现

**布局方案**：
- 使用 Tailwind Flexbox 固定比例分割
- 左侧：`w-[30%] min-w-[250px]`
- 右侧：`flex-1`
- 使用 ScrollArea 实现内容滚动

**数据流**：
```
路由参数 projectId
  → store.loadProject()
  → API: getProject(projectId)
  → 设置 currentProject
  → 组件响应式更新
```

#### 5. Shadcn 组件

新增安装：
- ✅ Badge - 参数风格标识

### 测试状态

✅ **功能测试通过**：
- ✅ 路由导航正常（项目列表 ↔ 项目详情）
- ✅ 创建表单功能正常
- ✅ 编辑表单功能正常
- ✅ 表单列表展示正常
- ✅ 表单详情预览正常
- ✅ 空状态显示正常

✅ **编译状态**：
- ✅ Vite 前端编译成功
- ✅ Rust 后端编译成功
- ⚠️ 1 个无关警告（`get_data_dir` 未使用）

---

## ✅ 阶段 6: 参数编辑器（已完成）

### 完成时间
2026-01-23

### 主要工作

#### 6.1 Store 增强 (`src/stores/project.ts`)
- ✅ **`addFormItem`** - 添加新参数项
- ✅ **`updateFormItem(itemId, field, value)`** - 更新参数字段
- ✅ **`deleteFormItem(itemId)`** - 删除参数项
- ✅ **`reorderFormItems(oldIndex, newIndex)`** - 拖拽排序
- ✅ **`updateDropdownOptions(itemId, options)`** - 更新下拉选项
- ✅ **`toggleDropdownMode(itemId, enabled)`** - 切换下拉模式

#### 6.2 核心组件创建

**`FormItemEditor.vue`**（172 行）：
- ✅ 拖拽手柄（GripVertical 图标）
- ✅ 启用/禁用 Switch
- ✅ 参数名 Input（等宽字体）
- ✅ 参数值 Input/Select（下拉模式切换）
- ✅ 参数风格 Select（Argparse/Hydra/Positional）
- ✅ 下拉模式切换按钮
- ✅ 管理下拉选项按钮
- ✅ 删除按钮（红色悬停效果）

**`DropdownOptionsDialog.vue`**（173 行）：
- ✅ 选项列表（可编辑）
- ✅ 添加/删除选项按钮
- ✅ 快捷填充（布尔值/学习率/优化器）
- ✅ 至少保留一个选项校验
- ✅ 保存到 Store

**`FormDetailEditor.vue`**（190 行）：
- ✅ 表单头部信息展示
- ✅ 实时命令预览卡片
- ✅ 拖拽参数列表（SortableJS）
- ✅ 添加参数按钮
- ✅ 空状态提示
- ✅ 复制命令到剪贴板

#### 6.3 Composables 实现

**`useCommandPreview.ts`**（63 行）：
- ✅ 监听表单变化（深度 watch）
- ✅ 300ms 防抖调用后端生成命令
- ✅ 返回 `commandPreview` 和 `isGenerating` 状态

**`useFormItems.ts`**（72 行）：
- ✅ 封装所有参数项 CRUD 操作
- ✅ 简化组件逻辑

#### 6.4 集成与修复

**`ProjectDetail.vue`** 更新：
- ✅ 将 `FormDetailPreview` 替换为 `FormDetailEditor`
- ✅ 完整的编辑模式集成

**`src/lib/tauri.ts`** 修复：
- ✅ 修复 TypeScript 类型冲突（ImportMeta）
- ✅ 修复 invokeCache 泛型问题

#### 6.5 技术实现

**拖拽排序**：
- 使用 SortableJS 实现
- `.drag-handle` 作为拖拽手柄
- `onEnd` 回调同步到后端
- 150ms 动画过渡

**实时预览**：
- 使用 `@vueuse/core` 的 `useDebounceFn` 实现防抖
- 避免频繁 IPC 调用

**状态同步**：
- 所有修改立即更新 `currentForm`
- 同步更新 `currentProject.forms` 列表
- 确保数据一致性

### 测试状态

✅ **编译测试通过**：
- Rust 单元测试：16/16 通过
- TypeScript 编译：0 错误
- Vite 构建：成功

✅ **功能测试**：
- 参数 CRUD（创建/读取/更新/删除）
- 拖拽排序
- 实时命令预览
- 下拉模式切换
- 下拉选项管理

---

## ⏳ 阶段 7: 主题系统（待开始）

### 预计时间
2-3 天

### 待完成任务

#### 7.1 Rust 端

- [ ] 创建 `ColorTheme` 结构（23 个颜色字段）
- [ ] 实现 `ThemeService`
  - TOML 主题文件加载/保存
  - 主题切换逻辑
- [ ] 实现 3 个 Tauri Commands：
  - `get_current_theme`
  - `list_themes`
  - `switch_theme`

#### 7.2 前端端

- [ ] 配置 Tailwind CSS 主题（动态 CSS 变量）
- [ ] 实现 `ThemeSwitcher.vue` 组件
- [ ] 实现 `useTheme.ts` composable
- [ ] 应用 CSS 变量逻辑

#### 7.3 主题配置

- [ ] 迁移 Python 版默认主题
- [ ] 支持 3-5 种预设主题：
  - 默认亮色
  - 默认暗色
  - Win11 蓝色
  - forest 绿色
  - 自定义主题

---

## ⏳ 阶段 8: 打磨与优化（待开始）

### 预计时间
3-4 天

### 待完成任务

#### 8.1 键盘快捷键

- [ ] Ctrl+N：新建项目
- [ ] Ctrl+S：保存（自动保存，无需手动）
- [ ] Ctrl+C：复制命令
- [ ] Delete：删除项目/表单/参数
- [ ] Escape：返回上一级

#### 8.2 用户体验优化

- [ ] Toast 提示（保存成功/错误提示）
- [ ] 空状态页面（无项目/无表单时的提示）
- [ ] 加载状态（Skeleton）
- [ ] 确认对话框（删除操作）

#### 8.3 性能优化

- [ ] 虚拟滚动（表单项 >50 时）
- [ ] 防抖保存（避免频繁文件 I/O）
- [ ] 懒加载组件

#### 8.4 Win11 细节优化

- [ ] Mica 材质背景
- [ ] 圆角统一（8px）
- [ ] Segoe UI 字体
- [ ] Hover/Active 动画过渡（200ms ease-out）
- [ ] 细窄滚动条

#### 8.5 打包配置

- [ ] Windows 安装包（NSIS）
- [ ] 图标配置
- [ ] 应用签名（可选）

#### 8.6 文档

- [ ] README 用户文档
- [ ] 快捷键列表
- [ ] 常见问题

---

## 测试验证

### 已完成的测试

- ✅ **单元测试**：16 个测试全部通过
  - models::project::tests - 3 个
  - services::storage::tests - 6 个
  - services::command::tests - 7 个

- ✅ **编译测试**：Rust 代码无错误编译
  - 只有 1 个无关紧要的警告（`get_data_dir` 未使用）

- ✅ **运行测试**：Tauri 开发服务器成功启动
  - Vite: http://localhost:1420
  - 后端 15 个命令全部注册

- ✅ **功能测试**（阶段 4 + 5）：
  - 项目管理（创建、编辑、删除、复制）
  - 表单管理（创建、编辑、查看详情）
  - 路由导航（项目列表 ↔ 项目详情）
  - 表单列表切换和详情预览

### 待完成的测试

- [ ] 端到端测试（创建项目到生成命令）
- [ ] 性能测试（100+ 参数项）
- [ ] 边界情况测试（空值/特殊字符）

---

## 风险与问题

### 已解决的问题

1. **Tailwind v4 不兼容** → 降级到 v3.4.19
2. **tauri-specta RC 版本 API 不稳定** → 改用手动类型定义
3. **嵌套 match 语句解析错误** → 改用 if let
4. **Borrow checker 错误** → 使用 clone() 解决

### 潜在风险

1. **拖拽排序性能问题** → 计划使用虚拟滚动
2. **文件 I/O 错误** → 已实现备份恢复机制
3. **Win11 Mica 材质兼容性** → 可降级到纯色背景

---

## 下一步工作

### 立即可做

1. **测试当前功能**
   - ✅ 项目管理（已测试）
   - ✅ 表单管理（已测试）
   - 继续完善细节交互

2. **继续阶段 6**
   - 参数编辑器组件
   - 拖拽排序功能
   - 实时命令生成

### 技术债务

- [ ] 清理 `get_data_dir` 未使用警告
- [ ] 添加更多错误提示（Toast）
- [ ] 完善单元测试覆盖率

---

## 文件清单

### 关键文件（5 个）

1. **src-tauri/src/lib.rs**
   - 所有 Tauri Commands 的注册入口
   - 状态管理（StorageService）

2. **src-tauri/src/models/project.rs**
   - 核心数据模型定义
   - TypeScript 类型导出的基础

3. **src-tauri/src/services/storage.rs**
   - JSON 文件读写逻辑
   - 备份恢复机制

4. **src/components/features/ProjectList.vue**
   - 项目管理主组件
   - 集成所有子组件

5. **src/stores/project.ts**
   - 项目状态管理
   - 业务逻辑层

### 完整文件树

```
arg-forge/
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   │   ├── project.rs         # 6 个命令
│   │   │   ├── form.rs            # 9 个命令
│   │   │   └── command_gen.rs     # 1 个命令
│   │   ├── services/
│   │   │   ├── storage.rs         # 存储服务 + 测试
│   │   │   └── command.rs         # 命令生成 + 测试
│   │   ├── models/
│   │   │   └── project.rs         # 数据模型 + 测试
│   │   ├── error.rs               # 错误处理
│   │   ├── lib.rs                 # 命令注册
│   │   └── main.rs
│   ├── Cargo.toml
│   └── build.rs
├── src/
│   ├── api/
│   │   ├── project.ts             # 6 个函数
│   │   ├── form.ts                # 9 个函数
│   │   └── command.ts             # 1 个函数
│   ├── router/
│   │   └── index.ts               # 路由配置 ⭐ 新增
│   ├── components/
│   │   ├── ui/                    # Shadcn 组件（30+ 文件）
│   │   ├── layout/
│   │   │   └── TitleBar.vue
│   │   └── features/
│   │       ├── ProjectCard.vue
│   │       ├── ProjectCardDropdown.vue
│   │       ├── CreateProjectDialog.vue
│   │       ├── EditProjectDialog.vue
│   │       ├── ProjectList.vue
│   │       ├── FormListItem.vue
│   │       ├── CreateFormDialog.vue
│   │       ├── EditFormDialog.vue
│   │       ├── FormItemEditor.vue         # ⭐ 阶段6新增
│   │       ├── DropdownOptionsDialog.vue  # ⭐ 阶段6新增
│   │       ├── FormDetailEditor.vue       # ⭐ 阶段6新增
│   │       └── ProjectDetail.vue          # 已更新（使用FormDetailEditor）
│   ├── composables/
│   │   ├── useCommandPreview.ts          # ⭐ 阶段6新增
│   │   └── useFormItems.ts               # ⭐ 阶段6新增
│   ├── stores/
│   │   ├── project.ts             # 项目状态（阶段6更新：新增FormItem操作）
│   │   └── ui.ts                  # UI 状态
│   ├── types/
│   │   └── bindings.ts            # TS 类型定义
│   ├── lib/
│   │   ├── tauri.ts               # Tauri API封装（阶段6修复）
│   │   └── utils.ts               # 工具函数
│   ├── styles.css
│   ├── App.vue                    # 已更新（集成路由）
│   └── main.ts                    # 已更新（注册路由）
├── components.json                # Shadcn 配置
├── tailwind.config.js
├── postcss.config.js
├── vite.config.ts
├── package.json
└── PROGRESS.md                    # 本文件
```

---

## 最后更新

**时间**：2026-01-23
**完成阶段**：6/8（75%）
**当前状态**：参数编辑器功能完成，支持 CRUD、拖拽排序、实时命令预览
**下一步**：阶段 7 - 主题系统（可选）或 阶段 8 - 打磨与优化
