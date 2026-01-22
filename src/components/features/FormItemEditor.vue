<template>
  <div
    :class="[
      'group flex items-center gap-3 rounded-lg border bg-card p-3 transition-all',
      'hover:border-primary/50 hover:shadow-sm',
      !item.enabled && 'opacity-50'
    ]"
  >
    <!-- 拖拽手柄 -->
    <div class="drag-handle cursor-grab active:cursor-grabbing text-muted-foreground hover:text-foreground">
      <GripVerticalIcon class="h-5 w-5" />
    </div>

    <!-- 启用开关 -->
    <Switch
      :checked="item.enabled"
      @update:checked="handleUpdate('enabled', $event)"
      class="shrink-0"
    />

    <!-- 参数名 -->
    <div class="flex-1 min-w-0">
      <Input
        :model-value="item.param_name"
        @update:model-value="handleUpdate('param_name', $event)"
        placeholder="参数名"
        class="font-mono text-sm"
      />
    </div>

    <!-- 参数风格 -->
    <Select
      :model-value="item.param_style"
      @update:model-value="handleUpdate('param_style', $event)"
    >
      <SelectTrigger class="w-[130px]">
        <SelectValue placeholder="风格" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="Argparse">
          <span class="font-mono text-xs">--key value</span>
        </SelectItem>
        <SelectItem value="Hydra">
          <span class="font-mono text-xs">key=value</span>
        </SelectItem>
        <SelectItem value="Positional">
          <span class="font-mono text-xs">value</span>
        </SelectItem>
      </SelectContent>
    </Select>

    <!-- 参数值 (输入框或下拉框) -->
    <div class="flex-1 min-w-0">
      <template v-if="item.use_dropdown && item.dropdown_options.length > 0">
        <Select
          :model-value="item.param_value"
          @update:model-value="handleUpdate('param_value', $event)"
        >
          <SelectTrigger class="w-full">
            <SelectValue placeholder="选择值" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem
              v-for="option in item.dropdown_options"
              :key="option"
              :value="option"
            >
              {{ option }}
            </SelectItem>
          </SelectContent>
        </Select>
      </template>
      <template v-else>
        <Input
          :model-value="item.param_value"
          @update:model-value="handleUpdate('param_value', $event)"
          placeholder="参数值"
          class="text-sm"
        />
      </template>
    </div>

    <!-- 下拉模式切换按钮 -->
    <Button
      variant="ghost"
      size="sm"
      @click="toggleDropdown"
      :class="[
        'shrink-0',
        item.use_dropdown && 'bg-primary/10 text-primary hover:bg-primary/20'
      ]"
      title="切换下拉模式"
    >
      <ListIcon class="h-4 w-4" />
    </Button>

    <!-- 下拉选项按钮 (仅在下拉模式时显示) -->
    <Button
      v-if="item.use_dropdown"
      variant="ghost"
      size="sm"
      @click="$emit('open-dropdown-options', item)"
      class="shrink-0"
      title="管理下拉选项"
    >
      <SettingsIcon class="h-4 w-4" />
    </Button>

    <!-- 删除按钮 -->
    <Button
      variant="ghost"
      size="sm"
      @click="$emit('delete', item.id)"
      class="shrink-0 text-destructive hover:text-destructive hover:bg-destructive/10"
      title="删除参数"
    >
      <TrashIcon class="h-4 w-4" />
    </Button>
  </div>
</template>

<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  GripVerticalIcon,
  ListIcon,
  SettingsIcon,
  TrashIcon,
} from "lucide-vue-next";
import { useProjectStore } from "@/stores/project";
import type { FormItem } from "@/types/bindings";

/** 组件属性 */
const props = defineProps<{
  item: FormItem;
}>();

/** 定义事件 */
defineEmits<{
  "open-dropdown-options": [item: FormItem];
  delete: [itemId: string];
}>();

/** 项目 store */
const store = useProjectStore();

/**
 * 更新表单项字段
 * @param field - 字段名
 * @param value - 新值
 */
async function handleUpdate(field: string, value: any) {
  await store.updateFormItem(props.item.id, field, value);
}

/**
 * 切换下拉模式
 */
async function toggleDropdown() {
  await store.toggleDropdownMode(props.item.id, !props.item.use_dropdown);
}
</script>
