<template>
  <div
    :class="[
      'group flex items-center gap-2 rounded-lg border p-2 transition-all',
      !item.enabled
        ? [
          'opacity-50 grayscale bg-slate-50 dark:bg-slate-900/30',
          'border-slate-200 dark:border-slate-700 text-slate-500 dark:text-slate-400'
        ]
        : item.item_type === 'Command'
          ? [
            'bg-emerald-100 dark:bg-emerald-950',
            'border-emerald-200 dark:border-emerald-700',
            'hover:border-emerald-400 dark:hover:border-emerald-500 hover:shadow-sm'
          ]
          : [
            'bg-teal-50 dark:bg-teal-950/20',
            'border-teal-100 dark:border-teal-900',
            'hover:border-teal-300 dark:hover:border-teal-700 hover:shadow-sm'
          ]
    ]"
  >
    <!-- 拖拽手柄 -->
    <div class="drag-handle shrink-0 cursor-grab active:cursor-grabbing text-muted-foreground hover:text-foreground relative">
      <GripVerticalIcon class="h-4 w-4 pointer-events-none" />
    </div>

    <!-- 启用开关 -->
    <Switch
      :model-value="item.enabled"
      @update:model-value="handleUpdate('enabled', $event)"
      class="shrink-0"
    />

    <!-- Command 类型：仅显示内容输入框 -->
    <template v-if="item.item_type === 'Command'">
      <div class="flex-1 min-w-0">
        <Input
          :model-value="item.content"
          @update:model-value="handleUpdate('content', $event)"
          :placeholder="$t('formItem.contentPlaceholder')"
          class="font-mono text-xs h-8"
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
          :placeholder="$t('formItem.paramNamePlaceholder')"
          class="font-mono text-xs h-8"
        />
      </div>

      <!-- 参数风格 -->
      <Select
        :key="`style-${locale}`"
        :model-value="item.param_style"
        @update:model-value="handleUpdate('param_style', $event)"
      >
        <SelectTrigger class="w-[110px] h-8 text-xs sm:w-[120px] md:w-[130px] lg:w-[140px]">
          <SelectValue :placeholder="$t('formItem.paramStyle')" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="KeyValue">
            <span class="font-mono text-xs">{{ $t('paramStyle.keyValue') }}</span>
          </SelectItem>
          <SelectItem value="EqualValue">
            <span class="font-mono text-xs">{{ $t('paramStyle.equalValue') }}</span>
          </SelectItem>
          <SelectItem value="ValueOnly">
            <span class="font-mono text-xs">{{ $t('paramStyle.valueOnly') }}</span>
          </SelectItem>
        </SelectContent>
      </Select>

      <!-- 参数值（统一使用 content） -->
      <div class="flex-1 min-w-0">
        <template v-if="item.use_dropdown && item.dropdown_options.length > 0">
          <Select
            :key="`value-${locale}`"
            :model-value="item.content"
            @update:model-value="handleUpdate('content', $event)"
          >
            <SelectTrigger class="w-full h-8 text-xs">
              <SelectValue :placeholder="$t('formItem.selectValue')" />
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
            :placeholder="$t('formItem.paramValuePlaceholder')"
            class="text-xs h-8"
          />
        </template>
      </div>
    </template>

    <!-- 设置按钮 -->
    <Button
      variant="ghost"
      size="sm"
      @click="$emit('open-settings', item)"
      class="shrink-0 h-8 w-8 p-0"
      :title="$t('formItem.settings')"
    >
      <SettingsIcon class="h-3.5 w-3.5" />
    </Button>

    <!-- 删除按钮 -->
    <Button
      variant="ghost"
      size="sm"
      @click="$emit('delete', item.id)"
      class="shrink-0 h-8 w-8 p-0 text-destructive dark:text-red-500 hover:text-destructive dark:hover:text-red-400 hover:bg-destructive/10 dark:hover:bg-red-500/10"
      :title="$t('formItem.delete')"
    >
      <TrashIcon class="h-3.5 w-3.5" />
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
  SettingsIcon,
  TrashIcon,
} from "lucide-vue-next";
import { useFormItems } from "@/composables/useFormItems";
import type { FormItem, FormItemFieldValue } from "@/types/bindings";
import { useI18n } from "vue-i18n";

/** 组件属性 */
const props = defineProps<{
  item: FormItem;
}>();

/** 定义事件 */
defineEmits<{
  "open-settings": [item: FormItem];
  delete: [itemId: string];
}>();

/** 表单项操作 */
const { updateFormItem } = useFormItems();

/** 获取当前语言（用于强制刷新 Select 组件） */
const { locale } = useI18n();

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
</script>
