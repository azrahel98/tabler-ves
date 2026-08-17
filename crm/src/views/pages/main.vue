<script setup lang="ts">
import { ref } from 'vue'
import Navbar from '@/components/layout/nav/Navbar.vue'
import Sidebar from '@/components/layout/Sidebar.vue'
import PanelDetalleDocumento from '@/components/comun/PanelDetalleDocumento.vue'

const menuLateralAbierto = ref(false)
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-gray-50 dark:bg-gray-950">
    <Sidebar :is-open="menuLateralAbierto" @close="menuLateralAbierto = false" />

    <div class="flex flex-1 flex-col overflow-hidden md:ml-16 xl:ml-64 transition-all">
      <Navbar @toggle-sidebar="menuLateralAbierto = !menuLateralAbierto" />

      <main class="flex-1 overflow-y-auto">
        <router-view />
      </main>
    </div>

    <PanelDetalleDocumento />
  </div>
</template>
