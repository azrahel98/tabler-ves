<template>
  <div class="flex h-screen overflow-hidden bg-surface dark:bg-gray-950">
    <Sidebar />
    <div class="relative flex flex-1 flex-col overflow-hidden">
      <div @click="sidebarToggle = false" :class="sidebarToggle ? 'block lg:hidden' : 'hidden'" class="fixed w-full h-screen z-9 bg-gray-900/50"></div>

      <Header />
      <main class="flex-1 overflow-y-auto">
        <div class="p-4 mx-auto max-w-(--breakpoint-2xl) md:p-6 md:pt-3">
          <RouterView v-slot="{ Component }">
            <Transition name="page" mode="out-in">
              <component :is="Component" :key="$route.path" />
            </Transition>
          </RouterView>
          <Loading v-if="configuracionStore.loading" :full-page="true" size="md" />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useRoute } from 'vue-router'
  import Sidebar from '../components/Sidebar/Sidebar.vue'
  import Header from '../components/Header/Header.vue'
  import Loading from '../components/ui/Loading.vue'

  import { useConfiguracionStore } from '../stores/layout'
  import { storeToRefs } from 'pinia'

  const $route = useRoute()
  const configuracionStore = useConfiguracionStore()
  const { menuLateralAbierto: sidebarToggle } = storeToRefs(configuracionStore)
</script>
