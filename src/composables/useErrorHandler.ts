/**
 * 统一错误处理 Composable
 * 提供一致的错误提示和日志记录
 */
import { useUiStore } from "@/stores/ui";

/**
 * 使用错误处理器
 * @returns 错误处理方法
 */
export function useErrorHandler() {
  const uiStore = useUiStore();

  /**
   * 处理错误并显示用户提示
   * @param error - 错误对象
   * @param context - 操作上下文 (如"创建项目")
   */
  function handleError(error: unknown, context: string) {
    // 记录详细日志
    console.error(`[${context}] 失败:`, error);

    // 提取错误消息
    let message = `${context}失败`;
    if (error instanceof Error) {
      message = error.message;
    } else if (typeof error === "string") {
      message = error;
    }

    // 显示用户提示
    uiStore.showToast(message, "error", 3000);
  }

  return { handleError };
}
