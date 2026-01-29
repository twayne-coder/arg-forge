# 自定义右键菜单实现指南

## 概述

本指南说明如何在 ArgForge 项目中实现自定义右键菜单。由于已全局禁用原生浏览器右键菜单，您需要在组件层面自行实现右键菜单功能。

本项目已具备完整的下拉菜单组件系统（基于 `reka-ui` 的 `DropdownMenu`），推荐基于现有组件实现自定义右键菜单。

---

## 快速开始

### 最简实现

```vue
<template>
  <div @contextmenu="handleContextMenu">
    右键点击此区域
  </div>

  <!-- 右键菜单 -->
  <div
    v-if="menuVisible"
    :style="{ position: 'fixed', left: `${menuX}px`, top: `${menuY}px`, zIndex: 9999 }"
    @click="menuVisible = false"
  >
    <DropdownMenuItem @click="handleAction">
      操作项
    </DropdownMenuItem>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const menuVisible = ref(false);
const menuX = ref(0);
const menuY = ref(0);

function handleContextMenu(e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();
  menuX.value = e.clientX;
  menuY.value = e.clientY;
  menuVisible.value = true;
}

function handleAction() {
  menuVisible.value = false;
  // 执行操作
}
</script>
```

---

## 完整示例：ProjectCard 右键菜单

以下是一个完整的、可直接使用的实现示例，展示如何在 `ProjectCard` 组件中添加右键菜单。

### 1. 模板部分

```vue
<template>
  <Card
    class="group cursor-pointer transition-all duration-200 hover:shadow-lg hover:border-primary/50"
    @click="handleClick"
    @contextmenu="handleContextMenu"
  >
    <CardHeader>
      <CardTitle class="flex items-center justify-between text-lg">
        <span class="truncate">{{ project.name }}</span>
        <!-- 保留原有的三点菜单 -->
        <ProjectCardDropdown
          :project="project"
          @edit="handleEdit"
          @duplicate="handleDuplicate"
          @delete="handleDelete"
        />
      </CardTitle>
      <CardDescription class="line-clamp-3">
        {{ project.description || $t('project.noDescription') }}
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div class="flex items-center gap-2 text-sm text-muted-foreground">
        <FolderOpenIcon class="h-4 w-4" />
        <span>{{ $t('project.formCount', { count: project.forms.length }) }}</span>
      </div>
    </CardContent>
  </Card>

  <!-- 右键菜单（使用绝对定位） -->
 Teleport to="body"
    <div
      v-if="contextMenuVisible"
      :style="{
        position: 'fixed',
        left: `${adjustedPosition.x}px`,
        top: `${adjustedPosition.y}px`,
        zIndex: 9999,
      }"
      class="min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md"
      @click.stop
    >
      <div
        class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="handleEdit"
      >
        <PencilIcon class="mr-2 h-4 w-4" />
        {{ $t('common.edit') }}
      </div>
      <div
        class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="handleDuplicate"
      >
        <CopyIcon class="mr-2 h-4 w-4" />
        {{ $t('common.duplicate') }}
      </div>
      <div class="my-1 h-px bg-muted" />
      <div
        class="relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground text-red-500"
        @click="handleDelete"
      >
        <TrashIcon class="mr-2 h-4 w-4" />
        {{ $t('common.delete') }}
      </div>
    </div>
  </Teleport>
</template>
```

### 2. 脚本部分

