import { createI18n } from "vue-i18n";
import zh from "./zh";
import en from "./en";

/**
 * 创建 i18n 实例
 */
export const i18n = createI18n({
	legacy: false, // 使用 Composition API 模式
	locale: localStorage.getItem("arg-forge-locale") || "zh",
	fallbackLocale: "zh",
	messages: { zh, en },
	globalInjection: true, // 全局注入 $t 函数
});

/**
 * 导出 i18n 实例供 main.ts 使用
 */
export default i18n;
