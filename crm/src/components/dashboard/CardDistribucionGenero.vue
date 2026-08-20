<script setup lang="ts">
import { ref, computed } from 'vue'
import { Doughnut } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement } from 'chart.js'
import type { ResumenData } from '@/api/types'
import { IconGenderMale, IconGenderFemale, IconChartBar, IconChartPie } from '@tabler/icons-vue'

ChartJS.register(Title, Tooltip, Legend, ArcElement)

const propiedades = defineProps<{
  resumen?: ResumenData | null
}>()

const modoVistaSexo = ref<'tarjetas' | 'grafico'>('tarjetas')

const totalActivos = computed(() => propiedades.resumen?.activos || 1)

const datosSexoProcesados = computed(() => {
  const lista = propiedades.resumen?.por_sexo || []
  const masculino = lista.find((i) => i.nombre === 'M')?.cantidad || 0
  const femenino = lista.find((i) => i.nombre === 'F')?.cantidad || 0
  const total = totalActivos.value
  const pctMasculino = Math.round((masculino / total) * 100)
  const pctFemenino = Math.round((femenino / total) * 100)
  const ratio = femenino > 0 ? (masculino / femenino).toFixed(1) : '1:0'
  return { masculino, femenino, pctMasculino, pctFemenino, ratio, total }
})

const datosSexoDoughnut = computed(() => {
  const { masculino, femenino } = datosSexoProcesados.value
  return {
    labels: ['Masculino', 'Femenino'],
    datasets: [
      {
        data: [masculino, femenino],
        backgroundColor: ['#3b82f6', '#818cf8'],
        hoverBackgroundColor: ['#2563eb', '#6366f1'],
        borderWidth: 3,
        borderColor: '#ffffff',
        borderRadius: 4,
      },
    ],
  }
})

const opcionesDoughnut = {
  responsive: true,
  maintainAspectRatio: false,
  cutout: '72%',
  plugins: {
    legend: { display: false },
    tooltip: {
      padding: 10,
      cornerRadius: 8,
      bodyFont: { size: 12, weight: 600 },
    },
  },
}
</script>

<template>
  <div class="flex flex-col rounded-xl border border-gray-200 bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800 h-min">
    <div class="flex items-center justify-between border-b border-gray-100 px-4 py-3 dark:border-gray-700">
      <div class="flex items-center gap-2">
        <div
          class="flex h-7 w-7 shrink-0 items-center justify-center rounded-lg bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
          <IconGenderMale class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
            Distribución por Género
          </h3>
          <p class="text-2xs font-medium text-gray-400">Balance y paridad</p>
        </div>
      </div>
      <div class="flex items-center rounded-lg bg-gray-100 p-0.5 dark:bg-gray-700">
        <button
          type="button"
          @click="modoVistaSexo = 'tarjetas'"
          :class="
            modoVistaSexo === 'tarjetas'
              ? 'bg-white text-blue-600 shadow-sm dark:bg-gray-600 dark:text-white'
              : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
          "
          class="rounded-md p-1.5 transition-all cursor-pointer"
          title="Vista KPI">
          <IconChartPie class="h-3.5 w-3.5" />
        </button>
        <button
          type="button"
          @click="modoVistaSexo = 'grafico'"
          :class="
            modoVistaSexo === 'grafico'
              ? 'bg-white text-blue-600 shadow-sm dark:bg-gray-600 dark:text-white'
              : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300'
          "
          class="rounded-md p-1.5 transition-all cursor-pointer"
          title="Vista Anillo">
          <IconChartBar class="h-3.5 w-3.5" />
        </button>
      </div>
    </div>

    <div class="p-4">
      <div v-if="modoVistaSexo === 'tarjetas'" class="space-y-4">
        <div class="grid grid-cols-2 gap-3">
          <div class="rounded-lg border border-gray-200 bg-gray-50 p-3 dark:border-gray-700 dark:bg-gray-900/50">
            <div class="flex items-center justify-between">
              <div class="rounded-md bg-blue-100 p-1 text-blue-600 dark:bg-blue-900/40 dark:text-blue-400">
                <IconGenderMale class="h-4 w-4" />
              </div>
              <span
                class="inline-flex items-center rounded-md bg-blue-50 px-1.5 py-0.5 text-2xs font-bold text-blue-700 dark:bg-blue-900/30 dark:text-blue-300">
                {{ datosSexoProcesados.pctMasculino }}%
              </span>
            </div>
            <p class="mt-2 text-2xs font-bold uppercase tracking-wider text-gray-400">Masculino</p>
            <p class="text-lg font-bold font-mono text-gray-900 dark:text-white mt-0.5">
              {{ datosSexoProcesados.masculino }}
            </p>
          </div>

          <div class="rounded-lg border border-gray-200 bg-gray-50 p-3 dark:border-gray-700 dark:bg-gray-900/50">
            <div class="flex items-center justify-between">
              <div class="rounded-md bg-purple-100 p-1 text-purple-600 dark:bg-purple-900/40 dark:text-purple-400">
                <IconGenderFemale class="h-4 w-4" />
              </div>
              <span
                class="inline-flex items-center rounded-md bg-purple-50 px-1.5 py-0.5 text-2xs font-bold text-purple-700 dark:bg-purple-900/30 dark:text-purple-300">
                {{ datosSexoProcesados.pctFemenino }}%
              </span>
            </div>
            <p class="mt-2 text-2xs font-bold uppercase tracking-wider text-gray-400">Femenino</p>
            <p class="text-lg font-bold font-mono text-gray-900 dark:text-white mt-0.5">
              {{ datosSexoProcesados.femenino }}
            </p>
          </div>
        </div>

        <div
          class="space-y-1.5 rounded-lg border border-gray-200 bg-gray-50 p-2.5 dark:border-gray-700 dark:bg-gray-900/50">
          <div class="flex items-center justify-between text-2xs text-gray-600 dark:text-gray-400">
            <span class="flex items-center gap-1 font-medium">
              <span class="h-2 w-2 rounded-full bg-blue-500" />
              Masculino
            </span>
            <span class="text-2xs text-gray-400 font-mono">Ratio: {{ datosSexoProcesados.ratio }}</span>
            <span class="flex items-center gap-1 font-medium">
              <span class="h-2 w-2 rounded-full bg-purple-500" />
              Femenino
            </span>
          </div>
          <div class="flex h-2 w-full overflow-hidden rounded-full bg-gray-200 dark:bg-gray-700">
            <div
              class="h-full bg-blue-500 transition-all duration-500"
              :style="{ width: `${datosSexoProcesados.pctMasculino}%` }" />
            <div
              class="h-full bg-purple-500 transition-all duration-500"
              :style="{ width: `${datosSexoProcesados.pctFemenino}%` }" />
          </div>
        </div>
      </div>

      <div v-else class="h-52 relative flex items-center justify-center">
        <Doughnut :data="datosSexoDoughnut" :options="opcionesDoughnut" />
        <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
          <span class="text-2xs text-gray-400 font-medium">Total</span>
          <span class="text-lg font-bold font-mono text-gray-900 dark:text-white">{{ datosSexoProcesados.total }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
