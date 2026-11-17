<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { Bar } from 'vue-chartjs'
import { useThemeStore } from '@/stores/theme'
import { useChartScale } from '@/lib/chart'
import type { AreaReport } from './types'
import type { ChartOptions, ChartData, Scale } from 'chart.js'
import {
  IconBuildingSkyscraper,
  IconChartBar,
  IconList,
  IconSearch,
  IconExternalLink,
  IconFilter,
  IconX,
} from '@tabler/icons-vue'

interface Props {
  areas: AreaReport[]
  isLoading?: boolean
  selectedArea?: string | null
  chartHeight?: string
  defaultView?: 'chart' | 'list'
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  selectedArea: null,
  chartHeight: 'h-56',
  defaultView: 'chart',
})

const emit = defineEmits<{
  (e: 'select-area', area: string | null): void
}>()

const router = useRouter()

const activeView = ref<'chart' | 'list'>(props.defaultView)
const searchQuery = ref('')

watch(
  () => props.defaultView,
  (val) => {
    if (val) activeView.value = val
  }
)

const { chartFontSize } = useChartScale()
const axisFontSize = computed(() => Math.max(9, chartFontSize.value - 1))
const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)

const totalPersonal = computed(() => {
  return props.areas.reduce((acc, a) => acc + a.cantidad, 0)
})

const maxCantidad = computed(() => {
  return Math.max(...props.areas.map((a) => a.cantidad), 1)
})

const sortedAreas = computed(() => {
  return [...props.areas].sort((a, b) => b.cantidad - a.cantidad)
})

const filteredAreas = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q) return sortedAreas.value
  return sortedAreas.value.filter((a) => a.nombre.toLowerCase().includes(q))
})

const topAreas = computed(() => {
  return sortedAreas.value.slice(0, 7)
})

const mayorArea = computed(() => {
  return sortedAreas.value[0] || null
})

const mayorAreaPct = computed(() => {
  if (!mayorArea.value || !totalPersonal.value) return '0.0'
  return ((mayorArea.value.cantidad / totalPersonal.value) * 100).toFixed(1)
})

const getAreaPct = (cantidad: number): string => {
  if (!totalPersonal.value) return '0.0'
  return ((cantidad / totalPersonal.value) * 100).toFixed(1)
}

const chartGridColor = computed(() => (isDark.value ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.1)'))
const chartTextColor = computed(() => (isDark.value ? '#a3a3a3' : '#525252'))
const chartBarColor = computed(() => (isDark.value ? '#3b82f6' : '#2563eb'))

const chartData = computed<ChartData<'bar'>>(() => {
  return {
    labels: topAreas.value.map((a) => a.nombre),
    datasets: [
      {
        label: 'Personal Activo',
        data: topAreas.value.map((a) => a.cantidad),
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
    layout: {
      padding: { left: 4, right: 8 },
    },
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
            return ` ${val} servidores (${getAreaPct(val)}%)`
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
          precision: 0,
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: axisFontSize.value,
          },
        },
      },
      y: {
        grid: {
          display: false,
        },
        afterFit: (scale: Scale) => {
          const maxWidth = scale.chart.width * 0.45
          scale.width = Math.min(Math.max(scale.width + 8, 175), maxWidth)
        },
        ticks: {
          padding: 6,
          color: isDark.value ? '#f1f5f9' : '#1e293b',
          font: {
            family: 'Inter, ui-sans-serif, system-ui, sans-serif',
            size: axisFontSize.value,
            weight: 400,
          },
          callback: (_value, index) => {
            const nombre = topAreas.value[index]?.nombre ?? ''
            return nombre.length > 32 ? `${nombre.slice(0, 32)}…` : nombre
          },
        },
      },
    },
    onHover: (event, elements) => {
      const target = event.native?.target as HTMLElement | null
      if (target) {
        target.style.cursor = elements.length > 0 ? 'pointer' : 'default'
      }
    },
    onClick: (_event, elements) => {
      if (elements.length > 0) {
        const index = elements[0].index
        const selected = topAreas.value[index]?.nombre
        if (selected) {
          router.push({ path: '/personal/area', query: { area: selected } })
        }
      }
    },
  }
})

const irAArea = (nombre: string) => {
  router.push({ path: '/personal/area', query: { area: nombre } })
}

