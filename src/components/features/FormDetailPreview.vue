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
        <!-- 命令模板 -->
        <Card v-if="form.command_template">
          <CardHeader>
            <CardTitle class="text-sm font-medium">命令模板</CardTitle>
          </CardHeader>
          <CardContent>
            <code class="block bg-muted p-3 rounded text-sm font-mono">
              {{ form.command_template }}
            </code>
          </CardContent>
        </Card>

        <!-- 表单项列表 -->
        <Card>
          <CardHeader>
            <CardTitle class="text-sm font-medium">
              参数项 ({{ form.items.length }})
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div v-if="form.items.length === 0" class="text-center py-8 text-muted-foreground">
              暂无参数项
            </div>
            <div v-else class="space-y-3">
              <div
                v-for="item in form.items"
                :key="item.id"
                :class="[
                  'rounded border p-3',
                  !item.enabled && 'opacity-50'
                ]"
              >
                <div class="flex items-center justify-between">
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                      <span
                        :class="[
                          'font-mono text-sm',
                          item.enabled ? 'text-foreground' : 'text-muted-foreground'
                        ]"
                      >
                        {{ item.param_name }}
                      </span>
                      <Badge
                        :variant="getParamStyleVariant(item.param_style)"
                        class="text-xs"
                      >
                        {{ getParamStyleLabel(item.param_style) }}
                      </Badge>
                    </div>
                    <div
                      v-if="item.param_value"
                      class="text-sm text-muted-foreground mt-1 truncate"
                    >
                      值: {{ item.param_value }}
                    </div>
                    <div
                      v-if="item.use_dropdown && item.dropdown_options.length > 0"
                      class="text-xs text-muted-foreground mt-1"
                    >
                      可选值: {{ item.dropdown_options.join(", ") }}
                    </div>
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
import { EditIcon } from "lucide-vue-next";
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
    case "Argparse":
      return "Argparse";
    case "Hydra":
      return "Hydra";
    case "Positional":
      return "位置参数";
    default:
      return "未知";
  }
}

/** 获取参数风格 Badge 变体 */
function getParamStyleVariant(style: ParamStyle): "default" | "secondary" {
  switch (style) {
    case "Argparse":
      return "default";
    case "Hydra":
      return "secondary";
    case "Positional":
      return "secondary";
    default:
      return "secondary";
  }
}
</script>
