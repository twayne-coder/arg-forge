<template>
  <div
    :class="[
      'group flex items-center gap-3 rounded-lg border p-3 transition-all',
      'hover:border-primary hover:shadow-sm',
      !item.enabled ? 'opacity-60 grayscale bg-muted text-muted-foreground' : 'bg-card',
      item.item_type === 'Command' && item.enabled && 'bg-emerald-50 dark:bg-emerald-950/20 border-emerald-200 dark:border-emerald-900 hover:!border-primary dark:hover:!border-primary'
    ]"
  >
    <!-- 拖拽手柄 -->
    <div class="drag-handle shrink-0 cursor-grab active:cursor-grabbing text-muted-foreground hover:text-foreground relative">
      <GripVerticalIcon class="h-5 w-5 pointer-events-none" />
    </div>

    <!-- 启用开关 -->
    <Switch
      :model-value="item.enabled"
      @update:model-value="handleUpdate('enabled', $event)"
      class="shrink-0"
    />

    <!-- 类型选择器 -->
    <Select
      :model-value="item.item_type"
      @update:model-value="handleUpdate('item_type', $event)"
    >
      <SelectTrigger class="w-[75px]">
        <SelectValue placeholder="类型" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="Command">
          <div class="flex items-center gap-2">
            <TerminalIcon class="h-3.5 w-3.5 text-emerald-600" />
            <span>命令</span>
          </div>
        </SelectItem>
        <SelectItem value="Parameter">
          <div class="flex items-center gap-2">
            <SlidersIcon class="h-3.5 w-3.5" />
            <span>参数</span>
          </div>
        </SelectItem>
      </SelectContent>
    </Select>

    <!-- Command 类型：仅显示内容输入框 -->
    <template v-if="item.item_type === 'Command'">
      <div class="flex-1 min-w-0">
        <Input
          :model-value="item.content"
          @update:model-value="handleUpdate('content', $event)"
          placeholder="命令内容（如 python train.py）"
          class="font-mono text-sm"
        />
      </div>
    </template>

    <!-- Parameter 类型：根据 param_style 显示 -->
    <template v-else>
      <!-- 参数名（ValueOnly 时隐藏） -->
      <div
        v-if="item.param_style !== 'ValueOnly'"
        class="flex-1 min-w-0"
      >
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
        <SelectTrigger class="w-[115px]">
          <SelectValue placeholder="风格" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="KeyValue">
            <span class="font-mono text-xs">--key value</span>
          </SelectItem>
          <SelectItem value="EqualValue">
            <span class="font-mono text-xs">key=value</span>
          </SelectItem>
          <SelectItem value="ValueOnly">
            <span class="font-mono text-xs">value</span>
          </SelectItem>
        </SelectContent>
      </Select>

      <!-- 参数值（统一使用 content） -->
      <div class="flex-1 min-w-0">
        <template v-if="item.use_dropdown && item.dropdown_options.length > 0">
          <Select
            :model-value="item.content"
            @update:model-value="handleUpdate('content', $event)"
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
            :model-value="item.content"
            @update:model-value="handleUpdate('content', $event)"
            placeholder="参数值"
            class="text-sm"
          />
        </template>
      </div>
    </template>

    <!-- 下拉选项按钮 -->
    <Button
      v-if="item.item_type === 'Parameter' && item.use_dropdown"
      variant="ghost"
      size="sm"
      @click="$emit('open-dropdown-options', item)"
      class="shrink-0"
      title="管理下拉选项"
    >
      <SettingsIcon class="h-4 w-4" />
    </Button>

    <!-- 下拉模式切换按钮（仅参数项显示） -->
    <Button
      v-if="item.item_type === 'Parameter'"
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

    <!-- 删除按钮 -->
    <Button
      variant="ghost"
      size="sm"
      @click="$emit('delete', item.id)"
      class="shrink-0 text-destructive hover:text-destructive hover:bg-destructive/10"
      title="删除"
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
  TerminalIcon,
  SlidersIcon,
} from "lucide-vue-next";
import { useFormItems } from "@/composables/useFormItems";
import type { FormItem, FormItemFieldValue } from "@/types/bindings";

/** 组件属性 */
const props = defineProps<{
  item: FormItem;
}>();

/** 定义事件 */
defineEmits<{
  "open-dropdown-options": [item: FormItem];
  delete: [itemId: string];
}>();

/** 表单项操作 */
const { updateFormItem, toggleDropdownMode } = useFormItems();

/**
 * 更新表单项字段
 * @param field - 字段名
 * @param value - 新值
 */
async function handleUpdate(field: string, value: unknown) {
  console.log("[FormItemEditor] handleUpdate:", { field, value, itemId: props.item.id });

  // 将 value 转换为 FormItemFieldValue 类型
  let convertedValue: FormItemFieldValue;

  if (value === null || value === undefined) {
    // 对于 null/undefined 值，使用空字符串
    convertedValue = "";
  } else if (typeof value === "number") {
    // 将数字转换为字符串
    convertedValue = String(value);
  } else if (
    typeof value === "string" ||
    typeof value === "boolean" ||
    Array.isArray(value)
  ) {
    // 已经是有效的 FormItemFieldValue 类型
    convertedValue = value as FormItemFieldValue;
  } else {
    // 其他类型转换为字符串
    convertedValue = String(value);
  }

  console.log("[FormItemEditor] 调用 updateFormItem:", convertedValue);
  await updateFormItem(props.item.id, field, convertedValue);
  console.log("[FormItemEditor] updateFormItem 完成");
}

/**
 * 切换下拉模式
 */
async function toggleDropdown() {
  await toggleDropdownMode(props.item.id, !props.item.use_dropdown);
}
</script>
