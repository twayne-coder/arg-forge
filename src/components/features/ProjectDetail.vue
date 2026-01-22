<template>
  <div class="project-detail h-full flex flex-col">
    <!-- 顶部导航栏 -->
    <header class="flex items-center justify-between px-6 py-4 border-b bg-card">
      <div class="flex items-center gap-2">
        <Button variant="ghost" size="sm" @click="goBack">
          <ArrowLeftIcon class="h-4 w-4 mr-1" />
          返回
        </Button>
        <Separator orientation="vertical" class="h-6" />
        <div>
          <h1 class="text-lg font-semibold">{{ project?.name || "加载中..." }}</h1>
          <p v-if="project" class="text-xs text-muted-foreground">
            {{ project.forms.length }} 个表单
          </p>
        </div>
      </div>
      <Button variant="ghost" size="sm" @click="openEditProjectDialog">
        <EditIcon class="h-4 w-4 mr-1" />
        编辑项目
      </Button>
    </header>

    <!-- 分割器布局 -->
    <div class="split-container flex flex-1 overflow-hidden">
      <!-- 左侧: 表单列表 (30%) -->
      <aside class="form-list-panel w-[30%] min-w-[250px] border-r bg-card flex flex-col">
        <div class="p-4 border-b">
          <div class="flex items-center justify-between">
            <h2 class="font-medium">表单列表</h2>
            <Button size="sm" @click="openCreateFormDialog">
              <PlusIcon class="h-4 w-4" />
            </Button>
          </div>
        </div>
        <ScrollArea class="flex-1">
          <div class="p-2 space-y-1">
            <FormListItem
              v-for="form in forms"
              :key="form.id"
              :form="form"
              :active="currentFormId === form.id"
              @click="selectForm(form.id)"
            />
            <div
              v-if="forms.length === 0"
              class="text-center py-8 text-sm text-muted-foreground"
            >
              暂无表单
            </div>
          </div>
        </ScrollArea>
      </aside>

      <!-- 右侧: 表单详情 (70%) -->
      <main class="form-detail-panel flex-1 bg-background">
        <FormDetailEditor
          v-if="currentForm"
          :form="currentForm"
          @edit="openEditFormDialog"
        />
        <div
          v-else
          class="h-full flex items-center justify-center text-muted-foreground"
        >
          <div class="text-center">
            <FileTextIcon class="h-12 w-12 mx-auto mb-3 opacity-50" />
            <p class="text-sm">选择一个表单查看详情</p>
          </div>
        </div>
      </main>
    </div>

    <!-- 创建表单对话框 -->
    <CreateFormDialog
      v-model:open="showCreateFormDialog"
      @success="handleFormSuccess"
    />

    <!-- 编辑表单对话框 -->
    <EditFormDialog
      v-if="editingForm"
      v-model:open="showEditFormDialog"
      :form="editingForm"
      @success="handleFormSuccess"
    />

    <!-- 编辑项目对话框 -->
    <EditProjectDialog
      v-if="project"
      v-model:open="showEditProjectDialog"
      :project="project"
      @success="handleProjectUpdated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { ArrowLeftIcon, EditIcon, PlusIcon, FileTextIcon } from "lucide-vue-next";
import { useProjectStore } from "@/stores/project";
import FormListItem from "./FormListItem.vue";
import FormDetailEditor from "./FormDetailEditor.vue";
import CreateFormDialog from "./CreateFormDialog.vue";
import EditFormDialog from "./EditFormDialog.vue";
import EditProjectDialog from "./EditProjectDialog.vue";
import type { Form } from "@/types/bindings";

/** 路由 */
const route = useRoute();
const router = useRouter();

/** 项目 store */
const projectStore = useProjectStore();

/** 当前项目 ID */
const projectId = computed(() => route.params.id as string);

/** 当前项目 */
const project = computed(() => projectStore.currentProject);

/** 表单列表 */
const forms = computed(() => project.value?.forms || []);

/** 当前表单 ID */
const currentFormId = computed(() => projectStore.currentForm?.id);

/** 当前表单 */
const currentForm = computed(() => {
  if (!currentFormId.value) return null;
  return forms.value.find(f => f.id === currentFormId.value) || null;
});

/** 是否显示创建表单对话框 */
const showCreateFormDialog = ref(false);

/** 是否显示编辑表单对话框 */
const showEditFormDialog = ref(false);

/** 是否显示编辑项目对话框 */
const showEditProjectDialog = ref(false);

/** 正在编辑的表单 */
const editingForm = ref<Form | null>(null);

/** 初始化 */
onMounted(async () => {
  await loadProject();
});

/** 加载项目数据 */
async function loadProject() {
  try {
    await projectStore.loadProject(projectId.value);
  } catch (error) {
    console.error("加载项目失败:", error);
    // 如果加载失败,返回项目列表
    router.replace("/");
  }
}

/** 返回项目列表 */
function goBack() {
  router.push("/");
}

/** 选择表单 */
function selectForm(formId: string) {
  const form = forms.value.find(f => f.id === formId);
  if (form) {
    projectStore.setCurrentForm(form);
  }
}

/** 打开创建表单对话框 */
function openCreateFormDialog() {
  showCreateFormDialog.value = true;
}

/** 打开编辑表单对话框 */
function openEditFormDialog() {
  if (currentForm.value) {
    editingForm.value = currentForm.value;
    showEditFormDialog.value = true;
  }
}

/** 打开编辑项目对话框 */
function openEditProjectDialog() {
  showEditProjectDialog.value = true;
}

/** 表单操作成功后的处理 */
async function handleFormSuccess() {
  // 重新加载项目数据
  await loadProject();
}

/** 项目更新后的处理 */
async function handleProjectUpdated() {
  // 重新加载项目数据
  await loadProject();
}
</script>

<style scoped>
.split-container {
  /* 减去标题栏和头部的高度 */
  height: calc(100vh - 120px);
}

.form-list-panel {
  /* 防止内容溢出 */
  overflow: hidden;
}

.form-detail-panel {
  /* 防止内容溢出 */
  overflow: hidden;
}
</style>
