/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

// Tauri 全局类型声明
interface Window {
  __TAURI__?: Record<string, unknown>;
}
