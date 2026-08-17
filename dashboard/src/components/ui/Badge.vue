<template>
  <span
    class="inline-flex items-center justify-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium uppercase tracking-wide"
    :class="[variantClasses, customClass]"
  >
    <slot name="icon" />
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  variant?: 'brand' | 'success' | 'warning' | 'error' | 'info' | 'gray';
  customClass?: string;
}>(), {
  variant: 'brand',
  customClass: '',
});

const variantClasses = computed(() => {
  switch (props.variant) {
    case 'brand':
      return 'bg-brand-50 text-brand-600 dark:bg-brand-500/10 dark:text-brand-400';
    case 'success':
      return 'bg-success-50 text-success-600 dark:bg-success-500/10 dark:text-success-400';
    case 'warning':
      return 'bg-warning-50 text-warning-600 dark:bg-warning-500/10 dark:text-warning-400';
    case 'error':
      return 'bg-error-50 text-error-600 dark:bg-error-500/10 dark:text-error-400';
    case 'info':
      return 'bg-blue-light-50 text-blue-light-600 dark:bg-blue-light-500/10 dark:text-blue-light-400';
    case 'gray':
      return 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400';
    default:
      return 'bg-brand-50 text-brand-600 dark:bg-brand-500/10 dark:text-brand-400';
  }
});
</script>
