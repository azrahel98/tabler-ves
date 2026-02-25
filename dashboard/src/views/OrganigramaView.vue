<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Organigrama</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Estructura jerárquica de la organización</p>
        </div>
        <button
          v-if="!cargando"
          @click="recargar"
          class="inline-flex items-center gap-2 rounded-xl border border-gray-200 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors">
          <RefreshCw class="h-4 w-4" />
          Actualizar
        </button>
      </div>

      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <Loading size="md" />
        <span class="text-sm text-gray-500 dark:text-gray-400">Cargando organigrama…</span>
      </div>

      <div v-else-if="organigrama.length === 0" class="flex flex-col items-center justify-center py-20 text-center">
        <Network class="h-16 w-16 text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-lg font-medium text-gray-500 dark:text-gray-400">Sin datos de organigrama</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">No se encontró información jerárquica disponible</p>
      </div>

      <template v-else>
        <!-- Móvil: tarjetas -->
        <div class="md:hidden flex flex-col gap-1">
          <FilaOrganigrama v-for="nodo in organigrama" :key="'m-' + nodo.id" :nodo="nodo" :nivel="0" modo="tarjeta" />
        </div>

        <!-- Desktop: tabla -->
        <div class="hidden md:block rounded-2xl border border-gray-200 bg-white shadow-sm dark:border-gray-800 dark:bg-gray-900 overflow-hidden">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-800/50">
                <th class="px-5 py-3 text-left text-[10px] font-bold uppercase tracking-wider text-gray-400 w-10"></th>
                <th class="px-4 py-3 text-left text-[10px] font-bold uppercase tracking-wider text-gray-400">Área / Gerencia</th>
                <th class="px-4 py-3 text-left text-[10px] font-bold uppercase tracking-wider text-gray-400">Jefe Responsable</th>
                <th class="px-4 py-3 text-left text-[10px] font-bold uppercase tracking-wider text-gray-400">DNI</th>
                <th class="px-4 py-3 text-center text-[10px] font-bold uppercase tracking-wider text-gray-400">Sub-áreas</th>
              </tr>
            </thead>
            <tbody>
              <FilaOrganigrama v-for="nodo in organigrama" :key="nodo.id" :nodo="nodo" :nivel="0" modo="tabla" />
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
  import { RefreshCw, Network } from 'lucide-vue-next'
  import FilaOrganigrama from '../components/organigrama/FilaOrganigrama.vue'
  import Loading from '../components/ui/Loading.vue'

  const tableroStore = useTableroStore()
  const { organigrama } = storeToRefs(tableroStore)
  const cargando = ref(false)

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
