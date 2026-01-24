<template>
  <div
    :class="[
      'form-list-item group cursor-pointer rounded-lg border p-3 transition-all',
      'hover:bg-accent hover:shadow-sm',
      active ? 'border-primary bg-accent' : 'border-border bg-card'
    ]"
    @click="$emit('click')"
  >
    <div class="flex items-center justify-between gap-2">
      <div class="flex-1 min-w-0">
        <h3
          :class="[
            'font-medium truncate',
            active ? 'text-primary' : 'text-foreground'
          ]"
        >
          {{ form.name }}
        </h3>
        <p class="text-xs text-muted-foreground mt-0.5">
          {{ $t('form.itemCount', { count: form.items.length }) }}
        </p>
      </div>
      <div class="text-xs text-muted-foreground whitespace-nowrap">
        {{ formatTime(form.updated_at) }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Form } from "@/types/bindings";

const { t } = useI18n();

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
    return t('time.today');
  } else if (diffDays === 1) {
    return t('time.yesterday');
  } else if (diffDays < 7) {
    return t('time.daysAgo', { days: diffDays });
  } else if (diffDays < 30) {
    const weeks = Math.floor(diffDays / 7);
    return t('time.weeksAgo', { weeks });
  } else if (diffDays < 365) {
    const months = Math.floor(diffDays / 30);
    return t('time.monthsAgo', { months });
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
