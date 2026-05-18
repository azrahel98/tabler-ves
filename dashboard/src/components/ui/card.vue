<template>
  <div class="rounded-2xl border border-gray-100 bg-card flex flex-col p-5 dark:border-white/6 dark:bg-white/3">
    <div v-if="title || $slots.icon || $slots.action" class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-4 shrink-0">
      <div class="flex items-center gap-2">
        <slot name="icon" />
        {{ title }}
      </div>
      <div class="flex items-center gap-1">
        <slot name="action" />
      </div>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>
