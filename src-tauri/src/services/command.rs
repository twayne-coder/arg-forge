use crate::error::Result;
use crate::models::{CommandFormat, Form, FormItem, ItemType, ParamStyle};

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
    /// 1. 过滤出启用的表单项
    /// 2. 过滤出有内容的表单项
    /// 3. 按顺序生成每个表单项的字符串表示
    /// 4. 根据 form.command_format 选择连接方式
    ///
    /// # 示例
    /// ```text
    /// 单行格式 (SingleLine):
    /// python train.py --lr 0.001 --batch-size 32
    ///
    /// 多行格式 (MultiLine):
    /// python train.py \
    /// --lr 0.001 \
    /// --batch-size 32
    /// ```
    pub fn generate_command(form: &Form) -> Result<String> {
        let mut command_parts = Vec::new();

        // 遍历所有表单项（保持顺序）
        for item in &form.items {
            // 跳过禁用的项
            if !item.enabled {
                continue;
            }

            // 跳过空内容
            if item.content.is_empty() {
                continue;
            }

            // 根据类型生成字符串
            let part = Self::format_item(item)?;
            if !part.is_empty() {
                command_parts.push(part);
            }
        }

        // 根据格式选择连接方式
        match form.command_format {
            CommandFormat::SingleLine => {
                // 单行：用空格连接
                Ok(command_parts.join(" "))
            }
            CommandFormat::MultiLine => {
                // 多行：每行后加 \ 换行（最后一行不加）
                if command_parts.is_empty() {
                    Ok(String::new())
                } else {
                    let lines: Vec<String> = command_parts
                        .iter()
                        .enumerate()
                        .map(|(i, part)| {
                            if i == command_parts.len() - 1 {
                                // 最后一行不加 \
                                part.clone()
                            } else {
                                // 其他行加 \
                                format!("{} \\", part)
                            }
                        })
                        .collect();
                    Ok(lines.join("\n"))
                }
            }
        }
    }

    /// 格式化单个表单项
    /// # 参数
    /// * `item` - 表单项
    /// # 返回
    /// 格式化后的字符串
    fn format_item(item: &FormItem) -> Result<String> {
        match item.item_type {
            // 命令项：直接返回内容
            ItemType::Command => Ok(item.content.clone()),

            // 参数项：根据参数风格格式化
            ItemType::Parameter => match item.param_style {
                // KeyValue 风格：--key value
                // 示例：--lr 0.001
                ParamStyle::KeyValue => {
                    if item.param_name.is_empty() {
                        // 如果没有参数名，只返回值
                        Ok(item.content.clone())
                    } else {
                        Ok(format!("--{} {}", item.param_name, item.content))
                    }
                }

                // EqualValue 风格：key=value
                // 示例：lr=0.001
                ParamStyle::EqualValue => {
                    if item.param_name.is_empty() {
                        // 如果没有参数名，只返回值
                        Ok(item.content.clone())
                    } else {
                        Ok(format!("{}={}", item.param_name, item.content))
                    }
                }

                // ValueOnly 风格：只有值
                // 示例：0.001
                ParamStyle::ValueOnly => Ok(item.content.clone()),
            },
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
    use crate::models::{FormItem, ItemType};
    use uuid::Uuid;

    fn create_test_form() -> Form {
        Form {
            id: Uuid::new_v4().to_string(),
            name: "测试表单".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "python train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: "lr".to_string(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "32".to_string(),
                    param_name: "batch_size".to_string(),
                    enabled: true,
                    param_style: ParamStyle::EqualValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "100".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::ValueOnly,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        }
    }

    #[test]
    fn test_generate_command_key_value() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: "lr".to_string(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py --lr 0.001");
    }

    #[test]
    fn test_generate_command_equal_value() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: "lr".to_string(),
                    enabled: true,
                    param_style: ParamStyle::EqualValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py lr=0.001");
    }

    #[test]
    fn test_generate_command_value_only() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::ValueOnly,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
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
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: "lr".to_string(),
                    enabled: false, // 禁用
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py"); // 只有命令，参数被排除
    }

    #[test]
    fn test_empty_content_excluded() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "".to_string(), // 空内容
                    param_name: "lr".to_string(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py"); // 空内容被排除
    }

    #[test]
    fn test_multiple_commands() {
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "conda activate env".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "&&".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "python train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "0.001".to_string(),
                    param_name: "lr".to_string(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "conda activate env && python train.py --lr 0.001");
    }

    #[test]
    fn test_parameter_without_name() {
        // 测试参数没有参数名时的行为
        let form = Form {
            id: Uuid::new_v4().to_string(),
            name: "测试".to_string(),
            description: String::new(),
            sort_order: 0,
            updated_at: chrono::Utc::now().to_rfc3339(),
            command_format: CommandFormat::SingleLine,
            items: vec![
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Command,
                    content: "train.py".to_string(),
                    param_name: String::new(),
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
                FormItem {
                    id: Uuid::new_v4().to_string(),
                    item_type: ItemType::Parameter,
                    content: "value1".to_string(),
                    param_name: "".to_string(), // 空参数名
                    enabled: true,
                    param_style: ParamStyle::KeyValue,
                    use_dropdown: false,
                    dropdown_options: vec![],
                },
            ],
        };

        let command = CommandService::generate_command(&form).unwrap();
        assert_eq!(command, "train.py value1"); // 只有值，没有参数名
    }
}
