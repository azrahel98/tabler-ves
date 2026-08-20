<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useInterfazStore } from '@/stores/interfaz'
import Navbar from '@/components/layout/nav/Navbar.vue'
import Sidebar from '@/components/layout/Sidebar.vue'
import PanelDetalleDocumento from '@/components/comun/PanelDetalleDocumento.vue'

const interfazStore = useInterfazStore()
const { menuLateralColapsado } = storeToRefs(interfazStore)
</script>

<template>
  <div class="flex h-screen overflow-hidden bg-gray-50 dark:bg-gray-950">
    <Sidebar />

    <div
      :class="[
        'flex flex-1 flex-col overflow-hidden transition-all duration-300 ease-in-out',
        menuLateralColapsado ? 'md:ml-20' : 'md:ml-64',
      ]">
      <Navbar />

      <main class="flex-1 overflow-y-auto">
        <router-view />
      </main>
    </div>

    <PanelDetalleDocumento />
  </div>
</template>
