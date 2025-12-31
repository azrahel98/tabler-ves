<template>
  <div class="rounded-sm bg-card text-card-foreground shadow-sm p-3 h-min">
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-3">
        <Users class="lucide lucide-users w-4 h-4 text-primary" />
        <h3 class="text-base font-semibold">Trabajadores Activos por Área</h3>
      </div>
      <div class="flex items-baseline gap-1">
        <span class="text-sm font-bold text-primary">{{ areas.length }}</span>
        <span class="text-sm text-muted-foreground">áreas</span>
      </div>
    </div>

    <div class="bg-background rounded-md overflow-hidden border border-border/50 max-h-[400px] overflow-y-auto">
      <table class="w-full text-left table-auto">
        <thead class="bg-background/80 border-b border-border/50">
          <tr>
            <th class="p-2 text-xs font-medium text-muted-foreground uppercase tracking-wider">Dependencia / Unidad</th>
            <th class="p-2 text-xs font-medium text-muted-foreground uppercase tracking-wider text-right w-1/5">Activos</th>
          </tr>
        </thead>

        <tbody>
          <tr v-for="area in areas" :key="area.nombre" class="hover:bg-background/60 transition-colors">
            <td class="p-2 text-sm font-normal text-card-foreground">
              {{ area.nombre }}
            </td>
            <td class="p-2 text-sm text-right font-medium text-card-foreground">
              {{ area.cantidad }}
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="!isLoading && !areas.length" class="p-4 text-center text-sm text-muted-foreground">No se encontraron trabajadores activos registrados.</div>

      <div v-if="isLoading" class="p-4 text-center text-sm text-muted-foreground animate-pulse">Cargando datos de personal activo...</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { api } from '@api/axios'

import { Users } from 'lucide-vue-next'
import { onMounted, ref } from 'vue'

interface AreaActivos {
  cantidad: number
  nombre: string
}

const areas = ref<AreaActivos[]>([])
const isLoading = ref(true)
const error = ref<string | null>(null)

async function fetchTrabajadoresActivos() {
  isLoading.value = true
  error.value = null
  try {
    const response = await api.post('/dash/area_report')
    const data: AreaActivos[] = response.data.sort((a: AreaActivos, b: AreaActivos) => b.cantidad - a.cantidad)

    areas.value = data
  } catch (err) {
    console.error('Error al cargar trabajadores activos:', err)
    error.value = 'Hubo un error al obtener los datos de activos.'
    areas.value = []
  } finally {
    isLoading.value = false
  }
}

onMounted(fetchTrabajadoresActivos)
</script>
