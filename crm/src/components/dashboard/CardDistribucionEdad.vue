<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'
import type { RangoEdadItem } from '@/api/types'
import { IconChartPie } from '@tabler/icons-vue'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const propiedades = defineProps<{
  rangosEdad?: RangoEdadItem[]
}>()

const datosEdad = computed(() => {
  const lista = propiedades.rangosEdad || []
  return {
    labels: lista.map((i) => `${i.nombre} años`),
    datasets: [
      {
        label: 'Cantidad',
        data: lista.map((i) => i.cantidad),
        backgroundColor: '#8b5cf6',
        borderRadius: 6,
      },
    ],
  }
})

const opcionesVisuales = {
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
          size: 10,
        },
      },
    },
  },
}
</script>

<template>
  <div class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2.5">
        <div
          class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconChartPie class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Distribución por Edad</h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">Rangos etarios</p>
        </div>
      </div>
    </div>

    <div class="p-4">
      <div class="h-56 relative flex items-center justify-center">
        <div v-if="!rangosEdad || rangosEdad.length === 0" class="text-sm text-gray-400">
          Sin información disponible
        </div>
        <Bar v-else :data="datosEdad" :options="opcionesVisuales" />
      </div>
    </div>
  </div>
</template>
