<script setup lang="ts">
import { ref, computed } from 'vue'
import { RouterLink } from 'vue-router'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'
import type { ResumenData, CantidadNombre } from '@/api/types'
import { IconBuildingCommunity, IconChartBar, IconList, IconUserCheck } from '@tabler/icons-vue'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const propiedades = defineProps<{
  resumen?: ResumenData | null
}>()

const modoVistaSindicatos = ref<'grafico' | 'lista'>('grafico')

const totalActivos = computed(() => propiedades.resumen?.activos || 1)

function agruparDatos(lista: CantidadNombre[], limite = 5): CantidadNombre[] {
  if (!lista || lista.length <= limite) return lista
  const ordenados = [...lista].sort((a, b) => b.cantidad - a.cantidad)
  const principales = ordenados.slice(0, limite)
  const restantes = ordenados.slice(limite)
  const sumaOtros = restantes.reduce((acc, curr) => acc + curr.cantidad, 0)
  if (sumaOtros > 0) {
    principales.push({ nombre: 'Otros', cantidad: sumaOtros })
  }
  return principales
}

const analisisSindicatos = computed(() => {
  const lista = propiedades.resumen?.por_sindicato || []
  const total = totalActivos.value
  const noAfiliadoItem = lista.find((i) => !i.nombre || i.nombre.toLowerCase().includes('no afiliado'))
  const noAfiliados = noAfiliadoItem ? noAfiliadoItem.cantidad : 0
  const afiliados = total - noAfiliados
  const pctAfiliados = Math.round((afiliados / total) * 100)
  const procesados = lista.map((item) => ({
    nombre: item.nombre || 'No Afiliado',
    cantidad: item.cantidad,
    porcentaje: Math.round((item.cantidad / total) * 100),
  }))
  return { total, afiliados, noAfiliados, pctAfiliados, procesados }
})

const datosSindicatoBar = computed(() => {
  const lista = agruparDatos(propiedades.resumen?.por_sindicato || [], 5)
  const paleta = [
    'rgba(16, 185, 129, 0.85)',
    'rgba(99, 102, 241, 0.85)',
    'rgba(245, 158, 11, 0.85)',
    'rgba(236, 72, 153, 0.85)',
    'rgba(59, 130, 246, 0.85)',
    'rgba(148, 163, 184, 0.85)',
  ]
  return {
    labels: lista.map((i) => i.nombre || 'No Afiliado'),
    datasets: [
      {
        label: 'Miembros',
        data: lista.map((i) => i.cantidad),
        backgroundColor: paleta.slice(0, lista.length),
        borderRadius: 6,
        borderSkipped: false,
      },
    ],
  }
})

const opcionesBarraHorizontal = {
  responsive: true,
  maintainAspectRatio: false,
  indexAxis: 'y' as const,
  plugins: {
    legend: { display: false },
    tooltip: {
      padding: 10,
      cornerRadius: 8,
      bodyFont: { size: 12, weight: 600 },
    },
  },
  scales: {
    x: {
      grid: { display: false },
      ticks: { font: { size: 10 } },
    },
    y: {
      grid: { display: false },
      ticks: { font: { size: 10, weight: 600 } },
    },
  },
}
</script>

<template>
  <div class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2">
        <div
          class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg bg-emerald-50 text-emerald-600 dark:bg-emerald-900/20 dark:text-emerald-400">
          <IconBuildingCommunity class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">Afiliación Sindical</h3>
          <p class="text-2xs font-medium text-gray-400">Distribución de sindicatos</p>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <span
          class="inline-flex items-center rounded-md bg-emerald-50 px-2 py-0.5 text-2xs font-bold text-emerald-700 dark:bg-emerald-900/20 dark:text-emerald-400">
          {{ analisisSindicatos.pctAfiliados }}% afiliados
        </span>
        <div class="flex items-center rounded-lg bg-gray-100 p-0.5 dark:bg-gray-700">
          <button
            type="button"
            @click="modoVistaSindicatos = 'grafico'"
            :class="
              modoVistaSindicatos === 'grafico'
                ? 'bg-white text-emerald-600 shadow-sm dark:bg-gray-600 dark:text-white'
                : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
            "
            class="rounded-md p-1.5 transition-all cursor-pointer"
            title="Vista Gráfico">
            <IconChartBar class="h-3.5 w-3.5" />
          </button>
          <button
            type="button"
            @click="modoVistaSindicatos = 'lista'"
            :class="
              modoVistaSindicatos === 'lista'
                ? 'bg-white text-emerald-600 shadow-sm dark:bg-gray-600 dark:text-white'
                : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
            "
            class="rounded-md p-1.5 transition-all cursor-pointer"
            title="Vista Lista">
            <IconList class="h-3.5 w-3.5" />
          </button>
        </div>
      </div>
    </div>

    <div class="p-4">
      <div v-if="modoVistaSindicatos === 'grafico'" class="h-56 relative flex items-center justify-center">
        <div v-if="!resumen?.por_sindicato || resumen.por_sindicato.length === 0" class="text-sm text-gray-400">
          Sin sindicatos registrados
        </div>
        <Bar v-else :data="datosSindicatoBar" :options="opcionesBarraHorizontal" />
      </div>

      <div v-else class="max-h-[220px] overflow-y-auto space-y-1.5 custom-scrollbar">
        <div
          v-if="!resumen?.por_sindicato || resumen.por_sindicato.length === 0"
          class="py-6 text-center text-sm text-gray-400">
          Sin sindicatos registrados
        </div>

        <RouterLink
          v-for="item in analisisSindicatos.procesados"
          :key="item.nombre"
          :to="`/sindicato/${encodeURIComponent(item.nombre)}`"
          class="block rounded-lg border border-gray-100 bg-gray-50 p-2 dark:border-gray-700 dark:bg-gray-900/50 space-y-1 hover:border-emerald-200 dark:hover:border-emerald-800 hover:bg-emerald-50/30 dark:hover:bg-emerald-950/20 transition-all cursor-pointer">
          <div class="flex items-center justify-between">
            <span class="flex items-center gap-1.5 text-xs font-medium text-gray-900 dark:text-gray-200">
              <IconUserCheck v-if="item.nombre !== 'No Afiliado'" class="h-3.5 w-3.5 text-emerald-500" />
              {{ item.nombre }}
            </span>
            <div class="flex items-center gap-1.5">
              <span class="text-xs text-gray-400">{{ item.porcentaje }}%</span>
              <span
                class="rounded bg-emerald-100 px-1.5 py-0.5 font-mono text-xs font-semibold text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400">
                {{ item.cantidad }}
              </span>
            </div>
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
            <div
              class="h-full rounded-full bg-emerald-500 transition-all duration-500"
              :style="{ width: `${item.porcentaje}%` }" />
          </div>
        </RouterLink>
      </div>
    </div>
  </div>
</template>
