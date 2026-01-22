use crate::error::{AppError, Result};
use crate::models::Project;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

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
    /// # 参数
    /// * `data_dir` - 应用数据目录路径
    /// # 返回
    /// 存储服务实例
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        // 确保数据目录存在
        fs::create_dir_all(&data_dir)?;

        // 确保项目子目录存在
        let projects_dir = data_dir.join("projects");
        fs::create_dir_all(&projects_dir)?;

        Ok(Self { data_dir })
    }

    /// 获取项目文件路径
    /// # 参数
    /// * `project_id` - 项目 ID
    /// # 返回
    /// 项目文件的完整路径
    fn get_project_path(&self, project_id: &str) -> PathBuf {
        self.data_dir
            .join("projects")
            .join(format!("{}.json", project_id))
    }

    /// 获取备份文件路径
    /// # 参数
    /// * `project_id` - 项目 ID
    /// # 返回
    /// 备份文件的完整路径
    fn get_backup_path(&self, project_id: &str) -> PathBuf {
        self.data_dir
            .join("projects")
            .join(format!("{}.json.bak", project_id))
    }

    /// 保存项目到文件
    /// # 参数
    /// * `project` - 要保存的项目
    ///
    /// # 工作流程
    /// 1. 如果已存在旧文件，先创建备份
    /// 2. 将项目序列化为 JSON
    /// 3. 写入文件
    pub fn save_project(&self, project: &Project) -> Result<()> {
        let file_path = self.get_project_path(&project.id);
        let backup_path = self.get_backup_path(&project.id);

        // 如果文件已存在，创建备份
        if file_path.exists() {
            fs::copy(&file_path, &backup_path)?;
        }

        // 序列化为格式化的 JSON
        let json = serde_json::to_string_pretty(project)?;

        // 写入文件
        fs::write(&file_path, json)?;

        Ok(())
    }

    /// 从文件加载项目
    /// # 参数
    /// * `project_id` - 项目 ID
    /// # 返回
    /// 加载的项目数据
    ///
    /// # 错误
    /// 如果文件不存在或解析失败，返回错误
    pub fn load_project(&self, project_id: &str) -> Result<Project> {
        let file_path = self.get_project_path(project_id);

        // 检查文件是否存在
        if !file_path.exists() {
            return Err(AppError::ProjectNotFound(project_id.to_string()));
        }

        // 读取文件内容
        let content = fs::read_to_string(&file_path)?;

        // 反序列化
        let project = serde_json::from_str(&content)?;

        Ok(project)
    }

    /// 删除项目文件
    /// # 参数
    /// * `project_id` - 项目 ID
    ///
    /// # 工作流程
    /// 1. 删除项目文件
    /// 2. 删除备份文件（如果存在）
    pub fn delete_project(&self, project_id: &str) -> Result<()> {
        let file_path = self.get_project_path(project_id);
        let backup_path = self.get_backup_path(project_id);

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
    /// # 返回
    /// 所有加载的项目列表
    ///
    /// # 工作流程
    /// 1. 扫描项目目录
    /// 2. 过滤出 .json 文件
    /// 3. 加载每个文件
    /// 4. 如果某个文件加载失败，尝试恢复备份
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let projects_dir = self.data_dir.join("projects");
        let mut projects = Vec::new();

        // 确保目录存在
        if !projects_dir.exists() {
            return Ok(projects);
        }

        // 读取目录中的所有条目
        let entries = fs::read_dir(&projects_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 只处理 .json 文件（排除备份文件）
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            // 跳过备份文件
            if path.to_str().map_or(false, |s| s.ends_with(".bak")) {
                continue;
            }

            // 尝试加载项目
            match self.load_project_from_path(&path) {
                Ok(project) => projects.push(project),
                Err(e) => {
                    eprintln!("加载项目失败 {:?}: {}", path, e);

                    // 尝试从备份恢复
                    let project_id = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    let backup_path = self.get_backup_path(project_id);

                    if backup_path.exists() {
                        eprintln!("尝试从备份恢复: {:?}", backup_path);
                        if let Ok(project) = self.load_project_from_path(&backup_path) {
                            eprintln!("备份恢复成功");
                            projects.push(project);
                        } else {
                            eprintln!("备份恢复也失败");
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
        self.get_project_path(project_id).exists()
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
        let backup_path = storage.get_backup_path(&project_id);
        assert!(backup_path.exists());
    }
}
