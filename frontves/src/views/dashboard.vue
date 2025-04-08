<template>
  <div class="app-container">
    <Sidebar :is-expanded="isExpanded" :toggle="togglesidebar" />
    <div class="main-content" @click="handleOutsideClick">
      <div class="page-body p-0 m-0">
        <div class="container-fluid">
          <router-view> </router-view>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import Sidebar from '@comp/sidebar.vue'
import { ref } from 'vue'

const isExpanded = ref(false)

const togglesidebar = () => {
  isExpanded.value = !isExpanded.value
}
const handleOutsideClick = (_event: MouseEvent) => {
  isExpanded.value = false
}
</script>

<style lang="scss" scoped>
.app-container {
  display: flex;
  height: 100vh;
  max-width: 100vw;
}
.main-content {
  flex: 1;
  padding-left: 60px;
  background-color: #f4f6fa;
  transition: margin-left 0.3s ease;
  width: 100%;
}
</style>
