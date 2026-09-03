<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { Bar } from 'vue-chartjs'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import type { AreaReport } from '@/services/dashboard'
import type { ChartOptions, ChartData } from 'chart.js'
import {
  IconBuildingSkyscraper,
  IconChartBar,
  IconList,
  IconX,
} from '@tabler/icons-vue'

interface Props {
  areas: AreaReport[]
  isLoading?: boolean
  selectedArea?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  selectedArea: null,
})

const emit = defineEmits<{
  (e: 'select-area', area: string | null): void
}>()

const activeView = ref<'chart' | 'list'>('chart')
const isDark = ref(document.documentElement.classList.contains('dark'))

const observer = new MutationObserver(() => {
  isDark.value = document.documentElement.classList.contains('dark')
})

onMounted(() => {
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})

const totalPersonal = computed(() => {
  return props.areas.reduce((acc, a) => acc + a.cantidad, 0)
})

const maxCantidad = computed(() => {
  return Math.max(...props.areas.map((a) => a.cantidad), 1)
})

const sortedAreas = computed(() => {
  return [...props.areas].sort((a, b) => b.cantidad - a.cantidad)
})

const top5Areas = computed(() => {
  return sortedAreas.value.slice(0, 5)
})

const mayorArea = computed(() => {
  return sortedAreas.value[0] || null
})

const chartGridColor = computed(() => isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.06)')
const chartTextColor = computed(() => isDark.value ? '#a3a3a3' : '#525252')
const chartBarColor = computed(() => isDark.value ? '#3b82f6' : '#2563eb')

