// Tauri Commands 模块
// 所有的前端-后端通信接口都在这里定义

pub mod command_gen;
pub mod form;
pub mod project;

// 使用 pub use 导出常用的命令，方便在 lib.rs 中注册
pub use command_gen::*;
pub use form::*;
pub use project::*;
