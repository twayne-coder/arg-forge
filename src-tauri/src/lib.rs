// 模块声明
mod commands;
mod error;
mod models;
mod services;

// 导出常用类型
pub use error::{AppError, Result};
pub use models::{Form, FormItem, ParamStyle, Project};
pub use services::{CommandService, StorageService};

// 导出公共服务函数（供文档测试使用）
pub use services::project_transaction::with_project_mut;
pub use services::validation::validate_project_id;

use tauri::Manager;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// 应用入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(fmt::layer().with_ansi(true))
        .init();

    let result = tauri::Builder::default()
        // 初始化 opener 插件
        .plugin(tauri_plugin_opener::init())
        // 设置应用状态
        .setup(|app| {
            tracing::info!("===== Tauri 应用初始化开始 =====");

            // 安全获取应用数据目录
            let data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("无法获取数据目录: {}", e))?;
            tracing::info!("应用数据目录: {:?}", data_dir);

            // 安全初始化存储服务
            tracing::info!("开始初始化存储服务...");
            let storage =
                StorageService::new(data_dir).map_err(|e| format!("无法初始化存储服务: {}", e))?;
            tracing::info!("✅ 存储服务初始化成功");

            // 将存储服务管理到全局状态中
            app.manage(storage);
            tracing::info!("✅ 存储服务已注册到全局状态");

            tracing::info!("===== Tauri 应用初始化完成 =====");
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
        .run(tauri::generate_context!());

    // 优雅处理启动失败
    if let Err(e) = result {
        eprintln!("启动 Tauri 应用失败: {}", e);
        std::process::exit(1);
    }
}
