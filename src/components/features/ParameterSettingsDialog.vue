<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-md max-h-[80vh] flex flex-col">
      <DialogHeader>
        <DialogTitle>{{ $t('settings.title') }}</DialogTitle>
        <DialogDescription>
          {{ $t('settings.description') }}
        </DialogDescription>
      </DialogHeader>

      <ScrollArea class="h-[calc(80vh-14rem)] py-3">
        <div class="space-y-4 pr-4">
        <!-- 参数类型选择 -->
        <div class="space-y-2">
          <Label class="text-xs font-medium">{{ $t('formItem.type') }}</Label>
          <RadioGroup :model-value="localItemType" @update:model-value="localItemType = $event as ItemType" class="flex gap-2">
            <Label
              for="command-type"
              class="flex-1 cursor-pointer"
            >
              <div class="flex items-center justify-center gap-1.5 rounded-md border p-2 transition-colors
                          hover:bg-accent/50 has-[:checked]:bg-primary/10
                          has-[:checked]:border-primary">
                <RadioGroupItem value="Command" id="command-type" class="peer border-primary h-4 w-4" />
                <TerminalIcon class="h-3.5 w-3.5 text-emerald-600" />
                <span class="text-xs">{{ $t('formItem.command') }}</span>
              </div>
            </Label>
            <Label
              for="parameter-type"
              class="flex-1 cursor-pointer"
            >
              <div class="flex items-center justify-center gap-1.5 rounded-md border p-2 transition-colors
                          hover:bg-accent/50 has-[:checked]:bg-primary/10
                          has-[:checked]:border-primary">
                <RadioGroupItem value="Parameter" id="parameter-type" class="peer border-primary h-4 w-4" />
                <SlidersIcon class="h-3.5 w-3.5" />
                <span class="text-xs">{{ $t('formItem.parameter') }}</span>
              </div>
            </Label>
          </RadioGroup>
        </div>

        <!-- 下拉模式（仅 Parameter 类型时显示） -->
        <template v-if="localItemType === 'Parameter'">
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <Label class="text-xs font-medium">{{ $t('settings.dropdownMode') }}</Label>
              <Switch
                :model-value="localUseDropdown"
                @update:model-value="localUseDropdown = $event"
              />
            </div>

            <!-- 下拉选项（仅启用下拉模式时显示） -->
            <div v-if="localUseDropdown" class="space-y-2 pt-2">
              <Label class="text-xs text-muted-foreground">{{ $t('dropdown.addOption') }}</Label>

              <!-- 选项列表 -->
              <div class="space-y-1.5">
                <div
                  v-for="(option, index) in localDropdownOptions"
                  :key="index"
                  class="flex items-center gap-1.5"
                >
                  <Input
                    :model-value="option"
                    @update:model-value="updateOption(index, $event as string)"
                    :placeholder="$t('dropdown.addOption')"
                    class="flex-1 h-8 text-xs"
                  />
                  <Button
                    variant="ghost"
                    size="sm"
                    @click="removeOption(index)"
                    :disabled="localDropdownOptions.length <= 1"
                    class="shrink-0 h-8 w-8 p-0 text-destructive hover:text-destructive"
                  >
                    <XIcon class="h-3.5 w-3.5" />
                  </Button>
                </div>
              </div>

              <!-- 添加按钮 -->
              <Button
                variant="outline"
                size="sm"
                @click="addOption"
                class="w-full h-8 text-xs"
              >
                <PlusIcon class="h-3.5 w-3.5 mr-1.5" />
                {{ $t('dropdown.addOption') }}
              </Button>

              <!-- 快捷填充 -->
              <div class="pt-2 border-t">
                <p class="text-xs text-muted-foreground mb-1.5">{{ $t('dropdown.quickFill') }}</p>
                <div class="flex flex-wrap gap-1.5">
                  <Button
                    variant="secondary"
                    size="sm"
                    @click="quickFill(['true', 'false'])"
                    class="h-7 text-xs"
                  >
                    {{ $t('dropdown.boolean') }}
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    @click="quickFill(['0.001', '0.01', '0.1', '1.0'])"
                    class="h-7 text-xs"
                  >
                    {{ $t('dropdown.learningRate') }}
                  </Button>
                  <Button
                    variant="secondary"
                    size="sm"
                    @click="quickFill(['sgd', 'adam', 'adamw'])"
                    class="h-7 text-xs"
                  >
                    {{ $t('dropdown.optimizer') }}
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </template>
      </div>
      </ScrollArea>

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
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { ScrollArea } from "@/components/ui/scroll-area";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { PlusIcon, XIcon, TerminalIcon, SlidersIcon } from "lucide-vue-next";
import { useFormItems } from "@/composables/useFormItems";
import type { FormItem, ItemType } from "@/types/bindings";

/** 组件属性 */
const props = defineProps<{
  open: boolean;
  item: FormItem | null;
}>();

/** 定义事件 */
const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

/** 国际化 */
const { t } = useI18n();

/** 表单项操作 */
const { updateFormItem, updateDropdownOptions } = useFormItems();

/** 本地状态 */
const localItemType = ref<ItemType>("Parameter");
const localUseDropdown = ref(false);
const localDropdownOptions = ref<string[]>([]);

/** 监听 item 变化，同步到本地 */
watch(() => props.item, (newItem) => {
  if (newItem) {
    localItemType.value = newItem.item_type;
    localUseDropdown.value = newItem.use_dropdown;
    localDropdownOptions.value = [...newItem.dropdown_options];
  }
}, { immediate: true });

/**
 * 更新选项
 */
function updateOption(index: number, value: string) {
  localDropdownOptions.value[index] = value;
}

/**
 * 添加新选项
 */
function addOption() {
  localDropdownOptions.value.push("");
}

/**
 * 删除选项
 */
function removeOption(index: number) {
  if (localDropdownOptions.value.length > 1) {
    localDropdownOptions.value.splice(index, 1);
  }
}

/**
 * 快捷填充
 */
function quickFill(options: string[]) {
  localDropdownOptions.value = [...options];
}

/**
 * 保存所有设置
 */
async function handleSave() {
  if (!props.item) return;

  const itemId = props.item.id;

  // 1. 更新参数类型
  if (localItemType.value !== props.item.item_type) {
    await updateFormItem(itemId, "item_type", localItemType.value);
  }

  // 2. 如果是 Parameter 类型，更新下拉模式和选项
  if (localItemType.value === "Parameter") {
    // 更新下拉模式开关
    if (localUseDropdown.value !== props.item.use_dropdown) {
      await updateFormItem(itemId, "use_dropdown", localUseDropdown.value);
    }

    // 如果启用了下拉模式，更新选项
    if (localUseDropdown.value) {
      // 过滤空选项
      const options = localDropdownOptions.value.filter(opt => opt.trim() !== "");
      await updateDropdownOptions(itemId, options);
    }
  }

  // 关闭对话框
  emit("update:open", false);
}
</script>
