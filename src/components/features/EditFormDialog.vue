<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>编辑表单</DialogTitle>
        <DialogDescription>
          修改表单的基本信息
        </DialogDescription>
      </DialogHeader>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- 表单名称 -->
        <div class="space-y-2">
          <Label for="name">表单名称 <span class="text-destructive">*</span></Label>
          <Input
            id="name"
            v-model="formData.name"
            placeholder="例如：训练参数配置"
            required
            :disabled="loading"
          />
        </div>

        <!-- 表单描述 -->
        <div class="space-y-2">
          <Label for="description">表单描述</Label>
          <Textarea
            id="description"
            v-model="formData.description"
            placeholder="简要描述此表单的用途..."
            rows="2"
            :disabled="loading"
          />
        </div>

        <DialogFooter>
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
import { useProjectStore } from "@/stores/project";
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

/** 监听对话框打开/关闭，初始化/重置表单 */
watch(() => props.open, (isOpen) => {
  if (isOpen && props.form) {
    // 对话框打开时，填充表单数据
    formData.value = {
      name: props.form.name,
      description: props.form.description || "",
    };
  }
});

/** 处理表单提交 */
async function handleSubmit() {
  if (!isFormValid.value) return;

  loading.value = true;
  try {
    await projectStore.updateForm(props.form.id, {
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
</script>
