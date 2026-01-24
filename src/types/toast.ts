/**
 * Toast 通知系统类型定义
 */

/** Toast 类型 */
export type ToastType = "success" | "error" | "info" | "warning";

/** Toast 通知接口 */
export interface Toast {
  /** 唯一标识 */
  id: string;
  /** 消息内容 */
  message: string;
  /** 类型 */
  type: ToastType;
  /** 显示时长（毫秒），默认 3000ms */
  duration?: number;
  /** 创建时间戳 */
  createdAt: number;
}
