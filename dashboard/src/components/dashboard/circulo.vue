<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/3 md:p-6">
    <div class="mb-4 justify-between gap-4 sm:flex">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Distribución por Régimen</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartThree" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-44">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    <!-- Leyenda Personalizada -->
    <div class="mt-4 flex flex-wrap gap-x-6 gap-y-3 justify-center">
      <div v-for="(regimen, index) in store.resumen?.por_regimen" :key="index" class="flex items-center gap-2">
        <span class="block h-3 w-3 rounded-full shrink-0" :style="{ backgroundColor: colorPara(regimen.nombre) }"></span>
        <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
          {{ regimen.nombre }}
        </span>
        <span class="text-sm font-semibold text-gray-800 dark:text-white/90 ml-1">
          {{ regimen.cantidad }}
        </span>
        <span class="text-xs text-gray-500 dark:text-gray-400"> ({{ calcularPorcentaje(regimen.cantidad) }}%) </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'

  ChartJS.register(ArcElement, Tooltip, Legend)

  const store = useTableroStore()

  const coloresPorRegimen: Record<string, string> = {
    'D.L 276': '#16a085',
    'D.L 728': '#0f3460',
    'D.L 1057': '#1a56db',
    'D.L 1057-F': '#4b7fe8',
    'D.L 1057 - T': '#85aaef',
  }
  const colorPorDefecto = '#6b7280'

  const chartData = computed(() => {
    const regimenes = store.resumen?.por_regimen || []
    const etiquetas = regimenes.map((r: any) => r.nombre)
    const datosCifra = regimenes.map((r: any) => r.cantidad)
    const coloresDeFondo = regimenes.map((r: any) => coloresPorRegimen[r.nombre] || colorPorDefecto)

    return {
      labels: etiquetas,
      datasets: [
        {
          backgroundColor: coloresDeFondo,
          data: datosCifra,
          hoverOffset: 4,
          borderWidth: 0,
        },
      ],
    }
  })

  // Funciones auxiliares para la leyenda personalizada
  const colorPara = (nombre: string) => {
    return coloresPorRegimen[nombre] || colorPorDefecto
  }

  const calcularPorcentaje = (cantidad: number) => {
    const total = store.resumen?.por_regimen?.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
    return ((cantidad / total) * 100).toFixed(1)
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false, // Ocultar la leyenda por defecto de Chart.js
      },
    },
    layout: {
      padding: {
        top: 0,
      },
    },
    elements: {
      arc: {
        borderWidth: 0,
      },
    },
  }
</script>
