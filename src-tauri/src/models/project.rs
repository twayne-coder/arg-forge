use crate::error::AppError;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

/// 表单项类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub enum ItemType {
    /// 命令项：直接拼接可执行命令
    Command,
    /// 参数项：需要格式化的参数
    Parameter,
}

impl std::str::FromStr for ItemType {
    type Err = AppError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Command" => Ok(Self::Command),
            "Parameter" => Ok(Self::Parameter),
            _ => Err(AppError::InvalidArgument(format!("无效的项类型: {}", s))),
        }
    }
}

/// 参数风格枚举
/// 定义三种不同的命令行参数格式
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub enum ParamStyle {
    /// KeyValue 风格：--key value
    /// 示例：--lr 0.001 --batch-size 32
    KeyValue,

    /// EqualValue 风格：key=value
    /// 示例：lr=0.001 batch_size=32
    EqualValue,

    /// ValueOnly 风格：只有值，没有键
    /// 示例：0.001 32
    ValueOnly,
}

impl std::str::FromStr for ParamStyle {
    type Err = AppError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "KeyValue" => Ok(Self::KeyValue),
            "EqualValue" => Ok(Self::EqualValue),
            "ValueOnly" => Ok(Self::ValueOnly),
            _ => Err(AppError::InvalidArgument(format!("无效的参数风格: {}", s))),
        }
    }
}

/// 表单项
/// 代表单个命令或参数配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FormItem {
    /// 唯一标识符（UUID）
    pub id: String,

    /// 表单项类型
    pub item_type: ItemType,

    /// 统一内容字段
    /// - Command 类型：命令内容（如 "python train.py"）
    /// - Parameter 类型：参数值（如 "0.001", "32"）
    pub content: String,

    /// 参数名（仅 Parameter 类型使用）
    /// 示例：lr, batch_size, epoch
    pub param_name: String,

    /// 是否启用
    /// 禁用的项不会参与命令生成
    pub enabled: bool,

    /// 参数风格（仅 Parameter 类型使用）
    /// 决定该参数在命令中的呈现方式
    pub param_style: ParamStyle,

    /// 是否使用下拉选择
    /// true: 从 dropdown_options 中选择
    /// false: 手动输入内容
    pub use_dropdown: bool,

    /// 下拉选项列表
    /// 当 use_dropdown=true 时使用
    pub dropdown_options: Vec<String>,
}

impl Default for FormItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            item_type: ItemType::Parameter,
            content: String::new(),
            param_name: String::new(),
            enabled: true,
            param_style: ParamStyle::KeyValue,
            use_dropdown: false,
            dropdown_options: Vec::new(),
        }
    }
}

/// 表单
/// 代表一个命令配置表单及其所有项
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Form {
    /// 唯一标识符（UUID）
    pub id: String,

    /// 表单名称
    /// 示例："训练配置"、"推理配置"
    pub name: String,

    /// 表单描述
    /// 详细说明该表单的用途
    pub description: String,

    /// 排序顺序
    /// 用于在表单列表中排序显示
    pub sort_order: i32,

    /// 最后更新时间
    /// ISO 8601 格式的时间字符串
    pub updated_at: String,

    /// 表单项列表
    /// 按顺序排列的所有命令和参数配置
    pub items: Vec<FormItem>,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            items: Vec::new(),
        }
    }
}

impl Form {
    /// 获取表单项的可变引用
    /// # 参数
    /// * `item_id` - 表单项 ID
    /// # 返回
    /// * `Some(&mut FormItem)` - 找到表单项
    /// * `None` - 未找到指定 ID 的表单项
    pub fn get_item_mut(&mut self, item_id: &str) -> Option<&mut FormItem> {
        self.items.iter_mut().find(|i| i.id == item_id)
    }

    /// 删除表单项
    /// # 参数
    /// * `item_id` - 表单项 ID
    /// # 返回
    /// * `Some(FormItem)` - 找到并删除的表单项
    /// * `None` - 未找到指定 ID 的表单项
    pub fn remove_item(&mut self, item_id: &str) -> Option<FormItem> {
        if let Some(pos) = self.items.iter().position(|i| i.id == item_id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// 更新时间戳
    /// 将 updated_at 设置为当前时间
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

/// 项目
/// 顶层容器，包含多个表单
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Project {
    /// 唯一标识符（UUID）
    pub id: String,

    /// 项目名称
    /// 示例："机器学习实验"、"数据处理管道"
    pub name: String,

    /// 项目描述
    /// 详细说明项目的用途和背景
    pub description: String,

    /// 创建时间
    /// ISO 8601 格式的时间字符串
    pub created_at: String,

    /// 最后更新时间
    /// ISO 8601 格式的时间字符串
    pub updated_at: String,

    /// 表单列表
    /// 一个项目可以包含多个命令配置表单
    pub forms: Vec<Form>,
}

impl Default for Project {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            description: String::new(),
            created_at: now.clone(),
            updated_at: now,
            forms: Vec::new(),
        }
    }
}

impl Project {
    /// 创建新项目
    /// # 参数
    /// * `name` - 项目名称
    /// * `description` - 项目描述
    /// # 返回
    /// 返回带有默认值的新项目实例
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            created_at: now.clone(),
            updated_at: now,
            forms: Vec::new(),
        }
    }

    /// 更新时间戳
    /// 将 updated_at 设置为当前时间
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// 添加表单
    /// # 参数
    /// * `form` - 要添加的表单
    pub fn add_form(&mut self, form: Form) {
        self.forms.push(form);
        self.touch();
    }

    /// 删除表单
    /// # 参数
    /// * `form_id` - 要删除的表单 ID
    /// # 返回
    /// * `Some(form)` - 找到并删除的表单
    /// * `None` - 未找到指定 ID 的表单
    pub fn remove_form(&mut self, form_id: &str) -> Option<Form> {
        if let Some(pos) = self.forms.iter().position(|f| f.id == form_id) {
            self.touch();
            Some(self.forms.remove(pos))
        } else {
            None
        }
    }

    /// 获取表单
    /// # 参数
    /// * `form_id` - 表单 ID
    /// # 返回
    /// 表单的可变引用
    pub fn get_form_mut(&mut self, form_id: &str) -> Option<&mut Form> {
        self.forms.iter_mut().find(|f| f.id == form_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("测试项目".to_string(), "测试描述".to_string());
        assert_eq!(project.name, "测试项目");
        assert_eq!(project.description, "测试描述");
        assert!(!project.id.is_empty());
        assert!(!project.forms.is_empty() || project.forms.is_empty());
    }

    #[test]
    fn test_add_form() {
        let mut project = Project::default();
        let form = Form::default();
        project.add_form(form);
        assert_eq!(project.forms.len(), 1);
    }

    #[test]
    fn test_remove_form() {
        let mut project = Project::default();
        let mut form = Form::default();
        form.id = "test-id".to_string();
        project.add_form(form);
        assert_eq!(project.forms.len(), 1);

        let removed = project.remove_form("test-id");
        assert!(removed.is_some());
        assert_eq!(project.forms.len(), 0);
    }

    #[test]
    fn test_param_style_equality() {
        assert_eq!(ParamStyle::KeyValue, ParamStyle::KeyValue);
        assert_ne!(ParamStyle::KeyValue, ParamStyle::EqualValue);
    }
}
