开发范式指南

## 1. 核心架构哲学 (The "Vibe")

*   **UI 优先 (Frontend First):** 界面必须呈现 Windows 11 原生质感（Mica 材质、圆角、Segoe UI 字体）。
*   **Rust 沉底 (Rust as Core):** Rust 仅负责重业务逻辑（SSH 连接、加密存储、文件 I/O），不处理 UI 状态。
*   **严格类型桥接 (Strict IPC):** 前后端通信必须有严格的 TypeScript 定义，禁止使用 `any`。

## 2. 技术栈选型 (The Stack)

*   **Core:** Tauri v2 (性能与安全性最佳)
*   **Frontend:** Vue 3 + TypeScript + Vite
*   **State Management:** Pinia (用于 UI 状态) + VueUse (通用 Hooks)
*   **UI Framework:** **Shadcn-vue** (配合 Tailwind CSS) + **Lucide Icons**。
    *   *理由：Shadcn 提供源码级控制，非常适合 AI 修改组件细节以匹配 Win11 风格。*
*   **Backend:** Rust
    *   SSH: `russh` 或 `ssh2`
    *   Async: `tokio`
    *   Error Handling: `thiserror` (库) + `anyhow` (顶层)
    *   Serialization: `serde` + `serde_json`

## 3. 目录结构规范

AI 生成代码时必须遵循此结构，保持上下文清晰：

```text
/src-tauri
  /src
    /commands       # 所有的 Tauri Commands 单独分文件
      mod.rs
      ssh.rs        # SSH 相关指令
      store.rs      # 数据持久化指令
    /services       # 纯 Rust 业务逻辑 (不含 Tauri 依赖)
    lib.rs          # 插件注册与入口
    main.rs
/src
  /components
    /ui             # Shadcn 组件
    /layout         # 布局 (Sidebar, Titlebar)
  /composables      # 逻辑复用 (useTerminal, useSSH)
  /stores           # Pinia 状态
  /types            # TS 类型定义 (需与 Rust Struct 对应)
  App.vue
```

## 4. 编码规范 (Coding Standards)

### 4.1. 前端 (Vue + TS)

*   **Script Setup:** 必须使用 `<script setup lang="ts">`。
*   **Tailwind 优先:** 样式尽量使用 Tailwind Utility Classes，避免手写 `<style>`。
*   **Win11 风格:**
    *   窗口背景透明，使用 Tauri 的 `WindowEffects` 实现 Mica 效果。
    *   自定义标题栏 (Titlebar)，隐藏系统原生标题栏。
    *   字体堆栈：`font-family: "Segoe UI Variable", "Segoe UI", sans-serif;`
*   **调用 Rust:**
    *   所有的 IPC 调用封装在 `src/api` 文件夹中，不要在组件内直接调用 `invoke`。

**❌ 错误示范:**
```typescript
// Component.vue
import { invoke } from "@tauri-apps/api/core";
invoke('connect_ssh', { ip: '1.1.1.1' });
```

**✅ 正确示范:**
```typescript
// src/api/ssh.ts
export async function connectSsh(host: HostConfig): Promise<SessionId> {
  return await invoke('connect_ssh', { config: host });
}
```

### 4.2. 后端 (Rust)

*   **命令模式:** 每个 `tauri::command` 必须返回 `Result<T, String>` (或自定义 Error 类型)。
*   **参数传递:** 所有参数必须通过 `serde::Deserialize` 的 Struct 传递，不要传散乱的参数。
*   **状态管理:** 使用 `tauri::State<T>` 管理全局状态 (如 SSH 连接池)，配合 `Mutex` 或 `RwLock`。

**Rust 范例:**
```rust
#[derive(serde::Deserialize)]
pub struct SshConfig {
    pub ip: String,
    pub user: String,
    // ...
}

#[tauri::command]
pub async fn connect_ssh(
    state: tauri::State<'_, SshState>, 
    config: SshConfig
) -> Result<String, String> {
    // Logic here
}
```

### 4.3. 自动类型同步 (Killer Feature)

*   **强制要求:** 使用 `tauri-specta` 或手动维护，**确保 Rust 的 Struct 修改后，AI 必须同步更新前端的 `/types/*.ts` 文件**。



