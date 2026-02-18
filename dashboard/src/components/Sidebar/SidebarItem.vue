<template>
  <li>
    <router-link
      :to="to"
      class="group relative flex items-center gap-2.5 rounded-sm py-2 px-4 font-medium text-gray-500 duration-300 ease-in-out hover:bg-gray-100 dark:hover:bg-meta-4 dark:text-gray-400 dark:hover:text-white"
      active-class="bg-gray-100 dark:bg-meta-4 text-black dark:text-white"
    >
      <slot></slot>
      {{ label }}
    </router-link>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  to: string;
  label: string;
}>();
</script>
