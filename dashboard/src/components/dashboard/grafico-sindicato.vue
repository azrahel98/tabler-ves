<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/[0.03] md:p-6 h-full flex flex-col">
    <div class="mb-4 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Distribución por Sindicato</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartSindicato" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-44">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    
    <div class="mt-auto flex flex-col gap-3 pt-4">
      <div v-for="(sindicato, index) in datosProcesados" :key="index" class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="block h-3 w-3 rounded-full" :style="{ backgroundColor: colorPara(sindicato.nombre) }"></span>
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            {{ sindicato.nombre }}
          </span>
        </div>
        <div class="flex items-center gap-4">
          <span class="text-sm font-semibold text-gray-800 dark:text-white/90">
            {{ sindicato.cantidad }}
          </span>
          <span class="text-sm text-gray-500 dark:text-gray-400 w-12 text-right"> {{ calcularPorcentaje(sindicato.cantidad) }}% </span>
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
