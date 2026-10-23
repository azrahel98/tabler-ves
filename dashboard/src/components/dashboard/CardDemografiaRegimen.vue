<script setup lang="ts">
import { ref, computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { IconDownload, IconLoader2 } from '@tabler/icons-vue'
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
  chartHeight: 'h-57.5',
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

const isDark = computed(() => themeStore.isDark)

const chartTextColor = computed(() => isDark.value ? '#94a3b8' : '#475569')

const chartData = computed<ChartData<'doughnut'>>(() => {
  const labels = props.resumen?.por_regimen.map((r) => r.nombre) || []
  const data = props.resumen?.por_regimen.map((r) => r.cantidad) || []
  return {
    labels,
    datasets: [
      {
        data,
        backgroundColor: ['#2563eb', '#38bdf8', '#6366f1', '#10b981', '#f59e0b'],
        borderWidth: 2,
        borderColor: isDark.value ? '#1e293b' : '#ffffff',
      },
    ],
  }
})

const chartOptions = computed<ChartOptions<'doughnut'>>(() => {
  const labels = props.resumen?.por_regimen.map((r) => r.nombre) || []
  return {
    responsive: true,
    maintainAspectRatio: false,
    cutout: '72%',
    plugins: {
      legend: {
        position: 'bottom',
        labels: {
          color: chartTextColor.value,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: 11,
            weight: 500,
          },
          usePointStyle: true,
          padding: 14,
        },
      },
    },
    onClick: (_event, elements) => {
      if (elements.length > 0) {
        const index = elements[0].index
        const selected = labels[index]
        if (selected) {
          emit('select-regimen', props.selectedRegimen === selected ? null : selected)
        }
      }
    },
  }
})
</script>

<template>
  <Card class="shadow-2xs">
    <div class="flex items-center justify-between border-b border-border pb-3 mb-3.5 flex-wrap gap-2">
      <div>
        <h3 class="font-semibold text-foreground tracking-tight text-sm">Régimen Laboral</h3>
        <p class="text-[11px] text-muted-foreground">Distribución por marco normativo</p>
      </div>
      <div class="flex items-center gap-2">
        <Badge v-if="selectedRegimen" variant="primary" size="xs">
          {{ selectedRegimen }}
        </Badge>
        <button
          type="button"
          aria-label="Descargar lista de trabajadores activos en Excel"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 text-[11px] font-medium text-foreground bg-card border border-border rounded-lg hover:bg-muted focus:outline-hidden transition cursor-pointer disabled:opacity-60 disabled:cursor-not-allowed"
          :disabled="isDownloading"
          @click="descargarExcel"
        >
          <IconLoader2 v-if="isDownloading" class="size-3.5 animate-spin" />
          <IconDownload v-else class="size-3.5 text-muted-foreground" />
          <span>Descargar</span>
        </button>
      </div>
    </div>

    <div v-if="isLoading" :class="[chartHeight, 'bg-muted/60 rounded-lg animate-pulse']" aria-busy="true"></div>
    <div v-else :class="[chartHeight, 'flex items-center justify-center relative w-full']">
      <Doughnut
        :data="chartData"
        :options="chartOptions"
        aria-label="Gráfico de distribución por régimen laboral"
      />
    </div>
  </Card>
</template>
