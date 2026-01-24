use crate::models::{Form, FormItem};
use crate::services::{with_project_mut, StorageService};
use serde_json::Value;
use tauri::State;

/// 创建表单
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `name` - 表单名称
/// * `description` - 表单描述
/// * `storage` - 存储服务实例
///
/// # 返回
/// 创建的表单对象
///
/// # 示例
/// ```javascript
/// const form = await invoke('create_form', {
///   projectId: 'uuid-here',
///   name: '训练配置',
///   description: '模型训练参数'
/// });
/// ```
#[tauri::command]
pub async fn create_form(
    project_id: String,
    name: String,
    description: String,
    storage: State<'_, StorageService>,
) -> Result<Form, String> {
    with_project_mut(&storage, &project_id, |project| {
        // 创建新表单
        let mut form = Form::default();
        form.name = name;
        form.description = description;
        form.sort_order = project.forms.len() as i32;

        // 添加到项目
        project.add_form(form.clone());

        Ok(form)
    })
    .map_err(|e| e.to_string())
}

/// 更新表单信息
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `name` - 新的表单名称
/// * `description` - 新的表单描述
/// * `storage` - 存储服务实例
///
/// # 返回
/// 更新后的表单对象
///
/// # 示例
/// ```javascript
/// const updated = await invoke('update_form', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   name: '新名称',
///   description: '新描述'
/// });
/// ```
#[tauri::command]
pub async fn update_form(
    project_id: String,
    form_id: String,
    name: String,
    description: String,
    storage: State<'_, StorageService>,
) -> Result<Form, String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单并更新
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 更新字段
        form.name = name;
        form.description = description;
        form.touch();

        Ok(form.clone())
    })
    .map_err(|e| e.to_string())
}

/// 删除表单
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `storage` - 存储服务实例
///
/// # 示例
/// ```javascript
/// await invoke('delete_form', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid'
/// });
/// ```
#[tauri::command]
pub async fn delete_form(
    project_id: String,
    form_id: String,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    with_project_mut(&storage, &project_id, |project| {
        // 删除表单
        project
            .remove_form(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;
        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// 更新表单项
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `item_id` - 表单项 ID
/// * `field_name` - 要更新的字段名
/// * `value` - 新值（JSON 格式）
/// * `storage` - 存储服务实例
///
/// # 支持的字段
/// - `item_type`: 表单项类型（字符串："Command", "Parameter"）
/// - `content`: 内容（字符串）
/// - `param_name`: 参数名（字符串，仅 Parameter 类型）
/// - `enabled`: 是否启用（布尔）
/// - `param_style`: 参数风格（字符串："KeyValue", "EqualValue", "ValueOnly"）
/// - `use_dropdown`: 是否使用下拉（布尔）
///
/// # 返回
/// 更新后的表单对象
///
/// # 示例
/// ```javascript
/// // 更新内容
/// const form = await invoke('update_form_item', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemId: 'item-uuid',
///   fieldName: 'content',
///   value: '0.001'
/// });
/// ```
#[tauri::command]
pub async fn update_form_item(
    project_id: String,
    form_id: String,
    item_id: String,
    field_name: String,
    value: Value,
    storage: State<'_, StorageService>,
) -> Result<Form, String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 查找表单项
        let item = form
            .get_item_mut(&item_id)
            .ok_or_else(|| crate::error::AppError::FormItemNotFound(item_id.clone()))?;

        // 根据字段名更新对应字段
        match field_name.as_str() {
            "item_type" => {
                let type_str = value.as_str().ok_or_else(|| {
                    crate::error::AppError::InvalidArgument("item_type 必须是字符串".to_string())
                })?;
                item.item_type = type_str.parse()?;
            }
            "content" => {
                item.content = value
                    .as_str()
                    .ok_or_else(|| {
                        crate::error::AppError::InvalidArgument("content 必须是字符串".to_string())
                    })?
                    .to_string();
            }
            "param_name" => {
                item.param_name = value
                    .as_str()
                    .ok_or_else(|| {
                        crate::error::AppError::InvalidArgument(
                            "param_name 必须是字符串".to_string(),
                        )
                    })?
                    .to_string();
            }
            "enabled" => {
                item.enabled = value.as_bool().ok_or_else(|| {
                    crate::error::AppError::InvalidArgument("enabled 必须是布尔值".to_string())
                })?;
            }
            "param_style" => {
                let style_str = value.as_str().ok_or_else(|| {
                    crate::error::AppError::InvalidArgument("param_style 必须是字符串".to_string())
                })?;
                item.param_style = style_str.parse()?;
            }
            "use_dropdown" => {
                item.use_dropdown = value.as_bool().ok_or_else(|| {
                    crate::error::AppError::InvalidArgument("use_dropdown 必须是布尔值".to_string())
                })?;
            }
            _ => {
                return Err(crate::error::AppError::InvalidArgument(format!(
                    "不支持的字段: {}",
                    field_name
                )));
            }
        }

        // 更新表单时间戳
        form.touch();

        Ok(form.clone())
    })
    .map_err(|e| e.to_string())
}

/// 添加新的表单项
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `item_type` - 表单项类型（"Command" 或 "Parameter"，默认为 "Parameter"）
/// * `storage` - 存储服务实例
///
/// # 返回
/// 新创建的表单项
///
/// # 示例
/// ```javascript
/// // 添加参数项
/// const newItem = await invoke('add_form_item', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemType: 'Parameter'
/// });
///
/// // 添加命令项
/// const cmdItem = await invoke('add_form_item', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemType: 'Command'
/// });
/// ```
#[tauri::command]
pub async fn add_form_item(
    project_id: String,
    form_id: String,
    item_type: Option<String>,
    storage: State<'_, StorageService>,
) -> Result<FormItem, String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 创建新表单项
        let mut item = FormItem::default();

        // 根据参数设置类型
        if let Some(type_str) = item_type {
            item.item_type = type_str.parse()?;
        }

        form.items.push(item.clone());
        form.touch();

        Ok(item)
    })
    .map_err(|e| e.to_string())
}

