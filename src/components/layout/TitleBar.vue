<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Minus, Square, X } from 'lucide-vue-next';

type WindowHandle = {
  minimize: () => Promise<void>;
  toggleMaximize: () => Promise<void>;
  close: () => Promise<void>;
};

declare global {
  interface Window {
    __TAURI__?: unknown;
  }
}

const appWindow = ref<WindowHandle | null>(null);

onMounted(async () => {
  // 只在 Tauri 环境中动态导入 API
  if (typeof window !== 'undefined' && window.__TAURI__) {
    try {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      appWindow.value = getCurrentWindow();
    } catch (error) {
      console.warn('无法初始化 Tauri 窗口:', error);
    }
  }
});

const minimize = () => {
  appWindow.value?.minimize();
};

const maximize = () => {
  appWindow.value?.toggleMaximize();
};

const close = () => {
  appWindow.value?.close();
};

// 双击标题栏切换最大化
const handleDoubleClick = () => {
  maximize();
};
</script>

<template>
  <div
    data-tauri-drag-region
    class="flex h-8 select-none items-center justify-between bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
    @dblclick="handleDoubleClick"
  >
    <!-- 左侧：标题 -->
    <div class="flex flex-1 items-center gap-2 px-3">
      <div class="text-sm font-medium text-foreground">ArgForge</div>
    </div>

    <!-- 右侧：窗口控制按钮 -->
    <div class="flex h-full">
      <button
        @click="minimize"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground"
        title="最小化"
      >
        <Minus :size="14" />
      </button>
      <button
        @click="maximize"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground"
        title="最大化"
      >
        <Square :size="12" />
      </button>
      <button
        @click="close"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-destructive hover:text-destructive-foreground"
        title="关闭"
      >
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
