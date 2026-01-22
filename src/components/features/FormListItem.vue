<template>
  <div
    :class="[
      'form-list-item group cursor-pointer rounded-lg border p-3 transition-all',
      'hover:bg-accent hover:shadow-sm',
      active ? 'border-primary bg-accent' : 'border-border bg-card'
    ]"
    @click="$emit('click')"
  >
    <div class="flex items-start justify-between gap-2">
      <div class="flex-1 min-w-0">
        <h3
          :class="[
            'font-medium truncate',
            active ? 'text-primary' : 'text-foreground'
          ]"
        >
          {{ form.name }}
        </h3>
        <p
          v-if="form.description"
          class="text-xs text-muted-foreground truncate mt-0.5"
        >
          {{ form.description }}
        </p>
      </div>
      <div class="text-xs text-muted-foreground whitespace-nowrap">
        {{ formatTime(form.updated_at) }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Form } from "@/types/bindings";

/** 组件属性 */
defineProps<{
  form: Form;
  active?: boolean;
}>();

/** 定义事件 */
defineEmits<{
  click: [];
}>();

/** 格式化时间 */
function formatTime(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) {
    return "今天";
  } else if (diffDays === 1) {
    return "昨天";
  } else if (diffDays < 7) {
    return `${diffDays} 天前`;
  } else if (diffDays < 30) {
    const weeks = Math.floor(diffDays / 7);
    return `${weeks} 周前`;
  } else if (diffDays < 365) {
    const months = Math.floor(diffDays / 30);
    return `${months} 月前`;
  } else {
    return date.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit"
    });
  }
}
</script>

<style scoped>
.form-list-item {
  user-select: none;
}
</style>
