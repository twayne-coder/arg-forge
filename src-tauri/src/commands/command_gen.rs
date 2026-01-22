use crate::services::CommandService;
use tauri::State;

/// 生成命令
///
/// # 参数
/// * `project_id` - 项目 ID
/// * `form_id` - 表单 ID
/// * `storage` - 存储服务实例
///
/// # 返回
/// 生成的完整命令字符串
///
/// # 工作流程
/// 1. 加载项目
/// 2. 查找指定表单
/// 3. 调用 CommandService 生成命令
/// 4. 返回命令字符串
///
/// # 示例
/// ```javascript
/// const command = await invoke('generate_command', {
///   projectId: 'project-uuid',
///   formId: 'form-uuid'
/// });
/// console.log(command); // "python train.py --lr 0.001 batch_size=32"
/// ```
#[tauri::command]
pub async fn generate_command(
    project_id: String,
    form_id: String,
    storage: State<'_, crate::StorageService>,
) -> Result<String, String> {
    // 加载项目
    let project = storage
        .load_project(&project_id)
        .map_err(|e: crate::error::AppError| e.to_string())?;

    // 查找表单
    let form = project
        .forms
        .iter()
        .find(|f| f.id == form_id)
        .ok_or_else(|| format!("表单未找到: {}", form_id))?;

    // 生成命令
    CommandService::generate_command(form).map_err(|e| e.to_string())
}
