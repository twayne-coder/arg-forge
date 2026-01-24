use crate::models::Project;
use crate::services::StorageService;
use tauri::State;
use serde::Deserialize;
use uuid::Uuid;

/// 项目创建配置
#[derive(Deserialize)]
pub struct CreateProjectConfig {
    pub name: String,
    pub description: String,
}

/// 创建新项目
///
/// 创建流程：
/// 1. 验证项目名称非空
/// 2. 创建新的 Project 实例（自动生成 UUID）
/// 3. 保存到文件系统
/// 4. 验证文件保存成功
///
/// # 参数
/// * `config` - 项目配置 (名称和描述)
/// * `storage` - 存储服务实例（由 Tauri 管理）
///
/// # 返回
/// 创建的项目对象
#[tauri::command]
pub async fn create_project(
    config: CreateProjectConfig,
    storage: State<'_, StorageService>,
) -> Result<Project, String> {
    // 验证输入：项目名称不能为空
    if config.name.trim().is_empty() {
        return Err("项目名称不能为空".to_string());
    }

    // 创建新项目实例（自动生成 UUID 和时间戳）
    let project = Project::new(config.name.clone(), config.description.clone());
    let project_id = project.id.clone();

    // 保存到文件系统
    storage
        .save_project(&project)
        .map_err(|e| e.to_string())?;

    // 验证文件保存成功
    if !storage.project_exists(&project_id) {
        return Err(format!("项目保存成功，但文件未找到: {}", project_id));
    }

    Ok(project)
}

/// 获取所有项目列表
///
/// 扫描项目目录并加载所有有效的项目文件。
///
/// # 参数
/// * `storage` - 存储服务实例
///
/// # 返回
/// 所有项目的列表
#[tauri::command]
pub async fn list_projects(
    storage: State<'_, StorageService>,
) -> Result<Vec<Project>, String> {
    storage
        .list_projects()
        .map_err(|e| e.to_string())
}

/// 获取单个项目详情
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `storage` - 存储服务实例
///
/// # 返回
/// 项目对象
///
/// # 错误
/// 如果项目不存在，返回错误
///
/// # 示例
/// ```javascript
/// const project = await invoke('get_project', {
///   projectId: 'uuid-here'
/// });
/// ```
#[tauri::command]
pub async fn get_project(
    project_id: String,
    storage: State<'_, StorageService>,
) -> Result<Project, String> {
    storage
        .load_project(&project_id)
        .map_err(|e| e.to_string())
}

/// 更新项目信息
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `name` - 新的项目名称
/// * `description` - 新的项目描述
/// * `storage` - 存储服务实例
///
/// # 返回
/// 更新后的项目对象
///
/// # 示例
/// ```javascript
/// const updated = await invoke('update_project', {
///   projectId: 'uuid-here',
///   name: '新名称',
///   description: '新描述'
/// });
/// ```
#[tauri::command]
pub async fn update_project(
    project_id: String,
    name: String,
    description: String,
    storage: State<'_, StorageService>,
) -> Result<Project, String> {
    // 加载现有项目
    let mut project = storage.load_project(&project_id).map_err(|e| e.to_string())?;

    // 更新字段
    project.name = name;
    project.description = description;
    project.touch(); // 更新时间戳

    // 保存
    storage
        .save_project(&project)
        .map_err(|e| e.to_string())?;

    Ok(project)
}

/// 删除项目
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `storage` - 存储服务实例
///
/// # 示例
/// ```javascript
/// await invoke('delete_project', { projectId: 'uuid-here' });
/// ```
#[tauri::command]
pub async fn delete_project(
    project_id: String,
    storage: State<'_, StorageService>,
) -> Result<(), String> {
    storage
        .delete_project(&project_id)
        .map_err(|e| e.to_string())
}

/// 复制项目（深拷贝）
///
/// # 参数
/// * `project_id` - 要复制的项目 ID
/// * `storage` - 存储服务实例
///
/// # 返回
/// 新创建的项目对象（新的 UUID）
///
/// # 工作流程
/// 1. 加载原项目
/// 2. 使用 JSON 序列化实现高效深拷贝
/// 3. 重新生成所有 UUID（项目、表单、表单项）
/// 4. 在名称后添加 "(副本)"
/// 5. 保存为新项目
///
/// # 示例
/// ```javascript
/// const copy = await invoke('duplicate_project', {
///   projectId: 'uuid-here'
/// });
/// console.log(copy.name); // "原项目名 (副本)"
/// ```
#[tauri::command]
pub async fn duplicate_project(
    project_id: String,
    storage: State<'_, StorageService>,
) -> Result<Project, String> {
    // 加载原项目
    let original = storage
        .load_project(&project_id)
        .map_err(|e| e.to_string())?;

    // 使用 JSON 序列化实现高效的深拷贝
    let json = serde_json::to_string(&original)
        .map_err(|e| format!("序列化项目失败: {}", e))?;
    let mut copy: Project = serde_json::from_str(&json)
        .map_err(|e| format!("反序列化项目失败: {}", e))?;

    // 重新生成顶层 UUID
    copy.id = Uuid::new_v4().to_string();
    copy.name = format!("{} (副本)", original.name);
    copy.created_at = chrono::Utc::now().to_rfc3339();
    copy.updated_at = chrono::Utc::now().to_rfc3339();

    // 重新生成所有表单和表单项的 UUID
    for form in &mut copy.forms {
        form.id = Uuid::new_v4().to_string();
        form.updated_at = chrono::Utc::now().to_rfc3339();
        for item in &mut form.items {
            item.id = Uuid::new_v4().to_string();
        }
    }

    // 保存副本
    storage
        .save_project(&copy)
        .map_err(|e| e.to_string())?;

    Ok(copy)
}
