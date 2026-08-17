<script setup lang="ts">
import Navbar from '@/components/layout/nav/Navbar.vue'

const emit = defineEmits<{
  (e: 'toggleSidebar'): void
}>()
</script>

<template>
  <Navbar @toggle-sidebar="emit('toggleSidebar')" />
</template>