```vue
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import type { Project } from "@/types/bindings";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  PencilIcon,
  CopyIcon,
  TrashIcon,
  FolderOpenIcon,
} from "lucide-vue-next";
import ProjectCardDropdown from "./ProjectCardDropdown.vue";

/** 项目对象 */
const props = defineProps<{
  project: Project;
}>();

/** 点击事件 */
const emit = defineEmits<{
  (e: "click", project: Project): void;
  (e: "edit", project: Project): void;
  (e: "duplicate", project: Project): void;
  (e: "delete", project: Project): void;
}>();

// 右键菜单状态
const contextMenuVisible = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });

/** 处理右键菜单 */
function handleContextMenu(e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();

  contextMenuPosition.value = { x: e.clientX, y: e.clientY };
  contextMenuVisible.value = true;
}

/** 关闭右键菜单 */
function closeContextMenu() {
  contextMenuVisible.value = false;
}

/** 计算调整后的菜单位置（防止超出视口） */
const adjustedPosition = computed(() => {
  const { x, y } = contextMenuPosition.value;
  const menuWidth = 200; // 预估菜单宽度
  const menuHeight = 150; // 预估菜单高度

  let adjustedX = x;
  let adjustedY = y;

  // 检测右边界
  if (x + menuWidth > window.innerWidth) {
    adjustedX = Math.max(0, x - menuWidth);
  }

  // 检测底边界
  if (y + menuHeight > window.innerHeight) {
    adjustedY = Math.max(0, y - menuHeight);
  }

  return { x: adjustedX, y: adjustedY };
});

/** 处理卡片点击 */
function handleClick() {
  emit("click", props.project);
}

/** 处理编辑 */
function handleEdit() {
  closeContextMenu();
  emit("edit", props.project);
}

/** 处理复制 */
function handleDuplicate() {
  closeContextMenu();
  emit("duplicate", props.project);
}

/** 处理删除 */
function handleDelete() {
  closeContextMenu();
  emit("delete", props.project);
}

// ====== 全局事件监听（自动关闭菜单） ======

function handleClickOutside(e: MouseEvent) {
  if (!contextMenuVisible.value) return;

  // 检查点击是否在菜单外部
  const target = e.target as Node;
  // 简单处理：点击任何地方都关闭菜单
  closeContextMenu();
}

function handleContextMenuAgain(e: MouseEvent) {
  if (!contextMenuVisible.value) return;

  // 再次右键时，关闭旧菜单（新位置会打开新菜单）
  closeContextMenu();
}

function handleEscape(e: KeyboardEvent) {
  if (e.key === "Escape" && contextMenuVisible.value) {
    closeContextMenu();
  }
}

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
  document.addEventListener("contextmenu", handleContextMenuAgain);
  document.addEventListener("keydown", handleEscape);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
  document.removeEventListener("contextmenu", handleContextMenuAgain);
  document.removeEventListener("keydown", handleEscape);
});
</script>
```

---

## 核心实现细节

### 1. 事件处理

#### `@contextmenu` 事件

```typescript
function handleContextMenu(e: MouseEvent) {
  e.preventDefault();    // 阻止默认右键菜单
  e.stopPropagation();   // 阻止事件冒泡

  // 记录鼠标位置
  contextMenuPosition.value = { x: e.clientX, y: e.clientY };

  // 显示菜单
  contextMenuVisible.value = true;
}
```

**关键点**：
- 必须调用 `e.preventDefault()` 阻止原生菜单
- 使用 `e.stopPropagation()` 防止事件冒泡到父元素
- 使用 `e.clientX` 和 `e.clientY` 获取鼠标坐标（相对于视口）

### 2. 位置计算（防止超出视口）

```typescript
const adjustedPosition = computed(() => {
  const { x, y } = contextMenuPosition.value;
  const menuWidth = 200;   // 预估菜单宽度
  const menuHeight = 150;  // 预估菜单高度

  let adjustedX = x;
  let adjustedY = y;

  // 检测右边界：菜单超出屏幕右侧
  if (x + menuWidth > window.innerWidth) {
    adjustedX = Math.max(0, x - menuWidth); // 移到鼠标左侧
  }

  // 检测底边界：菜单超出屏幕底部
  if (y + menuHeight > window.innerHeight) {
    adjustedY = Math.max(0, y - menuHeight); // 移到鼠标上方
  }

  return { x: adjustedX, y: adjustedY };
});
```

