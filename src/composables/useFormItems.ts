/**
 * 表单项管理 Composable
 * 封装表单项的 CRUD 操作，保持组件代码简洁
 */

import { useProjectStore } from "@/stores/project";

/**
 * 使用表单项管理
 * @returns 表单项操作方法
 */
export function useFormItems() {
  /** 项目 store */
  const projectStore = useProjectStore();

  /**
   * 添加新表单项
   */
  async function addFormItem() {
    await projectStore.addFormItem();
  }

  /**
   * 删除表单项
   * @param itemId - 表单项 ID
   */
  async function deleteFormItem(itemId: string) {
    await projectStore.deleteFormItem(itemId);
  }

  /**
   * 更新表单项字段
   * @param itemId - 表单项 ID
   * @param field - 字段名
   * @param value - 新值
   */
  async function updateFormItem(itemId: string, field: string, value: any) {
    await projectStore.updateFormItem(itemId, field, value);
  }

  /**
   * 重新排序表单项
   * @param oldIndex - 原始位置
   * @param newIndex - 新位置
   */
  async function reorderFormItems(oldIndex: number, newIndex: number) {
    await projectStore.reorderFormItems(oldIndex, newIndex);
  }

  /**
   * 更新下拉选项
   * @param itemId - 表单项 ID
   * @param options - 选项列表
   */
  async function updateDropdownOptions(itemId: string, options: string[]) {
    await projectStore.updateDropdownOptions(itemId, options);
  }

  /**
   * 切换下拉模式
   * @param itemId - 表单项 ID
   * @param useDropdown - 是否启用下拉模式
   */
  async function toggleDropdownMode(itemId: string, useDropdown: boolean) {
    await projectStore.toggleDropdownMode(itemId, useDropdown);
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
