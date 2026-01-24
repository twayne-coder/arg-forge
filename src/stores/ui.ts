/**
 * UI 状态管理
 * 管理界面交互状态（侧边栏、拖拽模式、保存状态等）
 */

import { defineStore } from "pinia";
import { ref } from "vue";
import type { Toast, ToastType } from "@/types/toast";

export type SaveStatus = "idle" | "saving" | "saved" | "error";

export const useUiStore = defineStore("ui", () => {
  // ========== 状态 ==========

  /** 侧边栏是否折叠 */
  const sidebarCollapsed = ref(false);

  /** 是否处于拖拽排序模式 */
  const isReorderMode = ref(false);

  /** 保存状态 */
  const saveStatus = ref<SaveStatus>("idle");

  /** 保存状态消息 */
  const saveMessage = ref("");

  /** 加载状态 */
  const isLoading = ref(false);

  /** 是否显示对话框（用于编辑项目/表单） */
  const showDialog = ref(false);

  /** 当前打开的对话框类型 */
  const dialogType = ref<"createProject" | "editProject" | "editForm" | null>(null);

  /** Toast 通知列表 */
  const toasts = ref<Toast[]>([]);

  // ========== Actions ==========

  /**
   * 切换侧边栏折叠状态
   */
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  /**
   * 设置侧边栏折叠状态
   * @param collapsed - 是否折叠
   */
  function setSidebarCollapsed(collapsed: boolean) {
    sidebarCollapsed.value = collapsed;
  }

  /**
   * 切换拖拽排序模式
   */
  function toggleReorderMode() {
    isReorderMode.value = !isReorderMode.value;
  }

  /**
   * 设置拖拽排序模式
   * @param enabled -是否启用
   */
  function setReorderMode(enabled: boolean) {
    isReorderMode.value = enabled;
  }

  /**
   * 设置保存状态
   * @param status - 保存状态
   * @param message - 状态消息
   */
  function setSaveStatus(status: SaveStatus, message = "") {
    saveStatus.value = status;
    saveMessage.value = message;

    // 如果是保存成功状态，3 秒后自动重置
    if (status === "saved") {
      setTimeout(() => {
        if (saveStatus.value === "saved") {
          saveStatus.value = "idle";
          saveMessage.value = "";
        }
      }, 3000);
    }

    // 如果是错误状态，5 秒后自动重置
    if (status === "error") {
      setTimeout(() => {
        if (saveStatus.value === "error") {
          saveStatus.value = "idle";
          saveMessage.value = "";
        }
      }, 5000);
    }
  }

  /**
   * 设置加载状态
   * @param loading - 是否加载中
   */
  function setLoading(loading: boolean) {
    isLoading.value = loading;
  }

  /**
   * 打开对话框
   * @param type - 对话框类型
   */
  function openDialog(type: "createProject" | "editProject" | "editForm") {
    dialogType.value = type;
    showDialog.value = true;
  }

  /**
   * 关闭对话框
   */
  function closeDialog() {
    showDialog.value = false;
    dialogType.value = null;
  }

  /**
   * 显示 Toast 通知
   * @param message - 消息内容
   * @param type - 类型（默认 success）
   * @param duration - 持续时间（默认 1000ms）
   */
  function showToast(message: string, type: ToastType = "success", duration: number = 1000) {
    const id = crypto.randomUUID();
    const toast: Toast = {
      id,
      message,
      type,
      duration,
      createdAt: Date.now(),
    };

    // 添加到队列头部（最新的在最前）
    toasts.value.unshift(toast);

    // 调试日志
    console.log("[Toast] 显示通知:", { id, message, type, duration, totalToasts: toasts.value.length });

    // 自动移除
    setTimeout(() => {
      console.log("[Toast] 移除通知:", id);
      removeToast(id);
    }, duration);
  }

  /**
   * 移除指定 Toast
   * @param id - Toast ID
   */
  function removeToast(id: string) {
    const index = toasts.value.findIndex((t) => t.id === id);
    if (index > -1) {
      toasts.value.splice(index, 1);
    }
  }

  /**
   * 清空所有 Toast
   */
  function clearToasts() {
    toasts.value = [];
  }

  /**
   * 清空所有状态
   */
  function $reset() {
    sidebarCollapsed.value = false;
    isReorderMode.value = false;
    saveStatus.value = "idle";
    saveMessage.value = "";
    isLoading.value = false;
    showDialog.value = false;
    dialogType.value = null;
    toasts.value = [];
  }

  return {
    // 状态
    sidebarCollapsed,
    isReorderMode,
    saveStatus,
    saveMessage,
    isLoading,
    showDialog,
    dialogType,
    toasts,

    // Actions
    toggleSidebar,
    setSidebarCollapsed,
    toggleReorderMode,
    setReorderMode,
    setSaveStatus,
    setLoading,
    openDialog,
    closeDialog,
    showToast,
    removeToast,
    clearToasts,
    $reset,
  };
});
