<template>
  <Card
    class="group cursor-pointer transition-all duration-200 hover:shadow-lg hover:border-primary/50"
    @click="handleClick"
  >
    <CardHeader>
      <CardTitle class="flex items-center justify-between">
        <span class="truncate">{{ project.name }}</span>
        <DropdownMenu :project="project" @click.stop />
      </CardTitle>
      <CardDescription class="line-clamp-2">
        {{ project.description || "暂无描述" }}
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div class="flex items-center gap-2 text-sm text-muted-foreground">
        <FolderOpenIcon class="h-4 w-4" />
        <span>{{ project.forms.length }} 个表单</span>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import type { Project } from "@/types/bindings";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { FolderOpenIcon } from "lucide-vue-next";
import DropdownMenu from "./ProjectCardDropdown.vue";

/** 项目对象 */
const props = defineProps<{
  project: Project;
}>();

/** 点击事件 */
const emit = defineEmits<{
  (e: "click", project: Project): void;
}>();

/** 处理卡片点击 */
function handleClick() {
  emit("click", props.project);
}
</script>

<style scoped>
/* 确保文本截断正常工作 */
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
