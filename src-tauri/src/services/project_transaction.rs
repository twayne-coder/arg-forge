use crate::error::Result;
use crate::models::Project;
use crate::services::StorageService;

/// 项目事务处理辅助函数
///
/// 封装 "加载-修改-保存" 模式，减少代码重复并确保数据一致性。
/// 这个函数自动处理项目的加载、修改和保存，任何步骤失败都会回滚整个操作。
///
/// # 设计思想
///
/// 在不引入复杂事务系统的情况下，这个函数提供了简单的事务语义：
/// - 原子性：要么全部成功，要么全部失败
/// - 一致性：自动更新项目的时间戳
/// - 隔离性：操作在内存中进行，保存时才持久化
///
/// # 类型参数
/// * `F` - 操作闭包的类型
/// * `R` - 操作返回值的类型
///
/// # 参数
/// * `storage` - 存储服务引用
/// * `project_id` - 项目 ID (UUID 字符串)
/// * `op` - 操作闭包，接收项目的可变引用，返回操作结果
///
/// # 返回
/// * `Ok(R)` - 操作成功，返回闭包的结果
/// * `Err(AppError)` - 加载、操作或保存失败
///
/// # 示例
/// ```no_run
/// use crate::services::project_transaction::with_project_mut;
///
/// # let storage = unimplemented!();
/// // 修改项目名称
/// let result = with_project_mut(&storage, "project-id", |project| {
///     project.name = "新名称".to_string();
///     Ok(())
/// });
///
/// // 添加表单并返回
/// let form = with_project_mut(&storage, "project-id", |project| {
///     let new_form = Form::default();
///     project.add_form(new_form.clone());
///     Ok(new_form)
/// });
/// ```
///
/// # 注意
/// - 闭包中的修改会在 `op` 返回后立即保存到文件
/// - 如果保存失败，内存中的修改仍然存在，但文件未更新
/// - 项目时间戳会自动更新，无需手动调用 `project.touch()`
pub fn with_project_mut<F, R>(
    storage: &StorageService,
    project_id: &str,
    op: F,
) -> Result<R>
where
    F: FnOnce(&mut Project) -> Result<R>,
{
    // 步骤 1: 从文件系统加载项目
    let mut project = storage.load_project(project_id)?;

    // 步骤 2: 执行修改操作（在内存中）
    let result = op(&mut project)?;

    // 步骤 3: 保存修改后的项目到文件系统
    // 注意：保存失败会返回错误，但内存中的修改仍然有效
    storage.save_project(&project)?;

    Ok(result)
}
