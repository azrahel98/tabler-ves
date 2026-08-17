<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useDistritoStore } from '@/stores/distrito'
import TablaTrabajadores from '@/components/comun/TablaTrabajadores.vue'
import {
  IconMapPin,
  IconUsers,
  IconBuildingCommunity,
  IconBriefcase,
  IconCake,
  IconRefresh,
  IconAlertTriangle,
  IconChartBar,
} from '@tabler/icons-vue'

const route = useRoute()
const distritoStore = useDistritoStore()

const { detalle, estaCargando, error, totalTrabajadores, areas, rangosEdad, personas } = storeToRefs(distritoStore)

const { cargarDistrito } = distritoStore

const nombreDistritoParametro = computed(() => {
  const param = route.params.nombre
  if (Array.isArray(param)) return param[0] || ''
  return param || ''
})

const tituloDistritoFormateado = computed(() => {
  const nombre = detalle.value?.distrito || nombreDistritoParametro.value
  if (!nombre) return 'Distrito'
  const omitir = new Set(['de', 'del', 'la', 'el', 'los', 'las', 'y', 'en', 'san', 'santa'])
  return nombre
    .toLowerCase()
    .split(' ')
    .map((palabra, indice) => {
      if (indice > 0 && omitir.has(palabra)) return palabra
      return palabra.charAt(0).toUpperCase() + palabra.slice(1)
    })
    .join(' ')
})

const maximoArea = computed(() => {
  if (!areas.value.length) return 1
  return Math.max(...areas.value.map((a) => a.cantidad), 1)
})

const maximoEdad = computed(() => {
  if (!rangosEdad.value.length) return 1
  return Math.max(...rangosEdad.value.map((r) => r.cantidad), 1)
})

const distribucionRegimenes = computed(() => {
  const conteo: Record<string, number> = {}
  for (const p of personas.value) {
    const nombre = p.regimen?.nombre || 'Sin Régimen'
    conteo[nombre] = (conteo[nombre] || 0) + 1
  }
  return Object.entries(conteo)
    .map(([nombre, cantidad]) => ({ nombre, cantidad }))
    .sort((a, b) => b.cantidad - a.cantidad)
})

const maximoRegimen = computed(() => {
  if (!distribucionRegimenes.value.length) return 1
  return Math.max(...distribucionRegimenes.value.map((r) => r.cantidad), 1)
})

const areaPrincipal = computed(() => {
  if (!areas.value.length) return null
  return [...areas.value].sort((a, b) => b.cantidad - a.cantidad)[0]
})

const rangoEdadPrincipal = computed(() => {
  if (!rangosEdad.value.length) return null
  return [...rangosEdad.value].sort((a, b) => b.cantidad - a.cantidad)[0]
})

function recargar() {
  if (nombreDistritoParametro.value) {
    cargarDistrito(nombreDistritoParametro.value)
  }
}

onMounted(() => {
  if (nombreDistritoParametro.value) {
    cargarDistrito(nombreDistritoParametro.value)
  }
})

