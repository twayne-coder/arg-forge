<template>
  <div class="form-detail-preview h-full flex flex-col">
    <!-- 表单头部 -->
    <Card class="border-b rounded-b-none">
      <CardHeader>
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <CardTitle class="text-xl">{{ form.name }}</CardTitle>
            <CardDescription v-if="form.description" class="mt-1">
              {{ form.description }}
            </CardDescription>
            <p class="text-xs text-muted-foreground mt-2">
              更新于 {{ formatTime(form.updated_at) }}
            </p>
          </div>
          <Button variant="ghost" size="sm" @click="$emit('edit')">
            <EditIcon class="h-4 w-4" />
          </Button>
        </div>
      </CardHeader>
    </Card>

    <!-- 内容区 -->
    <ScrollArea class="flex-1">
      <div class="p-6 space-y-6">
        <!-- 表单项列表 -->
        <Card>
          <CardHeader>
            <CardTitle class="text-sm font-medium">
              表单项 ({{ form.items.length }})
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div v-if="form.items.length === 0" class="text-center py-8 text-muted-foreground">
              暂无表单项
            </div>
            <div v-else class="space-y-3">
              <div
                v-for="item in form.items"
                :key="item.id"
                :class="[
                  'rounded border p-3',
                  !item.enabled && 'opacity-50',
                  item.item_type === 'Command' && 'bg-blue-50 dark:bg-blue-950/20 border-blue-200 dark:border-blue-800'
                ]"
              >
                <div class="flex items-center justify-between">
                  <div class="flex-1 min-w-0">
                    <!-- 命令项 -->
                    <template v-if="item.item_type === 'Command'">
                      <div class="flex items-center gap-2">
                        <TerminalIcon class="h-4 w-4 text-blue-600" />
                        <span
                          :class="[
                            'font-mono text-sm font-medium',
                            item.enabled ? 'text-foreground' : 'text-muted-foreground'
                          ]"
                        >
                          {{ item.content || '(空命令)' }}
                        </span>
                      </div>
                      <div
                        v-if="item.use_dropdown && item.dropdown_options.length > 0"
                        class="text-xs text-muted-foreground mt-1"
                      >
                        可选值: {{ item.dropdown_options.join(", ") }}
                      </div>
                    </template>

                    <!-- 参数项 -->
                    <template v-else>
                      <div class="flex items-center gap-2">
                        <SlidersIcon class="h-4 w-4" />
                        <span
                          v-if="item.param_style !== 'ValueOnly'"
                          :class="[
                            'font-mono text-sm',
                            item.enabled ? 'text-foreground' : 'text-muted-foreground'
                          ]"
                        >
                          {{ item.param_name || '(未命名)' }}
                        </span>
                        <Badge
                          :variant="getParamStyleVariant(item.param_style)"
                          class="text-xs"
                        >
                          {{ getParamStyleLabel(item.param_style) }}
                        </Badge>
                      </div>
                      <div
                        v-if="item.content"
                        class="text-sm text-muted-foreground mt-1"
                      >
                        值: {{ item.content }}
                      </div>
                      <div
                        v-if="item.use_dropdown && item.dropdown_options.length > 0"
                        class="text-xs text-muted-foreground mt-1"
                      >
                        可选值: {{ item.dropdown_options.join(", ") }}
                      </div>
                    </template>
                  </div>
                  <div class="ml-2">
                    <Switch :checked="item.enabled" disabled class="pointer-events-none" />
                  </div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  </div>
</template>

<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { EditIcon, TerminalIcon, SlidersIcon } from "lucide-vue-next";
import type { Form, ParamStyle } from "@/types/bindings";

/** 组件属性 */
defineProps<{
  form: Form;
}>();

/** 定义事件 */
defineEmits<{
  edit: [];
}>();

/** 格式化时间 */
function formatTime(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  });
}

/** 获取参数风格标签 */
function getParamStyleLabel(style: ParamStyle): string {
  switch (style) {
    case "KeyValue":
      return "--key value";
    case "EqualValue":
      return "key=value";
    case "ValueOnly":
      return "仅值";
    default:
      return "未知";
  }
}

/** 获取参数风格 Badge 变体 */
function getParamStyleVariant(style: ParamStyle): "default" | "secondary" {
  switch (style) {
    case "KeyValue":
      return "default";
    case "EqualValue":
      return "secondary";
    case "ValueOnly":
      return "secondary";
    default:
      return "secondary";
  }
}
</script>
