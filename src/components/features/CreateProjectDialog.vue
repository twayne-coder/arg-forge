<template>
  <Dialog :open="open" @update:open="emit('update:open', $event)">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>新建项目</DialogTitle>
        <DialogDescription>
          创建一个新的命令配置项目
        </DialogDescription>
      </DialogHeader>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <!-- 项目名称 -->
        <div class="space-y-2">
          <Label for="name">项目名称 <span class="text-destructive">*</span></Label>
          <Input
            id="name"
            v-model="formData.name"
            placeholder="例如：模型训练配置"
            required
            :disabled="loading"
          />
        </div>

        <!-- 项目描述 -->
        <div class="space-y-2">
          <Label for="description">项目描述</Label>
          <Textarea
            id="description"
            v-model="formData.description"
            placeholder="简要描述此项目的用途..."
            rows="3"
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
    };
  }
});

/** 处理表单提交 */
async function handleSubmit() {
  if (!isFormValid.value) return;

  loading.value = true;
  try {
    await projectStore.createProject(
      formData.value.name.trim(),
      formData.value.description.trim()
    );

    // 成功后关闭对话框并通知父组件
    emit("update:open", false);
    emit("success");
  } catch (error) {
    console.error("创建项目失败:", error);
    // TODO: 显示错误提示
  } finally {
    loading.value = false;
  }
}
</script>
