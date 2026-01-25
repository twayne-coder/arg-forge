// TypeScript 类型定义
// 手动维护与 Rust models/project.rs 对应的类型

/**
 * 表单项类型枚举
 */
export type ItemType = "Command" | "Parameter";

/**
 * 参数风格枚举
 */
export type ParamStyle = "KeyValue" | "EqualValue" | "ValueOnly";

/**
 * 命令格式枚举
 */
export type CommandFormat = "SingleLine" | "MultiLine";

/**
 * 表单项字段值类型
 * 用于 updateFormItem 函数的类型安全
 */
export type FormItemFieldValue =
  | string        // content, param_name
  | boolean       // enabled, use_dropdown
  | ItemType      // item_type
  | ParamStyle    // param_style
  | string[];     // dropdown_options

/**
 * 表单项（参数项）
 */
export interface FormItem {
  /** 唯一标识符（UUID） */
  id: string;
  /** 表单项类型 */
  item_type: ItemType;
  /** 统一内容字段（命令内容或参数值） */
  content: string;
  /** 参数名（仅 Parameter 类型使用） */
  param_name: string;
  /** 是否启用 */
  enabled: boolean;
  /** 参数风格（仅 Parameter 类型使用） */
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
  /** 表单项列表 */
  items: FormItem[];
  /** 命令格式 */
  command_format: CommandFormat;
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
