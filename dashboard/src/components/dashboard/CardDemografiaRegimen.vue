<script setup lang="ts">
import { ref, computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { IconDownload, IconLoader2, IconBriefcase, IconX } from '@tabler/icons-vue'
import { useThemeStore } from '@/stores/theme'
import { useToastStore } from '@/stores/toast'
import { getApiBaseUrl, openProtectedFile } from '@/services/api'
import type { ResumenPersonal } from './types'
import type { ChartOptions, ChartData } from 'chart.js'

interface Props {
  resumen: ResumenPersonal | null
  isLoading?: boolean
  selectedRegimen?: string | null
  chartHeight?: string
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  selectedRegimen: null,
  chartHeight: undefined,
})

const emit = defineEmits<{
  (e: 'select-regimen', regimen: string | null): void
}>()

const themeStore = useThemeStore()
const toast = useToastStore()
const isDownloading = ref(false)

const descargarExcel = async () => {
  isDownloading.value = true
  try {
    await openProtectedFile(`${getApiBaseUrl()}/api/dash/exportar_excel`, 'trabajadores_activos.xlsx', 'download')
    toast.success('Reporte de trabajadores activos descargado.')
  } catch {
    toast.error('No se pudo descargar el reporte.')
  } finally {
    isDownloading.value = false
  }
}

const REGIMEN_COLORS = ['#2563eb', '#0284c7', '#4f46e5', '#0d9488', '#d97706', '#64748b']

const isDark = computed(() => themeStore.isDark)

const itemsRegimen = computed(() => {
  const list = props.resumen?.por_regimen || []
  return [...list].sort((a, b) => b.cantidad - a.cantidad)
})

const totalRegimen = computed(() => {
  return itemsRegimen.value.reduce((acc, r) => acc + r.cantidad, 0)
})

const toggleRegimen = (nombre: string) => {
  emit('select-regimen', props.selectedRegimen === nombre ? null : nombre)
}

const getPercentage = (cantidad: number): string => {
  if (!totalRegimen.value) return '0.0'
  return ((cantidad / totalRegimen.value) * 100).toFixed(1)
}

const chartData = computed<ChartData<'doughnut'>>(() => {
  const labels = itemsRegimen.value.map((r) => r.nombre)
  const data = itemsRegimen.value.map((r) => r.cantidad)
  return {
    labels,
    datasets: [
      {
        data,
        backgroundColor: itemsRegimen.value.map((_, i) => REGIMEN_COLORS[i % REGIMEN_COLORS.length]),
        borderWidth: 2,
        borderColor: isDark.value ? '#262626' : '#ffffff',
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'doughnut'>>(() => {
  const labels = itemsRegimen.value.map((r) => r.nombre)
  return {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '74%',
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
            return ` ${val} servidores (${getPercentage(val)}%)`
          },
        },
      },
    },
    onClick: (_event, elements) => {
      if (elements.length > 0) {
        const index = elements[0].index
        const selected = labels[index]
        if (selected) {
          toggleRegimen(selected)
        }
      }
    },
  }
})
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden flex flex-col">
    <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-2 flex-wrap shrink-0">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconBriefcase class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Régimen Laboral</span>
        </h3>
        <p class="text-micro text-muted-foreground">Distribución por marco normativo</p>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        <button
          v-if="selectedRegimen"
          type="button"
          class="inline-flex items-center gap-1 px-2 h-control-xs rounded-md bg-primary/10 text-primary border border-primary/20 text-nano font-semibold hover:bg-primary/15 transition cursor-pointer"
          title="Quitar filtro de régimen"
          @click="emit('select-regimen', null)"
        >
          <span>{{ selectedRegimen }}</span>
          <IconX class="size-3" aria-hidden="true" />
        </button>

        <button
          type="button"
          aria-label="Descargar lista de trabajadores activos en Excel"
          class="inline-flex items-center gap-1.5 px-2.5 h-control-sm rounded-md border border-border bg-card text-micro font-semibold text-foreground shadow-xs hover:bg-muted transition cursor-pointer disabled:opacity-60 disabled:cursor-not-allowed"
          :disabled="isDownloading"
          @click="descargarExcel"
        >
          <IconLoader2 v-if="isDownloading" class="size-3.5 animate-spin" />
          <IconDownload v-else class="size-3.5 text-muted-foreground" />
          <span>Descargar</span>
        </button>
      </div>
    </div>

    <div class="p-3.5 flex-1 flex flex-col justify-center">
      <div
        v-if="isLoading"
        class="flex flex-col sm:flex-row items-center gap-3.5 animate-pulse"
        aria-busy="true"
      >
        <div class="size-32 rounded-full bg-muted/60 shrink-0"></div>
        <div class="w-full flex-1 space-y-2">
          <div v-for="i in 5" :key="i" class="h-6 bg-muted/60 rounded-md"></div>
        </div>
      </div>

      <div
        v-else-if="itemsRegimen.length === 0"
        class="py-8 text-center space-y-1.5 text-muted-foreground text-meta"
      >
        <IconBriefcase class="size-6 mx-auto text-muted-foreground/40" aria-hidden="true" />
        <p class="font-semibold text-foreground text-micro">Sin regímenes registrados</p>
        <p class="text-micro">No se encontró distribución de personal por marco normativo.</p>
      </div>

      <div v-else class="flex flex-col sm:flex-row items-center gap-3.5">
        <div :class="[props.chartHeight || 'size-32 sm:size-34', 'relative flex items-center justify-center shrink-0']">
          <Doughnut
            :data="chartData"
            :options="chartOptions"
            aria-label="Gráfico de distribución por régimen laboral"
          />
          <div class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center text-center">
            <span class="font-mono text-kpi font-bold text-foreground tabular-nums leading-none">
              {{ totalRegimen }}
            </span>
            <span class="text-micro font-semibold uppercase tracking-wider text-muted-foreground mt-0.5">
              Activos
            </span>
          </div>
        </div>

        <div class="w-full flex-1 min-w-0 divide-y divide-border border border-border rounded-lg overflow-hidden bg-card">
          <button
            v-for="(r, i) in itemsRegimen"
            :key="r.nombre"
            type="button"
            class="w-full px-2.5 py-1.5 text-left flex items-center justify-between gap-2 text-micro transition-colors cursor-pointer focus-visible:outline-hidden focus-visible:bg-muted/60"
            :class="selectedRegimen === r.nombre ? 'bg-primary/10 text-foreground font-semibold' : 'hover:bg-muted/40 text-foreground'"
            :title="`Filtrar por ${r.nombre}`"
            @click="toggleRegimen(r.nombre)"
          >
            <div class="flex items-center gap-2 min-w-0 flex-1">
              <span
                class="size-2 rounded-full shrink-0"
                :style="{ backgroundColor: REGIMEN_COLORS[i % REGIMEN_COLORS.length] }"
              />
              <span class="truncate font-medium">{{ r.nombre }}</span>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              <span class="font-mono font-semibold text-foreground tabular-nums text-right">
                {{ r.cantidad }}
              </span>
              <span class="font-mono text-nano text-muted-foreground tabular-nums w-9 text-right">
                {{ getPercentage(r.cantidad) }}%
              </span>
            </div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