const toggleFiltroArea = (nombre: string) => {
  emit('select-area', props.selectedArea === nombre ? null : nombre)
}
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="px-4 py-2.5 border-b border-border flex flex-wrap items-center justify-between gap-2.5">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconBuildingSkyscraper class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Personal por Área</span>
        </h3>
        <p class="text-micro text-muted-foreground">
          Distribución de dotación activa en {{ areas.length }} {{ areas.length === 1 ? 'dependencia' : 'dependencias' }}
        </p>
      </div>

      <div class="flex items-center gap-2 shrink-0">
        <button
          v-if="selectedArea"
          type="button"
          class="inline-flex items-center gap-1 px-2 h-control-xs rounded-md bg-primary/10 text-primary border border-primary/20 text-nano font-semibold hover:bg-primary/15 transition cursor-pointer max-w-44"
          :title="`Quitar filtro: ${selectedArea}`"
          @click="emit('select-area', null)"
        >
          <span class="truncate">{{ selectedArea }}</span>
          <IconX class="size-3 shrink-0" aria-hidden="true" />
        </button>

        <div
          class="inline-flex items-center rounded-lg border border-border p-0.5 bg-card"
          role="group"
          aria-label="Cambiar modo de visualización de áreas"
        >
          <button
            type="button"
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-micro transition cursor-pointer focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="
              activeView === 'chart'
                ? 'bg-muted text-foreground font-semibold'
                : 'text-muted-foreground hover:text-foreground font-medium'
            "
            :aria-pressed="activeView === 'chart'"
            @click="activeView = 'chart'"
          >
            <IconChartBar class="size-3.5" aria-hidden="true" />
            <span>Gráfico</span>
          </button>
          <button
            type="button"
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md text-micro transition cursor-pointer focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-primary"
            :class="
              activeView === 'list'
                ? 'bg-muted text-foreground font-semibold'
                : 'text-muted-foreground hover:text-foreground font-medium'
            "
            :aria-pressed="activeView === 'list'"
            @click="activeView = 'list'"
          >
            <IconList class="size-3.5" aria-hidden="true" />
            <span>Directorio ({{ areas.length }})</span>
          </button>
        </div>
      </div>
    </div>

    <div
      v-if="!isLoading && areas.length > 0"
      class="grid grid-cols-1 sm:grid-cols-3 border-b border-border/70 divide-y sm:divide-y-0 sm:divide-x divide-border/70 bg-muted/15"
    >
      <div class="px-4 py-2">
        <span class="text-micro font-semibold uppercase tracking-wider text-muted-foreground block">
          Dotación Total
        </span>
        <p class="font-semibold font-mono text-foreground text-meta mt-0.5 tabular-nums">
          {{ totalPersonal }}
          <span class="font-sans font-normal text-micro text-muted-foreground">servidores activos</span>
        </p>
      </div>
      <div class="px-4 py-2 min-w-0">
        <span class="text-micro font-semibold uppercase tracking-wider text-muted-foreground block">
          Mayor Concentración
        </span>
        <button
          v-if="mayorArea"
          type="button"
          class="font-semibold text-foreground hover:text-primary text-meta mt-0.5 truncate block max-w-full text-left cursor-pointer transition-colors"
          :title="`Ver personal de ${mayorArea.nombre}`"
          @click="irAArea(mayorArea.nombre)"
        >
          {{ mayorArea.nombre }}
        </button>
        <p v-else class="text-meta text-muted-foreground mt-0.5">-</p>
      </div>
      <div class="px-4 py-2">
        <span class="text-micro font-semibold uppercase tracking-wider text-muted-foreground block">
          Participación Líder
        </span>
        <p class="font-semibold font-mono text-primary text-meta mt-0.5 tabular-nums">
          {{ mayorAreaPct }}%
          <span class="font-sans font-normal text-micro text-muted-foreground">
            ({{ mayorArea?.cantidad || 0 }} plazas)
          </span>
        </p>
      </div>
    </div>

    <div v-if="isLoading" class="p-4">
      <div :class="[chartHeight, 'bg-muted/60 rounded-lg animate-pulse']" aria-busy="true"></div>
    </div>

    <div v-else-if="areas.length === 0" class="py-10 text-center space-y-1.5 text-muted-foreground text-meta">
      <IconBuildingSkyscraper class="size-6 mx-auto text-muted-foreground/40" aria-hidden="true" />
      <p class="font-semibold text-foreground text-micro">Sin áreas registradas</p>
      <p class="text-micro">No se encontraron unidades orgánicas con personal asignado.</p>
    </div>

    <div v-else-if="activeView === 'chart'" class="p-3.5 sm:p-4">
      <div :class="['w-full relative', chartHeight]">
        <Bar
          :data="chartData"
          :options="chartOptions"
          aria-label="Gráfico de distribución de personal por área"
        />
      </div>
      <div class="mt-2.5 pt-2 border-t border-border/60 flex flex-wrap items-center justify-between gap-2 text-micro text-muted-foreground">
        <span>Mostrando las {{ topAreas.length }} áreas con mayor dotación activa (clic en una barra para abrir su nómina).</span>
        <button
          v-if="areas.length > topAreas.length"
          type="button"
          class="text-primary hover:underline font-medium cursor-pointer"
          @click="activeView = 'list'"
        >
          Ver las {{ areas.length }} áreas →
        </button>
      </div>
    </div>

    <div v-else>
      <div class="p-2.5 border-b border-border flex flex-wrap items-center justify-between gap-2">
        <label class="relative flex-1 min-w-48 max-w-md">
          <IconSearch class="size-3 absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <input
            v-model="searchQuery"
            type="search"
            placeholder="Buscar área o gerencia por nombre..."
            class="w-full ps-8 pe-3 h-control-sm py-0 rounded-md bg-muted border-transparent text-micro text-foreground placeholder:text-muted-foreground focus:bg-card focus:border-primary focus:ring-primary"
          />
        </label>
        <span class="text-micro text-muted-foreground">
          <strong class="font-semibold text-foreground font-mono">{{ filteredAreas.length }}</strong>
          {{ filteredAreas.length === 1 ? 'resultado' : 'resultados' }}
        </span>
      </div>

      <div v-if="filteredAreas.length === 0" class="py-8 text-center text-micro text-muted-foreground">
        Ningún área coincide con "{{ searchQuery }}".
      </div>

      <div v-else class="max-h-80 overflow-y-auto overscroll-contain">
        <table class="w-full text-left text-meta" aria-label="Directorio de personal por área">
          <thead class="sticky top-0 z-10 bg-card border-b border-border text-micro text-muted-foreground select-none">
            <tr>
              <th scope="col" class="px-3 py-2 font-medium w-10">#</th>
              <th scope="col" class="px-3 py-2 font-medium">Área / Unidad Orgánica</th>
              <th scope="col" class="px-3 py-2 font-medium hidden sm:table-cell w-36">Proporción</th>
              <th scope="col" class="px-3 py-2 font-medium text-right w-28">Servidores</th>
              <th scope="col" class="px-3 py-2 font-medium text-right w-20">Filtro</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <tr
              v-for="(area, idx) in filteredAreas"
              :key="area.nombre"
              class="transition-colors hover:bg-muted/40"
              :class="selectedArea === area.nombre ? 'bg-primary/5' : ''"
            >
              <td class="px-3 py-2 font-mono text-nano text-muted-foreground tabular-nums">
                {{ idx + 1 }}
              </td>
              <td class="px-3 py-2 min-w-0">
                <button
                  type="button"
                  class="text-left font-medium text-micro text-foreground hover:text-primary hover:underline inline-flex items-center gap-1.5 cursor-pointer transition-colors wrap-break-word"
                  :title="`Abrir nómina de ${area.nombre}`"
                  @click="irAArea(area.nombre)"
                >
                  <span>{{ area.nombre }}</span>
                  <IconExternalLink class="size-3 shrink-0 opacity-60" aria-hidden="true" />
                </button>
              </td>
              <td class="px-3 py-2 hidden sm:table-cell">
                <div class="w-full bg-muted rounded-full h-1.5 overflow-hidden">
                  <div
                    class="h-full rounded-full bg-primary transition-all duration-300"
                    :style="{ width: `${(area.cantidad / maxCantidad) * 100}%` }"
                  />
                </div>
              </td>
              <td class="px-3 py-2 text-right whitespace-nowrap">
                <span class="font-mono font-semibold text-micro text-foreground tabular-nums">
                  {{ area.cantidad }}
                </span>
                <span class="font-mono text-nano text-muted-foreground tabular-nums ml-1">
                  ({{ getAreaPct(area.cantidad) }}%)
                </span>
              </td>
              <td class="px-3 py-2 text-right whitespace-nowrap">
                <button
                  type="button"
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded text-nano font-medium transition cursor-pointer"
                  :class="
                    selectedArea === area.nombre
                      ? 'bg-primary text-primary-foreground font-semibold'
                      : 'bg-muted text-muted-foreground hover:text-foreground'
                  "
                  :title="selectedArea === area.nombre ? 'Quitar filtro de área' : 'Filtrar movimientos por esta área'"
                  @click="toggleFiltroArea(area.nombre)"
                >
                  <IconFilter class="size-2.5" aria-hidden="true" />
                  <span>{{ selectedArea === area.nombre ? 'Activo' : 'Filtrar' }}</span>
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
