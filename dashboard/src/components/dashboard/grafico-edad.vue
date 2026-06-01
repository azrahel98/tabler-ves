<template>
  <div :class="sinContenedor ? 'h-full flex flex-col' : 'rounded-2xl border border-gray-200 bg-card p-4 md:p-5 flex flex-col h-full'">
    <div class="mb-3 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90">Distribución por Rangos de Edad</h3>
      </div>
    </div>

    <div class="mb-3 shrink-0 flex justify-center">
      <div id="chartEdad" class="mx-auto w-full">
        <div class="relative w-full h-40 flex justify-center">
          <PolarArea :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    <div class="mt-auto flex flex-wrap gap-x-4 gap-y-2 justify-center pt-2 border-t border-gray-100 dark:border-gray-800/50">
      <div v-for="(rango, index) in store.rangosEdad" :key="index" class="flex items-center gap-1.5">
        <span class="block h-2.5 w-2.5 rounded-full shrink-0" :style="{ backgroundColor: colorBordePara(index) }"></span>
        <span class="data-value text-gray-700 dark:text-gray-300">
          {{ rango.nombre }} años
        </span>
        <span class="text-xs font-bold text-gray-800 dark:text-white/90 ml-0.5">
          {{ rango.cantidad }}
        </span>
        <span class="text-2xs text-gray-500 dark:text-gray-400"> ({{ calcularPorcentaje(rango.cantidad) }}%) </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { Chart as ChartJS, RadialLinearScale, ArcElement, Tooltip, Legend } from 'chart.js'
  import { PolarArea } from 'vue-chartjs'

  defineProps<{
    sinContenedor?: boolean
  }>()

  ChartJS.register(RadialLinearScale, ArcElement, Tooltip, Legend)

  const store = useTableroStore()

  // Paleta de colores HSL con transparencia del 65% para superposición radial sofisticada
  const coloresFondo = [
    'hsla(243, 90%, 65%, 0.65)',  // 18-25: Indigo moderno
    'hsla(217, 91%, 60%, 0.65)',  // 26-35: Azul vibrante
    'hsla(199, 89%, 48%, 0.65)',  // 36-45: Celeste premium
    'hsla(162, 70%, 45%, 0.65)',  // 46-55: Esmeralda suave
    'hsla(13, 90%, 60%, 0.65)',   // 55+: Naranja coral
  ]

  const coloresBorde = [
    'hsl(243, 90%, 65%)',
    'hsl(217, 91%, 60%)',
    'hsl(199, 89%, 48%)',
    'hsl(162, 70%, 45%)',
    'hsl(13, 90%, 60%)',
  ]

  const colorFondoPara = (index: number) => {
    return coloresFondo[index % coloresFondo.length]
  }

  const colorBordePara = (index: number) => {
    return coloresBorde[index % coloresBorde.length]
  }

  const chartData = computed(() => {
    const dataEdad = store.rangosEdad || []
    const etiquetas = dataEdad.map((r: any) => r.nombre + ' años')
    const datosCifra = dataEdad.map((r: any) => r.cantidad)
    const coloresDeFondo = dataEdad.map((_, index) => colorFondoPara(index))
    const coloresDeBorde = dataEdad.map((_, index) => colorBordePara(index))

    return {
      labels: etiquetas,
      datasets: [
        {
          label: 'Cantidad',
          backgroundColor: coloresDeFondo,
          borderColor: coloresDeBorde,
          borderWidth: 1.5,
          data: datosCifra,
        },
      ],
    }
  })

  const totalEdad = computed(() =>
    store.rangosEdad?.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
  )

  const calcularPorcentaje = (cantidad: number) => {
    return ((cantidad / totalEdad.value) * 100).toFixed(1)
  }

  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      tooltip: {
        backgroundColor: 'rgba(15, 23, 42, 0.95)',
        titleColor: '#fff',
        bodyColor: '#fff',
        borderWidth: 1,
        borderColor: 'rgba(255, 255, 255, 0.1)',
        padding: 10,
        cornerRadius: 8,
        displayColors: true,
      }
    },
    scales: {
      r: {
        grid: {
          color: 'rgba(156, 163, 175, 0.15)',
        },
        angleLines: {
          color: 'rgba(156, 163, 175, 0.15)',
        },
        ticks: {
          display: false, // oculta valores concéntricos
        },
        pointLabels: {
          display: false,
        }
      }
    }
  }
</script>
