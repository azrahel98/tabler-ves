<template>
  <div class="rounded-2xl border border-gray-100 bg-card h-full p-6 dark:border-white/6 max-h-60 overflow-y-auto dark:bg-white/3">
    <div class="flex items-center justify-between gap-2 text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90 mb-6">
      <div class="flex items-center gap-2">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path
            d="M13 3a9 9 0 0 0-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42A8.954 8.954 0 0 0 13 21a9 9 0 0 0 0-18zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z" />
        </svg>
        Historial de Cambios
      </div>
      <button
        v-if="!cargado"
        @click="cargar"
        class="rounded-full flex items-center gap-1 px-2 py-1 text-gray-500 hover:bg-primary/10 hover:text-primary dark:text-gray-400 dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
        title="Cargar historial">
        <RefreshCw class="h-3.5 w-3.5" :class="{ 'animate-spin': cargando }" />
        <span class="text-[10px] font-medium">Cargar</span>
      </button>
    </div>

    <div v-if="!cargado" class="text-sm text-gray-500 text-center py-4">Presiona "Cargar" para ver el historial de operaciones.</div>

    <div v-else-if="historialCambios.length === 0" class="flex flex-col items-center gap-2 py-6 text-center">
      <div class="h-10 w-10 rounded-xl bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
        <ClockIcon class="h-5 w-5 text-gray-400 dark:text-gray-500" />
      </div>
      <p class="text-xs font-medium text-gray-400 dark:text-gray-500">Sin registros en el historial</p>
    </div>

    <div v-else class="space-y-3 max-h-64 overflow-y-auto pr-1">
      <div v-for="(item, i) in historialCambios" :key="i" class="flex gap-3 group">
        <div class="flex flex-col items-center">
          <div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full" :class="colorOperacion(item.operacion)">
            <component :is="iconoOperacion(item.operacion)" class="h-3.5 w-3.5" />
          </div>
          <div v-if="i < historialCambios.length - 1" class="w-px flex-1 bg-gray-200 dark:bg-gray-700 mt-1"></div>
        </div>
        <div class="pb-4 min-w-0">
          <div class="flex items-center gap-2 flex-wrap">
            <span class="text-[10px] font-semibold uppercase px-1.5 py-0.5 rounded" :class="colorOperacion(item.operacion)">
              {{ item.operacion }}
            </span>
            <span class="text-[10px] text-gray-400">{{ item.fecha }}</span>
          </div>
          <p class="text-sm text-gray-800 dark:text-white/90 mt-1 wrap-break-word">{{ item.detalle }}</p>
          <p class="text-[10px] text-gray-400 mt-0.5">por {{ item.nombre }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../stores/personal'
  import { RefreshCw, Pencil, Plus, Trash2, Eye, Clock as ClockIcon } from 'lucide-vue-next'

  const store = usePersonalStore()
  const { historialCambios, perfilActual } = storeToRefs(store)

  const cargado = ref(false)
  const cargando = ref(false)

  const cargar = async () => {
    if (!perfilActual.value?.dni) return
    cargando.value = true
    try {
      await store.obtenerHistorial(perfilActual.value.dni)
      cargado.value = true
    } finally {
      cargando.value = false
    }
  }

  const colorOperacion = (op: string) => {
    const lower = op?.toLowerCase() || ''
    if (lower.includes('editar') || lower.includes('actualizar')) return 'bg-amber-100 text-amber-600 dark:bg-amber-900/20 dark:text-amber-400'
    if (lower.includes('agregar') || lower.includes('crear') || lower.includes('registr')) return 'bg-emerald-100 text-emerald-600 dark:bg-emerald-900/20 dark:text-emerald-400'
    if (lower.includes('eliminar') || lower.includes('borrar') || lower.includes('renuncia')) return 'bg-red-100 text-red-600 dark:bg-red-900/20 dark:text-red-400'
    return 'bg-blue-100 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400'
  }

  const iconoOperacion = (op: string) => {
    const lower = op?.toLowerCase() || ''
    if (lower.includes('editar') || lower.includes('actualizar')) return Pencil
    if (lower.includes('agregar') || lower.includes('crear') || lower.includes('registr')) return Plus
    if (lower.includes('eliminar') || lower.includes('borrar') || lower.includes('renuncia')) return Trash2
    return Eye
  }
</script>
