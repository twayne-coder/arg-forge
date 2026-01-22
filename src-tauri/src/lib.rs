// 模块声明
mod commands;
mod error;
mod models;
mod services;

// 导出常用类型
pub use error::{AppError, Result};
pub use models::{Form, FormItem, ParamStyle, Project};
pub use services::{CommandService, StorageService};

use std::path::PathBuf;
use tauri::Manager;

/// 获取应用数据目录
/// 跨平台支持 Windows、macOS、Linux
fn get_data_dir() -> PathBuf {
    // 获取 Tauri 的应用数据目录
    // Windows: %APPDATA%/arg-forge/
    // macOS: ~/Library/Application Support/arg-forge/
    // Linux: ~/.config/arg-forge/
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("无法初始化 Tauri 应用")
        .app_handle()
        .path()
        .app_data_dir()
        .expect("无法获取应用数据目录");

    app
}

/// 应用入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 初始化 opener 插件
        .plugin(tauri_plugin_opener::init())
        // 设置应用状态
        .setup(|app| {
            // 获取应用数据目录
            let data_dir = app.path().app_data_dir().expect("无法获取数据目录");

            // 初始化存储服务
            let storage = StorageService::new(data_dir)
                .expect("无法初始化存储服务");

            // 将存储服务管理到全局状态中
            app.manage(storage);

            Ok(())
        })
        // 注册所有 Tauri Commands
        .invoke_handler(tauri::generate_handler![
            // 项目管理命令 (6 个)
            commands::create_project,
            commands::list_projects,
            commands::get_project,
            commands::update_project,
            commands::delete_project,
            commands::duplicate_project,
            // 表单管理命令 (8 个)
            commands::create_form,
            commands::update_form,
            commands::delete_form,
            commands::update_form_item,
            commands::add_form_item,
            commands::delete_form_item,
            commands::reorder_form_items,
            commands::update_dropdown_options,
            commands::toggle_dropdown_mode,
            // 命令生成命令 (1 个)
            commands::generate_command,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
