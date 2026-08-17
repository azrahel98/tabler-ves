<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useRoute, useRouter, RouterLink } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useRegimenStore } from '@/stores/regimen'
import TablaTrabajadores from '@/components/comun/TablaTrabajadores.vue'
import {
  IconBriefcase,
  IconBuildingCommunity,
  IconUsers,
  IconRefresh,
  IconAlertTriangle,
  IconArrowLeft,
  IconChartPie,
  IconLayersSubtract,
} from '@tabler/icons-vue'

const route = useRoute()
const router = useRouter()
const regimenStore = useRegimenStore()

const {
  identificadorRegimen,
  estaCargando,
  error,
  trabajadoresRegimen,
  totalGeneral,
  totalRegimen,
  porcentajeMunicipal,
  distribucionAreas,
  distribucionCargos,
  areaPrincipal,
} = storeToRefs(regimenStore)

const { cargarRegimen } = regimenStore

const maximoArea = computed(() => {
  if (!distribucionAreas.value.length) return 1
  return Math.max(...distribucionAreas.value.map((a) => a.cantidad), 1)
})

const maximoCargo = computed(() => {
  if (!distribucionCargos.value.length) return 1
  return Math.max(...distribucionCargos.value.map((c) => c.cantidad), 1)
})

onMounted(async () => {
  const param = route.params.id || route.params.nombre
  if (param && typeof param === 'string') {
    await cargarRegimen(param)
  }
})

