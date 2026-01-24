/**
 * 统一的日志工具
 * 根据环境变量控制日志输出
 */

const LOG_LEVELS = {
  DEBUG: 0,
  INFO: 1,
  WARN: 2,
  ERROR: 3,
} as const;

type LogLevel = keyof typeof LOG_LEVELS;

class Logger {
  constructor(private context: string) {}

  private shouldLog(level: LogLevel): boolean {
    // 开发环境：输出所有日志
    if (import.meta.env.DEV) {
      return true;
    }

    // 生产环境：根据环境变量控制
    const envLevel = (import.meta.env.VITE_LOG_LEVEL || 'ERROR') as LogLevel;
    return LOG_LEVELS[level] >= LOG_LEVELS[envLevel];
  }

  debug(message: string, ...args: unknown[]) {
    if (this.shouldLog('DEBUG')) {
      console.debug(`[${this.context}] ${message}`, ...args);
    }
  }

  info(message: string, ...args: unknown[]) {
    if (this.shouldLog('INFO')) {
      console.info(`[${this.context}] ${message}`, ...args);
    }
  }

  warn(message: string, ...args: unknown[]) {
    if (this.shouldLog('WARN')) {
      console.warn(`[${this.context}] ${message}`, ...args);
    }
  }

  error(message: string, error?: unknown) {
    if (this.shouldLog('ERROR')) {
      console.error(`[${this.context}] ${message}`, error);
    }
  }
}

/**
 * 创建日志实例
 * @param context - 日志上下文（通常是组件名或模块名）
 *
 * @example
 * const logger = useLogger('ProjectAPI');
 * logger.info('API 调用成功', { projectId });
 * logger.error('API 调用失败', error);
 */
export function useLogger(context: string) {
  return new Logger(context);
}
