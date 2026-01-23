/**
 * 主题管理 Composable
 * 提供主题切换、持久化和系统主题检测功能
 */

import { ref, computed, watch, onMounted } from "vue";
import type { Theme } from "@/types/theme";

const STORAGE_KEY = "arg-forge-theme";

/** 当前主题模式（响应式） */
const currentTheme = ref<Theme>("light");

/**
 * 检测系统主题偏好
 * @returns 系统主题（"light" | "dark"）
 */
function getSystemTheme(): "light" | "dark" {
	if (typeof window === "undefined") return "light";

	if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
		return "dark";
	}
	return "light";
}

/**
 * 从 localStorage 读取主题配置
 * @returns 保存的主题配置，默认为系统主题
 */
function loadThemeFromStorage(): Theme {
	if (typeof window === "undefined") return getSystemTheme();

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored === "light" || stored === "dark") {
			return stored;
		}
	} catch (error) {
		console.warn("读取主题配置失败:", error);
	}

	// 如果没有保存的配置，使用系统主题
	return getSystemTheme();
}

/**
 * 保存主题配置到 localStorage
 * @param theme - 主题配置
 */
function saveThemeToStorage(theme: Theme) {
	if (typeof window === "undefined") return;

	try {
		localStorage.setItem(STORAGE_KEY, theme);
	} catch (error) {
		console.warn("保存主题配置失败:", error);
	}
}

/**
 * 应用主题到 DOM（添加/移除 .dark class）
 * @param theme - 要应用的主题
 */
function applyThemeToDOM(theme: "light" | "dark") {
	if (typeof window === "undefined") return;

	const root = document.documentElement;
	if (theme === "dark") {
		root.classList.add("dark");
	} else {
		root.classList.remove("dark");
	}
}

/**
 * 主题管理 Composable
 * @returns 主题操作方法和状态
 */
export function useTheme() {
	/** 是否为深色模式 */
	const isDark = computed(() => currentTheme.value === "dark");

	/**
	 * 设置主题
	 * @param theme - 主题模式
	 */
	function setTheme(theme: Theme) {
		currentTheme.value = theme;
		saveThemeToStorage(theme);
		applyThemeToDOM(theme);
	}

	/**
	 * 切换主题（在 light/dark 之间切换）
	 */
	function toggleTheme() {
		setTheme(currentTheme.value === "dark" ? "light" : "dark");
	}

	/**
	 * 初始化主题
	 * - 从 localStorage 读取配置
	 * - 应用主题到 DOM
	 */
	function initTheme() {
		// 1. 读取保存的主题配置
		currentTheme.value = loadThemeFromStorage();

		// 2. 应用主题
		applyThemeToDOM(currentTheme.value);
	}

	// 组件挂载时初始化
	onMounted(() => {
		initTheme();
	});

	// 监听主题变化，自动应用到 DOM
	watch(currentTheme, (newTheme) => {
		applyThemeToDOM(newTheme);
	});

	return {
		// 状态
		theme: computed(() => currentTheme.value),
		isDark,

		// 方法
		setTheme,
		toggleTheme,
		initTheme,
	};
}
