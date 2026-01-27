<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>{{ $t('project.edit') }}</DialogTitle>
        <DialogDescription>
          {{ $t('project.editDescription') }}
        </DialogDescription>
      </DialogHeader>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- 项目名称 -->
        <div class="space-y-2">
          <Label for="edit-name">{{ $t('project.name') }} <span class="text-destructive">*</span></Label>
          <Input
            id="edit-name"
            v-model="formData.name"
            :placeholder="$t('project.namePlaceholder')"
            required
            :disabled="loading"
          />
        </div>

        <!-- 项目描述 -->
        <div class="space-y-2">
          <Label for="edit-description">{{ $t('project.description') }}</Label>
          <Textarea
            id="edit-description"
            v-model="formData.description"
            :placeholder="$t('project.descriptionPlaceholder')"
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
            <span>{{ $t('project.formCount', { count: project.forms.length }) }}</span>
          </div>
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            @click="emit('update:open', false)"
            :disabled="loading"
          >
            {{ $t('common.cancel') }}
          </Button>
          <Button type="submit" :disabled="loading || !isFormValid">
            {{ loading ? $t('project.saving') : $t('common.save') }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
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

/** 表单数据 */
const formData = ref({
  name: "",
  description: "",
});

/** 加载状态 */
const loading = ref(false);

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
