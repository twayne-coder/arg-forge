/**
 * 窗口控制 API
 * 封装 Tauri 窗口操作，遵循项目 IPC 封装规范
 */

import { isTauriApp } from '@/lib/tauri';
import { getCurrentWindow as getTauriWindow } from '@tauri-apps/api/window';

// 窗口 API 缓存
let windowInstance: any = null;

/**
 * 获取当前窗口实例
 */
export async function getCurrentWindow() {
	if (!isTauriApp()) {
		throw new Error('Window API 仅在 Tauri 环境中可用');
	}

	if (!windowInstance) {
		windowInstance = await getTauriWindow();
	}

	return windowInstance;
}

/**
 * 最小化窗口
 */
export async function minimizeWindow(): Promise<void> {
	const window = await getCurrentWindow();
	await window.minimize();
}

/**
 * 最大化窗口
 */
export async function maximizeWindow(): Promise<void> {
	const window = await getCurrentWindow();
	await window.maximize();
}

/**
 * 取消最大化
 */
export async function unmaximizeWindow(): Promise<void> {
	const window = await getCurrentWindow();
	await window.unmaximize();
}

/**
 * 切换最大化状态
 */
export async function toggleMaximize(): Promise<void> {
	const window = await getCurrentWindow();
	await window.toggleMaximize();
}

/**
 * 关闭窗口
 */
export async function closeWindow(): Promise<void> {
	const window = await getCurrentWindow();
	await window.close();
}

/**
 * 检查窗口是否最大化
 */
export async function isMaximized(): Promise<boolean> {
	const window = await getCurrentWindow();
	return await window.isMaximized();
}

/**
 * 开始拖动窗口
 */
export async function startDragging(): Promise<void> {
	const window = await getCurrentWindow();
	await window.startDragging();
}

export { isTauriApp };
