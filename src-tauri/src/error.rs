use thiserror::Error;

/// 应用错误类型
/// 使用 thiserror 定义所有可能的错误
#[derive(Error, Debug)]
pub enum AppError {
    /// IO 错误
    /// 文件读写、目录操作等失败
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// JSON 序列化/反序列化错误
    /// JSON 数据的序列化或反序列化失败
    #[error("JSON 错误: {0}")]
    Json(#[from] serde_json::Error),

    /// 项目未找到错误
    /// 尝试访问不存在的项目 ID
    #[error("项目未找到: {0}")]
    ProjectNotFound(String),

    /// 表单未找到错误
    /// 尝试访问不存在的表单 ID
    #[error("表单未找到: {0}")]
    FormNotFound(String),

    /// 表单项未找到错误
    /// 尝试访问不存在的表单项 ID
    #[error("表单项未找到: {0}")]
    FormItemNotFound(String),

    /// 无效参数错误
    /// 提供的参数不符合要求
    #[error("无效参数: {0}")]
    InvalidArgument(String),

    /// 命令生成错误
    /// 生成命令时遇到问题
    #[error("命令生成失败: {0}")]
    CommandGeneration(String),

    /// 备份恢复错误
    /// 从备份文件恢复数据时失败
    #[error("备份恢复失败: {0}")]
    BackupRestore(String),

    /// 应用初始化错误
    /// 应用启动或初始化过程中失败
    #[error("应用初始化失败: {0}")]
    Initialization(String),
}

/// 将 AppError 转换为 String
/// 用于 Tauri Command 的返回值
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

/// 结果类型别名
/// 简化返回类型的书写
pub type Result<T> = std::result::Result<T, AppError>;
