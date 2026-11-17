<script setup lang="ts">
import { computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { IconUsers } from '@tabler/icons-vue'
import { useThemeStore } from '@/stores/theme'
import type { ChartOptions, ChartData } from 'chart.js'

interface SexoItem {
  nombre: string
  cantidad: number
}

interface Props {
  porSexo: SexoItem[]
  isLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
})

const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const SEXO_COLORS = ['#0284c7', '#2563eb', '#64748b']

const totalSexo = computed(() => props.porSexo.reduce((acc, s) => acc + s.cantidad, 0))

const getPct = (val: number): string => {
  if (!totalSexo.value) return '0.0'
  return ((val / totalSexo.value) * 100).toFixed(1)
}

const formatLabel = (raw: string): string => {
  const clean = (raw || '').trim().toUpperCase()
  if (clean === 'F' || clean.startsWith('FEM')) return 'Femenino'
  if (clean === 'M' || clean.startsWith('MASC')) return 'Masculino'
  return raw || 'No especificado'
}

const chartData = computed<ChartData<'doughnut'>>(() => {
  return {
    labels: props.porSexo.map((s) => formatLabel(s.nombre)),
    datasets: [
      {
        data: props.porSexo.map((s) => s.cantidad),
        backgroundColor: props.porSexo.map((_, i) => SEXO_COLORS[i % SEXO_COLORS.length]),
        borderWidth: 2,
        borderColor: isDark.value ? '#262626' : '#ffffff',
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'doughnut'>>(() => {
  return {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '72%',
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
            return ` ${val} servidores (${getPct(val)}%)`
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
          <IconUsers class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Distribución por Sexo</span>
        </h3>
        <p class="text-micro text-muted-foreground">Composición demográfica activa</p>
      </div>
      <span
        v-if="totalSexo > 0"
        class="text-nano font-mono text-muted-foreground bg-muted px-2 py-0.5 rounded-md border border-border shrink-0"
      >
        {{ totalSexo }} activos
      </span>
    </div>

    <div class="p-3.5 sm:p-4 flex-1 flex flex-col justify-center">
      <div v-if="isLoading" class="h-38 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>
      <div
        v-else-if="porSexo.length === 0 || totalSexo === 0"
        class="h-38 flex items-center justify-center text-micro text-muted-foreground"
      >
        Sin datos de distribución disponibles
      </div>
      <div v-else class="flex items-center gap-4">
        <div class="relative size-28 sm:size-32 shrink-0 flex items-center justify-center">
          <Doughnut
            :data="chartData"
            :options="chartOptions"
            aria-label="Gráfico de distribución por género"
          />
        </div>

        <div class="flex-1 min-w-0 space-y-2">
          <div
            v-for="(item, idx) in porSexo"
            :key="item.nombre"
            class="p-2 rounded-lg border border-border bg-muted/20 flex flex-col gap-1"
          >
            <div class="flex items-center justify-between gap-2 text-micro">
              <span class="flex items-center gap-1.5 font-medium text-foreground truncate">
                <span
                  class="size-2 rounded-full shrink-0"
                  :style="{ backgroundColor: SEXO_COLORS[idx % SEXO_COLORS.length] }"
                />
                {{ formatLabel(item.nombre) }}
              </span>
              <span class="font-mono font-semibold text-foreground tabular-nums">
                {{ item.cantidad }}
                <span class="text-nano text-muted-foreground font-normal">({{ getPct(item.cantidad) }}%)</span>
              </span>
            </div>
            <div class="w-full bg-muted rounded-full h-1.5 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300"
                :style="{
                  width: `${totalSexo ? (item.cantidad / totalSexo) * 100 : 0}%`,
                  backgroundColor: SEXO_COLORS[idx % SEXO_COLORS.length],
                }"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
