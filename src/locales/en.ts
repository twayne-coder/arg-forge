export default {
	// 通用
	common: {
		cancel: "Cancel",
		confirm: "Confirm",
		delete: "Delete",
		edit: "Edit",
		duplicate: "Duplicate",
		save: "Save",
		create: "Create",
		loading: "Loading...",
		noData: "No data",
		back: "Back",
	},

	// 语言切换
	language: {
		switchToEn: "Switch to English",
		switchToZh: "Switch to Chinese",
		zh: "中",
		en: "En",
	},

	// 项目
	project: {
		title: "ArgForge",
		subtitle: "Visual Command Configuration Tool",
		create: "New Project",
		createDescription: "Create a new command configuration project",
		name: "Project Name",
		namePlaceholder: "e.g., Model Training Config",
		description: "Project Description",
		descriptionPlaceholder: "Briefly describe the purpose of this project...",
		noDescription: "No description",
		edit: "Edit Project",
		editDescription: "Modify project information or manage project",
		duplicateSuccess: "Project duplicated",
		duplicateFailed: "Failed to duplicate project",
		deleteFailed: "Failed to delete project",
		deleteConfirm: "Confirm Delete",
		deleteConfirmMessage: 'Are you sure to delete project "{name}"? This action cannot be undone.',
		deleteConfirmButton: "Confirm Delete",
		formCount: "{count} forms",
		noProjects: "No Projects",
		noProjectsHint: "Create your first command configuration project",
		sortCreatedAsc: "Created: Oldest First",
		sortCreatedDesc: "Created: Newest First",
		sortUpdatedAsc: "Updated: Oldest First",
		sortUpdatedDesc: "Updated: Newest First",
		createdAt: "Created",
		saving: "Saving...",
		creating: "Creating...",
		deleting: "Deleting...",
	},

	// 表单
	form: {
		title: "Form",
		create: "New Form",
		createDescription: "Create a new command configuration form for the current project",
		name: "Form Name",
		namePlaceholder: "e.g., Training Parameters",
		description: "Form Description",
		descriptionPlaceholder: "Briefly describe the purpose of this form...",
		edit: "Edit Form",
		editDescription: "Modify basic form information",
		list: "Form List",
		noForm: "No forms",
		selectHint: "Select a form to view details",
		itemCount: "{count} items",
		updatedAt: "Updated",
	},

	// 表单项
	formItem: {
		title: "Form Items",
		addItem: "Add Item",
		addFirst: "Add First Item",
		addCommand: "Add Command",
		addParameter: "Add Parameter",
		noItems: "No form items",
		type: "Type",
		command: "Command",
		parameter: "Parameter",
		paramStyle: "Style",
		paramName: "Param Name",
		paramNamePlaceholder: "Parameter name",
		paramValue: "Value",
		paramValuePlaceholder: "Parameter value",
		selectValue: "Select value",
		contentPlaceholder: "Command content (e.g., python train.py)",
		manageDropdown: "Manage dropdown options",
		toggleDropdown: "Toggle dropdown mode",
		delete: "Delete",
		optionalValues: "Options: {values}",
		emptyCommand: "(empty)",
		unnamed: "(unnamed)",
	},

	// 下拉选项
	dropdown: {
		title: "Manage Dropdown Options",
		description: 'Add or remove preset options for parameter "{name}"',
		addOption: "Add Option",
		quickFill: "Quick Fill:",
		boolean: "Boolean",
		learningRate: "Learning Rate",
		optimizer: "Optimizer",
	},

	// 参数风格
	paramStyle: {
		keyValue: "--key value",
		equalValue: "key=value",
		valueOnly: "value only",
	},

	// 命令预览
	command: {
		title: "Command Preview",
		copy: "Copy",
		copySuccess: "Command copied",
		copyFailed: "Copy failed",
		emptyHint: "Command will be generated after adding parameters",
		singleLine: "Single",
		multiLine: "Multi",
	},

	// 窗口控制
	window: {
		minimize: "Minimize",
		maximize: "Maximize",
		restore: "Restore",
		close: "Close",
	},

	// 主题
	theme: {
		switchToLight: "Switch to Light Mode",
		switchToDark: "Switch to Dark Mode",
	},

	// 时间
	time: {
		today: "Today",
		yesterday: "Yesterday",
		daysAgo: "{days} days ago",
		weeksAgo: "{weeks} weeks ago",
		monthsAgo: "{months} months ago",
	},
};
