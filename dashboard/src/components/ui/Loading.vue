<template>
  <div
    v-if="fullPage"
    class="fixed inset-0 z-[9999] flex items-center justify-center"
    style="background: rgba(249, 249, 255, 0.92);"
  >
    <div class="flex flex-col items-center gap-3">
      <div
        class="animate-spin rounded-full border-brand-500/20 border-t-brand-500"
        :class="sizeClass"
      ></div>
      <span style="font-size: 0.8125rem; color: #6b7280; font-weight: 400;">
        {{ message || 'Cargando datos...' }}
      </span>
    </div>
  </div>

  <div v-else class="flex items-center justify-center">
    <div
      class="animate-spin rounded-full border-brand-500/20 border-t-brand-500"
      :class="sizeClass"
    ></div>
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
  const map: Record<string, string> = {
    xs: 'h-3 w-3 border-2',
    sm: 'h-5 w-5 border-2',
    md: 'h-7 w-7 border-2',
    lg: 'h-10 w-10 border-[3px]',
  }
  return map[props.size]
})
</script>
