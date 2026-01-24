/**
 * 项目状态管理（简化版）
 * 只负责项目级别的 CRUD 操作
 * 表单管理已移至 form store
 */

import { defineStore } from "pinia";
import { ref, computed } from "vue";
import * as projectApi from "@/api/project";
import { cycleSortConfig as cycleSort, loadSortConfig, type SortConfig } from "@/composables/useProjectSort";
import type { Project } from "@/types/bindings";

export const useProjectStore = defineStore("project", () => {
  // ========== 状态 ==========
  /** 所有项目列表 */
  const projects = ref<Project[]>([]);

  /** 当前选中的项目 */
  const currentProject = ref<Project | null>(null);

  /** 非响应式排序配置 */
  let sortConfig: SortConfig = loadSortConfig();

  // ========== 计算属性 ==========
  /** 当前项目的表单列表 */
  const currentForms = computed(() => currentProject.value?.forms ?? []);

  /** 排序后的项目列表 */
  const sortedProjects = computed(() => {
    return [...projects.value].sort((a, b) => {
      const aTime = a[sortConfig.sortBy];
      const bTime = b[sortConfig.sortBy];
      const comparison = aTime.localeCompare(bTime);
      return sortConfig.order === 'asc' ? comparison : -comparison;
    });
  });

  /** 是否有项目 */
  const hasProjects = computed(() => sortedProjects.value.length > 0);

  // ========== Actions ==========

  /**
   * 循环切换排序配置
   * @returns 新的排序配置
   */
  function cycleSortConfigValue() {
    sortConfig = cycleSort(sortConfig);
    return sortConfig;
  }

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
    try {
      const newProject = await projectApi.createProject(name, description);
      await loadProjects();
      return newProject;
    } catch (error) {
      console.error("创建项目失败:", error);
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

      const index = projects.value.findIndex((p) => p.id === projectId);
      if (index !== -1) {
        projects.value[index] = updated;
      }

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
      projects.value = projects.value.filter((p) => p.id !== projectId);

      if (currentProject.value?.id === projectId) {
        currentProject.value = null;
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
        currentProject.value = await projectApi.getProject(project);
      } else {
        currentProject.value = project;
      }
    } catch (error) {
      console.error("设置当前项目失败:", error);
      throw error;
    }
  }

  /**
   * 清空所有状态
   */
  function $reset() {
    projects.value = [];
    currentProject.value = null;
  }

  return {
    // 状态
    projects,
    currentProject,

    // 计算属性
    currentForms,
    hasProjects,
    sortedProjects,

    // Actions
    loadProjects,
    createProject,
    updateProject,
    deleteProject,
    duplicateProject,
    setCurrentProject,
    cycleSortConfig: cycleSortConfigValue,
    $reset,
  };
});
