<script setup lang="ts">
import { useThemeStore } from '@/stores/theme'

const themeStore = useThemeStore()
themeStore.initTheme()
</script>

<template>
  <router-view />
</template>
