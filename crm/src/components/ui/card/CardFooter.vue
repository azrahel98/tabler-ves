<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  variant?: 'default' | 'surface' | 'subtle' | 'none'
  borderTop?: boolean
  size?: 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  borderTop: true,
  size: 'md',
})

const sizeClasses = computed(() => {
  if (props.size === 'sm') return 'p-3 sm:p-4'
  if (props.size === 'lg') return 'p-5 sm:p-6'
  return 'py-3 px-4 sm:px-5'
})

const variantClasses = computed(() => {
  if (props.variant === 'surface' || props.variant === 'subtle') return 'bg-muted/30'
  return ''
})

const computedClasses = computed(() => [
  'flex items-center justify-between gap-3',
  props.borderTop ? 'border-t border-border' : '',
  sizeClasses.value,
  variantClasses.value,
])
</script>

<template>
  <div :class="computedClasses">
    <slot />
  </div>
</template>
