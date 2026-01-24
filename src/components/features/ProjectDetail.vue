<template>
  <div class="project-detail h-full flex flex-col">
    <!-- 顶部导航栏 -->
    <header class="shrink-0 flex items-center justify-between px-6 py-4 border-b bg-card">
      <div class="flex items-center gap-2">
        <Button variant="ghost" size="sm" @click="goBack">
          <ArrowLeftIcon class="h-4 w-4 mr-1" />
          返回
        </Button>
        <Separator orientation="vertical" class="h-6" />
        <div class="flex items-center gap-2">
          <div>
            <h1 class="text-lg font-semibold">{{ project?.name }}</h1>
            <div class="flex items-center gap-2">
              <p v-if="project" class="text-xs text-muted-foreground">
                {{ project.forms.length }} 个表单
              </p>
              <p v-if="project?.description" class="text-xs text-muted-foreground">
                {{ project.description }}
              </p>
            </div>
          </div>
        </div>
      </div>
      <Button variant="ghost" size="sm" @click="openEditProjectDialog">
        <EditIcon class="h-4 w-4 mr-1" />
        编辑项目
      </Button>
    </header>

    <!-- 分割器布局 -->
    <Splitpanes class="flex-1 overflow-hidden">
      <!-- 左侧: 表单列表 -->
      <Pane :size="30" :min-size="15" :max-size="50">
        <aside class="form-list-panel h-full bg-card flex flex-col">
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
      </Pane>

      <!-- 右侧: 表单详情 -->
      <Pane :size="70" :min-size="40">
        <main class="form-detail-panel h-full bg-background">
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
      </Pane>
    </Splitpanes>

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
import { useRoute, useRouter, onBeforeRouteLeave } from "vue-router";
import { Splitpanes, Pane } from "splitpanes";
import "splitpanes/dist/splitpanes.css";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { ArrowLeftIcon, EditIcon, PlusIcon, FileTextIcon } from "lucide-vue-next";
import { useProjectStore } from "@/stores/project";
import { useFormStore } from "@/stores/form";
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

/** 表单 store */
const formStore = useFormStore();

/** 当前项目 ID */
const projectId = computed(() => route.params.id as string);

/** 当前项目 */
const project = computed(() => projectStore.currentProject);

/** 表单列表 */
const forms = computed(() => project.value?.forms || []);

/** 当前表单 ID */
const currentFormId = computed(() => formStore.currentForm?.id);

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

/** 离开页面前清理状态 */
onBeforeRouteLeave(() => {
  // 清空当前项目和表单状态，避免状态残留导致闪烁
  projectStore.$reset();
  formStore.setCurrentForm(null);
});

/** 加载项目数据 */
async function loadProject() {
  try {
    await projectStore.setCurrentProject(projectId.value);
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
    formStore.setCurrentForm(form);
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
  // 保存当前表单 ID
  const savedFormId = formStore.currentForm?.id;

  // 重新加载项目数据
  await loadProject();

  // 恢复表单选中状态
  if (savedFormId) {
    const formToRestore = forms.value.find(f => f.id === savedFormId);
    if (formToRestore) {
      formStore.setCurrentForm(formToRestore);
    }
  }
}

/** 项目更新后的处理 */
async function handleProjectUpdated() {
  // 保存当前表单 ID
  const savedFormId = formStore.currentForm?.id;

  // 重新加载项目数据
  await loadProject();

  // 恢复表单选中状态
  if (savedFormId) {
    const formToRestore = forms.value.find(f => f.id === savedFormId);
    if (formToRestore) {
      formStore.setCurrentForm(formToRestore);
    }
  }
}
</script>

<style scoped>
/* ========== 分割线优化：分离布局与交互 ========== */

/* 分割器在布局中只占 1px（不影响 flex 计算） */
:deep(.splitpanes__splitter) {
  width: 0 !important;
  min-width: 0 !important;
  border-left: 1px solid hsl(var(--border));
  position: relative;
  cursor: default;
  z-index: 10;
  transition: border-left-color 0.2s ease;
}

/* 透明伪元素扩展点击热区：左右各 4px，总共 9px */
:deep(.splitpanes__splitter)::before {
  content: '';
  position: absolute;
  top: 0;
  left: -4px; /* 向左扩展 4px */
  width: 9px; /* 左 4px + 自身 1px + 右 4px */
  height: 100%;
  background-color: transparent;
  cursor: col-resize; /* 热区显示拖拽光标 */
  z-index: 1;
}

/* 可选：悬浮时的中心指示器（提升视觉反馈） */
:deep(.splitpanes__splitter)::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 4px;
  height: 40px;
  background-color: hsl(var(--primary));
  border-radius: 2px;
  opacity: 0;
  transition: opacity 0.2s ease, width 0.2s ease;
  pointer-events: none; /* 不干扰交互 */
  z-index: 2;
}

/* 悬浮效果：背景高亮 + 指示器显示 */
:deep(.splitpanes__splitter:hover) {
  border-left-color: hsl(var(--primary));
}

:deep(.splitpanes__splitter:hover)::after {
  opacity: 1;
  width: 6px;
  height: 48px;
}

/* 拖拽状态（需要 JS 配合添加 dragging 类） */
:deep(.splitpanes__splitter.dragging) {
  border-left-color: hsl(var(--primary) / 0.5);
}

:deep(.splitpanes__splitter.dragging)::after {
  opacity: 1;
  width: 5px;
}

/* ========== 面板样式 ========== */
.form-list-panel,
.form-detail-panel {
  overflow: hidden;
}
</style>
