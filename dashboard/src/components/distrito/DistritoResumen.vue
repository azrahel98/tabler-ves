<script setup lang="ts">
import { computed, ref } from 'vue'
import { Line, Doughnut } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import { useThemeStore } from '@/stores/theme'
import { useChartScale } from '@/lib/chart'

interface Item {
  nombre: string
  cantidad: number
}

const props = defineProps<{
  distrito: string
  total: number
  areas: Item[]
  rangos: Item[]
  regimenes: Item[]
  sindicatos: Item[]
}>()

const emit = defineEmits<{ (e: 'select-area', nombre: string): void }>()

type Tab = 'areas' | 'edad' | 'regimen' | 'sindicato'
const tab = ref<Tab>('areas')
const verTodo = ref(false)
const LIMITE = 5

const tabs = computed(() => [
  { id: 'areas' as Tab, label: 'Áreas', n: props.areas.length },
  { id: 'edad' as Tab, label: 'Edad', n: props.rangos.length },
  { id: 'regimen' as Tab, label: 'Régimen', n: props.regimenes.length },
  { id: 'sindicato' as Tab, label: 'Sindicato', n: props.sindicatos.length },
])

const fuente = computed(() => ({ areas: props.areas, edad: props.rangos, regimen: props.regimenes, sindicato: props.sindicatos })[tab.value])
const ordenados = computed(() =>
  tab.value === 'edad' ? fuente.value : [...fuente.value].sort((a, b) => b.cantidad - a.cantidad)
)
const ordenadosReg = computed(() =>
  [...(tab.value === 'sindicato' ? props.sindicatos : props.regimenes)].sort((a, b) => b.cantidad - a.cantidad)
)
const visibles = computed(() => (verTodo.value ? ordenados.value : ordenados.value.slice(0, LIMITE)))
const max = computed(() => Math.max(...fuente.value.map((i) => i.cantidad), 1))
const pct = (c: number) => (props.total ? Math.round((c / props.total) * 100) : 0)

const { chartFontSize } = useChartScale()
const themeStore = useThemeStore()
const isDark = computed(() => themeStore.isDark)
const textColor = computed(() => (isDark.value ? '#94a3b8' : '#475569'))
const gridColor = computed(() => (isDark.value ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.06)'))
const font = computed(() => ({ family: 'Inter, ui-sans-serif, system-ui, sans-serif', size: chartFontSize.value }))
const PALETA = ['#3b82f6', '#10b981', '#f59e0b', '#8b5cf6', '#ef4444', '#06b6d4', '#ec4899']

const tooltip = computed(() => ({
  backgroundColor: isDark.value ? '#1e293b' : '#0f172a',
  padding: 8,
  cornerRadius: 6,
  callbacks: {
    label: (ctx: { raw: unknown }) => ` ${Number(ctx.raw)} servidores (${pct(Number(ctx.raw))}%)`,
  },
}))

const RANGOS = ['18-25', '26-35', '36-45', '46-55', '55+']
const rangosCompletos = computed(() =>
  RANGOS.map((nombre) => ({ nombre, cantidad: props.rangos.find((r) => r.nombre === nombre)?.cantidad ?? 0 }))
)

const edadData = computed<ChartData<'line'>>(() => ({
  labels: rangosCompletos.value.map((r) => r.nombre),
  datasets: [
    {
      label: 'Servidores',
      data: rangosCompletos.value.map((r) => r.cantidad),
      borderColor: isDark.value ? '#38bdf8' : '#0284c7',
      backgroundColor: isDark.value ? 'rgba(56,189,248,0.18)' : 'rgba(2,132,199,0.14)',
      fill: true,
      tension: 0.4,
      borderWidth: 2,
      pointRadius: 4,
      pointHoverRadius: 6,
      pointBackgroundColor: isDark.value ? '#0f172a' : '#ffffff',
      pointBorderColor: isDark.value ? '#38bdf8' : '#0284c7',
      pointBorderWidth: 2,
    },
  ],
}))

const edadOpts = computed<ChartOptions<'line'>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  layout: { padding: { left: 4, right: 12, top: 8 } },
  plugins: { legend: { display: false }, tooltip: tooltip.value },
  scales: {
    x: { grid: { display: false }, ticks: { color: textColor.value, font: font.value } },
    y: {
      beginAtZero: true,
      grace: '15%',
      grid: { color: gridColor.value },
      ticks: { color: textColor.value, precision: 0, font: font.value },
    },
  },
}))

const regData = computed<ChartData<'doughnut'>>(() => ({
  labels: ordenadosReg.value.map((r) => r.nombre),
  datasets: [
    {
      data: ordenadosReg.value.map((r) => r.cantidad),
      backgroundColor: ordenadosReg.value.map((_, i) => PALETA[i % PALETA.length]),
      borderWidth: 0,
      hoverOffset: 4,
    },
  ],
}))

