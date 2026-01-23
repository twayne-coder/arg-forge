/**
 * Tauri API 包装器
 * 处理浏览器环境和 Tauri 环境的差异
 */

import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * 类型安全的 Tauri 命令调用
 * @template T - 返回值类型
 * @param cmd - 命令名称
 * @param args - 命令参数
 * @returns Promise<T> - 命令执行结果
 *
 * @example
 * ```typescript
 * const projects = await invoke<Project[]>('list_projects');
 * const project = await invoke<Project>('create_project', {
 *   config: { name: 'My Project', description: 'Description' }
 * });
 * ```
 */
export async function invoke<T = unknown>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  console.log(`[invoke] 命令: ${cmd}`, args);

  // 检查是否在 Tauri 环境
  if (!isTauriApp()) {
    console.error(`[invoke] ❌ 不在 Tauri 环境中`);
    console.error(`[invoke] 当前 location: ${window.location.href}`);

    // 浏览器环境：返回 Mock 数据（仅开发模式）
    if (import.meta.env?.MODE === 'development') {
      return getMockData<T>(cmd, args);
    }

    throw new Error(`[invoke] 无法在浏览器环境中调用命令 "${cmd}"`);
  }

  // Tauri 环境：调用真实命令
  try {
    console.log(`[invoke] → 调用 Rust 后端命令...`);
    const result = await tauriInvoke<T>(cmd, args);
    console.log(`[invoke] ✅ 命令执行成功`);
    return result;
  } catch (error) {
    console.error(`[invoke] ❌ 命令执行失败:`, error);
    throw error;
  }
}

/**
 * 检查是否在 Tauri 环境中
 * 安全的检测方法，不会抛出异常
 */
export function isTauriApp(): boolean {
  try {
    // 检查全局对象是否存在
    return typeof window !== 'undefined' && (
      '__TAURI__' in window ||
      '__TAURI_INTERNALS__' in window ||
      // 检查 Tauri 的特有属性
      !!(window as any).__TAURI_INTERNALS__
    );
  } catch {
    // 如果检测过程出错，返回 false
    return false;
  }
}

/**
 * Mock 数据生成器（开发环境）
 * @template T - 返回值类型
 * @param cmd - 命令名称
 * @param args - 命令参数
 * @returns Mock 数据
 */
function getMockData<T>(cmd: string, args?: Record<string, unknown>): T {
  console.warn(`[invoke] ⚠️  使用 Mock 数据（开发模式）`);

  switch (cmd) {
    case 'list_projects':
      return [] as T;

    case 'create_project': {
      const config = args?.config as { name?: string; description?: string } | undefined;
      return {
        id: crypto.randomUUID(),
        name: config?.name || '',
        description: config?.description || '',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        forms: [],
      } as T;
    }

    case 'get_project':
    case 'update_project':
    case 'delete_project':
    case 'duplicate_project':
      throw new Error(`[invoke] Mock 暂不支持命令: ${cmd}`);

    default:
      throw new Error(`[invoke] 未知命令: ${cmd}`);
  }
}
