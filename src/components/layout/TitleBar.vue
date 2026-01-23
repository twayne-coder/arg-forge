<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { Minus, Square, Maximize2, X } from 'lucide-vue-next';
import * as windowApi from '@/api/window';
import ThemeToggle from './ThemeToggle.vue';

// 状态管理
const isReady = ref(false);
const isMaximized = ref(false);

onMounted(async () => {
	// 无论检测结果如何，先尝试初始化状态
	const isTauri = windowApi.isTauriApp();

	try {
		if (isTauri) {
			await checkMaximizedState().catch(err => console.warn('初始化获取窗口状态失败:', err));
		}
	} catch (err) {
		console.error('TitleBar 初始化过程出错:', err);
	} finally {
		// 最终始终设为 ready，确保按钮在 Tauri 环境下可点击
		// 如果不在 Tauri 环境，调用 API 时会自然报错，而不是点击无反应
		isReady.value = true;
	}
});

async function checkMaximizedState() {
	isMaximized.value = await windowApi.isMaximized();
}

async function minimize() {
	if (!isReady.value) return;
	try {
		await windowApi.minimizeWindow();
	} catch (err) {
		console.error('最小化失败:', err);
	}
}

async function toggleMaximize() {
	if (!isReady.value) return;
	try {
		await windowApi.toggleMaximize();
		await checkMaximizedState();
	} catch (err) {
		console.error('切换最大化失败:', err);
	}
}

async function close() {
	if (!isReady.value) return;
	try {
		await windowApi.closeWindow();
	} catch (err) {
		console.error('关闭窗口失败:', err);
	}
}

function handleDoubleClick() {
	toggleMaximize();
}
</script>

<template>
  <div
    class="relative z-50 flex h-8 select-none items-center justify-between bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
    data-tauri-drag-region
  >
    <!-- 左侧：标题（可拖动区域） -->
    <div
      class="flex flex-1 items-center gap-2 px-3 h-full"
      @dblclick="handleDoubleClick"
      data-tauri-drag-region
    >
      <img src="/icon.png" alt="ArgForge" class="w-4 h-4 pointer-events-none" />
      <div class="text-sm font-medium text-foreground pointer-events-none">ArgForge</div>
    </div>

    <!-- 右侧：窗口控制按钮（排除拖动） -->
    <div class="flex h-full" data-tauri-drag-region="false">
      <!-- 主题切换按钮 -->
      <ThemeToggle />

      <button
        @click.stop="minimize"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground"
        title="最小化"
      >
        <Minus :size="14" />
      </button>

      <button
        @click.stop="toggleMaximize"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground"
        :title="isMaximized ? '向下还原' : '最大化'"
      >
        <Maximize2 v-if="isMaximized" :size="14" />
        <Square v-else :size="12" />
      </button>

      <button
        @click.stop="close"
        class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-destructive hover:text-destructive-foreground"
        title="关闭"
      >
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
