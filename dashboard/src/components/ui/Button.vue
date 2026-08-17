<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    class="inline-flex items-center justify-center gap-2 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:ring-4 focus-visible:ring-brand-500/12"
    :class="[variantClasses, sizeClasses, customClass]"
  >
    <svg v-if="loading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
    <slot name="icon-left" v-if="!loading" />
    <slot />
    <slot name="icon-right" />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  type?: 'button' | 'submit' | 'reset';
  disabled?: boolean;
  loading?: boolean;
  customClass?: string;
}>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  disabled: false,
  loading: false,
  customClass: '',
});

const variantClasses = computed(() => {
  switch (props.variant) {
    case 'primary':
      return 'bg-brand-500 text-white hover:bg-brand-600 border border-transparent shadow-sm';
    case 'secondary':
      return 'bg-brand-50 text-brand-700 hover:bg-brand-100 border border-transparent dark:bg-brand-500/10 dark:text-brand-300 dark:hover:bg-brand-500/20';
    case 'outline':
      return 'bg-transparent text-gray-700 border border-gray-300 hover:bg-gray-50 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-800';
    case 'ghost':
      return 'bg-transparent text-gray-700 hover:bg-gray-100 border border-transparent dark:text-gray-300 dark:hover:bg-gray-800';
    case 'danger':
      return 'bg-error-500 text-white hover:bg-error-600 border border-transparent shadow-sm';
    default:
      return 'bg-brand-500 text-white hover:bg-brand-600 border border-transparent';
  }
});

const sizeClasses = computed(() => {
  switch (props.size) {
    case 'sm':
      return 'px-3 py-1.5 text-xs';
    case 'md':
      return 'px-5 py-2.5 text-sm';
    case 'lg':
      return 'px-6 py-3 text-base';
    default:
      return 'px-5 py-2.5 text-sm';
  }
});
</script>
