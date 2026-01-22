/**
 * 命令生成 API
 * 封装命令生成相关的 Tauri IPC 调用
 */

import { invoke } from "@/lib/tauri";

/**
 * 生成命令字符串
 * @param projectId - 项目 ID
 * @param formId - 表单 ID
 * @returns 生成的完整命令字符串
 */
export async function generateCommand(
  projectId: string,
  formId: string
): Promise<string> {
  return await invoke("generate_command", { projectId, formId });
}
