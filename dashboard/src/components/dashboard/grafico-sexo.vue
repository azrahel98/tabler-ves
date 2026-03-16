<template>
  <div class="rounded-2xl border border-gray-200 bg-white p-5 dark:border-gray-800 dark:bg-white/3 md:p-6 h-full flex flex-col">
    <div class="mb-4 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Distribución por Sexo</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartSexo" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-44">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    <!-- Leyenda Personalizada -->
    <div class="mt-auto flex flex-col gap-3 pt-4">
      <div v-for="(sexo, index) in store.resumen?.por_sexo" :key="index" class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="block h-3 w-3 rounded-full" :style="{ backgroundColor: colorPara(sexo.nombre) }"></span>
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            {{ formatSexo(sexo.nombre) }}
          </span>
        </div>
        <div class="flex items-center gap-4">
          <span class="text-sm font-semibold text-gray-800 dark:text-white/90">
            {{ sexo.cantidad }}
          </span>
          <span class="text-sm text-gray-500 dark:text-gray-400 w-12 text-right"> {{ calcularPorcentaje(sexo.cantidad) }}% </span>
        </div>
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

  // M = Masculino, F = Femenino
  const coloresPorSexo: Record<string, string> = {
    'M': '#1e3a8a', // Azul oscuro
    'F': '#db2777', // Rosa oscuro
  }
  const colorPorDefecto = '#9ca3af'

  const formatSexo = (sigla: string) => {
    if (sigla === 'M') return 'Masculino'
    if (sigla === 'F') return 'Femenino'
    return sigla
  }

  const chartData = computed(() => {
    const dataSexo = store.resumen?.por_sexo || []
    const etiquetas = dataSexo.map((s: any) => formatSexo(s.nombre))
    const datosCifra = dataSexo.map((s: any) => s.cantidad)
    const coloresDeFondo = dataSexo.map((s: any) => coloresPorSexo[s.nombre] || colorPorDefecto)

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
    return coloresPorSexo[nombre] || colorPorDefecto
  }

  const calcularPorcentaje = (cantidad: number) => {
    const total = store.resumen?.por_sexo?.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
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
