<script setup lang="ts">
import { onMounted, defineAsyncComponent } from 'vue'
import { storeToRefs } from 'pinia'
import WidgetCumpleanos from '@/components/dashboard/WidgetCumpleanos.vue'
import WidgetTrabajadoresRecientes from '@/components/dashboard/WidgetTrabajadoresRecientes.vue'
import { useDashboardStore } from '@/stores/dashboard'
import { IconAlertTriangle, IconRefresh } from '@tabler/icons-vue'

const CardDistribucionRegimen = defineAsyncComponent(() => import('@/components/dashboard/CardDistribucionRegimen.vue'))
const CardDistribucionGenero = defineAsyncComponent(() => import('@/components/dashboard/CardDistribucionGenero.vue'))
const CardAfiliacionSindical = defineAsyncComponent(() => import('@/components/dashboard/CardAfiliacionSindical.vue'))
const GraficoBarrasAreas = defineAsyncComponent(() => import('@/components/dashboard/GraficoBarrasAreas.vue'))
const CardDistribucionEdad = defineAsyncComponent(() => import('@/components/dashboard/CardDistribucionEdad.vue'))
const CardRangosAntiguedad = defineAsyncComponent(() => import('@/components/dashboard/CardRangosAntiguedad.vue'))
const MapaDistritos = defineAsyncComponent(() => import('@/components/dashboard/MapaDistritos.vue'))

const dashboardStore = useDashboardStore()
const {
  resumen,
  cumpleanos,
  areaReport,
  trabajadoresNuevos,
  rangosEdad,
  rangosAntiguedad,
  activosDistrito,
  isLoading: estaCargando,
  error: errorCarga,
} = storeToRefs(dashboardStore)

const cargarDashboard = dashboardStore.fetchDashboard

onMounted(async () => {
  await cargarDashboard()
})
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-5 max-w-[1600px] mx-auto">
    <div
      v-if="errorCarga"
      class="flex items-center justify-between rounded-xl border border-red-200 bg-red-50 px-4 py-3 dark:border-red-900/50 dark:bg-red-950/20">
      <div class="flex items-center gap-2.5">
        <IconAlertTriangle class="h-4 w-4 shrink-0 text-red-500" />
        <span class="text-xs text-red-700 dark:text-red-400">{{ errorCarga }}</span>
      </div>
      <button
        type="button"
        @click="cargarDashboard(true)"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-medium text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <div v-if="estaCargando && !resumen" class="space-y-5 animate-pulse">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="i in 3" :key="i" class="h-64 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="i in 3" :key="i" class="h-64 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="h-96 rounded-xl bg-gray-200 dark:bg-gray-800" />
    </div>

    <template v-else>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 items-start">
        <CardDistribucionRegimen :resumen="resumen" />
        <CardDistribucionGenero :resumen="resumen" />
        <CardAfiliacionSindical :resumen="resumen" />
      </div>

      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2 items-start">
        <GraficoBarrasAreas :areas="areaReport" />
        <WidgetTrabajadoresRecientes :trabajadores="trabajadoresNuevos" />
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 items-start">
        <CardDistribucionEdad :rangos-edad="rangosEdad" />
        <CardRangosAntiguedad :rangos-antiguedad="rangosAntiguedad" />
        <WidgetCumpleanos :cumpleanos="cumpleanos" />
      </div>

      <div>
        <MapaDistritos :activos-distrito="activosDistrito" />
      </div>
    </template>
  </div>
</template>
