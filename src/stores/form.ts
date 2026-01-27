/**
 * 表单状态管理
 * 从 project.ts 中拆分出来的表单相关逻辑
 */

import { defineStore } from "pinia";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import * as formApi from "@/api/form";
import { useProjectStore } from "@/stores/project";
import { useUiStore } from "@/stores/ui";
import type { CommandFormat, Form, FormItemFieldValue } from "@/types/bindings";

export const useFormStore = defineStore("form", () => {
  const { t } = useI18n();

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
   * 克隆表单
   */
  async function duplicateForm(projectId: string, formId: string) {
    const uiStore = useUiStore();

    try {
      const newForm = await formApi.duplicateForm(projectId, formId);

      // 同步更新 projectStore.currentProject 中的表单列表
      const projectStore = useProjectStore();
      if (projectStore.currentProject?.id === projectId) {
        projectStore.currentProject.forms = [
          ...projectStore.currentProject.forms,
          newForm
        ];
      }

      uiStore.showToast(t('form.duplicateSuccess'), "success", 2000);

      return newForm;
    } catch (error) {
      uiStore.showToast(t('form.duplicateFailed'), "error", 3000);
      throw error;
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
      throw new Error(t('form.noCurrentForm'));
    }

    const newItem = await formApi.addFormItem(
      projectId,
      currentForm.value.id,
      itemType
    );

    // 更新 formStore
    currentForm.value.items.push(newItem);

    // 同步更新 projectStore.currentProject 中的表单
    const projectStore = useProjectStore();
    if (projectStore.currentProject?.id === projectId) {
      const formIndex = projectStore.currentProject.forms.findIndex(
        f => f.id === currentForm.value!.id
      );
      if (formIndex !== -1) {
        const form = projectStore.currentProject.forms[formIndex];
        // 创建新的 items 数组引用以触发响应式更新
        const updatedForm = {
          ...form,
          items: [...form.items, newItem]
        };
        projectStore.currentProject.forms = [
          ...projectStore.currentProject.forms.slice(0, formIndex),
          updatedForm,
          ...projectStore.currentProject.forms.slice(formIndex + 1)
        ];
      }
    }

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
      throw new Error(t('form.noCurrentForm'));
    }

    const updatedForm = await formApi.updateFormItem(
      projectId,
      currentForm.value.id,
      itemId,
      field,
      value
    );

    // 更新 formStore
    currentForm.value = updatedForm;

    // 同步更新 projectStore.currentProject 中的表单
    const projectStore = useProjectStore();
    if (projectStore.currentProject?.id === projectId) {
      const formIndex = projectStore.currentProject.forms.findIndex(
        f => f.id === updatedForm.id
      );
      if (formIndex !== -1) {
        // 创建新的数组引用以触发响应式更新
        projectStore.currentProject.forms = [
          ...projectStore.currentProject.forms.slice(0, formIndex),
          updatedForm,
          ...projectStore.currentProject.forms.slice(formIndex + 1)
        ];
      }
    }

    return updatedForm;
  }

  /**
   * 删除表单项
   */
  async function deleteFormItem(projectId: string, itemId: string) {
    if (!currentForm.value) {
      throw new Error(t('form.noCurrentForm'));
    }

    await formApi.deleteFormItem(
      projectId,
      currentForm.value.id,
      itemId
    );

    // 更新 formStore
    currentForm.value.items = currentForm.value.items.filter(
      item => item.id !== itemId
    );

    // 同步更新 projectStore.currentProject 中的表单
    const projectStore = useProjectStore();
    if (projectStore.currentProject?.id === projectId) {
      const formIndex = projectStore.currentProject.forms.findIndex(
        f => f.id === currentForm.value!.id
      );
      if (formIndex !== -1) {
        const form = projectStore.currentProject.forms[formIndex];
        // 创建新的 items 数组引用以触发响应式更新
        const updatedForm = {
          ...form,
          items: form.items.filter(item => item.id !== itemId)
        };
        projectStore.currentProject.forms = [
          ...projectStore.currentProject.forms.slice(0, formIndex),
          updatedForm,
          ...projectStore.currentProject.forms.slice(formIndex + 1)
        ];
      }
    }
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
      throw new Error(t('form.noCurrentForm'));
    }

    const formId = currentForm.value.id;

    // 调用后端 API 保存到文件系统
    await formApi.reorderFormItems(
      projectId,
      formId,
      oldIndex,
      newIndex
    );

    // 重新加载项目数据，确保 formStore 和 projectStore 同步
    const projectStore = useProjectStore();
    if (projectStore.currentProject?.id === projectId) {
      await projectStore.setCurrentProject(projectId);

      // 恢复当前表单引用
      const updatedForm = projectStore.currentProject?.forms.find(
        f => f.id === formId
      );
      if (updatedForm) {
        currentForm.value = updatedForm;
      }
    }
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
      throw new Error(t('form.noCurrentForm'));
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
      throw new Error(t('form.noCurrentForm'));
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

  /**
   * 更新命令格式
   */
  async function updateCommandFormat(projectId: string, format: CommandFormat) {
    if (!currentForm.value) {
      throw new Error(t('form.noCurrentForm'));
    }

    const updatedForm = await formApi.updateCommandFormat(
      projectId,
      currentForm.value.id,
      format
    );

    // 更新本地状态
    currentForm.value = updatedForm;

    // 同步更新 projectStore
    const projectStore = useProjectStore();
    if (projectStore.currentProject?.id === projectId) {
      const formIndex = projectStore.currentProject.forms.findIndex(
        f => f.id === updatedForm.id
      );
      if (formIndex !== -1) {
        projectStore.currentProject.forms = [
          ...projectStore.currentProject.forms.slice(0, formIndex),
          updatedForm,
          ...projectStore.currentProject.forms.slice(formIndex + 1)
        ];
      }
    }

    return updatedForm;
  }

  return {
    currentForm,
    createForm,
    updateForm,
    deleteForm,
    duplicateForm,
    addFormItem,
    updateFormItem,
    deleteFormItem,
    reorderFormItems,
    updateDropdownOptions,
    toggleDropdownMode,
    updateCommandFormat,
    setCurrentForm,
  };
});
