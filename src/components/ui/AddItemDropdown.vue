<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button :size="size" :variant="variant">
        <PlusIcon class="h-4 w-4 mr-1" />
        <slot>添加项</slot>
        <ChevronDownIcon class="h-4 w-4 ml-1" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuItem @click="$emit('add', 'Parameter')">
        <SlidersIcon class="h-4 w-4 mr-2" />
        添加参数
      </DropdownMenuItem>
      <DropdownMenuItem @click="$emit('add', 'Command')">
        <TerminalIcon class="h-4 w-4 mr-2" />
        添加命令
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
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { PlusIcon, TerminalIcon, SlidersIcon, ChevronDownIcon } from "lucide-vue-next";

defineProps<{
  size?: "sm" | "default";
  variant?: "default" | "outline" | "ghost";
}>();

defineEmits<{
  add: [itemType: "Command" | "Parameter"];
}>();
</script>