watch(
  () => route.params.nombre,
  (nuevoNombre) => {
    if (nuevoNombre && typeof nuevoNombre === 'string') {
      cargarDistrito(nuevoNombre)
    }
  },
)
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-5 max-w-[1600px] mx-auto">
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <div class="flex items-center gap-2.5">
          <div
            class="flex h-9 w-9 items-center justify-center rounded-xl bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
            <IconMapPin class="h-5 w-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h1 class="text-sm sm:text-base font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                {{ tituloDistritoFormateado }}
              </h1>
              <span
                v-if="!estaCargando && detalle"
                class="inline-flex items-center gap-1 rounded-md bg-blue-50 px-2 py-0.5 text-2xs font-bold text-blue-700 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-100 dark:border-blue-800">
                {{ totalTrabajadores }} activos
              </span>
            </div>
            <p class="text-2xs font-medium text-gray-400">
              Reporte detallado y padrón de trabajadores residentes en este distrito
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          @click="recargar"
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
        @click="recargar"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-medium text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <div v-if="estaCargando && !detalle" class="space-y-5 animate-pulse">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <div v-for="i in 4" :key="i" class="h-24 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
        <div class="h-72 rounded-xl bg-gray-200 dark:bg-gray-800" />
      </div>
      <div class="h-96 rounded-xl bg-gray-200 dark:bg-gray-800" />
    </div>

    <template v-else-if="detalle">
      <div class="grid grid-cols-2 gap-3 sm:grid-cols-2 lg:grid-cols-4">
        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-blue-50 text-blue-600 dark:bg-blue-950/50 dark:text-blue-400">
              <IconUsers class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-gray-900 dark:text-white tabular-nums leading-none">
              {{ totalTrabajadores }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Total Activos
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              En {{ tituloDistritoFormateado }}
            </span>
          </div>
        </div>

        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-indigo-50 text-indigo-600 dark:bg-indigo-950/50 dark:text-indigo-400">
              <IconBuildingCommunity class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-indigo-700 dark:text-indigo-400 tabular-nums leading-none">
              {{ areas.length }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Áreas Laborales
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              Centros de trabajo
            </span>
          </div>
        </div>

        <div
          class="rounded-2xl border border-gray-200/80 bg-white p-3.5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col justify-between transition-all hover:border-gray-300 dark:hover:border-gray-600">
          <div class="flex items-center justify-between">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-amber-50 text-amber-600 dark:bg-amber-950/50 dark:text-amber-400">
              <IconBriefcase class="h-3.5 w-3.5" />
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
              <IconCake class="h-3.5 w-3.5" />
            </div>
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-emerald-700 dark:text-emerald-400 tabular-nums leading-none">
              {{ rangoEdadPrincipal ? rangoEdadPrincipal.cantidad : 0 }}
            </span>
          </div>
          <div class="mt-3">
            <span class="text-xs sm:text-sm font-semibold text-gray-700 dark:text-gray-200 block truncate">
              Grupo Etario Mayor
            </span>
            <span class="text-2xs text-gray-400 dark:text-gray-500 block truncate mt-0.5">
              {{ rangoEdadPrincipal ? `${rangoEdadPrincipal.nombre} años` : 'Sin datos' }}
            </span>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
        <div
          class="rounded-xl border border-gray-200 bg-white p-4 sm:p-5 shadow-xs dark:border-gray-700 dark:bg-gray-800 flex flex-col h-min">
          <div>
            <div class="flex items-center justify-between pb-3 border-b border-gray-100 dark:border-gray-700">
              <div class="flex items-center gap-2">
                <div
                  class="flex h-6 w-6 items-center justify-center rounded-md bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400">
                  <IconChartBar class="h-3.5 w-3.5" />
                </div>
                <h2 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                  Distribución por Áreas
                </h2>
              </div>
              <span class="text-2xs font-medium text-gray-400">{{ areas.length }} áreas</span>
            </div>

            <div v-if="areas.length === 0" class="py-8 text-center text-xs text-gray-400">
              No hay áreas registradas para este distrito
            </div>

            <div v-else class="mt-4 space-y-3 max-h-72 overflow-y-auto pr-1">
              <RouterLink
                v-for="area in areas"
                :key="area.id"
                :to="`/area/${encodeURIComponent(area.nombre)}`"
                class="block space-y-1 p-1.5 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors cursor-pointer">
                <div class="flex items-center justify-between text-2xs">
                  <span class="font-semibold text-gray-800 dark:text-gray-200 truncate max-w-[70%]">
                    {{ area.nombre }}
                  </span>
                  <div class="flex items-center gap-1.5">
                    <span class="font-mono font-bold text-gray-900 dark:text-white">{{ area.cantidad }}</span>
                    <span class="text-gray-400"
                      >({{ Math.round((area.cantidad / (totalTrabajadores || 1)) * 100) }}%)</span
                    >
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
                  <IconBriefcase class="h-3.5 w-3.5" />
                </div>
                <h2 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                  Distribución por Regímenes
                </h2>
              </div>
              <span class="text-2xs font-medium text-gray-400">{{ distribucionRegimenes.length }} regímenes</span>
            </div>

            <div v-if="distribucionRegimenes.length === 0" class="py-8 text-center text-xs text-gray-400">
              No hay regímenes registrados para este distrito
            </div>

            <div v-else class="mt-4 space-y-3 max-h-72 overflow-y-auto pr-1">
              <RouterLink
                v-for="regimen in distribucionRegimenes"
                :key="regimen.nombre"
                :to="`/regimen/${encodeURIComponent(regimen.nombre)}`"
                class="block space-y-1 p-1.5 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors cursor-pointer">
                <div class="flex items-center justify-between text-2xs">
                  <span class="font-semibold text-gray-800 dark:text-gray-200 truncate max-w-[70%]">
                    {{ regimen.nombre }}
                  </span>
                  <div class="flex items-center gap-1.5">
                    <span class="font-mono font-bold text-gray-900 dark:text-white">{{ regimen.cantidad }}</span>
                    <span class="text-gray-400"
                      >({{ Math.round((regimen.cantidad / (totalTrabajadores || 1)) * 100) }}%)</span
                    >
                  </div>
                </div>
                <div class="h-2 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <div
                    class="h-full rounded-full bg-indigo-500 transition-all duration-300"
                    :style="{ width: `${(regimen.cantidad / maximoRegimen) * 100}%` }" />
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
                  class="flex h-6 w-6 items-center justify-center rounded-md bg-emerald-50 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400">
                  <IconCake class="h-3.5 w-3.5" />
                </div>
                <h2 class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                  Distribución por Rangos de Edad
                </h2>
              </div>
              <span class="text-2xs font-medium text-gray-400">{{ rangosEdad.length }} rangos</span>
            </div>

            <div v-if="rangosEdad.length === 0" class="py-8 text-center text-xs text-gray-400">
              No hay datos de edades disponibles
            </div>

            <div v-else class="mt-4 space-y-3">
              <div v-for="rango in rangosEdad" :key="rango.nombre" class="space-y-1">
                <div class="flex items-center justify-between text-2xs">
                  <span class="font-semibold text-gray-800 dark:text-gray-200">{{ rango.nombre }} años</span>
                  <div class="flex items-center gap-1.5">
                    <span class="font-mono font-bold text-gray-900 dark:text-white">{{ rango.cantidad }}</span>
                    <span class="text-gray-400"
                      >({{ Math.round((rango.cantidad / (totalTrabajadores || 1)) * 100) }}%)</span
                    >
                  </div>
                </div>
                <div class="h-2 w-full overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <div
                    class="h-full rounded-full bg-emerald-500 transition-all duration-300"
                    :style="{ width: `${(rango.cantidad / maximoEdad) * 100}%` }" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <TablaTrabajadores
        :trabajadores="personas"
        titulo="Padrón de Trabajadores"
        :subtitulo="`Trabajadores registrados que residen en ${tituloDistritoFormateado}`"
        :mostrar-filtro-area="true"
        :mostrar-filtro-regimen="true"
        :mostrar-columna-direccion="true" />
    </template>
  </div>
</template>
