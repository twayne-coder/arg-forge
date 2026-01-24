/**
 * 语言切换 Composable
 * 提供语言状态和切换方法
 */

import { computed } from "vue";
import { useI18n as useVueI18n } from "vue-i18n";

const STORAGE_KEY = "arg-forge-locale";

/**
 * 语言切换 Composable
 * @returns 语言操作方法和状态
 */
export function useLocale() {
	const { locale } = useVueI18n();

	/**
	 * 当前语言
	 */
	const currentLocale = computed(() => locale.value as "zh" | "en");

	/**
	 * 切换语言
	 */
	function toggleLocale() {
		const newLocale = locale.value === "zh" ? "en" : "zh";
		locale.value = newLocale;
		localStorage.setItem(STORAGE_KEY, newLocale);
	}

	/**
	 * 设置语言
	 */
	function setLocale(newLocale: "zh" | "en") {
		locale.value = newLocale;
		localStorage.setItem(STORAGE_KEY, newLocale);
	}

	return {
		currentLocale,
		toggleLocale,
		setLocale,
	};
}
