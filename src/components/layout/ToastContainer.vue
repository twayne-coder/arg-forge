<script setup lang="ts">
import { ref, watch } from "vue";
import { useUiStore } from "@/stores/ui";
import { Check, X, AlertCircle, Info } from "lucide-vue-next";
import type { Toast } from "@/types/toast";

const store = useUiStore();

// 正在移除的 toast ID 集合（用于淡出动画）
const removingIds = ref<Set<string>>(new Set());

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
  gap: 0.75rem;
  padding: 0.875rem 1.5rem;
  background: hsl(var(--background));
  border-radius: calc(var(--radius) + 4px);
  box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
  border: 2px solid;
  backdrop-filter: blur(12px);
  user-select: none;
  pointer-events: auto;
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
  border-color: hsl(var(--primary));
  background: hsl(var(--background));
}

/* 深色模式：Success */
.dark .toast-success {
  border-color: hsl(var(--primary) / 0.5);
}

.toast-success .icon-success {
  color: white;
  background: hsl(var(--primary));
  border-radius: 50%;
  padding: 6px;
  box-shadow: 0 2px 8px hsl(var(--primary) / 0.3);
}

/* 类型样式：Error */
.toast-error {
  border-color: hsl(var(--destructive));
  background: hsl(var(--background));
}

/* 深色模式：Error */
.dark .toast-error {
  border-color: hsl(var(--destructive) / 0.5);
}

.toast-error .icon-error {
  color: white;
  background: hsl(var(--destructive));
  border-radius: 50%;
  padding: 6px;
  box-shadow: 0 2px 8px hsl(var(--destructive) / 0.3);
}

/* 类型样式：Info */
.toast-info {
  border-color: hsl(var(--ring));
  background: hsl(var(--background));
}

/* 深色模式：Info */
.dark .toast-info {
  border-color: hsl(var(--ring) / 0.5);
}

.toast-info .icon-info {
  color: white;
  background: hsl(var(--ring));
  border-radius: 50%;
  padding: 6px;
  box-shadow: 0 2px 8px hsl(var(--ring) / 0.3);
}

/* 类型样式：Warning */
.toast-warning {
  border-color: hsl(var(--warning));
  background: hsl(var(--background));
}

/* 深色模式：Warning */
.dark .toast-warning {
  border-color: hsl(var(--warning) / 0.5);
}

.toast-warning .icon-warning {
  color: white;
  background: hsl(var(--warning));
  border-radius: 50%;
  padding: 6px;
  box-shadow: 0 2px 8px hsl(var(--warning) / 0.3);
}

/* 图标样式 */
.toast-icon {
  flex-shrink: 0;
  width: 1.25rem;
  height: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
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
