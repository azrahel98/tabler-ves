<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { RouterLink } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useCalendarioStore } from '@/stores/calendario'
import TarjetaResumen from '@/components/comun/TarjetaResumen.vue'
import {
  IconCake,
  IconChevronLeft,
  IconChevronRight,
  IconSearch,
  IconX,
  IconRefresh,
  IconAlertTriangle,
  IconList,
  IconLayoutGrid,
  IconSparkles,
  IconHourglassEmpty,
  IconShieldCheck,
} from '@tabler/icons-vue'

const calendarioStore = useCalendarioStore()
const {
  estaCargando,
  error,
  mesSeleccionado,
  anoSeleccionado,
  diaSeleccionado,
  terminoBusqueda,
  soloCasF,
  mesesNombres,
  nombreMesActual,
  cumpleanerosMesActual,
  totalCumpleanosMes,
  totalCasFMes,
  edadPromedio,
  diaMayorActividad,
  diasMatriz,
  cumpleanerosDiaSeleccionado,
} = storeToRefs(calendarioStore)

const { cargarCumpleanos, mesSiguiente, mesAnterior, irAHoy, seleccionarDia } = calendarioStore

const modoVista = ref<'cuadricula' | 'lista'>('cuadricula')
const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
const erroresAvatar = ref(new Set<string>())

function obtenerAvatarUrl(dni: string, avatarDirecto?: string | null): string {
  if (avatarDirecto && avatarDirecto.startsWith('http')) return avatarDirecto
  return `${apiBaseUrl.replace(/\/$/, '')}/personal/avatar/${dni}`
}

function manejarErrorAvatar(dni: string) {
  erroresAvatar.value.add(dni)
}

function obtenerIniciales(nombre: string): string {
  if (!nombre) return 'T'
  const partes = nombre.trim().split(/\s+/)
  if (partes.length === 1) return partes[0].charAt(0).toUpperCase()
  return (partes[0].charAt(0) + partes[1].charAt(0)).toUpperCase()
}

const anosDisponibles = Array.from({ length: 7 }, (_, i) => new Date().getFullYear() - 3 + i)
const diasSemana = ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom']

