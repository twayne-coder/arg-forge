<script setup lang="ts">
import { computed } from "vue";
import { Sun, Moon } from "lucide-vue-next";
import { useTheme } from "@/composables/useTheme";
import { useI18n } from "vue-i18n";

const { isDark, toggleTheme } = useTheme();
const { t } = useI18n();

/** 按钮标题文本 */
const title = computed(() => {
	return isDark.value ? t("theme.switchToLight") : t("theme.switchToDark");
});
</script>

<template>
	<button
		@click.stop="toggleTheme"
		class="flex h-full w-11 items-center justify-center text-foreground/80 hover:bg-muted hover:text-foreground transition-colors"
		:title="title"
	>
		<!-- 浅色模式显示月亮图标 -->
		<Moon v-if="!isDark" :size="14" />

		<!-- 深色模式显示太阳图标 -->
		<Sun v-else :size="14" />
	</button>
</template>
