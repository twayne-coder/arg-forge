/**
 * 命令预览 Composable
 * 负责监听表单变化并实时生成命令预览
 */

import { ref, watch, computed } from "vue";
import { useProjectStore } from "@/stores/project";
import { useFormStore } from "@/stores/form";
import { generateCommand } from "@/api/command";

/**
 * 使用命令预览
 * @returns 命令预览相关的状态和方法
 */
export function useCommandPreview() {
  /** 项目 store */
  const projectStore = useProjectStore();

  /** 表单 store */
  const formStore = useFormStore();

  /** 生成的命令字符串 */
  const commandPreview = ref("");

  /** 当前项目 ID */
  const projectId = computed(() => projectStore.currentProject?.id);

  /** 当前表单 ID */
  const formId = computed(() => formStore.currentForm?.id);

  /**
   * 生成命令
   */
  const generateCommandPreview = async () => {
    if (!projectId.value || !formId.value) {
      commandPreview.value = "";
      return;
    }

    try {
      commandPreview.value = await generateCommand(projectId.value, formId.value);
    } catch (error) {
      console.error("生成命令失败:", error);
      commandPreview.value = "生成失败";
    }
  };

  /**
   * 监听表单变化，自动更新命令预览
   */
  watch(
    () => formStore.currentForm?.items,
    () => {
      generateCommandPreview();
    },
    { deep: true, immediate: true }
  );

  return {
    commandPreview,
    generateCommand: generateCommandPreview,
  };
}
