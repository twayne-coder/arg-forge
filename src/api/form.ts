/**
 * 表单 API
 * 封装所有表单相关的 Tauri IPC 调用
 */

import { invoke } from "@/lib/tauri";
import type { CommandFormat, Form, FormItem, ItemType } from "@/types/bindings";

/**
 * 创建表单
 * @param projectId - 项目 ID
 * @param name - 表单名称
 * @param description - 表单描述
 * @returns 创建的表单对象
 */
export async function createForm(
  projectId: string,
  name: string,
  description: string
): Promise<Form> {
  return await invoke("create_form", {
    projectId,
    name,
    description,
  });
}

/**
 * 更新表单信息
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param name - 新的表单名称
 * @param description - 新的表单描述
 * @returns 更新后的表单对象
 */
export async function updateForm(
  projectId: string,
  formId: string,
  name: string,
  description: string
): Promise<Form> {
  return await invoke("update_form", {
    projectId,
    formId,
    name,
    description,
  });
}

/**
 * 删除表单
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 */
export async function deleteForm(projectId: string, formId: string): Promise<void> {
  await invoke("delete_form", { projectId, formId });
}

/**
 * 更新表单项
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param itemId - 表单项 ID
 * @param fieldName - 要更新的字段名
 * @param value - 新值（JSON 格式）
 * @returns 更新后的表单对象
 */
export async function updateFormItem(
  projectId: string,
  formId: string,
  itemId: string,
  fieldName: string,
  value: unknown
): Promise<Form> {
  return await invoke("update_form_item", {
    projectId,
    formId,
    itemId,
    fieldName,
    value,
  });
}

/**
 * 添加新的表单项
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param itemType - 表单项类型（"Command" 或 "Parameter"）
 * @returns 新创建的表单项
 */
export async function addFormItem(
  projectId: string,
  formId: string,
  itemType?: ItemType
): Promise<FormItem> {
  return await invoke("add_form_item", {
    projectId,
    formId,
    itemType,
  });
}

/**
 * 删除表单项
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param itemId - 表单项 ID
 */
export async function deleteFormItem(
  projectId: string,
  formId: string,
  itemId: string
): Promise<void> {
  await invoke("delete_form_item", { projectId, formId, itemId });
}

/**
 * 重新排序表单项（拖拽排序）
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param oldIndex - 原始索引
 * @param newIndex - 新索引
 */
export async function reorderFormItems(
  projectId: string,
  formId: string,
  oldIndex: number,
  newIndex: number
): Promise<void> {
  await invoke("reorder_form_items", {
    projectId,
    formId,
    oldIndex,
    newIndex,
  });
}

/**
 * 更新下拉选项
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param itemId - 表单项 ID
 * @param options - 新的选项列表
 */
export async function updateDropdownOptions(
  projectId: string,
  formId: string,
  itemId: string,
  options: string[]
): Promise<void> {
  await invoke("update_dropdown_options", {
    projectId,
    formId,
    itemId,
    options,
  });
}

/**
 * 切换下拉/手动模式
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param itemId - 表单项 ID
 * @param useDropdown - 是否使用下拉模式
 */
export async function toggleDropdownMode(
  projectId: string,
  formId: string,
  itemId: string,
  useDropdown: boolean
): Promise<void> {
  await invoke("toggle_dropdown_mode", {
    projectId,
    formId,
    itemId,
    useDropdown,
  });
}

/**
 * 更新命令格式
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @param format - 命令格式（"SingleLine" 或 "MultiLine"）
 * @returns 更新后的表单对象
 */
export async function updateCommandFormat(
  projectId: string,
  formId: string,
  format: CommandFormat
): Promise<Form> {
  return await invoke("update_form_field", {
    projectId,
    formId,
    fieldName: "command_format",
    value: format,
  });
}
