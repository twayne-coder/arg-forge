<template>
  <DropdownMenu>
    <DropdownMenuTrigger>
      <Button
        variant="ghost"
        size="icon"
        class="opacity-0 group-hover:opacity-100 transition-opacity"
      >
        <MoreVerticalIcon class="h-4 w-4" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuItem @click="handleEdit">
        <PencilIcon class="mr-2 h-4 w-4" />
        {{ $t('common.edit') }}
      </DropdownMenuItem>
      <DropdownMenuItem @click="handleDuplicate">
        <CopyIcon class="mr-2 h-4 w-4" />
        {{ $t('common.duplicate') }}
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem @click="handleDelete" class="text-destructive">
        <TrashIcon class="mr-2 h-4 w-4" />
        {{ $t('common.delete') }}
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  MoreVerticalIcon,
  PencilIcon,
  CopyIcon,
  TrashIcon,
} from "lucide-vue-next";
import type { Project } from "@/types/bindings";

/** 项目对象 */
const props = defineProps<{
  project: Project;
}>();

/** 事件 */
const emit = defineEmits<{
  (e: "edit", project: Project): void;
  (e: "duplicate", project: Project): void;
  (e: "delete", project: Project): void;
}>();

/** 编辑 */
function handleEdit() {
  emit("edit", props.project);
}

/** 复制 */
function handleDuplicate() {
  emit("duplicate", props.project);
}

/** 删除 */
function handleDelete() {
  emit("delete", props.project);
}
</script>
