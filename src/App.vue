<script setup lang="ts">
import { onMounted } from "vue";
import TitleBar from "@/components/layout/TitleBar.vue";

onMounted(() => {
  console.log("===== 应用启动环境检测 =====");
  console.log("location.href:", window.location.href);
  console.log("window.__TAURI__:", window.__TAURI__);
  console.log("__TAURI_INTERNALS__:", (window as any).__TAURI_INTERNALS__);
  console.log("是否在 Tauri 环境:", !!window.__TAURI__ || !!(window as any).__TAURI_INTERNALS__);
  console.log("UserAgent:", navigator.userAgent);

  if (!window.__TAURI__ && !(window as any).__TAURI_INTERNALS__) {
    console.error("⚠️ 警告：未检测到 Tauri 环境！");
    console.error("⚠️ 当前运行在浏览器模式，无法调用后端功能");
    console.error("⚠️ 请确保在 Tauri 窗口中运行此应用");
  } else {
    console.log("✅ Tauri 环境检测成功");
  }
  console.log("================================");
});
</script>

<template>
  <div class="flex h-screen flex-col">
    <!-- 自定义标题栏 -->
    <TitleBar />

    <!-- 主内容区 - 路由视图 -->
    <main class="flex-1 overflow-hidden">
      <router-view />
    </main>
  </div>
</template>
