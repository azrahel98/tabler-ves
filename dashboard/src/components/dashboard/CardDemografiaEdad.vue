<script setup lang="ts">
import { computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { IconUsersGroup } from '@tabler/icons-vue'
import { useThemeStore } from '@/stores/theme'
import { useChartScale } from '@/lib/chart'
import type { RangoReport } from './types'
import type { ChartOptions, ChartData } from 'chart.js'

interface Props {
  rangos: RangoReport[]
  isLoading?: boolean
  chartHeight?: string
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  chartHeight: 'h-38',
})

const { chartFontSize } = useChartScale()
const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const chartGridColor = computed(() => (isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.1)'))
const chartTextColor = computed(() => (isDark.value ? '#94a3b8' : '#475569'))

const totalPersonas = computed(() => props.rangos.reduce((acc, r) => acc + r.cantidad, 0))

const rangoMayor = computed(() => {
  if (!props.rangos.length) return null
  return [...props.rangos].sort((a, b) => b.cantidad - a.cantidad)[0]
})

const porcentajeMayor = computed(() => {
  if (!rangoMayor.value || !totalPersonas.value) return 0
  return Math.round((rangoMayor.value.cantidad / totalPersonas.value) * 100)
})

const chartData = computed<ChartData<'bar'>>(() => {
  return {
    labels: props.rangos.map((r) => r.nombre),
    datasets: [
      {
        label: 'Servidores',
        data: props.rangos.map((r) => r.cantidad),
        backgroundColor: isDark.value ? '#38bdf8' : '#0284c7',
        borderRadius: 4,
        barThickness: 18,
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
      tooltip: {
        backgroundColor: isDark.value ? '#1e293b' : '#0f172a',
        titleColor: '#ffffff',
        bodyColor: '#e2e8f0',
        padding: 8,
        cornerRadius: 6,
        callbacks: {
          label: (ctx) => {
            const val = Number(ctx.raw || 0)
            const pct = totalPersonas.value ? ((val / totalPersonas.value) * 100).toFixed(1) : '0'
            return ` ${val} servidores (${pct}%)`
          },
        },
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
            size: chartFontSize.value,
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
          precision: 0,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: chartFontSize.value,
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
          <IconUsersGroup class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Distribución por Edad</span>
        </h3>
        <p class="text-micro text-muted-foreground">Grupos etarios del personal activo</p>
      </div>
      <span
        v-if="rangoMayor && totalPersonas > 0"
        class="text-nano font-mono text-muted-foreground bg-muted px-2 py-0.5 rounded-md border border-border shrink-0"
        :title="`Rango predominante: ${rangoMayor.nombre} (${rangoMayor.cantidad} servidores)`"
      >
        Mayoría: <strong class="text-foreground font-semibold">{{ rangoMayor.nombre }}</strong> ({{ porcentajeMayor }}%)
      </span>
    </div>

    <div class="p-3.5 sm:p-4 flex-1 flex flex-col justify-center">
      <div v-if="isLoading" :class="[chartHeight, 'bg-muted/60 rounded-lg animate-pulse']" aria-busy="true"></div>
      <div
        v-else-if="rangos.length === 0 || totalPersonas === 0"
        :class="[chartHeight, 'flex items-center justify-center text-micro text-muted-foreground']"
      >
        Sin datos de grupos etarios disponibles
      </div>
      <div v-else :class="[chartHeight, 'relative w-full']">
        <Bar
          :data="chartData"
          :options="chartOptions"
          aria-label="Gráfico de distribución por edad"
        />
      </div>
    </div>
  </div>
</template>