**边界情况**：
- **右边界**：将菜单移到鼠标左侧
- **底边界**：将菜单移到鼠标上方
- **右下角**：同时应用左移和上移
- **确保不超出左边界和上边界**：使用 `Math.max(0, ...)`

### 3. 自动关闭机制

菜单需要在以下情况自动关闭：

```typescript
// 1. 点击菜单外部
function handleClickOutside(e: MouseEvent) {
  if (!contextMenuVisible.value) return;
  closeContextMenu();
}

// 2. 再次右键（新位置打开新菜单）
function handleContextMenuAgain(e: MouseEvent) {
  if (!contextMenuVisible.value) return;
  closeContextMenu();
}

// 3. 按 ESC 键
function handleEscape(e: KeyboardEvent) {
  if (e.key === "Escape" && contextMenuVisible.value) {
    closeContextMenu();
  }
}

// 4. 菜单项点击后
function handleEdit() {
  closeContextMenu();  // 先关闭菜单
  emit("edit", props.project);  // 再执行操作
}
```

**生命周期管理**：

```typescript
onMounted(() => {
  document.addEventListener("click", handleClickOutside);
  document.addEventListener("contextmenu", handleContextMenuAgain);
  document.addEventListener("keydown", handleEscape);
});

onUnmounted(() => {
  // 重要：必须清理事件监听器，防止内存泄漏
  document.removeEventListener("click", handleClickOutside);
  document.removeEventListener("contextmenu", handleContextMenuAgain);
  document.removeEventListener("keydown", handleEscape);
});
```

### 4. 使用现有 DropdownMenu 组件

如果您想保持视觉一致性，可以基于现有的 `DropdownMenu` 组件系统：

```vue
<template>
  <div @contextmenu="handleContextMenu">
    <!-- 触发区域 -->
  </div>

  <!-- 使用 DropdownMenuContent 保持视觉一致性 -->
  <Teleport to="body">
    <div
      v-if="menuVisible"
      :style="{ position: 'fixed', left: `${x}px`, top: `${y}px`, zIndex: 9999 }"
      @click.stop
    >
      <DropdownMenuContent>
        <DropdownMenuItem @click="handleAction1">
          操作 1
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem @click="handleAction2">
          操作 2
        </DropdownMenuItem>
      </DropdownMenuContent>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator } from "@/components/ui/dropdown-menu";
// ...
</script>
```

**优势**：
- 与现有下拉菜单视觉风格完全一致
- 复用动画和样式
- 维护性更好

---

## 最佳实践

### 1. 与现有下拉菜单共存

ArgForge 项目中许多组件已经有点击触发的下拉菜单（如 `ProjectCardDropdown` 三点菜单）。右键菜单应该提供**快捷方式**，而不是完全替代现有菜单。

**推荐做法**：
- 保留现有的点击菜单（三点菜单）
- 右键菜单提供相同的核心功能
- 用户可自由选择使用哪种交互方式

### 2. 组件复用建议

如果您在多个组件中使用右键菜单，可以考虑以下方案：

#### 方案 A：创建 Composable

```typescript
// src/composables/useContextMenu.ts
import { ref, computed, onMounted, onUnmounted } from "vue";

export function useContextMenu() {
  const visible = ref(false);
  const position = ref({ x: 0, y: 0 });

  const adjustedPosition = computed(() => {
    // 位置计算逻辑...
  });

  function open(e: MouseEvent) {
    e.preventDefault();
    e.stopPropagation();
    position.value = { x: e.clientX, y: e.clientY };
    visible.value = true;
  }

  function close() {
    visible.value = false;
  }

  // 全局事件监听...
  onMounted(() => { /* ... */ });
  onUnmounted(() => { /* ... */ });

  return {
    visible,
    adjustedPosition,
    open,
    close,
  };
}
```

**使用**：

```vue
<script setup lang="ts">
import { useContextMenu } from "@/composables/useContextMenu";

const { visible, adjustedPosition, open, close } = useContextMenu();
</script>

<template>
  <div @contextmenu="open">
    <!-- ... -->
  </div>
</template>
```

