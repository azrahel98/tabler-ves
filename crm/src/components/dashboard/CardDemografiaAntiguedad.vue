<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { Line } from 'vue-chartjs'
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

const chartData = computed<ChartData<'line'>>(() => {
  return {
    labels: props.rangos.map((r) => r.nombre),
    datasets: [
      {
        label: 'Personal en rango',
        data: props.rangos.map((r) => r.cantidad),
        fill: true,
        borderColor: '#6366f1',
        backgroundColor: 'rgba(99, 102, 241, 0.15)',
        tension: 0.35,
        pointBackgroundColor: '#6366f1',
        pointBorderColor: '#ffffff',
        pointHoverRadius: 5,
        pointRadius: 3,
        borderWidth: 2,
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'line'>>(() => {
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
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Años de Antigüedad</h3>
        <p class="text-[11px] text-muted-foreground">Retención y permanencia laboral</p>
      </div>
      <Badge variant="outline" size="xs">Trayectoria</Badge>
    </div>

    <div v-if="isLoading" class="h-56 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>
    <div v-else class="h-57.5 relative w-full">
      <Line
        :data="chartData"
        :options="chartOptions"
        aria-label="Gráfico de años de antigüedad institucional"
      />
    </div>
  </Card>
</template>
