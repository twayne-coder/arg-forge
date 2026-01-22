<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>新建表单</DialogTitle>
        <DialogDescription>
          为当前项目创建一个新的命令配置表单
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

        <!-- 命令模板 -->
        <div class="space-y-2">
          <Label for="commandTemplate">命令模板</Label>
          <Input
            id="commandTemplate"
            v-model="formData.command_template"
            placeholder='例如：python train.py {params}'
            :disabled="loading"
          />
          <p class="text-xs text-muted-foreground">
            使用 <code class="bg-muted px-1 rounded">{params}</code> 作为参数占位符
          </p>
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
            {{ loading ? "创建中..." : "创建" }}
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

/** 是否打开对话框 */
const props = defineProps<{
  open: boolean;
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
  command_template: "",
});

/** 加载状态 */
const loading = ref(false);

/** 表单是否有效 */
const isFormValid = computed(() => {
  return formData.value.name.trim().length > 0;
});

/** 监听对话框打开/关闭，重置表单 */
watch(() => props.open, (isOpen) => {
  if (!isOpen) {
    // 对话框关闭时重置表单
    formData.value = {
      name: "",
      description: "",
      command_template: "",
    };
  }
});

/** 处理表单提交 */
async function handleSubmit() {
  if (!isFormValid.value) return;

  loading.value = true;
  try {
    await projectStore.createForm({
      name: formData.value.name.trim(),
      description: formData.value.description.trim(),
      command_template: formData.value.command_template.trim(),
    });

    // 成功后关闭对话框并通知父组件
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("创建表单失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}
</script>
