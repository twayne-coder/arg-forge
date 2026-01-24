/**
 * 表单状态管理
 * 从 project.ts 中拆分出来的表单相关逻辑
 */

import { defineStore } from "pinia";
import { ref } from "vue";
import * as formApi from "@/api/form";
import type { Form, FormItemFieldValue } from "@/types/bindings";

export const useFormStore = defineStore("form", () => {
  // 当前选中的表单
  const currentForm = ref<Form | null>(null);

  /**
   * 创建表单
   */
  async function createForm(
    projectId: string,
    data: { name: string; description?: string }
  ) {
    const newForm = await formApi.createForm(
      projectId,
      data.name,
      data.description || ""
    );
    return newForm;
  }

  /**
   * 更新表单
   */
  async function updateForm(
    projectId: string,
    formId: string,
    data: { name: string; description?: string }
  ) {
    const updated = await formApi.updateForm(
      projectId,
      formId,
      data.name,
      data.description || ""
    );

    if (currentForm.value?.id === formId) {
      currentForm.value = updated;
    }

    return updated;
  }

  /**
   * 删除表单
   */
  async function deleteForm(projectId: string, formId: string) {
    await formApi.deleteForm(projectId, formId);

    if (currentForm.value?.id === formId) {
      currentForm.value = null;
    }
  }

  /**
   * 添加表单项
   */
  async function addFormItem(
    projectId: string,
    itemType?: "Command" | "Parameter"
  ) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    const newItem = await formApi.addFormItem(
      projectId,
      currentForm.value.id,
      itemType
    );

    currentForm.value.items.push(newItem);
    return newItem;
  }

  /**
   * 更新表单项
   */
  async function updateFormItem(
    projectId: string,
    itemId: string,
    field: string,
    value: FormItemFieldValue
  ) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    const updatedForm = await formApi.updateFormItem(
      projectId,
      currentForm.value.id,
      itemId,
      field,
      value
    );

    currentForm.value = updatedForm;
    return updatedForm;
  }

  /**
   * 删除表单项
   */
  async function deleteFormItem(projectId: string, itemId: string) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    await formApi.deleteFormItem(
      projectId,
      currentForm.value.id,
      itemId
    );

    currentForm.value.items = currentForm.value.items.filter(
      item => item.id !== itemId
    );
  }

  /**
   * 重新排序表单项
   */
  async function reorderFormItems(
    projectId: string,
    oldIndex: number,
    newIndex: number
  ) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    // 乐观更新
    const items = [...currentForm.value.items];
    const [movedItem] = items.splice(oldIndex, 1);
    items.splice(newIndex, 0, movedItem);
    currentForm.value.items = items;

    await formApi.reorderFormItems(
      projectId,
      currentForm.value.id,
      oldIndex,
      newIndex
    );
  }

  /**
   * 更新下拉选项
   */
  async function updateDropdownOptions(
    projectId: string,
    itemId: string,
    options: string[]
  ) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    await formApi.updateDropdownOptions(
      projectId,
      currentForm.value.id,
      itemId,
      options
    );

    const item = currentForm.value.items.find(i => i.id === itemId);
    if (item) {
      item.dropdown_options = options;
    }
  }

  /**
   * 切换下拉模式
   */
  async function toggleDropdownMode(
    projectId: string,
    itemId: string,
    useDropdown: boolean
  ) {
    if (!currentForm.value) {
      throw new Error("没有当前表单");
    }

    await formApi.toggleDropdownMode(
      projectId,
      currentForm.value.id,
      itemId,
      useDropdown
    );

    const item = currentForm.value.items.find(i => i.id === itemId);
    if (item) {
      item.use_dropdown = useDropdown;
    }
  }

  /**
   * 设置当前表单
   */
  function setCurrentForm(form: Form | null) {
    currentForm.value = form;
  }

  return {
    currentForm,
    createForm,
    updateForm,
    deleteForm,
    addFormItem,
    updateFormItem,
    deleteFormItem,
    reorderFormItems,
    updateDropdownOptions,
    toggleDropdownMode,
    setCurrentForm,
  };
});
