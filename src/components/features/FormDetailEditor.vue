<template>
  <div class="form-detail-editor h-full flex flex-col overflow-hidden">
    <!-- 整体滚动容器 -->
    <div class="flex-1 overflow-y-auto">
      <!-- 表单头部 -->
      <div class="p-6">
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <CardTitle class="text-xl">{{ form.name }}</CardTitle>
            <CardDescription v-if="form.description" class="mt-1">
              {{ form.description }}
            </CardDescription>
          </div>
          <Button variant="ghost" size="sm" @click="$emit('edit')">
            <EditIcon class="h-4 w-4" />
          </Button>
        </div>
      </div>

      <!-- 分隔线 -->
      <Separator />

      <!-- 命令预览 -->
      <div>
        <div class="px-6 py-4">
          <div class="flex items-center justify-between">
            <CardTitle class="text-sm font-medium">{{ $t('command.title') }}</CardTitle>

            <div class="flex items-center gap-2">
              <!-- 格式切换按钮 -->
              <Button
                variant="ghost"
                size="sm"
                @click="toggleFormat"
                class="h-7 px-2"
              >
                <ListIcon class="h-3.5 w-3.5 mr-1" />
                {{ currentFormatLabel }}
              </Button>

              <!-- 复制按钮 -->
              <Button
                variant="ghost"
                size="sm"
                @click="copyCommand"
                :disabled="!commandPreview"
                class="h-7 px-2"
              >
                <CopyIcon class="h-3.5 w-3.5 mr-1" />
                {{ $t('command.copy') }}
              </Button>
            </div>
          </div>
          <div class="mt-3">
            <code
              v-if="commandPreview"
              class="block bg-muted p-3 text-sm font-mono break-all rounded-md whitespace-pre-wrap"
              v-text="commandPreview"
            />
            <div
              v-else
              class="text-sm text-muted-foreground text-center py-2"
            >
              {{ $t('command.emptyHint') }}
            </div>
          </div>
        </div>
      </div>

      <!-- 分隔线 -->
      <Separator />

      <!-- 参数列表 -->
      <div class="flex flex-col">
        <div class="px-6 py-3 bg-card">
          <div class="flex items-center justify-between">
            <h3 class="text-sm font-medium">
              {{ $t('formItem.title') }} ({{ form.items.length }})
            </h3>
            <DropdownMenu>
              <DropdownMenuTrigger as-child>
                <Button size="sm">
                  <PlusIcon class="h-4 w-4 mr-1" />
                  {{ $t('formItem.addItem') }}
                  <ChevronDownIcon class="h-4 w-4 ml-1" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent>
                <DropdownMenuItem @click="handleAddItem('Parameter')">
                  <SlidersIcon class="h-4 w-4 mr-2" />
                  {{ $t('formItem.addParameter') }}
                </DropdownMenuItem>
                <DropdownMenuItem @click="handleAddItem('Command')">
                  <TerminalIcon class="h-4 w-4 mr-2" />
                  {{ $t('formItem.addCommand') }}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>

        <div class="px-4 pt-4 pb-2 space-y-3">
          <!-- 拖拽区域 -->
          <div ref="listRef" class="space-y-3 pb-4">
            <FormItemEditor
              v-for="item in form.items"
              :key="item.id"
              :item="item"
              @open-dropdown-options="handleOpenDropdownOptions"
              @delete="handleDeleteItem"
            />
          </div>

          <!-- 空状态 -->
          <div
            v-if="form.items.length === 0"
            class="text-center py-12 text-muted-foreground"
          >
            <div class="flex flex-col items-center gap-3">
              <div class="p-3 bg-muted rounded-full">
                <PlusIcon class="h-6 w-6" />
              </div>
              <p class="text-sm">{{ $t('formItem.noItems') }}</p>
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button variant="outline" size="sm">
                    <PlusIcon class="h-4 w-4 mr-1" />
                    {{ $t('formItem.addFirst') }}
                    <ChevronDownIcon class="h-4 w-4 ml-1" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                  <DropdownMenuItem @click="handleAddItem('Parameter')">
                    <SlidersIcon class="h-4 w-4 mr-2" />
                    {{ $t('formItem.addParameter') }}
                  </DropdownMenuItem>
                  <DropdownMenuItem @click="handleAddItem('Command')">
                    <TerminalIcon class="h-4 w-4 mr-2" />
                    {{ $t('formItem.addCommand') }}
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 下拉选项对话框 -->
    <DropdownOptionsDialog
      v-model:open="showDropdownDialog"
      :item="selectedItem"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { useI18n } from "vue-i18n";
import Sortable from "sortablejs";
import { CardTitle, CardDescription } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { EditIcon, CopyIcon, PlusIcon, TerminalIcon, SlidersIcon, ChevronDownIcon, ListIcon } from "lucide-vue-next";
import { useCommandPreview } from "@/composables/useCommandPreview";
import { useFormItems } from "@/composables/useFormItems";
import { useUiStore } from "@/stores/ui";
import { useProjectStore } from "@/stores/project";
import { useFormStore } from "@/stores/form";
import FormItemEditor from "./FormItemEditor.vue";
import DropdownOptionsDialog from "./DropdownOptionsDialog.vue";
import type { CommandFormat, Form, FormItem } from "@/types/bindings";

