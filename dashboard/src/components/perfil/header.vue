<template>
  <div class="rounded-2xl border border-gray-100 bg-card p-5 dark:border-white/6 dark:bg-white/3 lg:p-6">
    <div v-if="store.perfilActual" class="flex justify-center items-center bg-card">
      <div class="flex flex-row items-center gap-6">
        <div class="w-20 h-20 shrink-0 overflow-hidden border border-gray-100 rounded-full dark:border-white/6">
          <img v-if="store.perfilActual.sexo == 'M'" src="/M.svg" alt="user" />
          <img v-else src="/F.svg" alt="user" />
        </div>

        <div class="flex flex-col">
          <h4 class="text-2xl font-semibold text-gray-800 dark:text-white/90">
            {{ store.perfilActual.nombre }}
          </h4>

          <div class="flex flex-row items-center gap-3 mt-1">
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ store.perfilActual.dni }}</p>
            <div class="h-3.5 w-px bg-gray-300 dark:bg-gray-700"></div>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ store.perfilActual.nacimiento }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { usePersonalStore } from '../../stores/personal'
  const store = usePersonalStore()
</script>
