<template>
  <div class="flex items-center justify-center" :class="containerClass">
    <div class="animate-spin rounded-full border-2 border-brand-500/20 border-t-brand-500" :class="sizeClass"></div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'

  const props = withDefaults(
    defineProps<{
      size?: 'xs' | 'sm' | 'md' | 'lg'
      fullPage?: boolean
    }>(),
    {
      size: 'md',
      fullPage: false,
    }
  )

  const sizeClass = computed(() => {
    const sizes: Record<string, string> = {
      xs: 'h-4 w-4',
      sm: 'h-6 w-6',
      md: 'h-8 w-8',
      lg: 'h-12 w-12',
    }
    return sizes[props.size]
  })

  const containerClass = computed(() => {
    return props.fullPage ? 'py-20' : ''
  })
</script>
