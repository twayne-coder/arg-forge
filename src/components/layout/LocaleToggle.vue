<script setup lang="ts">
import { computed } from "vue";
import { useLocale } from "@/composables/useLocale";
import { useI18n } from "vue-i18n";

const { currentLocale, toggleLocale } = useLocale();
const { t } = useI18n();

/** 按钮标题文本 */
const title = computed(() => {
  return currentLocale.value === "zh" ? t("language.switchToEn") : t("language.switchToZh");
});
</script>

<template>
  <button
    @click.stop="toggleLocale"
    class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground transition-colors"
    :title="title"
  >
    <span class="text-xs font-medium">{{ currentLocale === "zh" ? "中" : "En" }}</span>
  </button>
</template>
