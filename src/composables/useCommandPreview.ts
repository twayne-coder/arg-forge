/**
 * 命令预览 Composable
 * 负责监听表单变化并实时生成命令预览
 */

import { ref, watch, computed } from "vue";
import { useProjectStore } from "@/stores/project";
import { generateCommand } from "@/api/command";
import { useDebounceFn } from "@vueuse/core";

/**
 * 使用命令预览
 * @returns 命令预览相关的状态和方法
 */
export function useCommandPreview() {
  /** 项目 store */
  const projectStore = useProjectStore();

  /** 生成的命令字符串 */
  const commandPreview = ref("");

  /** 是否正在生成命令 */
  const isGenerating = ref(false);

  /** 当前项目 ID */
  const projectId = computed(() => projectStore.currentProject?.id);

  /** 当前表单 ID */
  const formId = computed(() => projectStore.currentForm?.id);

  /**
   * 生成命令（防抖处理）
   */
  const generateCommandDebounced = useDebounceFn(async () => {
    if (!projectId.value || !formId.value) {
      commandPreview.value = "";
      return;
    }

    try {
      isGenerating.value = true;
      commandPreview.value = await generateCommand(projectId.value, formId.value);
    } catch (error) {
      console.error("生成命令失败:", error);
      commandPreview.value = "生成失败";
    } finally {
      isGenerating.value = false;
    }
  }, 300); // 300ms 防抖

  /**
   * 监听表单变化，自动更新命令预览
   */
  watch(
    () => projectStore.currentForm,
    () => {
      generateCommandDebounced();
    },
    { deep: true }
  );

  return {
    commandPreview,
    isGenerating,
    generateCommand: generateCommandDebounced,
  };
}
