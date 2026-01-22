// Tauri Commands 模块
// 所有的前端-后端通信接口都在这里定义

pub mod project;
pub mod form;
pub mod command_gen;

// 使用 pub use 导出常用的命令，方便在 lib.rs 中注册
pub use project::*;
pub use form::*;
pub use command_gen::*;
