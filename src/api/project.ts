/**
 * 项目 API
 * 封装所有项目相关的 Tauri IPC 调用
 */

import { invoke } from "@/lib/tauri";
import type { Project } from "@/types/bindings";

/**
 * 创建新项目
 * @param name - 项目名称
 * @param description - 项目描述
 * @returns 创建的项目对象
 */
export async function createProject(
  name: string,
  description: string
): Promise<Project> {
  console.log("[API] createProject 被调用:", { name, description });
  const payload = { config: { name, description } };
  console.log("[API] 准备发送的 payload:", JSON.stringify(payload));

  try {
    const result = await invoke<Project>("create_project", payload);
    console.log("[API] create_project 调用成功，返回结果:", result);
    return result;
  } catch (error) {
    console.error("[API] create_project 调用失败，错误详情:", error);
    console.error("[API] 错误类型:", typeof error);
    console.error("[API] 错误字符串:", String(error));
    throw error;
  }
}

/**
 * 获取所有项目列表
 * @returns 项目数组
 */
export async function listProjects(): Promise<Project[]> {
  return await invoke("list_projects");
}

/**
 * 获取单个项目详情
 * @param projectId - 项目 ID
 * @returns 项目对象
 */
export async function getProject(projectId: string): Promise<Project> {
  return await invoke("get_project", { projectId });
}

/**
 * 更新项目信息
 * @param projectId - 项目 ID
 * @param name - 新的项目名称
 * @param description - 新的项目描述
 * @returns 更新后的项目对象
 */
export async function updateProject(
  projectId: string,
  name: string,
  description: string
): Promise<Project> {
  return await invoke("update_project", { projectId, name, description });
}

/**
 * 删除项目
 * @param projectId - 项目 ID
 */
export async function deleteProject(projectId: string): Promise<void> {
  await invoke("delete_project", { projectId });
}

/**
 * 复制项目（深拷贝）
 * @param projectId - 要复制的项目 ID
 * @returns 新创建的项目对象
 */
export async function duplicateProject(projectId: string): Promise<Project> {
  return await invoke("duplicate_project", { projectId });
}
