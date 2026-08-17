<template>
  <div :class="sinContenedor ? 'h-full flex flex-col' : 'rounded-2xl border border-gray-200 bg-card p-4 md:p-5 flex flex-col h-full'">
    <div class="mb-3 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90">Distribución por Sindicato</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartSindicato" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-36">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    
    <div class="mt-auto flex flex-wrap gap-x-4 gap-y-2 justify-center pt-2 border-t border-gray-100 dark:border-gray-800/50">
      <div v-for="(sindicato, index) in datosProcesados" :key="index" class="flex items-center gap-1.5">
        <span class="block h-2.5 w-2.5 rounded-full shrink-0" :style="{ backgroundColor: colorPara(sindicato.nombre) }"></span>
        <span class="text-xs font-semibold text-gray-700 dark:text-gray-300">
          {{ sindicato.nombre }}
        </span>
        <span class="text-xs font-bold text-gray-800 dark:text-white/90 ml-0.5">
          {{ sindicato.cantidad }}
        </span>
        <span class="text-2xs text-gray-500 dark:text-gray-400"> ({{ calcularPorcentaje(sindicato.cantidad) }}%) </span>
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

  const css = (v: string) => getComputedStyle(document.documentElement).getPropertyValue(v).trim()

  const coloresPorSindicato: Record<string, string> = {
    'SITRAMUN':    css('--color-warning-400'),  
    'SOMUN':       css('--color-secondary'),     
    'Sin Afiliar': css('--color-primary'),       
  }
  const colorPorDefecto = css('--color-accent') 

  const datosProcesados = computed(() => {
    if (!store.resumen) return []

    const sindicatos = store.resumen.por_sindicato || []
    const totalAfiliados = sindicatos.reduce((acc: number, s: any) => acc + s.cantidad, 0)
    const personalActivo = store.resumen.activos || 0
    
    
    const sinAfiliar = Math.max(0, personalActivo - totalAfiliados)

    const resultado = [...sindicatos]
    if (sinAfiliar > 0 || resultado.length === 0) {
      resultado.push({
        nombre: 'Sin Afiliar',
        cantidad: sinAfiliar
      })
    }
    
    return resultado
  })

  const chartData = computed(() => {
    const dataFiltrada = datosProcesados.value
    const etiquetas = dataFiltrada.map((s: any) => s.nombre)
    const datosCifra = dataFiltrada.map((s: any) => s.cantidad)
    const coloresDeFondo = dataFiltrada.map((s: any) => coloresPorSindicato[s.nombre] || colorPorDefecto)

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

  const colorPara = (nombre: string) => {
    return coloresPorSindicato[nombre] || colorPorDefecto
  }

  const calcularPorcentaje = (cantidad: number) => {
    const total = datosProcesados.value.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
    return ((cantidad / total) * 100).toFixed(1)
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
