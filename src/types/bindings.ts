// TypeScript 类型定义
// 手动维护与 Rust models/project.rs 对应的类型

/**
 * 参数风格枚举
 */
export type ParamStyle = "Argparse" | "Hydra" | "Positional";

/**
 * 表单项（参数项）
 */
export interface FormItem {
  /** 唯一标识符（UUID） */
  id: string;
  /** 参数名 */
  param_name: string;
  /** 参数值 */
  param_value: string;
  /** 是否启用 */
  enabled: boolean;
  /** 参数风格 */
  param_style: ParamStyle;
  /** 是否使用下拉选择 */
  use_dropdown: boolean;
  /** 下拉选项列表 */
  dropdown_options: string[];
}

/**
 * 表单
 */
export interface Form {
  /** 唯一标识符（UUID） */
  id: string;
  /** 表单名称 */
  name: string;
  /** 表单描述 */
  description: string;
  /** 排序顺序 */
  sort_order: number;
  /** 最后更新时间（ISO 8601） */
  updated_at: string;
  /** 命令前缀（在模板前添加） */
  command_prefix: string;
  /** 命令模板（支持 {params} 占位符） */
  command_template: string;
  /** 命令后缀（在模板后添加） */
  command_suffix: string;
  /** 参数项列表 */
  items: FormItem[];
}

/**
 * 项目
 */
export interface Project {
  /** 唯一标识符（UUID） */
  id: string;
  /** 项目名称 */
  name: string;
  /** 项目描述 */
  description: string;
  /** 创建时间（ISO 8601） */
  created_at: string;
  /** 最后更新时间（ISO 8601） */
  updated_at: string;
  /** 表单列表 */
  forms: Form[];
}
