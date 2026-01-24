import { createI18n } from "vue-i18n";
import zh from "./zh";
import en from "./en";

/**
 * 检测系统语言
 * @returns 系统语言代码 ("zh" 或 "en")
 */
function detectSystemLocale(): "zh" | "en" {
	const lang = navigator.language || navigator.languages?.[0] || "zh";
	return lang.startsWith("en") ? "en" : "zh";
}

/**
 * 获取初始语言
 * 优先级: 用户设置 > 系统语言 > 默认中文
 */
function getInitialLocale(): "zh" | "en" {
	const stored = localStorage.getItem("arg-forge-locale");
	if (stored === "zh" || stored === "en") {
		return stored;
	}
	return detectSystemLocale();
}

/**
 * 创建 i18n 实例
 */
export const i18n = createI18n({
	legacy: false, // 使用 Composition API 模式
	locale: getInitialLocale(),
	fallbackLocale: "zh",
	messages: { zh, en },
	globalInjection: true, // 全局注入 $t 函数
});

/**
 * 导出 i18n 实例供 main.ts 使用
 */
export default i18n;
