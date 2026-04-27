<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-title-lg font-bold leading-tight text-gray-900 dark:text-white tracking-tight">Organigrama</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Estructura jerárquica de la organización</p>
        </div>
        <div class="flex items-center gap-2">
          <button
            v-if="!cargando"
            @click="expandirTodo = !expandirTodo"
            class="inline-flex items-center gap-2 rounded-xl border border-gray-100 bg-card px-3.5 py-2 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-primary/5 dark:border-white/6 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
            <ChevronsUpDown class="h-4 w-4" />
            {{ expandirTodo ? 'Colapsar' : 'Expandir' }}
          </button>
          <button
            v-if="!cargando"
            @click="recargar"
            class="inline-flex items-center gap-2 rounded-xl border border-gray-100 bg-card px-3.5 py-2 text-sm font-medium text-gray-700 shadow-theme-xs hover:bg-primary/5 dark:border-white/6 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
            <RefreshCw class="h-4 w-4" />
            Actualizar
          </button>
        </div>
      </div>

      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando organigrama…</span>
      </div>

      <div v-else-if="organigrama.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <div class="flex items-center justify-center w-16 h-16 rounded-2xl bg-surface dark:bg-white/5 mb-4">
          <Network class="h-8 w-8 text-gray-400 dark:text-gray-500" />
        </div>
        <p class="text-base font-semibold text-gray-700 dark:text-gray-300">Sin datos de organigrama</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1 max-w-xs">No se encontró información jerárquica disponible</p>
      </div>

      <template v-else>
        <div class="md:hidden flex flex-col gap-1.5">
          <FilaOrganigrama
            v-for="nodo in organigrama"
            :key="'m-' + nodo.id"
            :nodo="nodo"
            :nivel="0"
            :expandir-todo="expandirTodo"
            modo="tarjeta"
          />
        </div>

        <div class="hidden md:block rounded-2xl border border-gray-100 bg-card shadow-theme-xs dark:border-white/6 dark:bg-white/3 overflow-hidden">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-100 dark:border-white/6 bg-surface dark:bg-white/3">
                <th class="px-3 py-3.5 text-left text-2xs font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500 w-10"></th>
                <th class="px-4 py-3.5 text-left text-2xs font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Área / Gerencia</th>
                <th class="px-4 py-3.5 text-left text-2xs font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Jefe Responsable</th>
                <th class="px-4 py-3.5 text-left text-2xs font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">DNI</th>
                <th class="px-4 py-3.5 text-center text-2xs font-bold uppercase tracking-widest text-gray-400 dark:text-gray-500">Sub-áreas</th>
              </tr>
            </thead>
            <tbody>
              <FilaOrganigrama
                v-for="nodo in organigrama"
                :key="nodo.id"
                :nodo="nodo"
                :nivel="0"
                :expandir-todo="expandirTodo"
                modo="tabla"
              />
            </tbody>
          </table>
        </div>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'
  import { useTableroStore } from '../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import { RefreshCw, Network, ChevronsUpDown } from 'lucide-vue-next'
  import FilaOrganigrama from '../components/organigrama/FilaOrganigrama.vue'
  import Loading from '../components/ui/Loading.vue'

  const tableroStore = useTableroStore()
  const { organigrama } = storeToRefs(tableroStore)
  const cargando = ref(false)
  const expandirTodo = ref(false)

  const recargar = async () => {
    cargando.value = true
    try {
      await tableroStore.obtenerOrganigrama()
    } finally {
      cargando.value = false
    }
  }

  onMounted(async () => {
    if (organigrama.value.length === 0) {
      await recargar()
    }
  })
</script>
