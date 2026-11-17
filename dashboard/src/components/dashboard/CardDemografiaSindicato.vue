<script setup lang="ts">
import { computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { IconUsersGroup } from '@tabler/icons-vue'
import { useThemeStore } from '@/stores/theme'
import type { ChartOptions, ChartData } from 'chart.js'

interface SindicatoItem {
  nombre: string
  cantidad: number
}

interface Props {
  porSindicato: SindicatoItem[]
  /** Total de personal activo: permite calcular quienes no están afiliados. */
  totalActivos: number
  isLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), { isLoading: false })

const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const COLORES = ['#2563eb', '#8b5cf6', '#0d9488', '#d97706']
const COLOR_SIN_AFILIAR = computed(() => (isDark.value ? '#475569' : '#cbd5e1'))

const afiliados = computed(() => props.porSindicato.reduce((a, s) => a + s.cantidad, 0))
const sinAfiliar = computed(() => Math.max(0, props.totalActivos - afiliados.value))
const total = computed(() => afiliados.value + sinAfiliar.value)

interface Segmento {
  nombre: string
  cantidad: number
  color: string
}

const segmentos = computed<Segmento[]>(() => {
  const lista: Segmento[] = [...props.porSindicato]
    .sort((a, b) => b.cantidad - a.cantidad)
    .map((s, i) => ({ nombre: s.nombre, cantidad: s.cantidad, color: COLORES[i % COLORES.length] }))
  if (sinAfiliar.value > 0) {
    lista.push({ nombre: 'Sin afiliación', cantidad: sinAfiliar.value, color: COLOR_SIN_AFILIAR.value })
  }
  return lista
})

const pct = (v: number, base = total.value) => (base ? ((v / base) * 100).toFixed(1) : '0.0')
const tasaAfiliacion = computed(() => (total.value ? Math.round((afiliados.value / total.value) * 100) : 0))

const chartData = computed<ChartData<'doughnut'>>(() => ({
  labels: segmentos.value.map((s) => s.nombre),
  datasets: [
    {
      data: segmentos.value.map((s) => s.cantidad),
      backgroundColor: segmentos.value.map((s) => s.color),
      borderWidth: 2,
      borderColor: isDark.value ? '#262626' : '#ffffff',
    },
  ],
}))

const chartOptions = computed<ChartOptions<'doughnut'>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  cutout: '70%',
  plugins: {
    legend: { display: false },
    tooltip: {
      backgroundColor: isDark.value ? '#1e293b' : '#0f172a',
      padding: 8,
      cornerRadius: 6,
      callbacks: {
        label: (ctx) => ` ${ctx.label}: ${Number(ctx.raw)} (${pct(Number(ctx.raw))}%)`,
      },
    },
  },
}))
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden flex flex-col">
    <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-2">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconUsersGroup class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Afiliación Sindical</span>
        </h3>
        <p class="text-micro text-muted-foreground">Distribución por sindicato</p>
      </div>
      <span
        v-if="total > 0"
        class="text-nano font-mono text-muted-foreground bg-muted px-2 py-0.5 rounded-md border border-border shrink-0"
      >
        {{ afiliados }} afiliados
      </span>
    </div>

    <div class="p-4 flex-1 flex flex-col justify-center gap-4">
      <div v-if="isLoading" class="h-56 bg-muted/60 rounded-lg animate-pulse" aria-busy="true"></div>

      <div v-else-if="total === 0" class="h-56 flex items-center justify-center text-micro text-muted-foreground">
        Sin datos de afiliación disponibles
      </div>

      <template v-else>
        <div class="relative size-36 mx-auto">
          <Doughnut :data="chartData" :options="chartOptions" aria-label="Gráfico de afiliación sindical" />
          <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
            <span class="text-kpi font-semibold font-mono tabular-nums text-foreground leading-none">{{ tasaAfiliacion }}%</span>
            <span class="mt-0.5 text-nano text-muted-foreground">afiliados</span>
          </div>
        </div>

        <ul class="space-y-2">
          <li v-for="s in segmentos" :key="s.nombre">
            <div class="flex items-center justify-between gap-2 text-micro">
              <span class="flex items-center gap-1.5 font-medium text-foreground min-w-0">
                <span class="size-2 rounded-full shrink-0" :style="{ backgroundColor: s.color }" />
                <span class="truncate" :title="s.nombre">{{ s.nombre }}</span>
              </span>
              <span class="font-mono tabular-nums shrink-0">
                <strong class="font-semibold text-foreground">{{ s.cantidad }}</strong>
                <span class="text-nano text-muted-foreground"> · {{ pct(s.cantidad) }}%</span>
              </span>
            </div>
            <div class="mt-1 w-full bg-muted rounded-full h-1.5 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :style="{ width: `${total ? (s.cantidad / total) * 100 : 0}%`, backgroundColor: s.color }"
              />
            </div>
          </li>
        </ul>
      </template>
    </div>
  </div>
</template>