#### 方案 B：封装 ContextMenu 组件

创建 `src/components/ui/context-menu/ContextMenu.vue` 组件，封装状态和逻辑。

### 3. 性能优化

- **使用 `Teleport`**：将菜单渲染到 `body`，避免 `z-index` 层级问题
- **避免不必要的计算**：只在菜单显示时计算位置
- **事件委托**：如果菜单项很多，使用事件委托而非每个项单独监听

### 4. 可访问性

虽然右键菜单本身不是主要的可访问性交互方式，但仍需考虑：

- **键盘导航**：用户可能需要通过键盘访问菜单项
- **ARIA 属性**：添加适当的 `role` 和 `aria` 属性
- **焦点管理**：菜单打开时管理焦点

示例：

```vue
<div
  role="menu"
  :aria-hidden="!menuVisible"
>
  <div
    role="menuitem"
    tabindex="0"
    @click="handleAction"
    @keydown.enter="handleAction"
  >
    操作项
  </div>
</div>
```

---

## 常见问题

### Q1: 菜单位置不正确？

**检查点**：
- 使用 `position: fixed`（而非 `absolute`）
- 使用 `e.clientX` 和 `e.clientY`（而非 `e.pageX` 和 `e.pageY`）
- 确保父元素没有 `transform`、`filter` 等 CSS 属性

### Q2: 菜单无法点击？

**可能原因**：
- 菜单被其他元素遮挡（检查 `z-index`）
- 使用了 `pointer-events: none` CSS
- 事件被 `@click.stop` 阻止

**解决方案**：
- 设置 `z-index: 9999` 或更高
- 在菜单容器上添加 `@click.stop` 防止事件冒泡

### Q3: 多个右键菜单同时存在？

**解决方案**：使用全局状态管理（Pinia Store）

```typescript
// src/stores/ui.ts
export const useUiStore = defineStore("ui", () => {
  const contextMenu = ref({
    visible: false,
    x: 0,
    y: 0,
  });

  function openContextMenu(x: number, y: number) {
    contextMenu.value.visible = false;  // 先关闭旧菜单
    nextTick(() => {
      contextMenu.value = { visible: true, x, y };
    });
  }

  function closeContextMenu() {
    contextMenu.value.visible = false;
  }

  return {
    contextMenu,
    openContextMenu,
    closeContextMenu,
  };
});
```

### Q4: 菜单内容动态加载？

如果菜单项需要异步加载数据：

```vue
<template>
  <div v-if="menuVisible">
    <div v-if="loading">加载中...</div>
    <div v-else>
      <!-- 菜单项 -->
    </div>
  </div>
</template>

<script setup lang="ts">
const loading = ref(false);

async function handleContextMenu(e: MouseEvent) {
  openMenu(e);
  loading.value = true;
  await loadMenuItems();
  loading.value = false;
}
</script>
```

---

## 参考资源

- [MDN - contextmenu event](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)
- [reka-ui - DropdownMenu](https://reka-ui.dev/components/dropdown-menu)
- [Vue 3 - Teleport](https://vuejs.org/guide/built-ins/teleport.html)
- [Tauri Discussion - Disable right click context menu](https://github.com/orgs/tauri-apps/discussions/11808)

---

## 总结

在 ArgForge 项目中实现自定义右键菜单的关键步骤：

1. **监听 `@contextmenu` 事件**：阻止默认行为并记录位置
2. **显示菜单**：使用 `position: fixed` 定位菜单
3. **位置计算**：防止菜单超出视口边界
4. **自动关闭**：监听点击外部、再次右键、ESC 键
5. **生命周期管理**：在 `onUnmounted` 中清理事件监听器

基于现有的 `DropdownMenu` 组件系统，可以快速实现视觉一致的右键菜单。记住与现有下拉菜单共存，为用户提供多种交互方式。
