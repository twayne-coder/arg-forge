/**
 * Tauri API 包装器
 * 处理浏览器环境和 Tauri 环境的差异
 */

declare global {
  interface Window {
    __TAURI__?: unknown;
  }

  interface ImportMeta {
    env?: {
      MODE: string;
    };
  }
}

let invokeCache: ((cmd: string, args?: Record<string, unknown>) => Promise<unknown>) | null = null;

/**
 * 安全的 invoke 函数
 * 只在 Tauri 环境中调用，浏览器环境返回 mock 数据
 */
export async function invoke<T = unknown>(
  cmd: string,
  args?: Record<string, unknown>
): Promise<T> {
  // 检查是否在 Tauri 环境中
  const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

  if (!isTauri) {
    // 浏览器环境：返回 mock 数据或抛出错误
    console.warn(`[Mock] invoke "${cmd}" called in browser environment`, args);

    // 对于开发环境，可以返回一些 mock 数据
    if (import.meta.env?.MODE === 'development') {
      // 根据不同的命令返回不同的 mock 数据
      if (cmd === 'list_projects') {
        return [] as T;
      }
      if (cmd === 'create_project') {
        return {
          id: crypto.randomUUID(),
          name: args?.name,
          description: args?.description,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          forms: [],
        } as T;
      }
    }

    throw new Error(`Cannot invoke "${cmd}" in browser environment`);
  }

  // Tauri 环境：动态导入并调用
  if (!invokeCache) {
    const { invoke: invokeFn } = await import('@tauri-apps/api/core');
    invokeCache = invokeFn;
  }

  return invokeCache<T>(cmd, args);
}

/**
 * 检查是否在 Tauri 环境中
 */
export function isTauriApp(): boolean {
  return typeof window !== 'undefined' && window.__TAURI__ !== undefined;
}
