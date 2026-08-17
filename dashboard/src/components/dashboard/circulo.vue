<template>
  <div :class="sinContenedor ? 'h-full flex flex-col' : 'rounded-2xl border border-gray-200 bg-card p-4 md:p-5 flex flex-col h-full'">
    <div class="mb-3 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90">Distribución por Régimen</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartThree" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-36">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    
    <div class="mt-auto flex flex-wrap gap-x-4 gap-y-2 justify-center pt-2 border-t border-gray-100 dark:border-gray-800/50">
      <div v-for="(regimen, index) in store.resumen?.por_regimen" :key="index" class="flex items-center gap-1.5">
        <span class="block h-2.5 w-2.5 rounded-full shrink-0" :style="{ backgroundColor: colorPara(regimen.nombre) }"></span>
        <span class="text-xs font-semibold text-gray-700 dark:text-gray-300">
          {{ regimen.nombre }}
        </span>
        <span class="text-xs font-bold text-gray-800 dark:text-white/90 ml-0.5">
          {{ regimen.cantidad }}
        </span>
        <span class="text-2xs text-gray-500 dark:text-gray-400"> ({{ calcularPorcentaje(regimen.cantidad) }}%) </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'

  defineProps<{
    sinContenedor?: boolean
  }>()

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
