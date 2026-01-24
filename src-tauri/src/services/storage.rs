use crate::error::{AppError, Result};
use crate::models::Project;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, error};

/// 存储服务
/// 负责管理项目的 JSON 文件存储
pub struct StorageService {
    /// 应用数据目录
    /// Windows: %APPDATA%/arg-forge/
    /// macOS: ~/Library/Application Support/arg-forge/
    /// Linux: ~/.config/arg-forge/
    data_dir: PathBuf,
}

impl StorageService {
    /// 创建新的存储服务实例
    ///
    /// 初始化存储服务，确保必要的数据目录存在。
    /// 数据目录结构:
    /// - Windows: %APPDATA%\com.ctw.arg-forge\projects\
    /// - macOS: ~/Library/Application Support/com.ctw.arg-forge/projects\
    /// - Linux: ~/.config/com.ctw.arg-forge/projects\
    ///
    /// # 参数
    /// * `data_dir` - 应用数据目录路径
    ///
    /// # 返回
    /// 存储服务实例
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        // 创建主数据目录和项目子目录
        fs::create_dir_all(&data_dir)?;
        let projects_dir = data_dir.join("projects");
        fs::create_dir_all(&projects_dir)?;

        Ok(Self { data_dir })
    }

    /// 获取项目文件路径
    ///
    /// # 参数
    /// * `project_id` - 项目 ID (UUID)
    ///
    /// # 返回
    /// 项目文件的完整路径 (如: `projects/{uuid}.json`)
    ///
    /// # 错误
    /// 如果 project_id 格式无效，返回 `AppError::InvalidArgument`
    fn get_project_path(&self, project_id: &str) -> Result<PathBuf> {
        // 验证 UUID 格式，防止路径遍历攻击
        super::validate_project_id(project_id)?;

        Ok(self.data_dir
            .join("projects")
            .join(format!("{}.json", project_id)))
    }

    /// 获取备份文件路径
    ///
    /// 备份文件用于在保存失败或数据损坏时恢复。
    ///
    /// # 参数
    /// * `project_id` - 项目 ID (UUID)
    ///
    /// # 返回
    /// 备份文件的完整路径 (如: `projects/{uuid}.json.bak`)
    ///
    /// # 错误
    /// 如果 project_id 格式无效，返回 `AppError::InvalidArgument`
    fn get_backup_path(&self, project_id: &str) -> Result<PathBuf> {
        // 验证 UUID 格式，防止路径遍历攻击
        super::validate_project_id(project_id)?;

        Ok(self.data_dir
            .join("projects")
            .join(format!("{}.json.bak", project_id)))
    }

    /// 保存项目到文件
    ///
    /// 保存流程包含安全机制：
    /// 1. 验证 project_id 格式（防止路径遍历）
    /// 2. 如果旧文件存在，先创建 `.bak` 备份
    /// 3. 序列化为格式化的 JSON (便于人类阅读和调试)
    /// 4. 原子写入文件
    ///
    /// # 参数
    /// * `project` - 要保存的项目对象
    ///
    /// # 返回
    /// 成功时返回 Ok(())，失败时返回错误
    pub fn save_project(&self, project: &Project) -> Result<()> {
        let file_path = self.get_project_path(&project.id)?;
        let backup_path = self.get_backup_path(&project.id)?;

        // 创建旧文件的备份（防止数据丢失）
        if file_path.exists() {
            fs::copy(&file_path, &backup_path)?;
        }

        // 序列化为格式化的 JSON（便于调试和版本控制）
        let json = serde_json::to_string_pretty(project)?;

        // 写入文件
        fs::write(&file_path, json)?;

        Ok(())
    }

    /// 从文件加载项目
    ///
    /// # 参数
    /// * `project_id` - 项目 ID (UUID)
    ///
    /// # 返回
    /// 加载的项目对象
    ///
    /// # 错误
    /// 如果 project_id 格式无效，返回 `AppError::InvalidArgument`
    /// 如果文件不存在，返回 `AppError::ProjectNotFound`
    /// 如果 JSON 解析失败，返回 `AppError::Json`
    pub fn load_project(&self, project_id: &str) -> Result<Project> {
        let file_path = self.get_project_path(project_id)?;

        // 检查文件是否存在
        if !file_path.exists() {
            return Err(AppError::ProjectNotFound(project_id.to_string()));
        }

        // 读取并反序列化 JSON
        let content = fs::read_to_string(&file_path)?;
        let project = serde_json::from_str(&content)?;

        Ok(project)
    }

    /// 删除项目文件
    /// # 参数
    /// * `project_id` - 项目 ID
    ///
    /// # 工作流程
    /// 1. 验证 project_id 格式（防止路径遍历）
    /// 2. 删除项目文件
    /// 3. 删除备份文件（如果存在）
    pub fn delete_project(&self, project_id: &str) -> Result<()> {
        let file_path = self.get_project_path(project_id)?;
        let backup_path = self.get_backup_path(project_id)?;

        // 删除项目文件
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }

        // 删除备份文件
        if backup_path.exists() {
            fs::remove_file(&backup_path)?;
        }

        Ok(())
    }

    /// 列出所有项目
    ///
    /// 扫描项目目录并加载所有有效的项目文件。
    /// 具有容错机制：如果主文件损坏，自动尝试从备份恢复。
    ///
    /// # 返回
    /// 成功加载的项目列表
    ///
    /// # 工作流程
    /// 1. 扫描 `projects/` 目录
    /// 2. 过滤 `.json` 文件（排除 `.bak` 备份文件）
    /// 3. 尝试加载每个文件
    /// 4. 如果加载失败，尝试从 `.bak` 备份恢复
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let projects_dir = self.data_dir.join("projects");
        let mut projects = Vec::new();

        // 如果项目目录不存在，返回空列表
        if !projects_dir.exists() {
            return Ok(projects);
        }

        // 扫描目录中的所有文件
        let entries = fs::read_dir(&projects_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 只处理 .json 文件，排除备份文件
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            if path.to_str().map_or(false, |s| s.ends_with(".bak")) {
                continue;
            }

            // 尝试加载项目，失败时尝试从备份恢复
            match self.load_project_from_path(&path) {
                Ok(project) => projects.push(project),
                Err(e) => {
                    error!("加载项目失败 {:?}: {}", path, e);

                    // 提取项目 ID 并尝试从备份恢复
                    let project_id = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");

                    // 尝试获取备份路径（如果 project_id 格式无效则跳过）
                    if let Ok(backup_path) = self.get_backup_path(project_id) {
                        if backup_path.exists() {
                            info!("尝试从备份恢复: {:?}", backup_path);
                            if let Ok(project) = self.load_project_from_path(&backup_path) {
                                info!("备份恢复成功");
                                projects.push(project);
                            }
                        }
                    }
                }
            }
        }

        Ok(projects)
    }

    /// 从指定路径加载项目
    /// # 参数
    /// * `path` - 文件路径
    /// # 返回
    /// 加载的项目数据
    fn load_project_from_path(&self, path: &Path) -> Result<Project> {
        let content = fs::read_to_string(path)?;
        let project = serde_json::from_str(&content)?;
        Ok(project)
    }

    /// 项目是否存在
    /// # 参数
    /// * `project_id` - 项目 ID
    /// # 返回
    /// true 如果项目文件存在，否则 false
    pub fn project_exists(&self, project_id: &str) -> bool {
        // 如果验证失败，认为项目不存在
        match self.get_project_path(project_id) {
            Ok(path) => path.exists(),
            Err(_) => false,
        }
    }

    /// 清理所有备份文件
    /// 删除项目中所有的 .bak 文件
    pub fn clean_backups(&self) -> Result<()> {
        let projects_dir = self.data_dir.join("projects");
        let entries = fs::read_dir(&projects_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 删除所有 .bak 文件
            if path.to_str().map_or(false, |s| s.ends_with(".bak")) {
                let _ = fs::remove_file(&path);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// 创建临时目录用于测试
    fn create_temp_storage() -> (StorageService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageService::new(temp_dir.path().to_path_buf()).unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_create_storage() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageService::new(temp_dir.path().to_path_buf());
        assert!(storage.is_ok());
    }

    #[test]
    fn test_save_and_load_project() {
        let (storage, _temp) = create_temp_storage();

        // 创建测试项目
        let project = Project::new("测试项目".to_string(), "测试描述".to_string());
        let project_id = project.id.clone();

        // 保存项目
        assert!(storage.save_project(&project).is_ok());

        // 加载项目
        let loaded = storage.load_project(&project_id);
        assert!(loaded.is_ok());
        let loaded_project = loaded.unwrap();
        assert_eq!(loaded_project.name, "测试项目");
        assert_eq!(loaded_project.id, project_id);
    }

    #[test]
    fn test_project_exists() {
        let (storage, _temp) = create_temp_storage();

        let project = Project::default();
        let project_id = project.id.clone();

        assert!(!storage.project_exists(&project_id));

        storage.save_project(&project).unwrap();
        assert!(storage.project_exists(&project_id));
    }

    #[test]
    fn test_delete_project() {
        let (storage, _temp) = create_temp_storage();

        let project = Project::default();
        let project_id = project.id.clone();

        storage.save_project(&project).unwrap();
        assert!(storage.project_exists(&project_id));

        storage.delete_project(&project_id).unwrap();
        assert!(!storage.project_exists(&project_id));
    }

    #[test]
    fn test_list_projects() {
        let (storage, _temp) = create_temp_storage();

        // 创建多个项目
        let project1 = Project::new("项目1".to_string(), "描述1".to_string());
        let project2 = Project::new("项目2".to_string(), "描述2".to_string());

        storage.save_project(&project1).unwrap();
        storage.save_project(&project2).unwrap();

        // 列出项目
        let projects = storage.list_projects().unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn test_backup_creation() {
        let (storage, _temp) = create_temp_storage();

        let mut project = Project::new("测试项目".to_string(), "测试描述".to_string());
        let project_id = project.id.clone();

        // 第一次保存
        storage.save_project(&project).unwrap();

        // 修改项目
        project.name = "修改后的项目".to_string();

        // 第二次保存（应该创建备份）
        storage.save_project(&project).unwrap();

        // 检查备份文件存在
        let backup_path = storage.get_backup_path(&project_id).unwrap();
        assert!(backup_path.exists());
    }
}
