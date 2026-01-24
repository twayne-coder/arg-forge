<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { useUiStore } from "@/stores/ui";
import { Check, X, AlertCircle, Info } from "lucide-vue-next";
import type { Toast } from "@/types/toast";

const store = useUiStore();

// 正在移除的 toast ID 集合（用于淡出动画）
const removingIds = ref<Set<string>>(new Set());

// 调试：监控 toasts 变化
watch(
  () => store.toasts,
  (newToasts) => {
    console.log("[ToastContainer] toasts 变化:", newToasts.length, newToasts);
  },
  { deep: true, immediate: true }
);

onMounted(() => {
  console.log("[ToastContainer] 组件已挂载");
});

/**
 * 获取图标组件
 */
function getIcon(type: Toast["type"]) {
  const icons = {
    success: Check,
    error: X,
    warning: AlertCircle,
    info: Info,
  };
  return icons[type];
}

/**
 * 处理移除动画
 */
async function handleRemove(id: string) {
  removingIds.value.add(id);
  // 等待 CSS 淡出动画完成（300ms）
  await new Promise((resolve) => setTimeout(resolve, 300));
  store.removeToast(id);
  removingIds.value.delete(id);
}

// 监听 store 中的 toasts 变化，同步移除本地状态
watch(
  () => store.toasts,
  (newToasts) => {
    const currentIds = new Set(newToasts.map((t) => t.id));
    // 清理已不存在于 store 的 ID
    for (const id of removingIds.value) {
      if (!currentIds.has(id)) {
        removingIds.value.delete(id);
      }
    }
  },
  { deep: true }
);
</script>

<template>
  <!-- 固定定位容器：屏幕中央 -->
  <div class="toast-container">
    <!-- Toast 列表：垂直排列 -->
    <TransitionGroup name="toast">
      <div
        v-for="toast in store.toasts"
        :key="toast.id"
        :class="[
          'toast-item',
          `toast-${toast.type}`,
          { 'toast-removing': removingIds.has(toast.id) }
        ]"
      >
        <!-- 图标 -->
        <component
          :is="getIcon(toast.type)"
          :class="['toast-icon', `icon-${toast.type}`]"
        />

        <!-- 消息文本 -->
        <span class="toast-message">{{ toast.message }}</span>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
/* 容器：固定在屏幕中央 */
.toast-container {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 99999;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  pointer-events: none; /* 让点击穿透，不阻挡底层交互 */
}

/* Toast 项基础样式 */
.toast-item {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.75rem 1.25rem;
  background: hsl(var(--background));
  border-radius: calc(var(--radius) + 2px);
  box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.15), 0 8px 10px -6px rgb(0 0 0 / 0.15);
  border-right: 1px solid hsl(var(--border));
  border-top: 1px solid hsl(var(--border));
  border-bottom: 1px solid hsl(var(--border));
  backdrop-filter: blur(8px);
  user-select: none;
  pointer-events: auto; /* 恢复 toast 自身的交互 */
  min-width: 200px;
  max-width: 380px;
  transition: opacity 0.3s ease-in, transform 0.3s ease-in;
}

.toast-item.toast-removing {
  opacity: 0;
  transform: scale(0.95);
}

/* 类型样式：Success */
.toast-success {
  border-left: 4px solid hsl(var(--primary));
  background-image: linear-gradient(90deg, hsl(var(--primary) / 0.12) 0%, hsl(var(--background)) 35%);
}

.toast-success .icon-success {
  color: hsl(var(--primary));
  filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1));
}

/* 类型样式：Error */
.toast-error {
  border-left: 4px solid hsl(var(--destructive));
  background-image: linear-gradient(90deg, hsl(var(--destructive) / 0.12) 0%, hsl(var(--background)) 35%);
}

.toast-error .icon-error {
  color: hsl(var(--destructive));
  filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1));
}

/* 类型样式：Info */
.toast-info {
  border-left: 4px solid hsl(var(--ring));
  background-image: linear-gradient(90deg, hsl(var(--ring) / 0.12) 0%, hsl(var(--background)) 35%);
}

.toast-info .icon-info {
  color: hsl(var(--ring));
  filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1));
}

/* 类型样式：Warning */
.toast-warning {
  border-left: 4px solid hsl(45 93% 47%);
  background-image: linear-gradient(90deg, hsl(45 93% 47% / 0.12) 0%, hsl(var(--background)) 35%);
}

.toast-warning .icon-warning {
  color: hsl(45 93% 47%);
  filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1));
}

/* 图标样式 */
.toast-icon {
  flex-shrink: 0;
  width: 1.25rem;
  height: 1.25rem;
}

/* 消息文本 */
.toast-message {
  font-size: 0.875rem;
  font-weight: 600;
  color: hsl(var(--foreground));
  letter-spacing: 0.01em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ===== Vue TransitionGroup 动画 ===== */

/* 进入前：透明 + 缩小 */
.toast-enter-from {
  opacity: 0;
  transform: scale(0.9) translateY(-10px);
}

/* 进入中：过渡效果 */
.toast-enter-active {
  transition: opacity 0.3s ease-out, transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
}

/* 进入后：正常状态 */
.toast-enter-to {
  opacity: 1;
  transform: scale(1) translateY(0);
}

/* 离开前：正常状态 */
.toast-leave-from {
  opacity: 1;
  transform: scale(1);
}

/* 离开中：淡出 + 缩小 */
.toast-leave-active {
  transition: opacity 0.3s ease-in, transform 0.3s ease-in;
}

/* 离开后：完全消失 */
.toast-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

/* 列表项移动时的平滑过渡 */
.toast-move {
  transition: transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
}
</style>
