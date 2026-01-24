<template>
  <div class="h-full flex flex-col">
    <!-- 头部 -->
    <div class="flex items-center justify-between px-6 py-4 border-b">
      <div>
        <h1 class="text-2xl font-semibold">ArgForge</h1>
        <p class="text-sm text-muted-foreground">可视化命令配置工具</p>
      </div>
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          @click="toggleSort"
          class="min-w-[140px]"
        >
          <component :is="sortDirectionIcon" class="h-4 w-4 mr-2" />
          {{ sortButtonText }}
        </Button>
        <Button @click="showCreateDialog = true">
          <PlusIcon class="h-4 w-4 mr-2" />
          新建项目
        </Button>
      </div>
    </div>

    <!-- 主内容区 -->
    <ScrollArea class="flex-1">
      <div class="p-6">
        <!-- 空状态 -->
        <div
          v-if="!hasProjects && !isLoading"
          class="flex flex-col items-center justify-center py-16 text-center"
        >
          <FolderOpenIcon class="h-16 w-16 text-muted-foreground/50 mb-4" />
          <h3 class="text-lg font-semibold mb-2">暂无项目</h3>
          <p class="text-sm text-muted-foreground mb-4">
            创建您的第一个命令配置项目
          </p>
          <Button @click="showCreateDialog = true">
            <PlusIcon class="h-4 w-4 mr-2" />
            新建项目
          </Button>
        </div>

        <!-- 项目网格 -->
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <ProjectCard
            v-for="project in projects"
            :key="project.id"
            :project="project"
            @click="handleProjectClick"
            @edit="handleEditProject"
            @duplicate="handleDuplicateProject"
            @delete="handleDeleteProject"
          />
        </div>
      </div>
    </ScrollArea>

    <!-- 新建项目对话框 -->
    <CreateProjectDialog
      v-model:open="showCreateDialog"
      @success="handleRefresh"
    />

    <!-- 编辑项目对话框 -->
    <EditProjectDialog
      v-if="editingProject"
      v-model:open="showEditDialog"
      :project="editingProject"
      @success="handleRefresh"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { PlusIcon, FolderOpenIcon, ArrowUpIcon, ArrowDownIcon } from "lucide-vue-next";
import { useProjectStore } from "@/stores/project";
import { useUiStore } from "@/stores/ui";
import { useProjectSort } from "@/composables/useProjectSort";
import ProjectCard from "./ProjectCard.vue";
import CreateProjectDialog from "./CreateProjectDialog.vue";
import EditProjectDialog from "./EditProjectDialog.vue";
import type { Project } from "@/types/bindings";

/** 路由 */
const router = useRouter();

/** 项目 store */
const projectStore = useProjectStore();

/** UI store */
const uiStore = useUiStore();

/** 排序配置 */
const { sortConfig } = useProjectSort();

/** 是否显示新建对话框 */
const showCreateDialog = ref(false);

/** 是否显示编辑对话框 */
const showEditDialog = ref(false);

/** 正在编辑的项目 */
const editingProject = ref<Project | null>(null);

/** 是否加载中 */
const isLoading = ref(false);

/** 项目列表 */
const projects = computed(() => projectStore.sortedProjects);

/** 是否有项目 */
const hasProjects = computed(() => projectStore.hasProjects);

/** 排序按钮文本 */
const sortButtonText = computed(() => {
  const { sortBy, order } = sortConfig;
  const field = sortBy === 'created_at' ? '创建时间' : '修改时间';
  const direction = order === 'asc' ? '正序' : '降序';
  return `${field}${direction}`;
});

/** 排序方向图标 */
const sortDirectionIcon = computed(() => {
  return sortConfig.order === 'asc' ? ArrowUpIcon : ArrowDownIcon;
});

/** 初始化 */
onMounted(async () => {
  await loadProjects();
});

/** 加载项目列表 */
async function loadProjects() {
  isLoading.value = true;
  try {
    await projectStore.loadProjects();
  } catch (error) {
    console.error("加载项目失败:", error);
    // TODO: 显示错误提示
  } finally {
    isLoading.value = false;
  }
}

/** 刷新（操作成功后调用） */
function handleRefresh() {
  loadProjects();
}

/** 切换排序模式 */
function toggleSort() {
  projectStore.cycleSortConfig();
}

/** 处理项目卡片点击 */
function handleProjectClick(project: Project) {
  // 导航到项目详情页
  router.push({ name: 'project-detail', params: { id: project.id } });
}

/** 处理编辑项目 */
function handleEditProject(project: Project) {
  editingProject.value = project;
  showEditDialog.value = true;
}

/** 处理复制项目 */
async function handleDuplicateProject(project: Project) {
  try {
    await projectStore.duplicateProject(project.id);
    await loadProjects();
    uiStore.showToast("项目已复制", "success");
  } catch (error) {
    console.error("复制项目失败:", error);
    uiStore.showToast("复制项目失败", "error");
  }
}

/** 处理删除项目 */
async function handleDeleteProject(project: Project) {
  try {
    await projectStore.deleteProject(project.id);
    await loadProjects();
  } catch (error) {
    console.error("删除项目失败:", error);
    // TODO: 显示错误提示
  }
}
</script>
