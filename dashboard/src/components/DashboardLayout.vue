<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import Sidebar from '@/components/Sidebar.vue'
import Navbar from '@/components/Navbar.vue'
import NotificationToast from '@/components/NotificationToast.vue'
import { useNotificacionesStore } from '@/stores/notificaciones'

const isSidebarOpen = ref(false)
const isSidebarCollapsed = ref(false)
const notifStore = useNotificacionesStore()

const evaluateSidebarResponsiveState = () => {
  const savedState = localStorage.getItem('crm_sidebar_collapsed')
  if (savedState !== null) {
    isSidebarCollapsed.value = savedState === 'true'
  } else {
    isSidebarCollapsed.value = window.innerWidth < 1366
  }
}

const onWindowResize = () => {
  if (localStorage.getItem('crm_sidebar_collapsed') === null) {
    isSidebarCollapsed.value = window.innerWidth < 1366
  }
}

onMounted(() => {
  evaluateSidebarResponsiveState()
  window.addEventListener('resize', onWindowResize)
  notifStore.inicializar()
})

onUnmounted(() => {
  window.removeEventListener('resize', onWindowResize)
  notifStore.detenerStream()
})

const toggleCollapse = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value
  localStorage.setItem('crm_sidebar_collapsed', String(isSidebarCollapsed.value))
}
</script>

<template>
  <div class="min-h-screen bg-background-1 text-foreground flex">
    <Sidebar :is-open="isSidebarOpen" :is-collapsed="isSidebarCollapsed" @close="isSidebarOpen = false"
      @toggle-collapse="toggleCollapse" />

    <div class="flex-1 flex flex-col min-w-0 min-h-screen transition-all duration-300 ease-in-out"
      :class="isSidebarCollapsed ? 'lg:ps-20' : 'lg:ps-64'">
      <Navbar @toggle-sidebar="isSidebarOpen = !isSidebarOpen" @toggle-collapse="toggleCollapse" />

      <main class="flex-1 p-3 pb-6 sm:p-4 sm:pb-8 2xl:p-6 2xl:pb-10 max-w-7xl w-full mx-auto">
        <router-view />
      </main>
    </div>

    <NotificationToast />
  </div>
</template>
