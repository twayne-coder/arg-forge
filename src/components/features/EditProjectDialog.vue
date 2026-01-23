<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>编辑项目</DialogTitle>
        <DialogDescription>
          修改项目信息或管理项目
        </DialogDescription>
      </DialogHeader>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- 项目名称 -->
        <div class="space-y-2">
          <Label for="edit-name">项目名称 <span class="text-destructive">*</span></Label>
          <Input
            id="edit-name"
            v-model="formData.name"
            placeholder="例如：模型训练配置"
            required
            :disabled="loading"
          />
        </div>

        <!-- 项目描述 -->
        <div class="space-y-2">
          <Label for="edit-description">项目描述</Label>
          <Textarea
            id="edit-description"
            v-model="formData.description"
            placeholder="简要描述此项目的用途..."
            rows="3"
            :disabled="loading"
          />
        </div>

        <!-- 项目元信息 -->
        <div class="flex items-center gap-4 text-sm text-muted-foreground">
          <div class="flex items-center gap-1">
            <CalendarIcon class="h-4 w-4" />
            <span>{{ formatDate(project.created_at) }}</span>
          </div>
          <div class="flex items-center gap-1">
            <FolderOpenIcon class="h-4 w-4" />
            <span>{{ project.forms.length }} 个表单</span>
          </div>
        </div>

        <DialogFooter class="gap-2">
          <!-- 危险操作区 -->
          <div class="flex gap-2 mr-auto">
            <Button
              type="button"
              variant="outline"
              size="sm"
              @click="handleDuplicate"
              :disabled="loading"
            >
              <CopyIcon class="h-4 w-4 mr-1" />
              复制
            </Button>
            <Button
              type="button"
              variant="destructive"
              size="sm"
              @click="showDeleteConfirm = true"
              :disabled="loading"
            >
              <TrashIcon class="h-4 w-4 mr-1" />
              删除
            </Button>
          </div>

          <!-- 标准操作区 -->
          <Button
            type="button"
            variant="outline"
            @click="emit('update:open', false)"
            :disabled="loading"
          >
            取消
          </Button>
          <Button type="submit" :disabled="loading || !isFormValid">
            {{ loading ? "保存中..." : "保存" }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>

    <!-- 删除确认对话框 -->
    <Dialog :open="showDeleteConfirm" @update:open="showDeleteConfirm = $event">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>确认删除</DialogTitle>
          <DialogDescription>
            确定要删除项目 "{{ project.name }}" 吗？此操作无法撤销。
          </DialogDescription>
        </DialogHeader>

        <DialogFooter>
          <Button
            variant="outline"
            @click="showDeleteConfirm = false"
            :disabled="loading"
          >
            取消
          </Button>
          <Button
            variant="destructive"
            @click="handleDelete"
            :disabled="loading"
          >
            {{ loading ? "删除中..." : "确认删除" }}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";
import {
  CalendarIcon,
  FolderOpenIcon,
  CopyIcon,
  TrashIcon,
} from "lucide-vue-next";
import { useProjectStore } from "@/stores/project";
import type { Project } from "@/types/bindings";

/** 是否打开对话框 */
const props = defineProps<{
  open: boolean;
  project: Project;
}>();

/** 事件 */
const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
  (e: "success"): void;
}>();

/** 项目 store */
const projectStore = useProjectStore();

/** 路由 */
const route = useRoute();
const router = useRouter();

/** 表单数据 */
const formData = ref({
  name: "",
  description: "",
});

/** 加载状态 */
const loading = ref(false);

/** 是否显示删除确认对话框 */
const showDeleteConfirm = ref(false);

/** 表单是否有效 */
const isFormValid = computed(() => {
  return formData.value.name.trim().length > 0;
});

/** 监听对话框打开/关闭，初始化表单 */
watch(() => props.open, (isOpen) => {
  if (isOpen && props.project) {
    // 对话框打开时初始化表单数据
    formData.value = {
      name: props.project.name,
      description: props.project.description,
    };
  }
});

/** 监听项目变化，更新表单 */
watch(() => props.project, (newProject) => {
  if (newProject) {
    formData.value = {
      name: newProject.name,
      description: newProject.description,
    };
  }
});

/** 处理表单提交 */
async function handleSubmit() {
  if (!isFormValid.value) return;

  loading.value = true;
  try {
    await projectStore.updateProject(
      props.project.id,
      formData.value.name.trim(),
      formData.value.description.trim()
    );

    // 成功后关闭对话框并通知父组件
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("更新项目失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}

/** 复制项目 */
async function handleDuplicate() {
  loading.value = true;
  try {
    await projectStore.duplicateProject(props.project.id);

    // 成功后关闭对话框并通知父组件
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("复制项目失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}

/** 删除项目 */
async function handleDelete() {
  loading.value = true;
  try {
    await projectStore.deleteProject(props.project.id);

    // 成功后关闭所有对话框并通知父组件
    showDeleteConfirm.value = false;
    emit("update:open", false);
    emit("success");

    // 如果当前在项目详情页且删除的是当前项目，则返回首页
    if (route.name === 'project-detail' && route.params.id === props.project.id) {
      router.push('/');
    }
  } catch (error) {
    console.error("删除项目失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}

/** 格式化日期 */
function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    });
  } catch {
    return dateString;
  }
}
</script>