watch(
  () => route.params.id || route.params.nombre,
  async (nuevoId) => {
    if (nuevoId && typeof nuevoId === 'string') {
      await cargarRegimen(nuevoId)
    }
  },
)
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-5 max-w-[1600px] mx-auto">
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <button
          type="button"
          @click="router.back()"
          class="flex h-9 w-9 items-center justify-center rounded-xl border border-gray-200 bg-white text-gray-600 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors shadow-xs cursor-pointer"
          title="Regresar">
          <IconArrowLeft class="h-4 w-4" />
        </button>

        <div class="flex items-center gap-2.5">
          <div
            class="flex h-9 w-9 items-center justify-center rounded-xl bg-indigo-50 text-indigo-600 dark:bg-indigo-900/20 dark:text-indigo-400">
            <IconBriefcase class="h-5 w-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h1 class="text-sm sm:text-base font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                {{ identificadorRegimen || 'Régimen Laboral' }}
              </h1>
              <span
                v-if="!estaCargando"
                class="inline-flex items-center gap-1 rounded-md bg-indigo-50 px-2 py-0.5 text-2xs font-bold text-indigo-700 dark:bg-indigo-900/40 dark:text-indigo-300 border border-indigo-100 dark:border-indigo-800">
                {{ totalRegimen }} trabajadores
              </span>
            </div>
            <p class="text-2xs font-medium text-gray-400">
              Detalle de dependencias, puestos y padrón asignado a este régimen normativo
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          @click="cargarRegimen(identificadorRegimen)"
          :disabled="estaCargando"
          class="inline-flex items-center gap-1.5 rounded-xl border border-gray-200 bg-white px-3 py-2 text-xs font-semibold text-gray-700 shadow-xs hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-200 dark:hover:bg-gray-700 transition-colors cursor-pointer">
          <IconRefresh :class="['h-3.5 w-3.5', estaCargando && 'animate-spin']" />
          <span>Actualizar</span>
        </button>
      </div>
    </div>

    <div
      v-if="error"
      class="flex items-center justify-between rounded-xl border border-red-200 bg-red-50 px-4 py-3 dark:border-red-900/50 dark:bg-red-950/20">
      <div class="flex items-center gap-2.5">
        <IconAlertTriangle class="h-4 w-4 shrink-0 text-red-500" />
        <span class="text-xs text-red-700 dark:text-red-400">{{ error }}</span>
      </div>
      <button
        type="button"
        @click="cargarRegimen(identificadorRegimen)"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-medium text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <div v-if="estaCargando && totalRegimen === 0" class="space-y-5 animate-pulse">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <div v-for="i in 4" :key="i" class="h-24 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="h-96 rounded-xl bg-gray-200 dark:bg-gray-800" />
    </div>

    <template v-else>
      <div class="grid grid-cols-2 gap-3 sm:grid-cols-2 lg:grid-cols-4">
        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-indigo-50 text-indigo-600 dark:bg-indigo-950/50 dark:text-indigo-400">
              <IconUsers class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-gray-900 dark:text-white tabular-nums leading-none">
              {{ totalRegimen }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Personal en el Régimen
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              Servidores activos
            </span>
          </div>
        </div>

        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-blue-50 text-blue-600 dark:bg-blue-950/50 dark:text-blue-400">
              <IconChartPie class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-blue-700 dark:text-blue-400 tabular-nums leading-none">
              {{ porcentajeMunicipal }}%
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Representatividad
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              De {{ totalGeneral }} trabajadores
            </span>
          </div>
        </div>

        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-amber-50 text-amber-600 dark:bg-amber-950/50 dark:text-amber-400">
              <IconBuildingCommunity class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-amber-700 dark:text-amber-400 tabular-nums leading-none">
              {{ areaPrincipal ? areaPrincipal.cantidad : 0 }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Área Principal
            </span>
            <span class="text-2xs text-amber-600/80 dark:text-amber-400/80 font-medium block truncate mt-0.5">
              {{ areaPrincipal?.nombre || 'Sin registros' }}
            </span>
          </div>
        </div>

        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-emerald-50 text-emerald-600 dark:bg-emerald-950/50 dark:text-emerald-400">
              <IconLayersSubtract class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-emerald-700 dark:text-emerald-400 tabular-nums leading-none">
              {{ distribucionAreas.length }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Áreas Asignadas
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              Dependencias distintas
            </span>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div
          class="rounded-xl border border-gray-200 bg-white p-4 sm:p-5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col h-min">
          <div>
            <div class="flex items-center justify-between pb-3 border-b border-gray-100 dark:border-gray-700">
              <div class="flex items-center gap-2">
                <div
                  class="flex h-6 w-6 items-center justify-center rounded-md bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400">
                  <IconBuildingCommunity class="h-3.5 w-3.5" />
                </div>
                <h2 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                  Distribución por Áreas
                </h2>
              </div>
              <span class="text-2xs font-medium text-gray-400">{{ distribucionAreas.length }} áreas</span>
            </div>

            <div v-if="distribucionAreas.length === 0" class="py-8 text-center text-xs text-gray-400">
              No hay áreas asignadas para este régimen
            </div>

            <div v-else class="mt-4 space-y-2.5 max-h-72 overflow-y-auto pr-1">
              <RouterLink
                v-for="area in distribucionAreas"
                :key="area.nombre"
                :to="`/area/${encodeURIComponent(area.nombre)}`"
                class="block space-y-1 p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors cursor-pointer"
                title="Ver detalle del área">
                <div class="flex items-center justify-between text-2xs">
                  <span class="font-semibold text-gray-800 dark:text-gray-200 truncate max-w-[70%]">
                    {{ area.nombre }}
                  </span>
                  <div class="flex items-center gap-1.5">
                    <span class="font-mono font-bold text-gray-900 dark:text-white">{{ area.cantidad }}</span>
                    <span class="text-gray-400">({{ Math.round((area.cantidad / (totalRegimen || 1)) * 100) }}%)</span>
                  </div>
                </div>
                <div class="h-2 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <div
                    class="h-full rounded-full bg-blue-600 transition-all duration-300"
                    :style="{ width: `${(area.cantidad / maximoArea) * 100}%` }" />
                </div>
              </RouterLink>
            </div>
          </div>
        </div>

        <div
          class="rounded-xl border border-gray-200 bg-white p-4 sm:p-5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col h-min">
          <div>
            <div class="flex items-center justify-between pb-3 border-b border-gray-100 dark:border-gray-700">
              <div class="flex items-center gap-2">
                <div
                  class="flex h-6 w-6 items-center justify-center rounded-md bg-indigo-50 text-indigo-600 dark:bg-indigo-900/30 dark:text-indigo-400">
                  <IconUsers class="h-3.5 w-3.5" />
                </div>
                <h2 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                  Principales Cargos
                </h2>
              </div>
              <span class="text-2xs font-medium text-gray-400">{{ distribucionCargos.length }} puestos</span>
            </div>

            <div v-if="distribucionCargos.length === 0" class="py-8 text-center text-xs text-gray-400">
              No hay cargos registrados para este régimen
            </div>

            <div v-else class="mt-4 space-y-2.5 max-h-72 overflow-y-auto pr-1">
              <div
                v-for="cargo in distribucionCargos"
                :key="cargo.nombre"
                class="space-y-1 p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                <div class="flex items-center justify-between text-2xs">
                  <span class="font-semibold text-gray-800 dark:text-gray-200 truncate max-w-[70%]">
                    {{ cargo.nombre }}
                  </span>
                  <div class="flex items-center gap-1.5">
                    <span class="font-mono font-bold text-gray-900 dark:text-white">{{ cargo.cantidad }}</span>
                    <span class="text-gray-400">({{ Math.round((cargo.cantidad / (totalRegimen || 1)) * 100) }}%)</span>
                  </div>
                </div>
                <div class="h-2 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <div
                    class="h-full rounded-full bg-indigo-500 transition-all duration-300"
                    :style="{ width: `${(cargo.cantidad / maximoCargo) * 100}%` }" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <TablaTrabajadores
        :trabajadores="trabajadoresRegimen"
        titulo="Personal Asignado"
        :subtitulo="`Trabajadores activos en ${identificadorRegimen}`"
        :mostrar-filtro-area="true"
        :mostrar-columna-regimen="false" />
    </template>
  </div>
</template>
