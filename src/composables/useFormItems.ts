/**
 * 表单项管理 Composable
 * 封装表单项的 CRUD 操作，保持组件代码简洁
 */

import { useProjectStore } from "@/stores/project";
import { useFormStore } from "@/stores/form";
import type { FormItemFieldValue } from "@/types/bindings";

/**
 * 使用表单项管理
 * @returns 表单项操作方法
 */
export function useFormItems() {
  /** 项目 store */
  const projectStore = useProjectStore();

  /** 表单 store */
  const formStore = useFormStore();

  /**
   * 添加新表单项
   * @param itemType - 表单项类型（"Command" 或 "Parameter"，默认为 "Parameter"）
   */
  async function addFormItem(itemType?: "Command" | "Parameter") {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.addFormItem(projectStore.currentProject.id, itemType);
  }

  /**
   * 删除表单项
   * @param itemId - 表单项 ID
   */
  async function deleteFormItem(itemId: string) {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.deleteFormItem(projectStore.currentProject.id, itemId);
  }

  /**
   * 更新表单项字段
   * @param itemId - 表单项 ID
   * @param field - 字段名
   * @param value - 新值
   */
  async function updateFormItem(itemId: string, field: string, value: FormItemFieldValue) {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.updateFormItem(projectStore.currentProject.id, itemId, field, value);
  }

  /**
   * 重新排序表单项
   * @param oldIndex - 原始位置
   * @param newIndex - 新位置
   */
  async function reorderFormItems(oldIndex: number, newIndex: number) {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.reorderFormItems(projectStore.currentProject.id, oldIndex, newIndex);
  }

  /**
   * 更新下拉选项
   * @param itemId - 表单项 ID
   * @param options - 选项列表
   */
  async function updateDropdownOptions(itemId: string, options: string[]) {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.updateDropdownOptions(projectStore.currentProject.id, itemId, options);
  }

  /**
   * 切换下拉模式
   * @param itemId - 表单项 ID
   * @param useDropdown - 是否启用下拉模式
   */
  async function toggleDropdownMode(itemId: string, useDropdown: boolean) {
    if (!projectStore.currentProject) {
      throw new Error("没有当前项目");
    }
    await formStore.toggleDropdownMode(projectStore.currentProject.id, itemId, useDropdown);
  }

  return {
    addFormItem,
    deleteFormItem,
    updateFormItem,
    reorderFormItems,
    updateDropdownOptions,
    toggleDropdownMode,
  };
}
