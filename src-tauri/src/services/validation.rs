use crate::error::{AppError, Result};
use uuid::Uuid;

/// 验证项目 ID 格式
///
/// 验证项目 ID 是否为有效的 UUID 格式，防止路径遍历攻击。
///
/// # 安全检查
/// - 防止路径遍历攻击（..、/、\）
/// - 验证 UUID 格式正确性
///
/// # 参数
/// * `project_id` - 待验证的项目 ID 字符串
///
/// # 返回
/// 成功时返回解析后的 Uuid 对象，失败时返回错误
///
/// # 示例
/// ```
/// use arg_forge_lib::validate_project_id;
///
/// // 有效的 UUID
/// assert!(validate_project_id("550e8400-e29b-41d4-a716-446655440000").is_ok());
///
/// // 路径遍历攻击
/// assert!(validate_project_id("../etc/passwd").is_err());
/// ```
pub fn validate_project_id(project_id: &str) -> Result<Uuid> {
    // 防止路径遍历攻击：检查非法字符
    if project_id.contains("..") {
        return Err(AppError::InvalidArgument(
            "项目 ID 包含非法字符 (路径遍历)".to_string(),
        ));
    }

    if project_id.contains('/') || project_id.contains('\\') {
        return Err(AppError::InvalidArgument(
            "项目 ID 包含路径分隔符".to_string(),
        ));
    }

    // 验证 UUID 格式
    Uuid::parse_str(project_id).map_err(|_| AppError::InvalidArgument(format!(
        "无效的项目 ID 格式: {}",
        project_id
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_id_valid_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let result = validate_project_id(uuid_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), uuid_str);
    }

    #[test]
    fn test_validate_project_id_path_traversal_double_dot() {
        assert!(validate_project_id("../etc/passwd").is_err());
        assert!(validate_project_id("..").is_err());
        assert!(validate_project_id("abc/../def").is_err());
    }

    #[test]
    fn test_validate_project_id_path_separators() {
        assert!(validate_project_id("path/to/file").is_err());
        assert!(validate_project_id("path\\to\\file").is_err());
    }

    #[test]
    fn test_validate_project_id_invalid_format() {
        assert!(validate_project_id("not-a-uuid").is_err());
        assert!(validate_project_id("12345").is_err());
        assert!(validate_project_id("").is_err());
    }

    #[test]
    fn test_validate_project_id_nil_uuid() {
        // nil UUID (00000000-0000-0000-0000-000000000000) 是有效的
        assert!(validate_project_id("00000000-0000-0000-0000-000000000000").is_ok());
    }
}
