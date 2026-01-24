<div align="center">
<img src="icon.png" alt="ArgForge Logo" width="128" height="128">

# ArgForge

**可视化命令行参数配置工具**

</div>

---

## 项目介绍

ArgForge 是一款桌面应用程序，旨在通过可视化表单界面简化命令行参数的配置过程。用户无需记忆复杂的参数格式，即可通过直观的界面生成标准化的命令行指令。

**适用场景**：
- 开发者日常工具链配置
- 复杂命令参数模板管理
- 团队命令规范统一
- 自动化脚本参数生成

---

## 核心功能

| 功能 | 描述 |
|------|------|
| 📁 项目管理 | 创建、编辑、删除、复制项目，支持项目分组 |
| 📝 表单配置 | 灵活的命令模板设计，支持 `{params}` 占位符 |
| 🎯 参数风格 | 支持 Argparse (`--key value`)、Hydra (`key=value`)、Positional (`value`) |
| 🔽 下拉选择 | 为参数配置预设选项，快速选择常用值 |
| 🖱️ 拖拽排序 | 直观拖拽调整表单项顺序 |
| 👁️ 实时预览 | 所见即所得的命令预览 |
| 💾 自动备份 | 数据变更前自动创建备份文件 |
| 🎨 现代 UI | Windows 11 风格界面 |

---

## 技术栈

### 前端
- **框架**：Vue 3 + TypeScript + Vite
- **状态管理**：Pinia
- **UI 组件**：Shadcn-vue + Reka UI
- **样式**：Tailwind CSS
- **图标**：Lucide Vue
- **路由**：Vue Router
- **工具**：VueUse, SortableJS

### 后端
- **框架**：Tauri v2
- **语言**：Rust (Edition 2021)
- **序列化**：serde + serde_json
- **类型同步**：tauri-specta
- **错误处理**：thiserror + anyhow
- **异步**：tokio
- **数据**：uuid, chrono

---

## 安装使用

### 前置要求

- **Node.js** >= 18
- **pnpm** >= 8
- **Rust** >= 1.70
- **系统依赖**：
  - Windows: WebView2 Runtime
  - macOS: 无额外依赖
  - Linux: webkit2gtk

### 开发模式

```bash
# 克隆项目
git clone https://github.com/yourusername/arg-forge.git
cd arg-forge

# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri dev
```

### 生产构建

```bash
# 构建应用
pnpm tauri build

# 构建产物位于 src-tauri/target/release/
```

---

## 项目结构

```
arg-forge/
├── src/                      # 前端源码
│   ├── api/                  # IPC 调用封装
│   ├── assets/               # 静态资源
│   ├── components/           # Vue 组件
│   │   ├── features/         # 业务组件
│   │   ├── layout/           # 布局组件
│   │   └── ui/               # 基础 UI 组件
│   ├── composables/          # 组合式函数
│   ├── lib/                  # 工具库
│   ├── router/               # 路由配置
│   ├── stores/               # Pinia 状态
│   └── types/                # TypeScript 类型
├── src-tauri/                # 后端源码
│   ├── src/
│   │   ├── commands/         # Tauri Commands
│   │   ├── models/           # 数据模型
│   │   ├── services/         # 业务逻辑
│   │   └── error.rs          # 错误定义
│   └── icons/                # 应用图标
├── public/                   # 公共资源
├── CLAUDE.md                 # 项目文档
└── README.md                 # 本文件
```

---

## 界面预览

> 简洁的 Windows 11 风格设计，支持自定义标题栏、Segoe UI 字体、圆角元素

---

## 路线图

- [ ] 命令执行历史记录
- [ ] 参数校验规则
- [ ] 导出/导入项目配置
- [ ] 环境变量支持
- [ ] 多语言支持

---

## 许可证

MIT License

---

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式前端框架
- [Shadcn-vue](https://www.shadcn-vue.com/) - Vue 组件库
