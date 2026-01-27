<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>{{ $t('form.edit') }}</DialogTitle>
        <DialogDescription>
          {{ $t('form.editDescription') }}
        </DialogDescription>
      </DialogHeader>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- 表单名称 -->
        <div class="space-y-2">
          <Label for="name">{{ $t('form.name') }} <span class="text-destructive">*</span></Label>
          <Input
            id="name"
            v-model="formData.name"
            :placeholder="$t('form.namePlaceholder')"
            required
            :disabled="loading"
          />
        </div>

        <!-- 表单描述 -->
        <div class="space-y-2">
          <Label for="description">{{ $t('form.description') }}</Label>
          <Textarea
            id="description"
            v-model="formData.description"
            :placeholder="$t('form.descriptionPlaceholder')"
            rows="2"
            :disabled="loading"
          />
        </div>

        <div class="flex flex-col-reverse sm:flex-row sm:justify-between gap-2">
          <div class="flex gap-2">
            <Button
              type="button"
              variant="destructive"
              @click="handleDelete"
              :disabled="loading || deleting"
            >
              {{ $t('common.delete') }}
            </Button>
            <Button
              type="button"
              variant="secondary"
              @click="handleDuplicate"
              :disabled="loading || deleting"
            >
              {{ $t('common.duplicate') }}
            </Button>
          </div>
          <div class="flex gap-2">
            <Button
              type="button"
              variant="outline"
              @click="emit('update:open', false)"
              :disabled="loading || deleting"
            >
              {{ $t('common.cancel') }}
            </Button>
            <Button type="submit" :disabled="loading || deleting || !isFormValid">
              {{ loading ? $t('project.saving') : $t('common.save') }}
            </Button>
          </div>
        </div>
      </form>
    </DialogContent>
  </Dialog>

  <!-- 删除确认对话框 -->
  <Dialog :open="showDeleteConfirm" @update:open="showDeleteConfirm = $event">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{{ $t('form.deleteConfirm') }}</DialogTitle>
        <DialogDescription>
          {{ $t('form.deleteConfirmMessage', { name: props.form.name || '' }) }}
        </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button
          variant="outline"
          @click="showDeleteConfirm = false"
          :disabled="deleting"
        >
          {{ $t('common.cancel') }}
        </Button>
        <Button
          variant="destructive"
          @click="confirmDelete"
          :disabled="deleting"
        >
          {{ deleting ? $t('form.deleting') : $t('form.deleteConfirmButton') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useI18n } from "vue-i18n";
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
import { useProjectStore } from "@/stores/project";
import { useFormStore } from "@/stores/form";
import { useUiStore } from "@/stores/ui";
import type { Form } from "@/types/bindings";

/** 是否打开对话框 */
const props = defineProps<{
  open: boolean;
  form: Form;
}>();

/** 事件 */
const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
  (e: "success"): void;
}>();

/** 国际化 */
const { t } = useI18n();

/** 项目 store */
const projectStore = useProjectStore();

/** 表单 store */
const formStore = useFormStore();

/** UI store */
const uiStore = useUiStore();

/** 表单数据 */
const formData = ref({
  name: "",
  description: "",
});

/** 加载状态 */
const loading = ref(false);

/** 是否显示删除确认对话框 */
const showDeleteConfirm = ref(false);

/** 删除中状态 */
const deleting = ref(false);

/** 表单是否有效 */
const isFormValid = computed(() => {
  return formData.value.name.trim().length > 0;
});

/** 监听对话框打开/关闭，初始化/重置表单 */
watch(() => props.open, (isOpen) => {
  if (isOpen && props.form) {
    // 对话框打开时，填充表单数据
    formData.value = {
      name: props.form.name,
      description: props.form.description || "",
    };
  }
}, { immediate: true });  // 立即执行，确保组件创建时填充数据

/** 处理表单提交 */
async function handleSubmit() {
  if (!isFormValid.value || !projectStore.currentProject) return;

  loading.value = true;
  try {
    await formStore.updateForm(projectStore.currentProject.id, props.form.id, {
      name: formData.value.name.trim(),
      description: formData.value.description.trim(),
    });

    // 成功后关闭对话框并通知父组件
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("更新表单失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}

/** 处理表单克隆 */
async function handleDuplicate() {
  if (!projectStore.currentProject) return;

  loading.value = true;
  try {
    await formStore.duplicateForm(
      projectStore.currentProject.id,
      props.form.id
    );

    // 成功后关闭对话框并通知父组件刷新
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("克隆表单失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}

/** 处理删除按钮点击 */
function handleDelete() {
  showDeleteConfirm.value = true;
}

/** 确认删除 */
async function confirmDelete() {
  if (!projectStore.currentProject) return;

  deleting.value = true;
  try {
    await formStore.deleteForm(
      projectStore.currentProject.id,
      props.form.id
    );

    uiStore.showToast(t('form.deleteSuccess'), "success");
    showDeleteConfirm.value = false;
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("删除表单失败:", error);
    uiStore.showToast(t('form.deleteFailed'), "error");
  } finally {
    deleting.value = false;
  }
}
</script>
