<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-md">
      <DialogHeader>
        <DialogTitle>{{ $t('dropdown.title') }}</DialogTitle>
        <DialogDescription>
          {{ $t('dropdown.description', { name: item?.param_name || '' }) }}
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-4 py-4">
        <!-- 选项列表 -->
        <div class="space-y-2">
          <div
            v-for="(option, index) in localOptions"
            :key="index"
            class="flex items-center gap-2"
          >
            <Input
              :model-value="option"
              @update:model-value="updateOption(index, $event as string)"
              :placeholder="$t('dropdown.addOption')"
              class="flex-1"
            />
            <Button
              variant="ghost"
              size="sm"
              @click="removeOption(index)"
              :disabled="localOptions.length <= 1"
              class="shrink-0 text-destructive hover:text-destructive"
            >
              <XIcon class="h-4 w-4" />
            </Button>
          </div>
        </div>

        <!-- 添加按钮 -->
        <Button
          variant="outline"
          size="sm"
          @click="addOption"
          class="w-full"
        >
          <PlusIcon class="h-4 w-4 mr-2" />
          {{ $t('dropdown.addOption') }}
        </Button>

        <!-- 快捷填充 -->
        <div class="pt-2 border-t">
          <p class="text-xs text-muted-foreground mb-2">{{ $t('dropdown.quickFill') }}</p>
          <div class="flex flex-wrap gap-2">
            <Button
              variant="secondary"
              size="sm"
              @click="quickFill(['true', 'false'])"
            >
              {{ $t('dropdown.boolean') }}
            </Button>
            <Button
              variant="secondary"
              size="sm"
              @click="quickFill(['0.001', '0.01', '0.1', '1.0'])"
            >
              {{ $t('dropdown.learningRate') }}
            </Button>
            <Button
              variant="secondary"
              size="sm"
              @click="quickFill(['sgd', 'adam', 'adamw'])"
            >
              {{ $t('dropdown.optimizer') }}
            </Button>
          </div>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="$emit('update:open', false)">
          {{ $t('common.cancel') }}
        </Button>
        <Button @click="handleSave">
          {{ $t('common.save') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
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
import { PlusIcon, XIcon } from "lucide-vue-next";
import { useFormItems } from "@/composables/useFormItems";
import type { FormItem } from "@/types/bindings";

/** 组件属性 */
const props = defineProps<{
  open: boolean;
  item: FormItem | null;
}>();

/** 定义事件 */
const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

/** 表单项操作 */
const { updateDropdownOptions } = useFormItems();

/** 本地选项列表 (用于编辑) */
const localOptions = ref<string[]>([]);

/** 监听 item 变化，同步到本地 */
watch(() => props.item, (newItem) => {
  if (newItem) {
    localOptions.value = [...newItem.dropdown_options];
  }
}, { immediate: true });

/**
 * 更新选项
 */
function updateOption(index: number, value: string) {
  localOptions.value[index] = value;
}

/**
 * 添加新选项
 */
function addOption() {
  localOptions.value.push("");
}

/**
 * 删除选项
 */
function removeOption(index: number) {
  if (localOptions.value.length > 1) {
    localOptions.value.splice(index, 1);
  }
}

/**
 * 快捷填充
 */
function quickFill(options: string[]) {
  localOptions.value = [...options];
}

/**
 * 保存选项
 */
async function handleSave() {
  if (!props.item) return;

  // 过滤空选项
  const options = localOptions.value.filter(opt => opt.trim() !== "");

  await updateDropdownOptions(props.item.id, options);

  // 关闭对话框
  emit("update:open", false);
}
</script>
