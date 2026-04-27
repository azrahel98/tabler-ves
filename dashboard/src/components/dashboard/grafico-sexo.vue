<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-5 dark:border-gray-800 dark:bg-white/[0.03] md:p-6 h-full flex flex-col">
    <div class="mb-4 justify-between gap-4 sm:flex shrink-0">
      <div>
        <h3 class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90">Distribución por Sexo</h3>
      </div>
    </div>

    <div class="mb-2 shrink-0">
      <div id="chartSexo" class="mx-auto flex justify-center">
        <div class="relative flex justify-center w-full h-44">
          <Doughnut :data="chartData" :options="chartOptions" />
        </div>
      </div>
    </div>

    
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
  import { computed, ref, onMounted } from 'vue'
  import { useTableroStore } from '../../stores/dashboard'
  import { Chart as ChartJS, ArcElement, Tooltip, Legend } from 'chart.js'
  import { Doughnut } from 'vue-chartjs'

  ChartJS.register(ArcElement, Tooltip, Legend)

  const store = useTableroStore()

  const css = (v: string) => getComputedStyle(document.documentElement).getPropertyValue(v).trim()

  
  const coloresPorSexo = ref<Record<string, string>>({})
  const colorPorDefecto = ref('#98a2b3')

  onMounted(() => {
    coloresPorSexo.value = {
      'M': css('--color-primary'),
      'F': css('--color-theme-pink-500'),
    }
    colorPorDefecto.value = css('--color-gray-400') || '#98a2b3'
  })

  const formatSexo = (sigla: string) => {
    if (sigla === 'M') return 'Masculino'
    if (sigla === 'F') return 'Femenino'
    return sigla
  }

  const chartData = computed(() => {
    const dataSexo = store.resumen?.por_sexo || []
    const etiquetas = dataSexo.map((s: any) => formatSexo(s.nombre))
    const datosCifra = dataSexo.map((s: any) => s.cantidad)
    const coloresDeFondo = dataSexo.map((s: any) => coloresPorSexo.value[s.nombre] || colorPorDefecto.value)

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
    return coloresPorSexo.value[nombre] || colorPorDefecto.value
  }

  const totalSexo = computed(() =>
    store.resumen?.por_sexo?.reduce((acc: number, item: any) => acc + item.cantidad, 0) || 1
  )

  const calcularPorcentaje = (cantidad: number) => {
    return ((cantidad / totalSexo.value) * 100).toFixed(1)
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
