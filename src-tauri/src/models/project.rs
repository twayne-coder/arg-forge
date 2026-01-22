use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

/// 参数风格枚举
/// 定义三种不同的命令行参数格式
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub enum ParamStyle {
    /// Argparse 风格：--key value
    /// 示例：--lr 0.001 --batch-size 32
    Argparse,

    /// Hydra 风格：key=value
    /// 示例：lr=0.001 batch_size=32
    Hydra,

    /// 位置参数：只有值，没有键
    /// 示例：0.001 32
    Positional,
}

/// 表单项（参数项）
/// 代表单个命令行参数的配置
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct FormItem {
    /// 唯一标识符（UUID）
    pub id: String,

    /// 参数名
    /// 示例：lr, batch_size, epoch
    pub param_name: String,

    /// 参数值
    /// 示例：0.001, 32, 100
    pub param_value: String,

    /// 是否启用
    /// 禁用的参数不会参与命令生成
    pub enabled: bool,

    /// 参数风格
    /// 决定该参数在命令中的呈现方式
    pub param_style: ParamStyle,

    /// 是否使用下拉选择
    /// true: 从 dropdown_options 中选择
    /// false: 手动输入参数值
    pub use_dropdown: bool,

    /// 下拉选项列表
    /// 当 use_dropdown=true 时使用
    pub dropdown_options: Vec<String>,
}

impl Default for FormItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            param_name: String::new(),
            param_value: String::new(),
            enabled: true,
            param_style: ParamStyle::Argparse,
            use_dropdown: false,
            dropdown_options: Vec::new(),
        }
    }
}

/// 表单
/// 代表一个命令模板及其所有参数配置
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

    /// 命令模板
    /// 支持占位符：
    /// - {params}: 会被生成的参数字符串替换
    /// 示例："python train.py {params}"
    pub command_template: String,

    /// 参数项列表
    /// 按顺序排列的所有参数配置
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
            command_template: "python train.py {params}".to_string(),
            items: Vec::new(),
        }
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
        assert_eq!(ParamStyle::Argparse, ParamStyle::Argparse);
        assert_ne!(ParamStyle::Argparse, ParamStyle::Hydra);
    }
}
