<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
    <div class="flex items-start justify-between">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">
        {{ title }}
      </h3>
    </div>

    <div class="mt-2 max-h-[calc(70vh-150px)] overflow-y-auto">
      <div class="flex items-center justify-between border-b border-gray-100 pb-2 dark:border-gray-800">
        <span class="text-theme-xs text-gray-400" v-for="x in columns">
          {{ x }}
        </span>
      </div>

      <slot />
    </div>
  </div>
</template>

<script lang="ts" setup>
  defineProps({
    title: {
      type: String,
      default: '',
    },
    columns: {
      type: Array,
      default: () => [],
    },
    data: {
      type: Array,
      default: () => [],
    },
    uri: {
      type: String,
    },
  })
</script>