onMounted(async () => {
  await cargarCumpleanos()
})
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-4 max-w-[1600px] mx-auto overflow-x-hidden">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-3">
        <div
          class="flex h-9 w-9 items-center justify-center rounded-xl bg-amber-50 text-amber-600 dark:bg-amber-900/20 dark:text-amber-400 shadow-2xs">
          <IconCake class="h-5 w-5" />
        </div>
        <div>
          <div class="flex flex-wrap items-center gap-2">
            <h1 class="text-base sm:text-lg font-bold tracking-tight text-navy-700 dark:text-white">Cumpleaños</h1>
            <span
              v-if="!estaCargando"
              class="inline-flex items-center gap-1 rounded-md bg-amber-50 px-1.5 py-0.5 text-[10px] font-semibold text-amber-700 dark:bg-amber-900/40 dark:text-amber-300 border border-amber-200/80 dark:border-amber-800/60 tabular-nums">
              {{ totalCumpleanosMes }} en {{ nombreMesActual }}
            </span>
            <span
              v-if="!estaCargando && totalCasFMes > 0"
              class="inline-flex items-center gap-1 rounded-md bg-indigo-50 px-1.5 py-0.5 text-[10px] font-semibold text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300 border border-indigo-200/80 dark:border-indigo-800/60 tabular-nums">
              <IconShieldCheck class="h-3 w-3 text-indigo-600 dark:text-indigo-400" />
              {{ totalCasFMes }} CAS-F
            </span>
          </div>
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <IconSearch class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-secondaryGray-500" />
          <input
            v-model="terminoBusqueda"
            type="text"
            placeholder="Buscar por nombre, DNI..."
            class="w-full pl-8 pr-7 py-1.5 rounded-xl border border-gray-200 bg-white text-xs font-medium text-navy-700 placeholder:text-secondaryGray-500 focus:border-amber-500 focus:ring-1 focus:ring-amber-500 dark:border-navy-700 dark:bg-navy-800 dark:text-white dark:placeholder:text-secondaryGray-500 shadow-xs" />
          <button
            v-if="terminoBusqueda"
            type="button"
            @click="terminoBusqueda = ''"
            class="absolute right-2 top-1/2 -translate-y-1/2 text-secondaryGray-500 hover:text-navy-700 dark:hover:text-white cursor-pointer">
            <IconX class="h-3.5 w-3.5" />
          </button>
        </div>

        <div
          class="flex items-center rounded-xl border border-gray-200 bg-white p-1 shadow-xs dark:border-navy-700 dark:bg-navy-800">
          <button
            type="button"
            @click="mesAnterior"
            class="flex h-7 w-7 items-center justify-center rounded-lg text-secondaryGray-600 hover:bg-gray-100 dark:text-secondaryGray-400 dark:hover:bg-navy-700 transition-colors cursor-pointer"
            title="Mes anterior">
            <IconChevronLeft class="h-4 w-4" />
          </button>

          <div class="flex items-center gap-1 px-1">
            <select
              v-model="mesSeleccionado"
              class="bg-transparent text-xs font-bold text-navy-700 dark:text-white border-0 py-0 pl-1 pr-5 focus:ring-0 cursor-pointer">
              <option v-for="(mes, idx) in mesesNombres" :key="mes" :value="idx" class="dark:bg-navy-800">
                {{ mes }}
              </option>
            </select>

            <select
              v-model="anoSeleccionado"
              class="bg-transparent text-xs font-bold text-navy-700 dark:text-white border-0 py-0 pl-1 pr-5 focus:ring-0 cursor-pointer tabular-nums">
              <option v-for="ano in anosDisponibles" :key="ano" :value="ano" class="dark:bg-navy-800">
                {{ ano }}
              </option>
            </select>
          </div>

          <button
            type="button"
            @click="mesSiguiente"
            class="flex h-7 w-7 items-center justify-center rounded-lg text-secondaryGray-600 hover:bg-gray-100 dark:text-secondaryGray-400 dark:hover:bg-navy-700 transition-colors cursor-pointer"
            title="Mes siguiente">
            <IconChevronRight class="h-4 w-4" />
          </button>
        </div>

        <button
          type="button"
          @click="irAHoy"
          class="inline-flex items-center gap-1.5 rounded-xl border border-gray-200 bg-white px-2.5 py-1.5 text-xs font-semibold text-navy-700 shadow-xs hover:bg-gray-50 dark:border-navy-700 dark:bg-navy-800 dark:text-white dark:hover:bg-navy-700 transition-colors cursor-pointer">
          <IconSparkles class="h-3.5 w-3.5 text-amber-500" />
          <span>Hoy</span>
        </button>

        <button
          type="button"
          @click="soloCasF = !soloCasF"
          :class="[
            'inline-flex items-center gap-1.5 rounded-xl border px-2.5 py-1.5 text-xs font-semibold shadow-xs transition-colors cursor-pointer',
            soloCasF
              ? 'border-indigo-300 bg-indigo-50 text-indigo-700 dark:border-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300 font-bold'
              : 'border-gray-200 bg-white text-navy-700 hover:bg-gray-50 dark:border-navy-700 dark:bg-navy-800 dark:text-secondaryGray-400 dark:hover:bg-navy-700',
          ]">
          <IconShieldCheck
            :class="['h-3.5 w-3.5', soloCasF ? 'text-indigo-600 dark:text-indigo-400' : 'text-secondaryGray-500']" />
          <span>CAS-F</span>
          <span
            v-if="totalCasFMes > 0"
            :class="[
              'inline-flex items-center justify-center rounded-full px-1 py-0.2 text-[9px] font-bold tabular-nums',
              soloCasF
                ? 'bg-indigo-200 text-indigo-800 dark:bg-indigo-900 dark:text-indigo-200'
                : 'bg-gray-100 text-secondaryGray-600 dark:bg-navy-700 dark:text-secondaryGray-300',
            ]">
            {{ totalCasFMes }}
          </span>
        </button>

        <div class="flex items-center rounded-xl bg-gray-100 p-0.5 dark:bg-navy-700/80">
          <button
            type="button"
            @click="modoVista = 'cuadricula'"
            :class="[
              'rounded-lg p-1.5 transition-all cursor-pointer',
              modoVista === 'cuadricula'
                ? 'bg-white text-amber-600 shadow-xs dark:bg-navy-800 dark:text-amber-400'
                : 'text-secondaryGray-500 hover:text-navy-700 dark:hover:text-white',
            ]"
            title="Vista Cuadrícula">
            <IconLayoutGrid class="h-4 w-4" />
          </button>
          <button
            type="button"
            @click="modoVista = 'lista'"
            :class="[
              'rounded-lg p-1.5 transition-all cursor-pointer',
              modoVista === 'lista'
                ? 'bg-white text-amber-600 shadow-xs dark:bg-navy-800 dark:text-amber-400'
                : 'text-secondaryGray-500 hover:text-navy-700 dark:hover:text-white',
            ]"
            title="Vista Cronograma">
            <IconList class="h-4 w-4" />
          </button>
        </div>

        <button
          type="button"
          @click="cargarCumpleanos"
          :disabled="estaCargando"
          class="inline-flex items-center justify-center h-8 w-8 rounded-xl border border-gray-200 bg-white text-secondaryGray-600 hover:text-navy-700 hover:bg-gray-50 dark:border-navy-700 dark:bg-navy-800 dark:text-secondaryGray-400 dark:hover:text-white dark:hover:bg-navy-700 transition-colors shadow-xs cursor-pointer"
          title="Actualizar">
          <IconRefresh :class="['h-3.5 w-3.5', estaCargando && 'animate-spin']" />
        </button>
      </div>
    </div>

    <div
      v-if="error"
      class="flex items-center justify-between rounded-xl border border-red-200 bg-red-50 px-4 py-3 dark:border-red-900/50 dark:bg-red-950/20">
      <div class="flex items-center gap-2.5">
        <IconAlertTriangle class="h-4 w-4 shrink-0 text-red-500" />
        <span class="text-xs font-medium text-red-700 dark:text-red-400">{{ error }}</span>
      </div>
      <button
        type="button"
        @click="cargarCumpleanos"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-semibold text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
      <TarjetaResumen
        titulo="Festejados"
        :valor="totalCumpleanosMes"
        :subtitulo="`En ${nombreMesActual}`"
        color="amber"
        :icono="IconCake" />

      <TarjetaResumen
        titulo="Edad Promedio"
        :valor="edadPromedio"
        subtitulo="Personal activo"
        color="blue"
        :icono="IconHourglassEmpty">
        <template #valor>
          <span
            class="text-xl sm:text-xl font-bold tracking-tight text-gray-900 dark:text-white tabular-nums leading-none">
            {{ edadPromedio }}<span class="text-xs font-normal text-gray-400 dark:text-gray-500 ml-1">años</span>
          </span>
        </template>
      </TarjetaResumen>

      <TarjetaResumen
        titulo="Pico de Festejos"
        :valor="diaMayorActividad ? diaMayorActividad.cantidad : 0"
        :subtitulo="diaMayorActividad ? `${diaMayorActividad.dia} de ${nombreMesActual}` : 'Sin registros'"
        color="emerald"
        claseSubtitulo="text-emerald-600 dark:text-emerald-400 font-medium"
        :icono="IconSparkles"
        class="col-span-2 sm:col-span-1" />
    </div>

    <div v-if="modoVista === 'cuadricula'" class="grid grid-cols-1 lg:grid-cols-12 gap-4 items-start">
      <div
        class="lg:col-span-8 xl:col-span-9 rounded-xl border border-gray-200 bg-white shadow-xs dark:border-navy-700 dark:bg-navy-800 overflow-hidden">
        <div class="overflow-x-auto custom-scrollbar">
          <div class="min-w-[620px] sm:min-w-0">
            <div
              class="grid grid-cols-7 border-b border-gray-200 dark:border-navy-700 text-center bg-gray-50/70 dark:bg-navy-900/50">
              <div
                v-for="dia in diasSemana"
                :key="dia"
                class="py-2 text-2xs font-bold uppercase tracking-wider text-secondaryGray-600 dark:text-secondaryGray-400">
                {{ dia }}
              </div>
            </div>

            <div class="grid grid-cols-7 divide-x divide-y divide-gray-100 dark:divide-navy-700/60">
          <div
            v-for="(celda, index) in diasMatriz"
            :key="index"
            @click="celda.esMesActual ? seleccionarDia(celda.numeroDia) : null"
            :class="[
              'min-h-[92px] sm:min-h-[105px] p-1.5 flex flex-col justify-between transition-all select-none',
              celda.esMesActual ? 'cursor-pointer' : 'bg-gray-50/40 dark:bg-navy-900/20 opacity-40',
              celda.esMesActual && diaSeleccionado === celda.numeroDia
                ? 'bg-amber-50/40 dark:bg-amber-950/20 ring-2 ring-amber-500 ring-inset'
                : 'hover:bg-gray-50/80 dark:hover:bg-navy-700/50',
            ]">
            <div class="flex items-center justify-between">
              <span
                :class="[
                  'inline-flex h-6 w-6 items-center justify-center rounded-full text-xs font-bold tabular-nums',
                  celda.esHoy
                    ? 'bg-amber-600 text-white shadow-xs'
                    : celda.esMesActual
                      ? 'text-navy-700 dark:text-white'
                      : 'text-secondaryGray-500 dark:text-secondaryGray-600',
                ]">
                {{ celda.numeroDia }}
              </span>

              <div class="flex items-center gap-1">
                <span
                  v-if="celda.cumpleaneros.some((c) => c.esCasF)"
                  class="inline-flex h-4 px-1 items-center justify-center rounded-full bg-indigo-100 text-indigo-700 dark:bg-indigo-900/70 dark:text-indigo-300"
                  title="Tiene cumpleañeros CAS-F">
                  <IconShieldCheck class="h-2.5 w-2.5" />
                </span>
                <span
                  v-if="celda.cumpleaneros.length > 0"
                  class="inline-flex h-4 px-1.5 items-center justify-center rounded-full bg-amber-100 text-[9px] font-bold text-amber-700 dark:bg-amber-900/60 dark:text-amber-300 tabular-nums">
                  <IconCake class="h-2 w-2 mr-0.5" />
                  {{ celda.cumpleaneros.length }}
                </span>
              </div>
            </div>

            <div class="space-y-1 mt-1 overflow-hidden">
              <div
                v-for="persona in celda.cumpleaneros.slice(0, 2)"
                :key="persona.id"
                :class="[
                  'flex items-center justify-between gap-1 rounded px-1.5 py-0.5 text-[10px] truncate border transition-all',
                  persona.esCasF
                    ? 'bg-indigo-50 text-indigo-900 border-indigo-200 dark:bg-indigo-950/60 dark:text-indigo-200 dark:border-indigo-800/80 font-bold ring-1 ring-indigo-400/20'
                    : 'bg-amber-50 text-amber-800 dark:bg-amber-950/40 dark:text-amber-300 border-amber-100 dark:border-amber-900/40 font-medium',
                ]">
                <div class="flex items-center gap-1 min-w-0 truncate">
                  <IconCake
                    :class="[
                      'h-2.5 w-2.5 shrink-0',
                      persona.esCasF ? 'text-indigo-600 dark:text-indigo-400' : 'text-amber-600 dark:text-amber-400',
                    ]" />
                  <span class="truncate">{{ persona.nombre }}</span>
                </div>
                <span
                  v-if="persona.esCasF"
                  class="inline-flex shrink-0 items-center px-1 py-0.2 rounded text-[8px] font-bold uppercase tracking-tight bg-indigo-200/80 text-indigo-800 dark:bg-indigo-900/80 dark:text-indigo-200">
                  CAS-F
                </span>
              </div>

              <div
                v-if="celda.cumpleaneros.length > 2"
                class="text-[9px] font-semibold text-secondaryGray-500 dark:text-secondaryGray-400 pl-1 tabular-nums">
                +{{ celda.cumpleaneros.length - 2 }} más
              </div>
            </div>
          </div>
        </div>
          </div>
        </div>
      </div>

      <div class="lg:col-span-4 xl:col-span-3 space-y-3">
        <div
          class="rounded-xl border border-gray-200 bg-white p-3.5 shadow-xs dark:border-navy-700 dark:bg-navy-800 space-y-3">
          <div class="flex items-center justify-between pb-2.5 border-b border-gray-100 dark:border-navy-700">
            <div>
              <h3 class="text-xs font-bold uppercase tracking-wider text-navy-700 dark:text-white">
                {{ diaSeleccionado ? `${diaSeleccionado} de ${nombreMesActual}` : 'Detalle del Día' }}
              </h3>
              <p class="text-2xs font-medium text-secondaryGray-500 tabular-nums">
                {{ cumpleanerosDiaSeleccionado.length }} festejados
              </p>
            </div>
            <div
              class="flex h-7 w-7 items-center justify-center rounded-lg bg-amber-50 text-amber-600 dark:bg-amber-900/30 dark:text-amber-400">
              <IconCake class="h-4 w-4" />
            </div>
          </div>

          <div v-if="cumpleanerosDiaSeleccionado.length === 0" class="py-8 text-center text-xs text-secondaryGray-500">
            Sin festejados en esta fecha
          </div>

          <div v-else class="space-y-2 max-h-[480px] overflow-y-auto pr-1 custom-scrollbar">
            <RouterLink
              v-for="persona in cumpleanerosDiaSeleccionado"
              :key="persona.id"
              :to="`/perfil/${persona.dni}`"
              :class="[
                'block rounded-lg border p-2 transition-all cursor-pointer group',
                persona.esCasF
                  ? 'border-indigo-200 bg-indigo-50/40 dark:border-indigo-800/60 dark:bg-indigo-950/20 hover:border-indigo-300 dark:hover:border-indigo-700 shadow-xs'
                  : 'border-gray-100 bg-gray-50 dark:border-navy-700 dark:bg-navy-900/50 hover:border-amber-200 dark:hover:border-amber-800',
              ]">
              <div class="flex items-center gap-2.5">
                <div
                  :class="[
                    'relative flex h-8 w-8 shrink-0 items-center justify-center overflow-hidden rounded-full border group-hover:scale-105 transition-transform shadow-2xs',
                    persona.esCasF
                      ? 'border-indigo-300 bg-indigo-50 dark:border-indigo-700 dark:bg-indigo-900/40 ring-1 ring-indigo-400/30'
                      : 'border-gray-200/70 bg-amber-50 dark:border-navy-700 dark:bg-amber-900/20',
                  ]">
                  <img
                    v-if="persona.dni && !erroresAvatar.has(persona.dni)"
                    :src="obtenerAvatarUrl(persona.dni, persona.avatar)"
                    :alt="persona.nombre"
                    class="h-full w-full object-cover"
                    loading="lazy"
                    @error="manejarErrorAvatar(persona.dni)" />
                  <span
                    v-else
                    :class="[
                      'flex h-full w-full items-center justify-center text-2xs font-bold text-white uppercase',
                      persona.esCasF ? 'bg-indigo-600' : 'bg-amber-600',
                    ]">
                    {{ obtenerIniciales(persona.nombre) }}
                  </span>
                </div>

                <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between gap-1">
                    <p
                      :class="[
                        'text-xs font-bold tracking-tight truncate transition-colors',
                        persona.esCasF
                          ? 'text-indigo-950 dark:text-indigo-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400'
                          : 'text-navy-700 dark:text-white group-hover:text-amber-600 dark:group-hover:text-amber-400',
                      ]">
                      {{ persona.nombre }}
                    </p>
                    <div class="flex items-center gap-1 shrink-0">
                      <span
                        v-if="persona.esCasF"
                        class="inline-flex items-center px-1.5 py-0.2 rounded text-[9px] font-bold bg-indigo-100 text-indigo-800 border border-indigo-200 dark:bg-indigo-900/60 dark:text-indigo-300 dark:border-indigo-800">
                        CAS-F
                      </span>
                      <span
                        class="inline-flex items-center gap-1 rounded-md px-1.5 py-0.2 text-[10px] font-medium bg-amber-100 text-amber-800 dark:bg-amber-900/40 dark:text-amber-300 tabular-nums">
                        <IconCake class="h-2 w-2" />
                        {{ persona.edad }} años
                      </span>
                    </div>
                  </div>

                  <p
                    class="text-2xs font-normal text-secondaryGray-500 dark:text-secondaryGray-400 mt-0.5 tabular-nums">
                    DNI: {{ persona.dni }} · {{ persona.nacimiento }}
                    <span
                      v-if="persona.regimen"
                      class="ml-1 text-secondaryGray-600 dark:text-secondaryGray-300 font-sans">
                      · {{ persona.regimen }}
                    </span>
                  </p>
                </div>
              </div>
            </RouterLink>
          </div>
        </div>
      </div>
    </div>

    <div
      v-else
      class="rounded-xl border border-gray-200 bg-white shadow-xs dark:border-navy-700 dark:bg-navy-800 overflow-hidden">
      <div class="px-4 py-3 border-b border-gray-100 dark:border-navy-700 flex items-center justify-between">
        <h3 class="text-xs font-bold uppercase tracking-wider text-navy-700 dark:text-white">
          Cronograma - {{ nombreMesActual }} {{ anoSeleccionado }}
        </h3>
        <span class="text-2xs font-medium text-secondaryGray-500 tabular-nums">
          {{ cumpleanerosMesActual.length }} festejados
        </span>
      </div>

      <div v-if="cumpleanerosMesActual.length === 0" class="py-12 text-center text-secondaryGray-500 text-xs">
        No se encontraron festejados para el mes seleccionado
      </div>

      <div v-else class="divide-y divide-gray-100 dark:divide-navy-700">
        <div
          v-for="persona in cumpleanerosMesActual"
          :key="persona.id"
          :class="[
            'px-4 py-3 transition-colors flex items-center justify-between gap-3',
            persona.esCasF
              ? 'bg-indigo-50/30 dark:bg-indigo-950/15 hover:bg-indigo-50/60 dark:hover:bg-indigo-950/30'
              : 'hover:bg-gray-50/70 dark:hover:bg-navy-700/50',
          ]">
          <div class="flex items-center gap-3 min-w-0">
            <div
              :class="[
                'flex flex-col items-center justify-center h-10 w-10 rounded-xl shrink-0 border shadow-2xs',
                persona.esCasF
                  ? 'bg-indigo-50 text-indigo-600 dark:bg-indigo-950/40 dark:text-indigo-300 border-indigo-200 dark:border-indigo-800/80'
                  : 'bg-amber-50 text-amber-600 dark:bg-amber-950/40 dark:text-amber-400 border-amber-100 dark:border-amber-900/40',
              ]">
              <span class="text-3xs font-bold uppercase tracking-wider">{{ nombreMesActual.slice(0, 3) }}</span>
              <span class="text-sm font-bold tracking-tight leading-none tabular-nums">{{ persona.dia }}</span>
            </div>

            <div
              :class="[
                'relative flex h-8 w-8 shrink-0 items-center justify-center overflow-hidden rounded-full border shadow-2xs',
                persona.esCasF
                  ? 'border-indigo-300 bg-indigo-50 dark:border-indigo-700 dark:bg-indigo-900/40 ring-1 ring-indigo-400/30'
                  : 'border-gray-200 bg-gray-100 dark:border-navy-700 dark:bg-navy-700',
              ]">
              <img
                v-if="persona.dni && !erroresAvatar.has(persona.dni)"
                :src="obtenerAvatarUrl(persona.dni, persona.avatar)"
                :alt="persona.nombre"
                class="h-full w-full object-cover"
                loading="lazy"
                @error="manejarErrorAvatar(persona.dni)" />
              <span
                v-else
                :class="[
                  'flex h-full w-full items-center justify-center text-2xs font-bold text-white uppercase',
                  persona.esCasF ? 'bg-indigo-600' : 'bg-amber-600',
                ]">
                {{ obtenerIniciales(persona.nombre) }}
              </span>
            </div>

            <div class="min-w-0">
              <div class="flex items-center gap-2">
                <RouterLink
                  :to="`/perfil/${persona.dni}`"
                  :class="[
                    'font-bold text-xs sm:text-sm tracking-tight truncate transition-colors',
                    persona.esCasF
                      ? 'text-indigo-950 dark:text-indigo-100 hover:text-indigo-600 dark:hover:text-indigo-400'
                      : 'text-navy-700 dark:text-white hover:text-amber-600 dark:hover:text-amber-400',
                  ]">
                  {{ persona.nombre }}
                </RouterLink>
                <span
                  v-if="persona.esCasF"
                  class="inline-flex items-center px-1.5 py-0.2 rounded text-[9px] font-bold bg-indigo-100 text-indigo-800 border border-indigo-200 dark:bg-indigo-900/60 dark:text-indigo-300 dark:border-indigo-800">
                  CAS-F
                </span>
                <span
                  class="inline-flex items-center gap-1 rounded-md px-1.5 py-0.2 text-[10px] font-medium shrink-0 bg-amber-100 text-amber-800 dark:bg-amber-900/40 dark:text-amber-300 tabular-nums">
                  <IconCake class="h-2 w-2" />
                  {{ persona.edad }} años
                </span>
              </div>

              <div
                class="flex items-center gap-2 text-2xs text-secondaryGray-500 dark:text-secondaryGray-400 mt-0.5 tabular-nums">
                <span>DNI: {{ persona.dni }}</span>
                <span>· {{ persona.nacimiento }}</span>
                <span v-if="persona.regimen" class="text-secondaryGray-600 dark:text-secondaryGray-300 font-sans">
                  · {{ persona.regimen }}
                </span>
              </div>
            </div>
          </div>

          <RouterLink
            :to="`/perfil/${persona.dni}`"
            :class="[
              'inline-flex items-center rounded-lg border px-2.5 py-1 text-2xs font-semibold transition-colors shadow-2xs shrink-0',
              persona.esCasF
                ? 'border-indigo-200 bg-white text-indigo-600 hover:bg-indigo-50 dark:border-indigo-800 dark:bg-navy-800 dark:text-indigo-300 dark:hover:bg-indigo-950/40'
                : 'border-gray-200 bg-white text-amber-600 hover:bg-amber-50 dark:border-navy-700 dark:bg-navy-800 dark:text-amber-400 dark:hover:bg-amber-950/30',
            ]">
            Ver Perfil
          </RouterLink>
        </div>
      </div>
    </div>
  </div>
</template>
