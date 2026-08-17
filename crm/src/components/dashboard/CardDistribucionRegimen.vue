<script setup lang="ts">
import { computed } from 'vue'
import { PolarArea } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, RadialLinearScale } from 'chart.js'
import type { ResumenData, CantidadNombre } from '@/api/types'
import { IconBriefcase } from '@tabler/icons-vue'

ChartJS.register(Title, Tooltip, Legend, ArcElement, RadialLinearScale)

const propiedades = defineProps<{
  resumen?: ResumenData | null
}>()

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

const datosRegimen = computed(() => {
  const lista = agruparDatos(propiedades.resumen?.por_regimen || [], 5)
  return {
    labels: lista.map((i) => i.nombre || 'Sin Régimen'),
    datasets: [
      {
        data: lista.map((i) => i.cantidad),
        backgroundColor: [
          'rgba(66, 42, 251, 0.85)',
          'rgba(99, 102, 241, 0.85)',
          'rgba(16, 185, 129, 0.85)',
          'rgba(20, 184, 166, 0.85)',
          'rgba(139, 92, 246, 0.85)',
          'rgba(148, 163, 184, 0.85)',
        ],
        borderWidth: 2,
        borderColor: '#ffffff',
      },
    ],
  }
})

const opcionesPolar = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom' as const,
      labels: {
        usePointStyle: true,
        padding: 10,
        font: { size: 10, weight: 600 },
      },
    },
    tooltip: {
      padding: 10,
      cornerRadius: 8,
      bodyFont: { size: 12, weight: 600 },
    },
  },
  scales: {
    r: {
      ticks: { display: false },
      grid: { color: 'rgba(226, 232, 240, 0.6)' },
    },
  },
}
</script>

<template>
  <div class="rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2">
        <div
          class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconBriefcase class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">Régimen Laboral</h3>
          <p class="text-2xs font-medium text-gray-400">Distribución por modalidad</p>
        </div>
      </div>

      <span
        class="inline-flex items-center rounded-md bg-gray-100 px-2 py-0.5 text-2xs font-bold text-gray-600 dark:bg-gray-700 dark:text-gray-300">
        {{ resumen?.por_regimen?.length || 0 }} regímenes
      </span>
    </div>

    <div class="p-4 space-y-3">
      <div class="h-56 relative flex items-center justify-center">
        <div v-if="!resumen?.por_regimen || resumen.por_regimen.length === 0" class="text-xs text-gray-400">
          Sin información disponible
        </div>
        <PolarArea v-else :data="datosRegimen" :options="opcionesPolar" />
      </div>

      <div
        v-if="resumen?.por_regimen && resumen.por_regimen.length > 0"
        class="flex flex-wrap gap-1.5 pt-2 border-t border-gray-100 dark:border-gray-700">
        <RouterLink
          v-for="item in resumen.por_regimen"
          :key="item.nombre"
          :to="`/regimen/${encodeURIComponent(item.nombre)}`"
          class="inline-flex items-center gap-1 rounded-md bg-gray-50 px-2 py-1 text-2xs font-semibold text-gray-700 hover:bg-indigo-50 hover:text-indigo-700 dark:bg-gray-700/50 dark:text-gray-200 dark:hover:bg-indigo-950/40 dark:hover:text-indigo-300 transition-colors">
          <span>{{ item.nombre }}</span>
          <span class="font-mono text-2xs text-gray-400">({{ item.cantidad }})</span>
        </RouterLink>
      </div>
    </div>
  </div>
</template>
