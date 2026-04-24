<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/[0.03] md:p-6">
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

  
  
  const mappingColores: Record<string, string> = {
    'D. Leg. Nº 276':      '#3641f5', 
    'D. Leg. Nº 728':      '#252dae', 
    'D.L 1057':     '#7592ff', 
    'D.L 1057-F':   '#9cb9ff', 
    'D.L 1057 - T': '#465fff', 
  }
  const colorPorDefecto = '#98a2b3' 


  const chartData = computed(() => {
    const regimenes = store.resumen?.por_regimen || []
    const etiquetas = regimenes.map((r: any) => r.nombre)
    const datosCifra = regimenes.map((r: any) => r.cantidad)
    const coloresDeFondo = regimenes.map((r: any) => mappingColores[r.nombre] || colorPorDefecto)

    return {
      labels: etiquetas,
      datasets: [
        {
          backgroundColor: coloresDeFondo,
          data: datosCifra,
          hoverOffset: 6,
          borderWidth: 0,
        },
      ],
    }
  })

  
  const colorPara = (nombre: string) => {
    return mappingColores[nombre] || colorPorDefecto
  }

  const totalRegimen = computed(() =>
    store.resumen?.por_regimen?.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
  )

  const calcularPorcentaje = (cantidad: number) => {
    return ((cantidad / totalRegimen.value) * 100).toFixed(1)
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false, 
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
