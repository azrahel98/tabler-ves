<template>
  <div class="flex h-screen overflow-hidden bg-gray-100 dark:bg-gray-900">
    <Sidebar />
    <div
      class="relative flex flex-1 flex-col overflow-y-auto overflow-x-hidden"
    >
      <div
        @click="sidebarToggle = false"
        :class="sidebarToggle ? 'block lg:hidden' : 'hidden'"
        class="fixed w-full h-screen z-9 bg-gray-900/50"
      ></div>

      <Header />
      <main>
        <div class="p-4 mx-auto max-w-(--breakpoint-2xl) md:p-6">
          <router-view />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import Sidebar from "../components/Sidebar/Sidebar.vue";
import Header from "../components/Header/Header.vue";
import { useConfiguracionStore } from "../stores/layout";
import { storeToRefs } from "pinia";

const configuracionStore = useConfiguracionStore();
const { menuLateralAbierto: sidebarToggle } = storeToRefs(configuracionStore);
</script>
