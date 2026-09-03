<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Sidebar from '@/components/Sidebar.vue'
import Navbar from '@/components/Navbar.vue'

const isSidebarOpen = ref(false)
const isSidebarCollapsed = ref(false)

onMounted(() => {
  const savedState = localStorage.getItem('crm_sidebar_collapsed')
  if (savedState !== null) {
    isSidebarCollapsed.value = savedState === 'true'
  }
})

const toggleCollapse = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
  localStorage.setItem('crm_sidebar_collapsed', String(isSidebarCollapsed.value))
}
</script>

<template>
  <div class="min-h-screen bg-background-1 text-foreground flex flex-col">
    <Sidebar :is-open="isSidebarOpen" :is-collapsed="isSidebarCollapsed" @close="isSidebarOpen = false"
      @toggle-collapse="toggleCollapse" />

    <div class="flex-1 flex flex-col min-w-0 transition-all duration-300 ease-in-out"
      :class="isSidebarCollapsed ? 'lg:ps-20' : 'lg:ps-64'">
      <Navbar @toggle-sidebar="isSidebarOpen = !isSidebarOpen" @toggle-collapse="toggleCollapse" />

      <main class="flex-1 p-4 pb-0 sm:p-6 lg:p-8 max-w-7xl w-full mx-auto">
        <router-view />
      </main>
    </div>
  </div>
</template>
