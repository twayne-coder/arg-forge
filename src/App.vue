<script setup lang="ts">
import TitleBar from "@/components/layout/TitleBar.vue";
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
