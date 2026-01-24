# ArgForge 编码规范

> 聚焦最容易出问题的点，确保代码质量

---

## 一、通用原则

### 类型安全
- **Rust**: 禁止使用 `unwrap()`，优先使用 `?` 传播错误
- **TypeScript**: 禁止使用 `any`，必须明确类型定义
- **类型同步**: 修改 Rust Struct 后必须同步更新 `src/types/bindings.ts`

### 错误处理
- **Rust**: 使用统一的 `AppError` 类型（基于 `thiserror`）
- **Frontend**: 使用 try-catch 捕获错误，通过 `console.error` 记录
- **用户反馈**: 错误信息必须对用户友好，避免技术术语

### 命名约定
- **Rust**: `snake_case`（函数/变量）、`PascalCase`（类型/枚举）
- **TypeScript**: `camelCase`（变量/函数）、`PascalCase`（类型/接口）
- **常量**: `UPPER_SNAKE_CASE`

---

## 二、Rust 后端规范

### 易错点 1: 禁止 `unwrap()`

```rust
// 正确 - 使用 ? 传播错误
let project = storage.load_project(&id)?;

// 错误 - unwrap 会 panic
let project = storage.load_project(&id).unwrap();
```

### 易错点 2: Command 参数必须使用结构体

```rust
// 正确 - 使用结构体
#[derive(Deserialize)]
pub struct CreateProjectConfig {
    pub name: String,
    pub description: String,
}

#[tauri::command]
pub async fn create_project(
    config: CreateProjectConfig,
) -> Result<Project, String>

// 错误 - 散参
#[tauri::command]
pub async fn create_project(
    name: String,
    description: String,
) -> Result<Project, String>
```

### 易错点 3: 返回类型规范

必须返回 `Result<T, String>` 或自定义 Result：

```rust
// 正确
#[tauri::command]
pub async fn get_project(id: String) -> Result<Project, String>

// 自定义 Result 类型
pub type Result<T> = std::result::Result<T, AppError>;
```

### 易错点 4: 错误处理模式

使用 `thiserror` 定义错误：

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("项目未找到: {0}")]
    ProjectNotFound(String),

    #[error("无效参数: {0}")]
    InvalidArgument(String),
}
```

### 易错点 5: 文档注释规范

必须使用三斜线注释，包含功能说明和参数：

```rust
/// 创建新项目
///
/// # 参数
/// * `config` - 项目配置 (名称和描述)
///
/// # 返回
/// 创建的项目对象
///
/// # 错误
/// - 项目名称为空时返回错误
#[tauri::command]
pub async fn create_project(
    config: CreateProjectConfig,
) -> Result<Project, String>
```

### 代码组织

```
src-tauri/src/
  commands/    # Tauri Commands（API 层）
  services/    # 业务逻辑层（纯 Rust，无 Tauri 依赖）
  models/      # 数据结构定义
  error.rs     # 统一错误定义
  lib.rs       # Commands 注册
```

**职责分离**：
- **Commands**: 处理 IPC 调用，参数验证
- **Services**: 核心业务逻辑，可独立测试
- **Models**: 数据结构，序列化/反序列化

---

## 三、Vue 前端规范

### 易错点 1: 禁止 `any`

```typescript
// 正确 - 明确类型
interface ProjectConfig {
  name: string;
  description: string;
}

async function createProject(config: ProjectConfig): Promise<Project> {
  // ...
}

// 错误 - 使用 any
async function createProject(config: any): Promise<any> {
  // ...
}
```

### 易错点 2: IPC 调用必须封装

所有 IPC 调用必须封装在 `src/api/` 中：

```typescript
// 正确 - src/api/project.ts
import { invoke } from "@tauri-apps/api/core";

export async function createProject(
  name: string,
  description: string
): Promise<Project> {
  return await invoke("create_project", {
    config: { name, description },
  });
}

// 错误 - 组件内直接调用
import { invoke } from "@tauri-apps/api/core";
invoke("create_project", { name, description });
```

**封装函数必须包含 JSDoc**：

```typescript
/**
 * 创建新项目
 * @param name - 项目名称
 * @param description - 项目描述
 * @returns 创建的项目对象
 * @throws {Error} 项目名称为空时抛出错误
 */
export async function createProject(
  name: string,
  description: string
): Promise<Project>
```

### 易错点 3: 必须使用 Script Setup

```vue
<!-- 正确 -->
<script setup lang="ts">
import { ref } from "vue";
import { useProjectStore } from "@/stores/project";

const name = ref("");
</script>

<!-- 错误 -->
<script>
export default {
  data() {
    return { name: "" };
  }
};
</script>
```

### 易错点 4: 类型同步

修改 Rust Struct 后必须同步 `src/types/bindings.ts`：

```bash
# 重新生成类型定义
pnpm tauri build
```

### 状态管理

使用 Pinia 进行状态管理：

```typescript
// stores/project.ts
export const useProjectStore = defineStore("project", () => {
  const projects = ref<Project[]>([]);

  async function loadProjects() {
    projects.value = await projectApi.listProjects();
  }

  return { projects, loadProjects };
});
```

### 样式规范

优先使用 Tailwind，避免手写 CSS：

```vue
<!-- 正确 -->
<div class="flex items-center gap-4 p-4 rounded-lg bg-white">
  <Input v-model="name" class="flex-1" />
</div>

<!-- 错误 -->
<style scoped>
.container {
  display: flex;
  align-items: center;
  gap: 1rem;
}
</style>
```

---

## 四、Git 规范

### 提交信息格式

遵循 **Conventional Commits**：

```
<type>(<scope>): <subject>
```

**类型**：
- `feat`: 新功能
- `fix`: Bug 修复
- `refactor`: 重构（不改变功能）
- `docs`: 文档更新
- `test`: 测试相关
- `chore`: 构建/工具配置

**示例**：

```
feat(project): 添加项目复制功能

- 实现 duplicate_project command
- 前端添加复制按钮和确认对话框

Closes #123
```

### 分支策略

```
main          - 主分支
  ├── develop - 开发分支
  └── feature/* - 功能分支
  └── fix/*     - 修复分支
```

---

## 五、代码审查清单

### Rust 后端
- [ ] 所有公共函数都有文档注释
- [ ] 使用 `AppError` 而非 `String` 作为错误类型
- [ ] 避免使用 `unwrap()`，使用 `?` 传播错误
- [ ] Command 参数使用 Struct 而非散参
- [ ] 类型定义同步到 `src/types/bindings.ts`

### Vue 前端
- [ ] 组件使用 `<script setup lang="ts">`
- [ ] 无 `any` 类型使用
- [ ] IPC 调用封装在 `src/api/` 中
- [ ] API 函数有 JSDoc 注释
- [ ] 优先使用 Tailwind 而非内联样式

### 通用
- [ ] 代码通过 `cargo clippy` 检查
- [ ] 代码通过 `pnpm build` 类型检查
- [ ] 提交信息遵循 Conventional Commits
- [ ] 无 `console.log` 调试代码残留

---

## 六、开发工作流

### 启动开发环境

```bash
# 安装依赖
pnpm install

# 启动 Tauri 开发模式
pnpm tauri dev
```

### 代码检查

```bash
# Rust 代码检查
cd src-tauri
cargo clippy
cargo fmt --check

# TypeScript 类型检查
pnpm build
```

### 构建生产版本

```bash
# 构建前端和后端
pnpm build

# 构建桌面应用
pnpm tauri build -- --release
```