/// 删除表单项
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `item_id` - 表单项 ID
/// * `storage` - 存储服务实例
///
/// # 示例
/// ```javascript
/// await invoke('delete_form_item', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemId: 'item-uuid'
/// });
/// ```
#[tauri::command]
pub async fn delete_form_item(
    project_id: String,
    form_id: String,
    item_id: String,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 删除表单项
        form.remove_item(&item_id)
            .ok_or_else(|| crate::error::AppError::FormItemNotFound(item_id.clone()))?;

        // 更新表单时间戳
        form.touch();

        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// 重新排序表单项（拖拽排序）
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `old_index` - 原始索引
/// * `new_index` - 新索引
/// * `storage` - 存储服务实例
///
/// # 工作流程
/// 1. 移除旧位置的元素
/// 2. 在新位置插入元素
/// 3. 保存到文件
///
/// # 示例
/// ```javascript
/// await invoke('reorder_form_items', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   oldIndex: 0,
///   newIndex: 2
/// });
/// ```
#[tauri::command]
pub async fn reorder_form_items(
    project_id: String,
    form_id: String,
    old_index: usize,
    new_index: usize,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 检查索引有效性
        if old_index >= form.items.len() || new_index >= form.items.len() {
            return Err(crate::error::AppError::InvalidArgument(
                "索引超出范围".to_string(),
            ));
        }

        // 移动元素
        let item = form.items.remove(old_index);
        form.items.insert(new_index, item);

        // 更新表单时间戳
        form.touch();

        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// 更新下拉选项
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `item_id` - 表单项 ID
/// * `options` - 新的选项列表
/// * `storage` - 存储服务实例
///
/// # 示例
/// ```javascript
/// await invoke('update_dropdown_options', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemId: 'item-uuid',
///   options: ['0.001', '0.0001', '0.01']
/// });
/// ```
#[tauri::command]
pub async fn update_dropdown_options(
    project_id: String,
    form_id: String,
    item_id: String,
    options: Vec<String>,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 查找表单项
        let item = form
            .get_item_mut(&item_id)
            .ok_or_else(|| crate::error::AppError::FormItemNotFound(item_id.clone()))?;

        // 更新下拉选项
        item.dropdown_options = options;

        // 更新表单时间戳
        form.touch();

        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// 切换下拉/手动模式
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `item_id` - 表单项 ID
/// * `use_dropdown` - 是否使用下拉模式
/// * `storage` - 存储服务实例
///
/// # 示例
/// ```javascript
/// await invoke('toggle_dropdown_mode', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid',
///   itemId: 'item-uuid',
///   useDropdown: true
/// });
/// ```
#[tauri::command]
pub async fn toggle_dropdown_mode(
    project_id: String,
    form_id: String,
    item_id: String,
    use_dropdown: bool,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    with_project_mut(&storage, &project_id, |project| {
        // 查找表单
        let form = project
            .get_form_mut(&form_id)
            .ok_or_else(|| crate::error::AppError::FormNotFound(form_id.clone()))?;

        // 查找表单项
        let item = form
            .get_item_mut(&item_id)
            .ok_or_else(|| crate::error::AppError::FormItemNotFound(item_id.clone()))?;

        // 切换模式
        item.use_dropdown = use_dropdown;

        // 如果切换到下拉模式且没有选项,添加默认选项
        if use_dropdown && item.dropdown_options.is_empty() {
            item.dropdown_options = vec![item.content.clone()];
        }

        // 更新表单时间戳
        form.touch();

        Ok(())
    })
    .map_err(|e| e.to_string())
}
