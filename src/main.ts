import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import router from "./router";
import i18n from "./locales";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);
app.mount("#app");

// 全局禁用右键菜单（兜底方案）
document.addEventListener(
  "contextmenu",
  (e) => {
    e.preventDefault();
  },
  { capture: true },
);