const regOpts = computed<ChartOptions<'doughnut'>>(() => ({
  responsive: true,
  maintainAspectRatio: false,
  cutout: '68%',
  plugins: { legend: { display: false }, tooltip: tooltip.value },
}))

const elegir = (t: Tab) => {
  tab.value = t
  verTodo.value = false
}
</script>

<template>
  <section class="grid lg:grid-cols-[15rem_1fr] rounded-xl border border-border bg-card shadow-2xs overflow-hidden">
    <div class="p-5 flex flex-col justify-center border-b lg:border-b-0 lg:border-r border-border bg-muted/20">
      <p class="text-micro text-muted-foreground">Servidores activos en</p>
      <p class="text-meta font-semibold text-foreground truncate" :title="distrito">{{ distrito }}</p>
      <p class="mt-3 text-5xl font-semibold tracking-tight font-mono tabular-nums text-foreground leading-none">
        {{ total }}
      </p>
    </div>

    <div class="p-4 min-w-0">
      <div class="flex items-center gap-1 mb-3" role="tablist">
        <button
          v-for="t in tabs"
          :key="t.id"
          type="button"
          role="tab"
          :aria-selected="tab === t.id"
          class="px-2.5 py-1 rounded-md text-micro cursor-pointer transition-colors"
          :class="
            tab === t.id
              ? 'bg-muted text-foreground font-semibold'
              : 'text-muted-foreground hover:text-foreground font-medium'
          "
          @click="elegir(t.id)"
        >
          {{ t.label }}
          <span class="font-mono text-nano text-muted-foreground ms-0.5">{{ t.n }}</span>
        </button>
      </div>

      <p v-if="ordenados.length === 0" class="py-6 text-center text-micro text-muted-foreground">Sin datos</p>

      <div v-else-if="tab === 'edad'" class="h-44 relative w-full">
        <Line :data="edadData" :options="edadOpts" aria-label="Distribución por edad" />
      </div>

      <div v-else-if="tab === 'regimen' || tab === 'sindicato'" class="flex flex-wrap items-center gap-x-8 gap-y-3 h-auto sm:h-44">
        <div class="size-40 relative shrink-0 mx-auto sm:mx-0">
          <Doughnut :data="regData" :options="regOpts" :aria-label="`Distribución por ${tab}`" />
          <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
            <span class="text-kpi font-semibold font-mono tabular-nums text-foreground leading-none">{{ total }}</span>
            <span class="text-nano text-muted-foreground">servidores</span>
          </div>
        </div>
        <ul class="flex-1 min-w-48 max-w-xs space-y-1.5">
          <li v-for="(r, i) in ordenadosReg" :key="r.nombre" class="flex items-center justify-between gap-3 text-micro">
            <span class="flex items-center gap-2 min-w-0">
              <span class="size-2 rounded-full shrink-0" :style="{ background: PALETA[i % PALETA.length] }" />
              <span class="truncate text-foreground" :title="r.nombre">{{ r.nombre }}</span>
            </span>
            <span class="font-mono tabular-nums shrink-0">
              <strong class="font-semibold text-foreground">{{ r.cantidad }}</strong>
              <span class="text-muted-foreground"> · {{ pct(r.cantidad) }}%</span>
            </span>
          </li>
        </ul>
      </div>

      <ul
        v-else
        class="space-y-1.5"
        :class="verTodo ? 'max-h-60 overflow-y-auto overscroll-contain pe-1' : ''"
      >
        <li v-for="item in visibles" :key="item.nombre">
          <component
            :is="tab === 'areas' ? 'button' : 'div'"
            :type="tab === 'areas' ? 'button' : undefined"
            class="relative w-full flex items-center justify-between gap-3 rounded-md px-2.5 py-1.5 text-left overflow-hidden"
            :class="tab === 'areas' ? 'cursor-pointer hover:bg-muted/60 transition-colors' : ''"
            @click="tab === 'areas' && emit('select-area', item.nombre)"
          >
            <span
              class="absolute inset-y-0 start-0 rounded-md bg-blue-500/15 transition-all duration-500"
              :style="{ width: `${(item.cantidad / max) * 100}%` }"
              aria-hidden="true"
            />
            <span class="relative text-micro text-foreground truncate" :title="item.nombre">{{ item.nombre }}</span>
            <span class="relative text-micro font-mono tabular-nums shrink-0">
              <strong class="font-semibold text-foreground">{{ item.cantidad }}</strong>
              <span class="text-muted-foreground"> · {{ pct(item.cantidad) }}%</span>
            </span>
          </component>
        </li>
      </ul>

      <button
        v-if="tab === 'areas' && ordenados.length > LIMITE"
        type="button"
        class="mt-2 text-micro text-primary font-medium hover:underline cursor-pointer"
        @click="verTodo = !verTodo"
      >
        {{ verTodo ? 'Ver menos' : `Ver las ${ordenados.length}` }}
      </button>
    </div>
  </section>
</template>
