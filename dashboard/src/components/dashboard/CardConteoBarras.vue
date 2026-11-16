<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { IconChartBar } from '@tabler/icons-vue'
import { useThemeStore } from '@/stores/theme'
import { useChartScale } from '@/lib/chart'
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
  /** Color de las barras en claro y oscuro; por defecto azul. */
  barColor?: { light: string; dark: string }
  /** Sustantivo del tooltip en singular y plural; por defecto "servidor". */
  unidad?: { singular: string; plural: string }
  /** Largo máximo de las etiquetas del eje; el resto se acorta con puntos suspensivos. */
  maxLabelLength?: number
}

const props = withDefaults(defineProps<Props>(), {
  titulo: 'Cargos',
  subtitulo: 'Distribución por cargo',
  badgeLabel: undefined,
  isLoading: false,
  chartHeight: 'h-52',
  barColor: () => ({ light: '#2563eb', dark: '#3b82f6' }),
  unidad: () => ({ singular: 'servidor', plural: 'servidores' }),
  maxLabelLength: 32,
})

const { chartFontSize } = useChartScale()
const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const chartGridColor = computed(() => (isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.1)'))
const chartTextColor = computed(() => (isDark.value ? '#94a3b8' : '#475569'))
const chartBarColor = computed(() => (isDark.value ? props.barColor.dark : props.barColor.light))

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
        padding: 8,
        caretSize: 6,
        cornerRadius: 6,
        callbacks: {
          label: (context) => {
            const val = Number(context.raw || 0)
            const pct = totalCantidad.value ? ((val / totalCantidad.value) * 100).toFixed(1) : '0'
            return ` ${val} ${val === 1 ? props.unidad.singular : props.unidad.plural} (${pct}%)`
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
            size: chartFontSize.value,
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
            size: chartFontSize.value,
            weight: 500,
          },
          callback: (_value, index) => {
            const nombre = props.items[index]?.nombre ?? ''
            return nombre.length > props.maxLabelLength ? `${nombre.slice(0, props.maxLabelLength)}…` : nombre
          },
        },
      },
    },
  }
})
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden flex flex-col">
    <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-2 flex-wrap">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconChartBar class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>{{ titulo }}</span>
        </h3>
        <p class="text-micro text-muted-foreground">{{ subtitulo }}</p>
      </div>
      <span
        v-if="badgeLabel || items.length > 0"
        class="text-nano font-mono text-muted-foreground bg-muted px-2 py-0.5 rounded-md border border-border shrink-0"
      >
        {{ badgeLabel || `${items.length} registros` }}
      </span>
    </div>

    <div class="p-3.5 sm:p-4 flex-1 flex flex-col justify-center">
      <div v-if="isLoading" :class="[chartHeight, 'bg-muted/60 rounded-lg animate-pulse']" aria-busy="true"></div>
      <div
        v-else-if="items.length === 0"
        :class="[chartHeight, 'flex items-center justify-center text-micro text-muted-foreground']"
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
    </div>
  </div>
</template>
