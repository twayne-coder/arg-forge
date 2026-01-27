export default {
	// 通用
	common: {
		cancel: "取消",
		confirm: "确认",
		delete: "删除",
		edit: "编辑",
		duplicate: "复制",
		save: "保存",
		create: "创建",
		loading: "加载中...",
		noData: "暂无数据",
		back: "返回",
	},

	// 语言切换
	language: {
		switchToEn: "切换为英文",
		switchToZh: "Switch to Chinese",
		zh: "中",
		en: "En",
	},

	// 项目
	project: {
		title: "ArgForge",
		subtitle: "可视化命令配置工具",
		create: "新建项目",
		createDescription: "创建一个新的命令配置项目",
		name: "项目名称",
		namePlaceholder: "例如：模型训练配置",
		description: "项目描述",
		descriptionPlaceholder: "简要描述此项目的用途...",
		noDescription: "暂无描述",
		edit: "编辑项目",
		editDescription: "修改项目信息或管理项目",
		duplicateSuccess: "项目已复制",
		duplicateFailed: "复制项目失败",
		deleteSuccess: "项目已删除",
		deleteFailed: "删除项目失败",
		deleteConfirm: "确认删除",
		deleteConfirmMessage: '确定要删除项目 "{name}" 吗？此操作无法撤销。',
		deleteConfirmButton: "确认删除",
		formCount: "{count} 个表单",
		noProjects: "暂无项目",
		noProjectsHint: "创建您的第一个命令配置项目",
		sortCreatedAsc: "创建时间",
		sortCreatedDesc: "创建时间",
		sortUpdatedAsc: "更新时间",
		sortUpdatedDesc: "更新时间",
		createdAt: "创建于",
		saving: "保存中...",
		creating: "创建中...",
		deleting: "删除中...",
	},

	// 表单
	form: {
		title: "表单",
		create: "新建表单",
		createDescription: "为当前项目创建一个新的命令配置表单",
		name: "表单名称",
		namePlaceholder: "例如：训练参数配置",
		description: "表单描述",
		descriptionPlaceholder: "简要描述此表单的用途...",
		edit: "编辑表单",
		editDescription: "修改表单的基本信息",
		list: "表单列表",
		noForm: "暂无表单",
		selectHint: "选择一个表单查看详情",
		itemCount: "{count} 项",
		updatedAt: "更新于",
		duplicateSuccess: "表单已复制",
		duplicateFailed: "复制表单失败",
		noCurrentForm: "没有当前表单",
		deleteSuccess: "表单已删除",
		deleteFailed: "删除表单失败",
		deleteConfirm: "确认删除",
		deleteConfirmMessage: '确定要删除表单 "{name}" 吗？此操作无法撤销。',
		deleteConfirmButton: "确认删除",
		deleting: "删除中...",
	},

	// 表单项
	formItem: {
		title: "表单项",
		addItem: "添加项",
		addFirst: "添加第一项",
		addCommand: "添加命令",
		addParameter: "添加参数",
		noItems: "暂无表单项",
		type: "类型",
		command: "命令",
		parameter: "参数",
		paramStyle: "风格",
		paramName: "参数名",
		paramNamePlaceholder: "参数名",
		paramValue: "参数值",
		paramValuePlaceholder: "参数值",
		selectValue: "选择值",
		contentPlaceholder: "命令内容（如 python train.py）",
		manageDropdown: "管理下拉选项",
		toggleDropdown: "切换下拉模式",
		delete: "删除",
		optionalValues: "可选值: {values}",
		emptyCommand: "(空命令)",
		unnamed: "(未命名)",
		cmd: "命令",
		param: "参数",
	},

	// 下拉选项
	dropdown: {
		title: "管理下拉选项",
		description: '为参数 "{name}" 添加或删除预设选项',
		addOption: "添加选项",
		quickFill: "快捷填充:",
		boolean: "布尔值",
		learningRate: "学习率",
		optimizer: "优化器",
	},

	// 参数风格
	paramStyle: {
		keyValue: "--key value",
		equalValue: "key=value",
		valueOnly: "仅值",
	},

	// 命令预览
	command: {
		title: "命令预览",
		copy: "复制",
		copySuccess: "命令已复制",
		copyFailed: "复制失败",
		emptyHint: "添加参数后自动生成命令",
		singleLine: "单行",
		multiLine: "多行",
	},

	// 窗口控制
	window: {
		minimize: "最小化",
		maximize: "最大化",
		restore: "向下还原",
		close: "关闭",
	},

	// 主题
	theme: {
		switchToLight: "切换到浅色模式",
		switchToDark: "切换到深色模式",
	},

	// 时间
	time: {
		today: "今天",
		yesterday: "昨天",
		daysAgo: "{days} 天前",
		weeksAgo: "{weeks} 周前",
		monthsAgo: "{months} 月前",
	},
};
