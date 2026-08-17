<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useJubilacionStore } from '@/stores/jubilacion'
import { formatearFecha } from '@/utils/fechas'
import TarjetaResumen from '@/components/comun/TarjetaResumen.vue'
import TablaTrabajadores from '@/components/comun/TablaTrabajadores.vue'
import {
  IconHourglass,
  IconCalendarTime,
  IconAlertTriangle,
  IconAlertCircle,
  IconClock,
  IconUsers,
  IconSearch,
  IconX,
  IconRefresh,
  IconArrowLeft,
  IconCalendar,
  IconCheck,
} from '@tabler/icons-vue'

const router = useRouter()
const jubilacionStore = useJubilacionStore()

const {
  estaCargando,
  error,
  busqueda,
  filtroEstado,
  filtroRegimen,
  edadMinima,
  totalServidores,
  totalCumplenEsteMes,
  totalProximos,
  totalEnExtension,
  totalLimiteSuperado,
  regimenesDisponibles,
  servidoresFiltrados,
} = storeToRefs(jubilacionStore)

const { cargarServidores, cambiarEdadMinima, limpiarFiltros } = jubilacionStore

onMounted(async () => {
  await cargarServidores(edadMinima.value)
})

function formatearDias(dias: number): string {
  if (dias === 0) return 'Hoy'
  if (dias > 0) {
    if (dias === 1) return 'En 1 día'
    return `En ${dias} días`
  }
  const diasPasados = Math.abs(dias)
  if (diasPasados === 1) return 'Hace 1 día'
  return `Hace ${diasPasados} días`
}

function obtenerColorDias(dias: number): string {
  if (dias < 0) return 'text-red-600 dark:text-red-400 font-semibold'
  if (dias <= 30) return 'text-amber-600 dark:text-amber-400 font-semibold'
  if (dias <= 90) return 'text-orange-500 dark:text-orange-400'
  return 'text-gray-600 dark:text-gray-400'
}

function obtenerEtiquetaEstado(estado: string): string {
  switch (estado) {
    case 'CUMPLE_ESTE_MES':
      return 'Cumple este mes'
    case 'PROXIMO_A_CUMPLIR':
      return 'Próximo a cumplir'
    case 'EN_PERIODO_EXTENSION':
      return 'En extensión anual'
    case 'LIMITE_SUPERADO':
      return 'Límite superado'
    default:
      return estado
  }
}

