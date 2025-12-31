<template>
  <div class="flex h-screen overflow-hidden bg-[#F5F7FA]">
    <div v-if="isSidebarOpen" class="fixed inset-0 bg-black/50 z-40 md:hidden" @click="closeSidebar"></div>

    <Sidebar :is-open="isSidebarOpen" @close="closeSidebar" />

    <div class="flex-1 flex flex-col overflow-hidden w-full">
      <Header @toggle-sidebar="toggleSidebar" />
      <main class="flex-1 overflow-y-auto px-8">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import Sidebar from '@comp/layout/sidebar.vue'
import Header from '@comp/layout/header.vue'
const isSidebarOpen = ref(false)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}

const closeSidebar = () => {
  isSidebarOpen.value = false
}
</script>
