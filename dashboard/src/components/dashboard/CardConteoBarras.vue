<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { useThemeStore } from '@/stores/theme'
import type { ChartOptions, ChartData } from 'chart.js'

export interface ItemConteo {
  nombre: string
  cantidad: number
}

interface Props {
  titulo?: string
  subtitulo?: string
  badgeLabel?: string
  items: ItemConteo[]
  isLoading?: boolean
  chartHeight?: string
}

const props = withDefaults(defineProps<Props>(), {
  titulo: 'Cargos',
  subtitulo: 'Distribución por cargo',
  badgeLabel: undefined,
  isLoading: false,
  chartHeight: 'h-60',
})

const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const chartGridColor = computed(() => (isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.06)'))
const chartTextColor = computed(() => (isDark.value ? '#94a3b8' : '#475569'))
const chartBarColor = computed(() => (isDark.value ? '#3b82f6' : '#2563eb'))

const totalCantidad = computed(() => {
  return props.items.reduce((acc, it) => acc + it.cantidad, 0)
})

const chartData = computed<ChartData<'bar'>>(() => {
  return {
    labels: props.items.map((i) => i.nombre),
    datasets: [
      {
        label: props.titulo,
        data: props.items.map((i) => i.cantidad),
        backgroundColor: chartBarColor.value,
        borderRadius: 4,
        barThickness: 18,
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'bar'>>(() => {
  return {
    indexAxis: 'y',
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        display: false,
      },
      tooltip: {
        backgroundColor: isDark.value ? '#1e293b' : '#0f172a',
        titleColor: '#ffffff',
        bodyColor: '#e2e8f0',
        padding: 10,
        caretSize: 8,
        cornerRadius: 8,
        callbacks: {
          label: (context) => {
            const val = Number(context.raw || 0)
            const pct = totalCantidad.value ? ((val / totalCantidad.value) * 100).toFixed(1) : '0'
            return ` ${val} ${val === 1 ? 'servidor' : 'servidores'} (${pct}%)`
          },
        },
      },
    },
    scales: {
      x: {
        grid: {
          color: chartGridColor.value,
        },
        ticks: {
          color: chartTextColor.value,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
          },
          precision: 0,
        },
      },
      y: {
        grid: {
          display: false,
        },
        ticks: {
          color: isDark.value ? '#f1f5f9' : '#1e293b',
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
            weight: 500,
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
        <h3 class="font-semibold text-foreground tracking-tight text-sm">{{ titulo }}</h3>
        <p class="text-meta text-muted-foreground">{{ subtitulo }}</p>
      </div>
      <Badge v-if="badgeLabel" variant="outline" size="xs">{{ badgeLabel }}</Badge>
    </div>

    <div v-if="isLoading" :class="[chartHeight, 'bg-muted/60 rounded-lg animate-pulse']" aria-busy="true"></div>
    <div
      v-else-if="items.length === 0"
      :class="[chartHeight, 'flex items-center justify-center text-xs text-muted-foreground']"
    >
      Sin datos disponibles
    </div>
    <div v-else :class="[chartHeight, 'relative w-full']">
      <Bar
        :data="chartData"
        :options="chartOptions"
        :aria-label="titulo"
      />
    </div>
  </Card>
</template>
