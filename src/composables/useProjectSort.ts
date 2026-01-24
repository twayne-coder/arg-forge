/**
 * 项目排序配置管理
 * 使用非响应式配置，避免不必要的性能开销
 */

export interface SortConfig {
  sortBy: 'created_at' | 'updated_at';
  order: 'asc' | 'desc';
}

const STORAGE_KEY = 'project-sort-config';

/**
 * 加载排序配置
 */
export function loadSortConfig(): SortConfig {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved) {
    try {
      const parsed = JSON.parse(saved) as SortConfig;
      if (isValidConfig(parsed)) {
        return parsed;
      }
    } catch {
      // 忽略解析错误
    }
  }
  return { sortBy: 'updated_at', order: 'desc' };
}

/**
 * 保存排序配置
 */
export function saveSortConfig(config: SortConfig): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
}

/**
 * 循环切换排序配置
 * 顺序：修改时间降序 → 修改时间正序 → 创建时间降序 → 创建时间正序 → 回到修改时间降序
 */
export function cycleSortConfig(current: SortConfig): SortConfig {
  const { sortBy, order } = current;
  let newConfig: SortConfig;

  if (sortBy === 'updated_at' && order === 'desc') {
    newConfig = { sortBy: 'updated_at', order: 'asc' };
  } else if (sortBy === 'updated_at' && order === 'asc') {
    newConfig = { sortBy: 'created_at', order: 'desc' };
  } else if (sortBy === 'created_at' && order === 'desc') {
    newConfig = { sortBy: 'created_at', order: 'asc' };
  } else {
    newConfig = { sortBy: 'updated_at', order: 'desc' };
  }

  saveSortConfig(newConfig);
  return newConfig;
}

/**
 * 验证配置有效性
 */
function isValidConfig(config: unknown): config is SortConfig {
  if (typeof config !== 'object' || config === null) return false;
  const c = config as Record<string, unknown>;
  return (
    (c.sortBy === 'created_at' || c.sortBy === 'updated_at') &&
    (c.order === 'asc' || c.order === 'desc')
  );
}

/**
 * 使用排序配置
 */
export function useProjectSort() {
  const sortConfig = loadSortConfig();

  return {
    sortConfig,
    cycleSortConfig: () => cycleSortConfig(sortConfig),
  };
}