const { t } = useI18n();

/** 组件属性 */
const props = defineProps<{
  form: Form;
}>();

/** 定义事件 */
defineEmits<{
  edit: [];
}>();

/** 表单项操作 */
const { addFormItem, deleteFormItem, reorderFormItems } = useFormItems();

/** 命令预览 */
const { commandPreview } = useCommandPreview();

/** UI 状态管理 */
const uiStore = useUiStore();

/** 项目和表单状态管理 */
const projectStore = useProjectStore();
const formStore = useFormStore();

/** 当前格式标签 */
const currentFormatLabel = computed(() => {
  if (!props.form) return "";
  return props.form.command_format === "SingleLine" ? "单行" : "多行";
});

/**
 * 切换命令格式
 */
async function toggleFormat() {
  if (!props.form || !projectStore.currentProject) return;

  const newFormat: CommandFormat =
    props.form.command_format === "SingleLine" ? "MultiLine" : "SingleLine";

  try {
    await formStore.updateCommandFormat(
      projectStore.currentProject.id,
      newFormat
    );
  } catch (error) {
    console.error("切换格式失败:", error);
    uiStore.showToast("切换格式失败", "error");
  }
}

/** 列表 DOM 引用 */
const listRef = ref<HTMLElement | null>(null);

/** Sortable 实例 */
let sortableInstance: Sortable | null = null;

/** 是否显示下拉选项对话框 */
const showDropdownDialog = ref(false);

/** 选中的表单项 (用于下拉选项) */
const selectedItem = ref<FormItem | null>(null);

/**
 * 初始化拖拽排序
 */
onMounted(() => {
  if (!listRef.value) {
    return;
  }

  sortableInstance = Sortable.create(listRef.value, {
    handle: ".drag-handle",

    // 动画配置
    animation: 200,
    easing: "cubic-bezier(0.2, 0, 0.2, 1)",

    // 交换配置
    swapThreshold: 0.5,
    invertSwap: false,

    // 样式类名
    ghostClass: "sortable-ghost",
    dragClass: "sortable-drag",
    fallbackClass: "sortable-fallback",

    // 滚动配置
    scroll: true,
    bubbleScroll: true,

    // Fallback 配置
    forceFallback: true,
    fallbackOnBody: true,
    fallbackTolerance: 5,
    removeCloneOnHide: false,

    onEnd: async (evt) => {
      const { oldIndex, newIndex } = evt;
      if (oldIndex === undefined || newIndex === undefined || oldIndex === newIndex) {
        return;
      }

      await reorderFormItems(oldIndex, newIndex);
    },
  });
});

/**
 * 销毁拖拽实例
 */
onBeforeUnmount(() => {
  if (sortableInstance) {
    sortableInstance.destroy();
    sortableInstance = null;
  }
});

/**
 * 添加新表单项
 * @param itemType - 表单项类型（"Command" 或 "Parameter"）
 */
async function handleAddItem(itemType: "Command" | "Parameter" = "Parameter") {
  await addFormItem(itemType);
}

/**
 * 删除参数
 */
async function handleDeleteItem(itemId: string) {
  await deleteFormItem(itemId);
}

/**
 * 打开下拉选项对话框
 */
function handleOpenDropdownOptions(item: FormItem) {
  selectedItem.value = item;
  showDropdownDialog.value = true;
}

/**
 * 复制命令到剪贴板
 */
async function copyCommand() {
  if (!commandPreview.value) return;

  try {
    await navigator.clipboard.writeText(commandPreview.value);
    uiStore.showToast(t('command.copySuccess'), "success");
  } catch (error) {
    console.error("复制失败:", error);
    uiStore.showToast(t('command.copyFailed'), "error");
  }
}
</script>

<style scoped>
/* 拖拽时的占位符样式（原位置） */
.sortable-ghost {
  opacity: 0.3;
  background-color: hsl(var(--accent) / 0.5);
  border: 2px dashed hsl(var(--primary) / 0.6);
  transform: scale(0.95);
  transition: all 200ms cubic-bezier(0.2, 0, 0.2, 1) !important;
}

/* 正在被拖拽的元素样式 */
.sortable-drag {
  opacity: 1;
  box-shadow: 0 20px 50px -12px rgba(0, 0, 0, 0.4);
  transform: scale(1.05);
  cursor: grabbing;
  z-index: 1000;
  transition: none !important;
}

/* Fallback 拖拽样式（跨浏览器兼容） */
.sortable-fallback {
  opacity: 0.95;
  background-color: hsl(var(--card));
  box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.5);
  border-radius: 0.5rem;
  cursor: grabbing;
  transform: scale(1.05);
}

/* 所有可拖拽项添加过渡效果 */
:deep(.group) {
  transition: transform 200ms cubic-bezier(0.2, 0, 0.2, 1) !important,
              box-shadow 200ms ease,
              border-color 200ms ease,
              background-color 200ms ease !important;
  will-change: transform;
}

/* 拖拽时移除过渡（跟随鼠标） */
:deep(.sortable-drag) {
  transition: none !important;
}

/* 占位符保留过渡 */
:deep(.sortable-ghost) {
  transition: all 200ms cubic-bezier(0.2, 0, 0.2, 1) !important;
}
</style>
