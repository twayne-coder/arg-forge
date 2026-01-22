use crate::error::Result;
use crate::models::{Form, FormItem, ParamStyle};

/// 命令生成服务
/// 负责将表单配置转换为实际的命令行字符串
pub struct CommandService;

impl CommandService {
    /// 生成命令字符串
    /// # 参数
    /// * `form` - 表单配置
    /// # 返回
    /// 生成的完整命令字符串
    ///
    /// # 工作流程
    /// 1. 过滤出启用的参数项
    /// 2. 过滤出有值的参数项
    /// 3. 按顺序生成每个参数的字符串表示
    /// 4. 替换命令模板中的占位符
    ///
    /// # 示例
    /// ```text
    /// 假设有以下配置:
    /// - command_template: "python train.py {params}"
    /// - items: [
    ///     ParamStyle::Argparse: --lr 0.001
    ///     ParamStyle::Hydra: batch_size=32
    ///     ParamStyle::Positional: 100
    ///   ]
    /// 生成结果: "python train.py --lr 0.001 batch_size=32 100"
    /// ```
    pub fn generate_command(form: &Form) -> Result<String> {
        // 1. 生成参数字符串
        let params_str = Self::generate_params_string(form)?;

        // 2. 替换命令模板中的占位符
        let command = form
            .command_template
            .replace("{params}", &params_str)
            .replace("{params}", &params_str); // 支持多次替换

        Ok(command)
    }

    /// 生成参数字符串
    /// # 参数
    /// * `form` - 表单配置
    /// # 返回
    /// 参数字符串（不含命令模板）
    fn generate_params_string(form: &Form) -> Result<String> {
        let mut param_parts = Vec::new();

        // 遍历所有表单项（保持顺序）
        for item in &form.items {
            // 跳过禁用的参数
            if !item.enabled {
                continue;
            }

            // 跳过空值参数
            if item.param_value.is_empty() {
                continue;
            }

            // 根据参数风格生成字符串
            let param_str = Self::format_item(item)?;
            if !param_str.is_empty() {
                param_parts.push(param_str);
            }
        }

        // 用空格连接所有参数
        Ok(param_parts.join(" "))
    }

    /// 格式化单个表单项
    /// # 参数
    /// * `item` - 表单项
    /// # 返回
    /// 格式化后的参数字符串
    fn format_item(item: &FormItem) -> Result<String> {
        match item.param_style {
            // Argparse 风格：--key value
            // 示例：--lr 0.001
            ParamStyle::Argparse => {
                if item.param_name.is_empty() {
                    // 如果没有参数名，只返回值
                    Ok(item.param_value.clone())
                } else {
                    Ok(format!("--{} {}", item.param_name, item.param_value))
                }
            }

            // Hydra 风格：key=value
            // 示例：lr=0.001
            ParamStyle::Hydra => {
                if item.param_name.is_empty() {
                    // 如果没有参数名，只返回值
                    Ok(item.param_value.clone())
                } else {
                    Ok(format!("{}={}", item.param_name, item.param_value))
                }
            }

            // 位置参数：只有值
            // 示例：0.001
            ParamStyle::Positional => Ok(item.param_value.clone()),
        }
    }

    /// 预览命令（不实际生成，用于实时预览）
    /// # 参数
    /// * `form` - 表单配置
    /// # 返回
    /// 预览的命令字符串
    ///
    /// 与 `generate_command` 相同，但方法名更语义化
    pub fn preview(form: &Form) -> Result<String> {
        Self::generate_command(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FormItem;
    use uuid::Uuid;

    fn create_test_form() -> Form {
        Form {
            id: Uuid::new_v4().to_string(),
            name: "测试表单".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "python train.py {params}".to_string(),
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    param_name: "lr".to_string(),
                    param_value: "0.001".to_string(),
                    enabled: true,
                    param_style: ParamStyle::Argparse,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    param_name: "batch_size".to_string(),
                    param_value: "32".to_string(),
                    enabled: true,
                    param_style: ParamStyle::Hydra,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    param_name: "".to_string(),
                    param_value: "100".to_string(),
                    enabled: true,
                    param_style: ParamStyle::Positional,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        }
    }

    #[test]
    fn test_generate_command_argparse() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "train.py {params}".to_string(),
            items: vec![FormItem {
                id: Uuid::new_v4().to_string(),
                param_name: "lr".to_string(),
                param_value: "0.001".to_string(),
                enabled: true,
                param_style: ParamStyle::Argparse,
                use_dropdown: false,
                dropdown_options: vec![],
            }],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py --lr 0.001");
    }

    #[test]
    fn test_generate_command_hydra() {
        let mut form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "train.py {params}".to_string(),
            items: vec![FormItem {
                id: Uuid::new_v4().to_string(),
                param_name: "lr".to_string(),
                param_value: "0.001".to_string(),
                enabled: true,
                param_style: ParamStyle::Hydra,
                use_dropdown: false,
                dropdown_options: vec![],
            }],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py lr=0.001");
    }

    #[test]
    fn test_generate_command_positional() {
        let mut form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "train.py {params}".to_string(),
            items: vec![FormItem {
                id: Uuid::new_v4().to_string(),
                param_name: "".to_string(),
                param_value: "0.001".to_string(),
                enabled: true,
                param_style: ParamStyle::Positional,
                use_dropdown: false,
                dropdown_options: vec![],
            }],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py 0.001");
    }

    #[test]
    fn test_generate_command_mixed() {
        let form = create_test_form();
        let command = CommandService::generate_command(&form).unwrap();

        // 应该生成: python train.py --lr 0.001 batch_size=32 100
        assert!(command.contains("python train.py"));
        assert!(command.contains("--lr 0.001"));
        assert!(command.contains("batch_size=32"));
        assert!(command.contains("100"));
    }

    #[test]
    fn test_disabled_item_excluded() {
        let mut form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "train.py {params}".to_string(),
            items: vec![FormItem {
                id: Uuid::new_v4().to_string(),
                param_name: "lr".to_string(),
                param_value: "0.001".to_string(),
                enabled: false, // 禁用
                param_style: ParamStyle::Argparse,
                use_dropdown: false,
                dropdown_options: vec![],
            }],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py "); // 只有命令模板，没有参数
    }

    #[test]
    fn test_empty_value_excluded() {
        let mut form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_template: "train.py {params}".to_string(),
            items: vec![FormItem {
                id: Uuid::new_v4().to_string(),
                param_name: "lr".to_string(),
                param_value: "".to_string(), // 空值
                enabled: true,
                param_style: ParamStyle::Argparse,
                use_dropdown: false,
                dropdown_options: vec![],
            }],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py "); // 空值被排除
    }
}
