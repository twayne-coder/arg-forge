/**
 * 项目状态管理
 * 管理项目列表、当前项目、当前表单等状态
 */

import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as projectApi from "@/api/project";
import * as formApi from "@/api/form";
import type { Project, Form } from "@/types/bindings";

export const useProjectStore = defineStore("project", () => {
  // ========== 状态 ==========
  /** 所有项目列表 */
  const projects = ref<Project[]>([]);

  /** 当前选中的项目 */
  const currentProject = ref<Project | null>(null);

  /** 当前选中的表单 */
  const currentForm = ref<Form | null>(null);

  // ========== 计算属性 ==========
  /** 当前项目的表单列表 */
  const currentForms = computed(() => currentProject.value?.forms ?? []);

  /** 是否有项目 */
  const hasProjects = computed(() => projects.value.length > 0);

  // ========== Actions ==========

  /**
   * 加载所有项目
   */
  async function loadProjects() {
    try {
      projects.value = await projectApi.listProjects();
    } catch (error) {
      console.error("加载项目失败:", error);
      throw error;
    }
  }

  /**
   * 创建新项目
   * @param name - 项目名称
   * @param description - 项目描述
   */
  async function createProject(name: string, description: string) {
    console.log("[Store] createProject 开始:", { name, description });
    try {
      const newProject = await projectApi.createProject(name, description);
      console.log("[Store] API 返回成功，项目对象:", newProject);

      // 从后端重新加载列表，确保状态一致
      console.log("[Store] 开始重新加载项目列表...");
      await loadProjects();
      console.log("[Store] 项目列表重新加载完成，当前项目数量:", projects.value.length);

      return newProject;
    } catch (error) {
      console.error("[Store] 创建项目失败，错误详情:", error);
      console.error("[Store] 错误堆栈:", error instanceof Error ? error.stack : "无堆栈");
      throw error;
    }
  }

  /**
   * 更新项目信息
   * @param projectId - 项目 ID
   * @param name - 新的项目名称
   * @param description - 新的项目描述
   */
  async function updateProject(projectId: string, name: string, description: string) {
    try {
      const updated = await projectApi.updateProject(projectId, name, description);

      // 更新项目列表中的项目
      const index = projects.value.findIndex((p) => p.id === projectId);
      if (index !== -1) {
        projects.value[index] = updated;
      }

      // 如果是当前项目，也更新当前项目
      if (currentProject.value?.id === projectId) {
        currentProject.value = updated;
      }

      return updated;
    } catch (error) {
      console.error("更新项目失败:", error);
      throw error;
    }
  }

  /**
   * 删除项目
   * @param projectId - 项目 ID
   */
  async function deleteProject(projectId: string) {
    try {
      await projectApi.deleteProject(projectId);

      // 从项目列表中移除
      projects.value = projects.value.filter((p) => p.id !== projectId);

      // 如果删除的是当前项目，清空当前项目
      if (currentProject.value?.id === projectId) {
        currentProject.value = null;
        currentForm.value = null;
      }
    } catch (error) {
      console.error("删除项目失败:", error);
      throw error;
    }
  }

  /**
   * 复制项目
   * @param projectId - 要复制的项目 ID
   */
  async function duplicateProject(projectId: string) {
    try {
      const duplicated = await projectApi.duplicateProject(projectId);
      projects.value.push(duplicated);
      return duplicated;
    } catch (error) {
      console.error("复制项目失败:", error);
      throw error;
    }
  }

  /**
   * 设置当前项目
   * @param project - 项目对象或项目 ID
   */
  async function setCurrentProject(project: Project | string) {
    try {
      if (typeof project === "string") {
        // 如果传入的是 ID，从后端加载完整数据
        currentProject.value = await projectApi.getProject(project);
      } else {
        currentProject.value = project;
      }
      // 清空当前表单
      currentForm.value = null;
    } catch (error) {
      console.error("设置当前项目失败:", error);
      throw error;
    }
  }

  /**
   * 加载单个项目的完整数据
   * @param projectId - 项目 ID
   */
  async function loadProject(projectId: string) {
    try {
      currentProject.value = await projectApi.getProject(projectId);
      // 清空当前表单
      currentForm.value = null;
    } catch (error) {
      console.error("加载项目失败:", error);
      throw error;
    }
  }

  /**
   * 创建表单
   * @param data - 表单数据 { name, description }
   */
  async function createForm(data: {
    name: string;
    description?: string;
  }) {
    if (!currentProject.value) {
      throw new Error("没有当前项目");
    }

    try {
      // 创建表单
      const newForm = await formApi.createForm(
        currentProject.value.id,
        data.name,
        data.description || ""
      );

      // 更新当前项目的表单列表
      if (currentProject.value) {
        currentProject.value.forms.push(newForm);
      }

      // 重新加载项目列表以同步
      await loadProjects();

      return newForm;
    } catch (error) {
      console.error("创建表单失败:", error);
      throw error;
    }
  }

  /**
   * 更新表单
   * @param formId - 表单 ID
   * @param data - 表单数据 { name, description }
   */
  async function updateForm(
    formId: string,
    data: {
      name: string;
      description?: string;
    }
  ) {
    if (!currentProject.value) {
      throw new Error("没有当前项目");
    }

    try {
      const updated = await formApi.updateForm(
        currentProject.value.id,
        formId,
        data.name,
        data.description || ""
      );

      // 更新当前项目中的表单
      if (currentProject.value) {
        const index = currentProject.value.forms.findIndex((f) => f.id === formId);
        if (index !== -1) {
          currentProject.value.forms[index] = updated;
        }
      }

      // 如果是当前表单，也更新
      if (currentForm.value?.id === formId) {
        currentForm.value = updated;
      }

      return updated;
    } catch (error) {
      console.error("更新表单失败:", error);
      throw error;
    }
  }

  /**
   * 删除表单
   * @param formId - 表单 ID
   */
  async function deleteForm(formId: string) {
    if (!currentProject.value) {
      throw new Error("没有当前项目");
    }

    try {
      await formApi.deleteForm(currentProject.value.id, formId);

      // 从当前项目中移除表单
      if (currentProject.value) {
        currentProject.value.forms = currentProject.value.forms.filter(
          (f) => f.id !== formId
        );
      }

      // 如果删除的是当前表单，清空
      if (currentForm.value?.id === formId) {
        currentForm.value = null;
      }
    } catch (error) {
      console.error("删除表单失败:", error);
      throw error;
    }
  }

  /**
   * 添加表单项
   */
  async function addFormItem(itemType?: "Command" | "Parameter") {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      const newItem = await formApi.addFormItem(
        currentProject.value.id,
        currentForm.value.id,
        itemType
      );

      // 更新本地状态
      // 注意：currentForm 和 currentProject.forms[formIndex] 是同一个引用，
      // 只需要 push 一次即可
      currentForm.value.items.push(newItem);

      return newItem;
    } catch (error) {
      console.error("添加表单项失败:", error);
      throw error;
    }
  }

  /**
   * 更新表单项
   * @param itemId - 表单项 ID
   * @param field - 字段名
   * @param value - 新值
   */
  async function updateFormItem(itemId: string, field: string, value: any) {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      const updatedForm = await formApi.updateFormItem(
        currentProject.value.id,
        currentForm.value.id,
        itemId,
        field,
        value
      );

      // 更新本地状态 (currentForm)
      currentForm.value = updatedForm;

      // 更新本地状态 (currentProject.forms)
      const formIndex = currentProject.value.forms.findIndex(f => f.id === updatedForm.id);
      if (formIndex !== -1) {
        currentProject.value.forms[formIndex] = updatedForm;
      }

      return updatedForm;
    } catch (error) {
      console.error("更新表单项失败:", error);
      throw error;
    }
  }

  /**
   * 删除表单项
   * @param itemId - 表单项 ID
   */
  async function deleteFormItem(itemId: string) {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      await formApi.deleteFormItem(
        currentProject.value.id,
        currentForm.value.id,
        itemId
      );

      // 更新本地状态
      // 注意：currentForm 和 currentProject.forms[formIndex] 是同一个引用，
      // 只需要更新一次即可
      currentForm.value.items = currentForm.value.items.filter(item => item.id !== itemId);
    } catch (error) {
      console.error("删除表单项失败:", error);
      throw error;
    }
  }

  /**
   * 重新排序表单项
   * @param oldIndex - 原始位置
   * @param newIndex - 新位置
   */
  async function reorderFormItems(oldIndex: number, newIndex: number) {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      // 乐观更新：先在前端移动位置
      const items = [...currentForm.value.items];
      const [movedItem] = items.splice(oldIndex, 1);
      items.splice(newIndex, 0, movedItem);
      currentForm.value.items = items;

      // 同步到后端
      await formApi.reorderFormItems(
        currentProject.value.id,
        currentForm.value.id,
        oldIndex,
        newIndex
      );
    } catch (error) {
      console.error("排序失败:", error);
      // 如果后端失败，这里理想情况下应该回滚状态，但在本地 JSON 存储场景下通常能保持一致
      throw error;
    }
  }

  /**
   * 更新下拉选项
   * @param itemId - 表单项 ID
   * @param options - 选项列表
   */
  async function updateDropdownOptions(itemId: string, options: string[]) {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      await formApi.updateDropdownOptions(
        currentProject.value.id,
        currentForm.value.id,
        itemId,
        options
      );

      // 更新本地状态
      const item = currentForm.value.items.find(i => i.id === itemId);
      if (item) {
        item.dropdown_options = options;
      }
    } catch (error) {
      console.error("更新下拉选项失败:", error);
      throw error;
    }
  }

  /**
   * 切换下拉模式
   * @param itemId - 表单项 ID
   * @param useDropdown - 是否启用
   */
  async function toggleDropdownMode(itemId: string, useDropdown: boolean) {
    if (!currentProject.value || !currentForm.value) {
      throw new Error("没有选中的项目或表单");
    }

    try {
      await formApi.toggleDropdownMode(
        currentProject.value.id,
        currentForm.value.id,
        itemId,
        useDropdown
      );

      // 更新本地状态
      const item = currentForm.value.items.find(i => i.id === itemId);
      if (item) {
        item.use_dropdown = useDropdown;
      }
    } catch (error) {
      console.error("切换模式失败:", error);
      throw error;
    }
  }

  /**
   * 设置当前表单
   * @param form - 表单对象
   */
  function setCurrentForm(form: Form | null) {
    currentForm.value = form;
  }

  /**
   * 清空所有状态
   */
  function $reset() {
    projects.value = [];
    currentProject.value = null;
    currentForm.value = null;
  }

  return {
    // 状态
    projects,
    currentProject,
    currentForm,

    // 计算属性
    currentForms,
    hasProjects,

    // Actions
    loadProjects,
    loadProject,
    createProject,
    updateProject,
    deleteProject,
    duplicateProject,
    setCurrentProject,
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
    $reset,
  };
});
