<template>
  <!-- Preset: Table Skeleton -->
  <div v-if="preset === 'table'" class="w-full space-y-4">
    <div class="space-y-3">
      <div v-for="r in rows" :key="r" class="flex items-center gap-4 justify-between py-3 border-b border-gray-100 dark:border-white/5 last:border-b-0 animate-pulse">
        <div class="flex items-center gap-3 w-1/3 shrink-0">
          <div v-if="showAvatarInTable" class="h-8 w-8 rounded-full bg-gray-250 dark:bg-gray-800 animate-shimmer shrink-0"></div>
          <div class="h-4 w-3/4 rounded-md bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        </div>
        <div class="h-4 w-1/4 rounded-md bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        <div class="h-4 w-1/6 rounded-md bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        <div class="h-8 w-12 rounded-lg bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
      </div>
    </div>
  </div>

  <!-- Preset: Card Skeleton -->
  <div v-else-if="preset === 'card'" class="rounded-xl border border-gray-100 dark:border-white/6 bg-card dark:bg-white/3 p-5 space-y-4">
    <div class="flex items-center gap-3">
      <div class="h-12 w-12 rounded-full bg-gray-200 dark:bg-gray-800 animate-shimmer shrink-0"></div>
      <div class="flex-1 space-y-2">
        <div class="h-4 w-2/3 rounded-md bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        <div class="h-3 w-1/2 rounded-md bg-gray-200/60 dark:bg-gray-800/60 animate-shimmer"></div>
      </div>
    </div>
    <div class="space-y-2 pt-2">
      <div class="h-3 w-full rounded bg-gray-200/50 dark:bg-gray-800/50 animate-shimmer"></div>
      <div class="h-3 w-5/6 rounded bg-gray-200/50 dark:bg-gray-800/50 animate-shimmer"></div>
    </div>
  </div>

  <!-- Preset: List Skeleton -->
  <div v-else-if="preset === 'list'" class="space-y-3">
    <div v-for="l in rows" :key="l" class="flex items-center gap-3 p-3 rounded-lg border border-gray-100 dark:border-white/5 bg-card dark:bg-white/3">
      <div class="h-8 w-8 rounded-full bg-gray-200 dark:bg-gray-800 animate-shimmer shrink-0"></div>
      <div class="flex-1 space-y-2">
        <div class="h-3.5 w-1/3 rounded-md bg-gray-200 dark:bg-gray-800 animate-shimmer"></div>
        <div class="h-2.5 w-1/4 rounded-md bg-gray-200/60 dark:bg-gray-800/60 animate-shimmer"></div>
      </div>
    </div>
  </div>

  <!-- Base Custom Skeleton (Line or Shape) -->
  <div
    v-else
    :class="[
      'animate-shimmer',
      variantClass,
      customClass
    ]"
    :style="styleObject"
  ></div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'

  const props = withDefaults(
    defineProps<{
      preset?: 'table' | 'card' | 'list' | 'custom'
      variant?: 'circle' | 'rounded' | 'rectangle'
      width?: string
      height?: string
      rows?: number
      showAvatarInTable?: boolean
      customClass?: string
    }>(),
    {
      preset: 'custom',
      variant: 'rounded',
      width: '100%',
      height: '1rem',
      rows: 5,
      showAvatarInTable: false,
      customClass: '',
    }
  )

  const variantClass = computed(() => {
    switch (props.variant) {
      case 'circle':
        return 'rounded-full bg-gray-200 dark:bg-gray-800'
      case 'rectangle':
        return 'rounded-none bg-gray-200 dark:bg-gray-800'
      case 'rounded':
      default:
        return 'rounded-lg bg-gray-200 dark:bg-gray-800'
    }
  })

  const styleObject = computed(() => {
    if (props.preset !== 'custom') return {}
    return {
      width: props.width,
      height: props.height,
    }
  })
</script>
