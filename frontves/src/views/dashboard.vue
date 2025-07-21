<script setup lang="ts">
import Sidebar from '@comp/sidebar/sidebar.vue'
</script>

<template>
  <div class="layout">
    <Sidebar class="sidebar" />
    <div class="main-content">
      <RouterView />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.layout {
  display: flex;
  flex-direction: column;
  height: 100vh;

  .sidebar {
    flex-shrink: 0;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    height: 100vh;
    padding: 1rem;
  }
}
</style>