const chartData = computed<ChartData<'bar'>>(() => {
  return {
    labels: top5Areas.value.map((a) => a.nombre),
    datasets: [
      {
        label: 'Personal Activo',
        data: top5Areas.value.map((a) => a.cantidad),
        backgroundColor: chartBarColor.value,
        borderRadius: 6,
        barThickness: 20,
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
        caretSize:9,
        cornerRadius: 8,
        callbacks: {
          label: (context) => {
            const val = Number(context.raw || 0)
            const pct = totalPersonal.value ? ((val / totalPersonal.value) * 100).toFixed(1) : '0'
            return ` ${val} servidores (${pct}%)`
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
    onClick: (_event, elements) => {
      if (elements.length > 0) {
        const index = elements[0].index
        const selected = top5Areas.value[index]?.nombre
        if (selected) {
          emit('select-area', props.selectedArea === selected ? null : selected)
        }
      }
    },
  }
})

const toggleArea = (nombre: string) => {
  emit('select-area', props.selectedArea === nombre ? null : nombre)
}
</script>

<template>
  <Card :no-padding="true" class="shadow-2xs w-full overflow-hidden border border-border bg-card">
    <div class="flex flex-col gap-3 border-b border-border p-4 sm:px-5 sm:flex-row sm:items-center sm:justify-between">
      <div class="flex items-center gap-3">
        <div class="flex size-7 shrink-0 items-center justify-center rounded-lg bg-blue-500/10 text-blue-600 dark:text-blue-400">
          <IconBuildingSkyscraper class="size-5" aria-hidden="true" />
        </div>
        <div>
          <h3 class="font-semibold text-foreground tracking-tight text-sm">Personal por Área</h3>
          <p class="text-[11px] p-0 m-0 text-muted-foreground">
            Distribución de dotación activa por dependencia
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div v-if="selectedArea" class="flex items-center gap-1">
          <Badge variant="primary" size="sm">
            {{ selectedArea }}
          </Badge>
          <button
            type="button"
            class="flex size-6 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground focus-visible:outline-hidden"
            title="Quitar filtro"
            aria-label="Quitar filtro de área"
            @click="emit('select-area', null)"
          >
            <IconX class="size-3.5" aria-hidden="true" />
          </button>
        </div>

        <div class="flex items-center rounded-lg border border-border/60 bg-muted/50 p-0.5" role="group" aria-label="Cambiar vista">
          <button
            type="button"
            class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-all cursor-pointer focus-visible:outline-hidden"
            :class="activeView === 'chart' ? 'bg-card text-foreground shadow-2xs font-medium' : 'text-muted-foreground hover:text-foreground'"
            title="Vista Gráfico de Barras"
            @click="activeView = 'chart'"
          >
            <IconChartBar class="size-3.5" aria-hidden="true" />
            <span class="hidden sm:inline">Gráfico</span>
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium transition-all cursor-pointer focus-visible:outline-hidden"
            :class="activeView === 'list' ? 'bg-card text-foreground shadow-2xs font-medium' : 'text-muted-foreground hover:text-foreground'"
            title="Vista Lista Detallada"
            @click="activeView = 'list'"
          >
            <IconList class="size-3.5" aria-hidden="true" />
            <span class="hidden sm:inline">Lista</span>
          </button>
        </div>

        <Badge variant="outline" size="sm">
          {{ areas.length }} {{ areas.length === 1 ? 'Área' : 'Áreas' }}
        </Badge>
      </div>
    </div>

    <div v-if="!isLoading && areas.length > 0" class="grid grid-cols-2 sm:grid-cols-3 gap-px bg-border/60 border-b border-border text-xs">
      <div class="bg-card px-4 py-2.5 sm:px-5">
        <span class="text-[11px] font-medium text-muted-foreground block">Dotación Total</span>
        <span class="text-sm font-medium text-foreground font-mono tabular-nums">{{ totalPersonal }}</span>
        <span class="text-[11px] text-muted-foreground ml-1">servidores</span>
      </div>
      <div class="bg-card px-4 py-2.5 sm:px-5">
        <span class="text-[11px] font-medium text-muted-foreground block">Mayor Concentración</span>
        <span class="text-xs font-medium text-foreground block wrap-break-word" :title="mayorArea?.nombre">
          {{ mayorArea ? mayorArea.nombre : '-' }}
        </span>
      </div>
      <div class="bg-card px-4 py-2.5 sm:px-5 col-span-2 sm:col-span-1">
        <span class="text-[11px] font-medium text-muted-foreground block">Participación Líder</span>
        <span class="text-sm font-medium text-primary font-mono tabular-nums">
          {{ mayorArea && totalPersonal ? Math.round((mayorArea.cantidad / totalPersonal) * 100) : 0 }}%
        </span>
        <span class="text-[11px] text-muted-foreground ml-1">del total</span>
      </div>
    </div>

    <div class="p-4 sm:p-5">
      <div v-if="isLoading" class="h-60 bg-muted/60 rounded-xl animate-pulse" aria-busy="true"></div>

      <div v-else-if="areas.length === 0" class="py-12 text-center text-xs text-muted-foreground">
        No se registran datos de áreas disponibles.
      </div>

      <div v-else-if="activeView === 'chart'" class="min-h-60 flex items-center justify-center w-full">
        <div class="w-full h-64 relative">
          <Bar
            :data="chartData"
            :options="chartOptions"
            aria-label="Gráfico de distribución de personal por área"
          />
        </div>
      </div>

      <div v-else class="space-y-2 max-h-85 overflow-y-auto pr-1">
        <button
          v-for="(area, idx) in sortedAreas"
          :key="area.nombre"
          type="button"
          class="w-full rounded-xl border p-3 text-left transition-all flex items-center justify-between gap-3 group focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-primary/40 cursor-pointer"
          :class="[
            selectedArea === area.nombre
              ? 'bg-primary/10 border-primary ring-1 ring-primary shadow-2xs'
              : 'bg-card hover:bg-muted/40 border-border hover:border-primary/40'
          ]"
          @click="toggleArea(area.nombre)"
        >
          <div class="min-w-0 flex-1">
            <div class="flex items-center justify-between gap-2 mb-1.5">
              <div class="flex items-center gap-2 min-w-0">
                <span
                  class="flex size-5 shrink-0 items-center justify-center rounded-md font-mono text-[11px] font-medium"
                  :class="idx < 3 ? 'bg-primary/15 text-primary' : 'bg-muted text-muted-foreground'"
                >
                  {{ idx + 1 }}
                </span>
                <span class="text-xs font-medium text-foreground wrap-break-word" :title="area.nombre">
                  {{ area.nombre }}
                </span>
              </div>

              <div class="flex items-center gap-2 shrink-0">
                <span class="text-xs font-medium text-foreground font-mono tabular-nums">
                  {{ area.cantidad }}
                </span>
                <Badge variant="outline" size="xs">
                  {{ totalPersonal ? Math.round((area.cantidad / totalPersonal) * 100) : 0 }}%
                </Badge>
              </div>
            </div>

            <div class="w-full bg-muted/80 rounded-full h-2 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="selectedArea === area.nombre ? 'bg-primary' : 'bg-primary/80 group-hover:bg-primary'"
                :style="{ width: `${(area.cantidad / maxCantidad) * 100}%` }"
              ></div>
            </div>
          </div>
        </button>
      </div>
    </div>
  </Card>
</template>
