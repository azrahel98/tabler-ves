<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { Bar } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { RangoReport } from '@/services/dashboard'
import type { ChartOptions, ChartData } from 'chart.js'

interface Props {
  rangos: RangoReport[]
  isLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
})

const isDark = ref(document.documentElement.classList.contains('dark'))

const observer = new MutationObserver(() => {
  isDark.value = document.documentElement.classList.contains('dark')
})

onMounted(() => {
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})

const chartGridColor = computed(() => isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.06)')
const chartTextColor = computed(() => isDark.value ? '#94a3b8' : '#475569')

const chartData = computed<ChartData<'bar'>>(() => {
  return {
    labels: props.rangos.map((r) => r.nombre),
    datasets: [
      {
        label: 'Trabajadores',
        data: props.rangos.map((r) => r.cantidad),
        backgroundColor: '#0284c7',
        borderRadius: 4,
        barThickness: 20,
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'bar'>>(() => {
  return {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
    },
    scales: {
      x: {
        grid: {
          display: false,
        },
        ticks: {
          color: chartTextColor.value,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
            weight: 500,
          },
        },
      },
      y: {
        grid: {
          color: chartGridColor.value,
        },
        ticks: {
          color: chartTextColor.value,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
          },
        },
      },
    },
  }
})
</script>

<template>
  <Card class="shadow-2xs">
    <div class="flex items-center justify-between border-b border-border pb-3 mb-3.5 flex-wrap gap-2">
      <div>
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Distribución por Edad</h3>
        <p class="text-[11px] text-muted-foreground">Grupos etarios del personal</p>
      </div>
      <Badge variant="outline" size="xs">Pirámide Activa</Badge>
    </div>

    <div v-if="isLoading" class="h-56 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>
    <div v-else class="h-57.5 relative w-full">
      <Bar
        :data="chartData"
        :options="chartOptions"
        aria-label="Gráfico de distribución por edad"
      />
    </div>
  </Card>
</template>