function obtenerClaseBadgeEstado(estado: string): string {
  switch (estado) {
    case 'CUMPLE_ESTE_MES':
      return 'bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-950/40 dark:text-amber-300 dark:border-amber-800'
    case 'PROXIMO_A_CUMPLIR':
      return 'bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-950/40 dark:text-blue-300 dark:border-blue-800'
    case 'EN_PERIODO_EXTENSION':
      return 'bg-purple-50 text-purple-700 border-purple-200 dark:bg-purple-950/40 dark:text-purple-300 dark:border-purple-800'
    case 'LIMITE_SUPERADO':
      return 'bg-red-50 text-red-700 border-red-200 dark:bg-red-950/40 dark:text-red-300 dark:border-red-800'
    default:
      return 'bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700'
  }
}
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
            class="flex h-9 w-9 items-center justify-center rounded-xl bg-amber-50 text-amber-600 dark:bg-amber-900/20 dark:text-amber-400">
            <IconHourglass class="h-5 w-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h1 class="text-sm sm:text-base font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                Alerta de Jubilación (70 Años)
              </h1>
              <span
                v-if="!estaCargando"
                class="inline-flex items-center gap-1 rounded-md bg-amber-50 px-2 py-0.5 text-2xs font-bold text-amber-700 dark:bg-amber-900/40 dark:text-amber-300 border border-amber-200 dark:border-amber-800">
                {{ totalServidores }} servidores
              </span>
            </div>
            <p class="text-2xs font-medium text-gray-400">
              Control del límite legal de permanencia y cese definitivo por cumplimiento de edad
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <div class="flex items-center rounded-xl border border-gray-200 bg-white p-1 dark:border-gray-700 dark:bg-gray-800 shadow-xs">
          <button
            type="button"
            @click="cambiarEdadMinima(69)"
            :class="[
              'px-2.5 py-1 text-2xs font-bold rounded-lg transition-colors cursor-pointer',
              edadMinima === 69
                ? 'bg-amber-500 text-white shadow-xs'
                : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white',
            ]">
            ≥ 69 Años
          </button>
          <button
            type="button"
            @click="cambiarEdadMinima(70)"
            :class="[
              'px-2.5 py-1 text-2xs font-bold rounded-lg transition-colors cursor-pointer',
              edadMinima === 70
                ? 'bg-amber-500 text-white shadow-xs'
                : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white',
            ]">
            ≥ 70 Años
          </button>
          <button
            type="button"
            @click="cambiarEdadMinima(68)"
            :class="[
              'px-2.5 py-1 text-2xs font-bold rounded-lg transition-colors cursor-pointer',
              edadMinima === 68
                ? 'bg-amber-500 text-white shadow-xs'
                : 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white',
            ]">
            ≥ 68 Años
          </button>
        </div>

        <button
          type="button"
          @click="cargarServidores(edadMinima, true)"
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
        @click="cargarServidores(edadMinima, true)"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-medium text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-5">
      <TarjetaResumen
        titulo="Total en Alerta"
        :valor="totalServidores"
        subtitulo="Personal en monitoreo"
        color="blue"
        :icono="IconUsers"
        interactivo
        :activo="filtroEstado === 'TODOS'"
        claseActivo="border-gray-400 bg-gray-50/80 ring-2 ring-gray-400/30 dark:border-gray-500 dark:bg-gray-800/80"
        @click="filtroEstado = 'TODOS'" />

      <TarjetaResumen
        titulo="Cumplen Este Mes"
        :valor="totalCumplenEsteMes"
        subtitulo="Prioridad alta"
        color="amber"
        :icono="IconClock"
        claseValor="text-amber-700 dark:text-amber-400"
        claseSubtitulo="text-amber-600/80 dark:text-amber-400/80 font-medium"
        interactivo
        :activo="filtroEstado === 'CUMPLE_ESTE_MES'"
        @click="filtroEstado = 'CUMPLE_ESTE_MES'" />

      <TarjetaResumen
        titulo="Próximos a Cumplir"
        :valor="totalProximos"
        subtitulo="< 70 años"
        color="sky"
        :icono="IconCalendarTime"
        claseValor="text-blue-700 dark:text-blue-400"
        claseSubtitulo="text-blue-600/80 dark:text-blue-400/80 font-medium"
        interactivo
        :activo="filtroEstado === 'PROXIMO_A_CUMPLIR'"
        @click="filtroEstado = 'PROXIMO_A_CUMPLIR'" />

      <TarjetaResumen
        titulo="Extensión Anual"
        :valor="totalEnExtension"
        subtitulo="Hasta 31 de dic"
        color="purple"
        :icono="IconCalendar"
        claseValor="text-purple-700 dark:text-purple-400"
        claseSubtitulo="text-purple-600/80 dark:text-purple-400/80 font-medium"
        interactivo
        :activo="filtroEstado === 'EN_PERIODO_EXTENSION'"
        @click="filtroEstado = 'EN_PERIODO_EXTENSION'" />

      <TarjetaResumen
        titulo="Límite Superado"
        :valor="totalLimiteSuperado"
        subtitulo="Cese pendiente"
        color="red"
        :icono="IconAlertCircle"
        claseValor="text-red-700 dark:text-red-400"
        claseSubtitulo="text-red-600/80 dark:text-red-400/80 font-medium"
        interactivo
        :activo="filtroEstado === 'LIMITE_SUPERADO'"
        class="col-span-2 sm:col-span-1 lg:col-span-1"
        @click="filtroEstado = 'LIMITE_SUPERADO'" />
    </div>

    <div class="rounded-xl border border-gray-200 bg-white p-3.5 shadow-xs dark:border-gray-800 dark:bg-gray-900">
      <div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
        <div class="relative flex-1">
          <IconSearch class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
          <input
            type="text"
            v-model="busqueda"
            placeholder="Buscar por DNI, nombres, cargo, área o plaza..."
            class="w-full rounded-lg border border-gray-200 bg-gray-50/50 py-2 pl-9 pr-8 text-xs text-gray-900 placeholder:text-gray-400 focus:border-blue-500 focus:bg-white focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-white dark:placeholder:text-gray-500" />
          <button
            v-if="busqueda"
            type="button"
            @click="busqueda = ''"
            class="absolute right-2.5 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
            <IconX class="h-3.5 w-3.5" />
          </button>
        </div>

        <div class="flex flex-wrap items-center gap-2">
          <select
            v-model="filtroEstado"
            class="rounded-lg border border-gray-200 bg-white py-2 px-2.5 text-xs text-gray-700 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-200">
            <option value="TODOS">Todos los Estados</option>
            <option value="CUMPLE_ESTE_MES">Cumplen este mes</option>
            <option value="PROXIMO_A_CUMPLIR">Próximos a cumplir</option>
            <option value="EN_PERIODO_EXTENSION">En extensión anual</option>
            <option value="LIMITE_SUPERADO">Límite superado</option>
          </select>

          <select
            v-model="filtroRegimen"
            class="rounded-lg border border-gray-200 bg-white py-2 px-2.5 text-xs text-gray-700 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-200">
            <option value="TODOS">Todos los Regímenes</option>
            <option v-for="regimen in regimenesDisponibles" :key="regimen" :value="regimen">
              {{ regimen }}
            </option>
          </select>

          <button
            v-if="busqueda || filtroEstado !== 'TODOS' || filtroRegimen !== 'TODOS'"
            type="button"
            @click="limpiarFiltros"
            class="inline-flex items-center gap-1 rounded-lg border border-gray-200 bg-gray-50 px-2.5 py-2 text-2xs font-medium text-gray-600 hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors cursor-pointer">
            <IconX class="h-3 w-3" />
            Limpiar filtros
          </button>
        </div>
      </div>
    </div>

    <div v-if="estaCargando && totalServidores === 0" class="space-y-3 animate-pulse">
      <div class="h-12 rounded-xl bg-gray-200 dark:bg-gray-800" />
      <div v-for="i in 6" :key="i" class="h-16 rounded-xl bg-gray-200 dark:bg-gray-800" />
    </div>

    <div
      v-else-if="servidoresFiltrados.length === 0"
      class="flex flex-col items-center justify-center rounded-2xl border border-dashed border-gray-300 bg-white py-12 px-4 text-center dark:border-gray-700 dark:bg-gray-900">
      <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-amber-50 text-amber-600 dark:bg-amber-950/40 dark:text-amber-400">
        <IconCheck class="h-6 w-6" />
      </div>
      <h3 class="mt-3 text-sm font-bold text-gray-900 dark:text-white">
        No se encontraron servidores
      </h3>
      <p class="mt-1 max-w-sm text-xs text-gray-500 dark:text-gray-400">
        No hay registros que coincidan con los filtros seleccionados o no hay servidores que alcancen la edad configurada.
      </p>
      <button
        v-if="busqueda || filtroEstado !== 'TODOS' || filtroRegimen !== 'TODOS'"
        type="button"
        @click="limpiarFiltros"
        class="mt-4 inline-flex items-center gap-1.5 rounded-xl bg-blue-600 px-3 py-2 text-xs font-medium text-white shadow-xs hover:bg-blue-700 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Restablecer filtros
      </button>
    </div>

    <div v-else>
      <TablaTrabajadores
        :trabajadores="servidoresFiltrados"
        titulo="Padrón de Servidores Próximos a Jubilación"
        :subtitulo="`Servidores con ${edadMinima} años o más`"
        :mostrar-pestanas="false"
        :mostrar-checkboxes="false"
        :mostrar-columna-estado="false"
        :mostrar-columna-ingreso="false"
        :mostrar-columna-direccion="false"
        :mostrar-columna-area="true"
        :mostrar-columna-cargo="true"
        :mostrar-columna-regimen="true"
        :mostrar-filtro-area="true"
        :mostrar-filtro-regimen="true">
        <template #encabezados-extra>
          <th scope="col" class="px-4 py-3 min-w-[130px]">Nacimiento (Edad)</th>
          <th scope="col" class="px-4 py-3 min-w-[150px]">Cumplimiento 70 Años</th>
          <th scope="col" class="px-4 py-3 min-w-[150px]">Cese Legal Previsto</th>
          <th scope="col" class="px-4 py-3 min-w-[140px]">Estado de Alerta</th>
        </template>

        <template #celdas-extra="{ trabajador }">
          <td class="px-4 py-3 whitespace-nowrap">
            <div>
              <span class="font-mono text-gray-900 dark:text-gray-200">
                {{ formatearFecha((trabajador as any).nacimiento) }}
              </span>
              <div class="text-2xs font-bold text-gray-500 dark:text-gray-400 mt-0.5">
                {{ (trabajador as any).edad_actual }} años
              </div>
            </div>
          </td>

          <td class="px-4 py-3 whitespace-nowrap">
            <div>
              <span class="font-mono font-medium text-gray-900 dark:text-gray-100">
                {{ formatearFecha((trabajador as any).fecha_70_anos) }}
              </span>
              <div :class="['text-2xs mt-0.5', obtenerColorDias((trabajador as any).dias_para_70)]">
                {{ formatearDias((trabajador as any).dias_para_70) }}
              </div>
            </div>
          </td>

          <td class="px-4 py-3 whitespace-nowrap">
            <div class="text-2xs space-y-0.5">
              <div class="text-gray-600 dark:text-gray-300">
                <span class="text-gray-400">Fin de mes:</span> {{ formatearFecha((trabajador as any).fecha_limite_mes) }}
              </div>
              <div class="text-gray-600 dark:text-gray-300">
                <span class="text-gray-400">Fin de año:</span> {{ formatearFecha((trabajador as any).fecha_extension_fin_ano) }}
              </div>
            </div>
          </td>

          <td class="px-4 py-3 whitespace-nowrap">
            <span
              :class="[
                'inline-flex items-center gap-1 rounded-md border px-2 py-1 text-2xs font-bold',
                obtenerClaseBadgeEstado((trabajador as any).estado_alerta),
              ]">
              <span class="h-1.5 w-1.5 rounded-full bg-current" />
              {{ obtenerEtiquetaEstado((trabajador as any).estado_alerta) }}
            </span>
          </td>
        </template>
      </TablaTrabajadores>
    </div>
  </div>
</template>
