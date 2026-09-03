<script setup lang="ts">
import { computed } from 'vue'

type BadgeVariant =
  | 'default'
  | 'primary'
  | 'secondary'
  | 'success'
  | 'warning'
  | 'danger'
  | 'outline'
  | 'neutral'

type BadgeSize = 'xs' | 'sm' | 'md'

interface Props {
  variant?: BadgeVariant
  size?: BadgeSize
  dot?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  size: 'xs',
  dot: false,
})

const variantClasses: Record<BadgeVariant, string> = {
  default: 'bg-primary/10 text-primary border-primary/20',
  primary: 'bg-primary/10 text-primary border-primary/20',
  secondary: 'bg-muted/80 text-foreground border-border',
  success: 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20',
  warning: 'bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20',
  danger: 'bg-destructive/10 text-destructive border-destructive/20',
  outline: 'bg-transparent text-foreground border-border',
  neutral: 'bg-neutral-500/10 text-neutral-600 dark:text-neutral-300 border-neutral-500/20',
}

const dotColors: Record<BadgeVariant, string> = {
  default: 'bg-primary',
  primary: 'bg-primary',
  secondary: 'bg-muted-foreground',
  success: 'bg-emerald-500',
  warning: 'bg-amber-500',
  danger: 'bg-destructive',
  outline: 'bg-foreground',
  neutral: 'bg-neutral-500',
}

const sizeClasses: Record<BadgeSize, string> = {
  xs: 'px-1.5 py-0.5 text-[10px] gap-1',
  sm: 'px-2 py-0.5 text-xs gap-1.5',
  md: 'px-2.5 py-1 text-xs gap-1.5 font-semibold',
}

const computedClasses = computed(() => [
  'inline-flex items-center font-medium rounded-full border border-solid select-none leading-none tracking-wide transition-colors',
  variantClasses[props.variant],
  sizeClasses[props.size],
])
</script>

<template>
  <span :class="computedClasses">
    <span
      v-if="dot"
      class="size-1.5 rounded-full shrink-0"
      :class="dotColors[variant]"
    />
    <slot name="prefix" />
    <slot />
    <slot name="suffix" />
  </span>
</template>
