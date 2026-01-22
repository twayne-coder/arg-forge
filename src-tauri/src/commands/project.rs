use crate::models::Project;
use crate::services::StorageService;
use tauri::State;

/// 创建新项目
///
/// # 参数
/// * `name` - 项目名称
/// * `description` - 项目描述
/// * `storage` - 存储服务实例（由 Tauri 管理）
///
/// # 返回
/// 创建的项目对象
///
/// # 示例
/// ```javascript
/// const project = await invoke('create_project', {
///   name: '机器学习实验',
///   description: '深度学习模型训练配置'
/// });
/// ```
#[tauri::command]
pub async fn create_project(
    name: String,
    description: String,
    storage: State<'_, StorageService>,
) -> Result<Project, String> {
    // 创建新项目实例
    let project = Project::new(name, description);

    // 保存到文件
    storage
        .save_project(&project)
        .map_err(|e| e.to_string())?;

    Ok(project)
}

/// 获取所有项目列表
///
/// # 参数
/// * `storage` - 存储服务实例
///
/// # 返回
/// 所有项目的列表
///
/// # 示例
/// ```javascript
/// const projects = await invoke('list_projects');
/// console.log(projects); // [{ id, name, description, ... }, ...]
/// ```
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
/// 2. 生成新的 UUID
/// 3. 在名称后添加 "(副本)"
/// 4. 保存为新项目
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

    // 创建副本
    let mut copy = Project::new(
        format!("{} (副本)", original.name),
        original.description.clone(),
    );

    // 复制表单（会生成新的 UUID）
    copy.forms = original.forms;

    // 保存副本
    storage
        .save_project(&copy)
        .map_err(|e| e.to_string())?;

    Ok(copy)
}
