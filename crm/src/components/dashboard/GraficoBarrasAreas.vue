<script setup lang="ts">
import { ref, computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'
import type { AreaReportItem } from '@/api/types'
import { IconBuildingSkyscraper, IconChartBar, IconList } from '@tabler/icons-vue'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const propiedades = defineProps<{
  areas: AreaReportItem[]
}>()

const modoVista = ref<'grafico' | 'lista'>('grafico')

const areasProcesadas = computed(() => {
  const lista = propiedades.areas || []
  if (lista.length <= 6) return lista
  const ordenadas = [...lista].sort((a, b) => b.cantidad - a.cantidad)
  const principales = ordenadas.slice(0, 6)
  const restantes = ordenadas.slice(6)
  const sumaOtros = restantes.reduce((acc, curr) => acc + curr.cantidad, 0)
  if (sumaOtros > 0) {
    principales.push({ nombre: 'Otras áreas', cantidad: sumaOtros })
  }
  return principales
})

const cantidadMaxima = computed(() => {
  const lista = propiedades.areas || []
  if (lista.length === 0) return 1
  return Math.max(...lista.map((a) => a.cantidad), 1)
})

const datosBarras = computed(() => {
  const lista = areasProcesadas.value
  return {
    labels: lista.map((a) => a.nombre),
    datasets: [
      {
        label: 'Personal Activo',
        data: lista.map((a) => a.cantidad),
        backgroundColor: '#3b82f6',
        borderRadius: 6,
        borderSkipped: false,
      },
    ],
  }
})

const opcionesBarras = {
  indexAxis: 'y' as const,
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false,
    },
    tooltip: {
      padding: 10,
      cornerRadius: 8,
      bodyFont: {
        size: 12,
        weight: 600,
      },
    },
  },
  scales: {
    x: {
      grid: {
        display: false,
      },
      ticks: {
        font: {
          size: 10,
        },
      },
    },
    y: {
      grid: {
        display: false,
      },
      ticks: {
        font: {
          size: 11,
          weight: 600,
        },
      },
    },
  },
}
</script>

<template>
  <div class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2">
        <div
          class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconBuildingSkyscraper class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">Personal por Área</h3>
          <p class="text-2xs font-medium text-gray-400">Distribución por dependencia</p>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div class="flex items-center rounded-lg bg-gray-100 p-0.5 dark:bg-gray-700">
          <button
            @click="modoVista = 'grafico'"
            :class="
              modoVista === 'grafico'
                ? 'bg-white text-blue-600 shadow-sm dark:bg-gray-600 dark:text-white'
                : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
            "
            class="rounded-md p-1.5 transition-all cursor-pointer"
            title="Vista Gráfico">
            <IconChartBar class="h-3.5 w-3.5" />
          </button>
          <button
            @click="modoVista = 'lista'"
            :class="
              modoVista === 'lista'
                ? 'bg-white text-blue-600 shadow-sm dark:bg-gray-600 dark:text-white'
                : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
            "
            class="rounded-md p-1.5 transition-all cursor-pointer"
            title="Vista Lista">
            <IconList class="h-3.5 w-3.5" />
          </button>
        </div>
        <span
          class="inline-flex items-center rounded-md bg-gray-100 px-2 py-0.5 text-2xs font-bold text-gray-600 dark:bg-gray-700 dark:text-gray-300">
          {{ areas.length }} áreas
        </span>
      </div>
    </div>

    <div class="p-4">
      <div v-if="modoVista === 'grafico'" class="h-72 relative flex items-center justify-center">
        <div v-if="!areas || areas.length === 0" class="text-2xs text-gray-400">No hay datos disponibles</div>
        <Bar v-else :data="datosBarras" :options="opcionesBarras" />
      </div>

      <div v-else class="max-h-[280px] overflow-y-auto space-y-1.5 custom-scrollbar">
        <div v-if="!areas || areas.length === 0" class="py-6 text-center text-2xs text-gray-400">
          No hay datos de áreas disponibles
        </div>
        <RouterLink
          v-for="area in areas"
          :key="area.nombre"
          :to="`/area/${encodeURIComponent(area.nombre)}`"
          class="flex flex-col gap-1 rounded-lg p-2 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors cursor-pointer">
          <div class="flex justify-between items-center">
            <span class="text-2xs font-semibold text-gray-700 dark:text-gray-300">{{ area.nombre }}</span>
            <span
              class="text-2xs font-bold font-mono text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-700 px-2 py-0.5 rounded-md">
              {{ area.cantidad }}
            </span>
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
            <div
              class="h-full rounded-full bg-blue-500 transition-all duration-500"
              :style="{ width: `${Math.round((area.cantidad / cantidadMaxima) * 100)}%` }" />
          </div>
        </RouterLink>
      </div>
    </div>
  </div>
</template>
