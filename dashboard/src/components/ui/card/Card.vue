<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  as?: string
  title?: string
  description?: string
  hoverable?: boolean
  noPadding?: boolean
  topBorder?: boolean | 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'info'
  horizontal?: boolean
  headerClass?: string
  bodyClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  as: 'div',
  title: undefined,
  description: undefined,
  hoverable: false,
  noPadding: false,
  topBorder: false,
  horizontal: false,
  headerClass: '',
  bodyClass: '',
})

const topBorderClass = computed(() => {
  if (!props.topBorder) return ''
  if (props.topBorder === true || props.topBorder === 'primary') return 'border-t-4 border-t-primary'
  if (props.topBorder === 'success') return 'border-t-4 border-t-emerald-600 dark:border-t-emerald-500'
  if (props.topBorder === 'danger') return 'border-t-4 border-t-rose-600 dark:border-t-rose-500'
  if (props.topBorder === 'warning') return 'border-t-4 border-t-amber-500'
  if (props.topBorder === 'info') return 'border-t-4 border-t-sky-500'
  if (props.topBorder === 'secondary') return 'border-t-4 border-t-muted-foreground'
  return 'border-t-4 border-t-primary'
})

const cardClasses = computed(() => [
  'bg-card text-foreground rounded-xl border border-border shadow-xs overflow-hidden transition-all duration-200',
  props.horizontal ? 'sm:flex' : 'flex flex-col',
  props.hoverable ? 'hover:shadow-md hover:border-primary/50 cursor-pointer group' : '',
  props.noPadding ? '' : 'p-5',
  topBorderClass.value,
])
</script>

<template>
  <component :is="props.as" :class="cardClasses">
    <div
      v-if="props.title || props.description || $slots.header || $slots.action"
      :class="[
        'flex items-center justify-between border-b border-border pb-3 mb-3.5 flex-wrap gap-2',
        props.headerClass,
      ]"
    >
      <slot name="header">
        <div class="space-y-0.5">
          <h3 v-if="props.title" class="font-semibold text-foreground tracking-tight text-sm">
            {{ props.title }}
          </h3>
          <p v-if="props.description" class="text-[11px] text-muted-foreground">
            {{ props.description }}
          </p>
        </div>
      </slot>
      <div v-if="$slots.action" class="shrink-0 flex items-center gap-2">
        <slot name="action" />
      </div>
    </div>
    <div v-if="props.bodyClass" :class="props.bodyClass">
      <slot />
    </div>
    <slot v-else />
  </component>
</template>
