<template>
  <div v-if="fullPage" class="fixed inset-0 z-[9999] flex flex-col items-center justify-center bg-white/40 dark:bg-gray-950/45 backdrop-blur-md">
    <div class="relative flex flex-col items-center">
      <!-- Glow effect in the background -->
      <div class="absolute -inset-6 rounded-full bg-brand-500/10 blur-2xl dark:bg-brand-500/20"></div>
      
      <!-- Inner glowing spinner layout -->
      <div class="relative flex items-center justify-center">
        <!-- Dual ring loader with different speeds -->
        <div class="animate-spin rounded-full border-4 border-brand-500/10 border-t-brand-500" :class="sizeClass"></div>
        <div class="absolute animate-spin rounded-full border-4 border-transparent border-b-secondary/40" :class="secondarySpinnerClass" style="animation-direction: reverse; animation-duration: 1s;"></div>
      </div>
      
      <span class="mt-6 text-sm font-semibold tracking-wide text-gray-700 dark:text-gray-300 animate-pulse">
        {{ message || 'Cargando datos...' }}
      </span>
    </div>
  </div>

  <div v-else class="flex items-center justify-center" :class="containerClass">
    <div class="relative flex items-center justify-center">
      <div class="animate-spin rounded-full border-2 border-brand-500/10 border-t-brand-500" :class="sizeClass"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'

  const props = withDefaults(
    defineProps<{
      size?: 'xs' | 'sm' | 'md' | 'lg'
      fullPage?: boolean
      message?: string
    }>(),
    {
      size: 'md',
      fullPage: false,
      message: '',
    }
  )

  const sizeClass = computed(() => {
    const sizes: Record<string, string> = {
      xs: 'h-4 w-4 border-2',
      sm: 'h-6 w-6 border-2',
      md: 'h-8 w-8 border-[3px]',
      lg: 'h-12 w-12 border-4',
    }
    return sizes[props.size]
  })

  const secondarySpinnerClass = computed(() => {
    const sizes: Record<string, string> = {
      xs: 'h-2 w-2 border-2',
      sm: 'h-4 w-4 border-2',
      md: 'h-6 w-6 border-[3px]',
      lg: 'h-9 w-9 border-4',
    }
    return sizes[props.size]
  })

  const containerClass = computed(() => {
    return props.fullPage ? 'py-20' : ''
  })
</script>
