<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { dashboardApi } from '@/api/dashboard'
import type { ResultadoComparacionMef } from '@/api/types'
import TarjetaResumen from '@/components/comun/TarjetaResumen.vue'
import {
  IconFileSpreadsheet,
  IconAlertCircle,
  IconX,
  IconRefresh,
  IconSearch,
  IconCheck,
  IconAlertTriangle,
  IconDownload,
  IconArrowLeft,
  IconArrowsDiff,
  IconChevronLeft,
  IconChevronRight,
  IconFileCheck,
} from '@tabler/icons-vue'


const router = useRouter()

const inputCas = ref<HTMLInputElement | null>(null)
const inputOtros = ref<HTMLInputElement | null>(null)
const archivoCas = ref<File | null>(null)
const archivoOtros = ref<File | null>(null)
const estaCargando = ref(false)
const estaExportando = ref(false)
const error = ref<string | null>(null)
const resultado = ref<ResultadoComparacionMef | null>(null)

const filtroEstado = ref<string>('DIFERENCIA')
const filtroRegimen = ref<string>('TODOS')
const busqueda = ref('')
const paginaActual = ref(1)
const ELEMENTOS_POR_PAGINA = 50

const filtrosEstado = [
  {
    valor: 'TODOS',
    etiqueta: 'Todos',
    icono: IconFileSpreadsheet,
    claseActiva: 'border-gray-400 bg-gray-100 text-gray-800 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200',
  },
  {
    valor: 'DIFERENCIA',
    etiqueta: 'Diferencias',
    icono: IconAlertTriangle,
    claseActiva: 'border-amber-400 bg-amber-50 text-amber-800 dark:border-amber-700 dark:bg-amber-950/40 dark:text-amber-300',
  },
  {
    valor: 'NO_EXISTE_EN_MEF',
    etiqueta: 'Solo en sistema',
    icono: IconAlertCircle,
    claseActiva: 'border-red-400 bg-red-50 text-red-800 dark:border-red-700 dark:bg-red-950/40 dark:text-red-300',
  },
  {
    valor: 'NO_EXISTE_EN_SISTEMA',
    etiqueta: 'Solo en MEF',
    icono: IconAlertCircle,
    claseActiva: 'border-purple-400 bg-purple-50 text-purple-800 dark:border-purple-700 dark:bg-purple-950/40 dark:text-purple-300',
  },
  {
    valor: 'OK',
    etiqueta: 'Correctos',
    icono: IconCheck,
    claseActiva: 'border-emerald-400 bg-emerald-50 text-emerald-800 dark:border-emerald-700 dark:bg-emerald-950/40 dark:text-emerald-300',
  },
]

const filtrosRegimen = computed(() => {
  const lista = resultado.value?.comparaciones ?? []
  const conjunto = new Set(lista.map((c) => c.regimen).filter(Boolean))
  const opciones = [
    {
      valor: 'TODOS',
      etiqueta: 'Todos los regímenes',
      claseActiva: 'border-gray-400 bg-gray-100 text-gray-800 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200',
    },
  ]
  const clases: Record<string, string> = {
    CAS: 'border-blue-400 bg-blue-50 text-blue-800 dark:border-blue-700 dark:bg-blue-950/40 dark:text-blue-300',
    '276': 'border-violet-400 bg-violet-50 text-violet-800 dark:border-violet-700 dark:bg-violet-950/40 dark:text-violet-300',
    '728': 'border-teal-400 bg-teal-50 text-teal-800 dark:border-teal-700 dark:bg-teal-950/40 dark:text-teal-300',
  }
  for (const r of ['CAS', '276', '728', ...conjunto]) {
    if (r !== 'TODOS' && conjunto.has(r)) {
      opciones.push({
        valor: r,
        etiqueta: r,
        claseActiva: clases[r] ?? 'border-gray-400 bg-gray-100 text-gray-800 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200',
      })
    }
  }
  return opciones
})

const conteosPorEstado = computed(() => {
  const lista = resultado.value?.comparaciones ?? []
  return {
    TODOS: lista.length,
    OK: lista.filter((c) => c.resultado === 'OK').length,
    DIFERENCIA: lista.filter((c) => c.resultado === 'DIFERENCIA').length,
    NO_EXISTE_EN_MEF: lista.filter((c) => c.resultado === 'NO_EXISTE_EN_MEF').length,
    NO_EXISTE_EN_SISTEMA: lista.filter((c) => c.resultado === 'NO_EXISTE_EN_SISTEMA').length,
  }
})

const conteosPorRegimen = computed(() => {
  const lista = resultado.value?.comparaciones ?? []
  const base: Record<string, number> = { TODOS: lista.length }
  for (const c of lista) {
    if (c.regimen) base[c.regimen] = (base[c.regimen] ?? 0) + 1
  }
  return base
})

const filasFiltradas = computed(() => {
  let lista = resultado.value?.comparaciones ?? []
  if (filtroEstado.value !== 'TODOS') {
    lista = lista.filter((c) => c.resultado === filtroEstado.value)
  }
  if (filtroRegimen.value !== 'TODOS') {
    lista = lista.filter((c) => c.regimen === filtroRegimen.value)
  }
  if (busqueda.value.trim()) {
    const termino = busqueda.value.trim().toLowerCase()
    lista = lista.filter(
      (c) =>
        c.dni.toLowerCase().includes(termino) ||
        c.nombre.toLowerCase().includes(termino) ||
        c.campo.toLowerCase().includes(termino),
    )
  }
  return lista
})

const totalPaginas = computed(() => Math.max(1, Math.ceil(filasFiltradas.value.length / ELEMENTOS_POR_PAGINA)))
const inicioRango = computed(() => (paginaActual.value - 1) * ELEMENTOS_POR_PAGINA + 1)
const finRango = computed(() => Math.min(paginaActual.value * ELEMENTOS_POR_PAGINA, filasFiltradas.value.length))
const filasPaginadas = computed(() =>
  filasFiltradas.value.slice((paginaActual.value - 1) * ELEMENTOS_POR_PAGINA, paginaActual.value * ELEMENTOS_POR_PAGINA),
)

watch([filtroEstado, filtroRegimen, busqueda], () => {
  paginaActual.value = 1
})

function onArchivoSeleccionado(evento: Event, tipo: 'cas' | 'otros') {
  const input = evento.target as HTMLInputElement
  const archivo = input.files?.[0]
  if (!archivo) return
  if (tipo === 'cas') archivoCas.value = archivo
  else archivoOtros.value = archivo
}

function onDrop(evento: DragEvent, tipo: 'cas' | 'otros') {
  const archivo = evento.dataTransfer?.files?.[0]
  if (!archivo) return
  if (tipo === 'cas') archivoCas.value = archivo
  else archivoOtros.value = archivo
}

async function procesarArchivos() {
  if (!archivoCas.value && !archivoOtros.value) return
  error.value = null
  estaCargando.value = true
  try {
    const datos = await dashboardApi.compararMef(archivoCas.value, archivoOtros.value)
    resultado.value = datos
    filtroEstado.value = 'DIFERENCIA'
    filtroRegimen.value = 'TODOS'
  } catch (e: any) {
    error.value = e?.message || e?.error || 'Error al procesar los archivos con el MEF'
  } finally {
    estaCargando.value = false
  }
}

async function exportarExcel() {
  if (!resultado.value?.comparaciones?.length) return
  estaExportando.value = true
  try {
    const blob = await dashboardApi.generarExcelMef(resultado.value.comparaciones)
    const url = URL.createObjectURL(blob)
    const enlace = document.createElement('a')
    enlace.href = url
    const fecha = resultado.value.resumen.fecha_comparacion.replace(/[/:]/g, '-').replace(/ /g, '_')
    enlace.download = `comparacion_mef_${fecha}.xlsx`
    enlace.click()
    URL.revokeObjectURL(url)
  } catch (e: any) {
    error.value = e?.message || e?.error || 'Error al exportar archivo Excel'
  } finally {
    estaExportando.value = false
  }
}

function reiniciar() {
  resultado.value = null
  error.value = null
  busqueda.value = ''
  paginaActual.value = 1
  archivoCas.value = null
  archivoOtros.value = null
  if (inputCas.value) inputCas.value.value = ''
  if (inputOtros.value) inputOtros.value.value = ''
}

function etiquetaResultado(r: string) {
  if (r === 'OK') return 'OK'
  if (r === 'DIFERENCIA') return 'Diferencia'
  if (r === 'NO_EXISTE_EN_MEF') return 'Solo en sistema'
  if (r === 'NO_EXISTE_EN_SISTEMA') return 'Solo en MEF'
  return r
}

function claseBadgeRegimen(r: string): string {
  if (r === 'CAS') return 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
  if (r === '276') return 'bg-violet-100 text-violet-700 dark:bg-violet-900/30 dark:text-violet-400'
  if (r === '728') return 'bg-teal-100 text-teal-700 dark:bg-teal-900/30 dark:text-teal-400'
  return 'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-300'
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
            class="flex h-9 w-9 items-center justify-center rounded-xl bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
            <IconArrowsDiff class="h-5 w-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h1 class="text-sm sm:text-base font-bold uppercase tracking-wider text-gray-900 dark:text-white">
                Comparación con MEF
              </h1>
              <span
                v-if="resultado"
                class="inline-flex items-center gap-1 rounded-md bg-blue-50 px-2 py-0.5 text-2xs font-bold text-blue-700 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200 dark:border-blue-800">
                Procesado: {{ resultado.resumen.fecha_comparacion }}
              </span>
            </div>
            <p class="text-2xs font-medium text-gray-400">
              Contraste de datos de personal y cuentas del sistema contra los archivos oficiales del MEF
            </p>
          </div>
        </div>
      </div>

      <div v-if="resultado" class="flex items-center gap-2">
        <button
          type="button"
          @click="exportarExcel"
          :disabled="estaExportando"
          class="inline-flex items-center gap-1.5 rounded-xl border border-emerald-200 bg-emerald-50 px-3 py-2 text-xs font-semibold text-emerald-700 shadow-xs hover:bg-emerald-100 dark:border-emerald-900/50 dark:bg-emerald-950/30 dark:text-emerald-300 dark:hover:bg-emerald-900/40 transition-colors cursor-pointer disabled:opacity-50">
          <IconDownload v-if="!estaExportando" class="h-3.5 w-3.5" />
          <div v-else class="h-3.5 w-3.5 animate-spin rounded-full border-2 border-emerald-600 border-t-transparent" />
          <span>Exportar Excel</span>
        </button>

        <button
          type="button"
          @click="reiniciar"
          class="inline-flex items-center gap-1.5 rounded-xl border border-gray-200 bg-white px-3 py-2 text-xs font-semibold text-gray-700 shadow-xs hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-200 dark:hover:bg-gray-700 transition-colors cursor-pointer">
          <IconRefresh class="h-3.5 w-3.5" />
          <span>Nueva Comparación</span>
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
        @click="error = null"
        class="text-red-400 hover:text-red-600 dark:hover:text-red-300">
        <IconX class="h-4 w-4" />
      </button>
    </div>

    <template v-if="!resultado && !estaCargando">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div
          @click="inputCas?.click()"
          @dragover.prevent
          @drop.prevent="(e) => onDrop(e, 'cas')"
          :class="[
            'flex flex-col items-center justify-center rounded-2xl border-2 border-dashed p-8 text-center cursor-pointer transition-all duration-200 shadow-xs',
            archivoCas
              ? 'border-blue-500 bg-blue-50/50 dark:border-blue-500/70 dark:bg-blue-950/20'
              : 'border-gray-200 bg-white hover:border-blue-400 hover:bg-blue-50/20 dark:border-gray-800 dark:bg-gray-900 dark:hover:border-blue-500/40 dark:hover:bg-gray-800/40',
          ]">
          <div
            :class="[
              'flex h-12 w-12 items-center justify-center rounded-2xl mb-3 shadow-xs transition-colors',
              archivoCas
                ? 'bg-blue-600 text-white'
                : 'bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400',
            ]">
            <IconFileSpreadsheet v-if="!archivoCas" class="h-6 w-6" />
            <IconFileCheck v-else class="h-6 w-6" />
          </div>
          <p class="text-sm font-bold text-gray-900 dark:text-white">
            Archivo <span class="text-blue-600 dark:text-blue-400">CAS</span>
          </p>
          <p v-if="archivoCas" class="mt-1 text-xs text-blue-600 dark:text-blue-400 font-semibold truncate max-w-xs">
            {{ archivoCas.name }}
          </p>
          <p v-else class="mt-1 text-2xs text-gray-400">
            Arrastre o haga clic para seleccionar archivo (.xlsx / .xls)
          </p>
          <input
            ref="inputCas"
            type="file"
            accept=".xlsx,.xls"
            class="hidden"
            @change="(e) => onArchivoSeleccionado(e, 'cas')" />
        </div>

        <div
          @click="inputOtros?.click()"
          @dragover.prevent
          @drop.prevent="(e) => onDrop(e, 'otros')"
          :class="[
            'flex flex-col items-center justify-center rounded-2xl border-2 border-dashed p-8 text-center cursor-pointer transition-all duration-200 shadow-xs',
            archivoOtros
              ? 'border-violet-500 bg-violet-50/50 dark:border-violet-500/70 dark:bg-violet-950/20'
              : 'border-gray-200 bg-white hover:border-violet-400 hover:bg-violet-50/20 dark:border-gray-800 dark:bg-gray-900 dark:hover:border-violet-500/40 dark:hover:bg-gray-800/40',
          ]">
          <div
            :class="[
              'flex h-12 w-12 items-center justify-center rounded-2xl mb-3 shadow-xs transition-colors',
              archivoOtros
                ? 'bg-violet-600 text-white'
                : 'bg-violet-50 text-violet-600 dark:bg-violet-900/20 dark:text-violet-400',
            ]">
            <IconFileSpreadsheet v-if="!archivoOtros" class="h-6 w-6" />
            <IconFileCheck v-else class="h-6 w-6" />
          </div>
          <p class="text-sm font-bold text-gray-900 dark:text-white">
            Archivo <span class="text-violet-600 dark:text-violet-400">276 / 728</span>
          </p>
          <p v-if="archivoOtros" class="mt-1 text-xs text-violet-600 dark:text-violet-400 font-semibold truncate max-w-xs">
            {{ archivoOtros.name }}
          </p>
          <p v-else class="mt-1 text-2xs text-gray-400">
            Arrastre o haga clic para seleccionar archivo (.xlsx / .xls)
          </p>
          <input
            ref="inputOtros"
            type="file"
            accept=".xlsx,.xls"
            class="hidden"
            @change="(e) => onArchivoSeleccionado(e, 'otros')" />
        </div>
      </div>

      <div class="flex justify-center pt-2">
        <button
          type="button"
          @click="procesarArchivos"
          :disabled="!archivoCas && !archivoOtros"
          class="inline-flex items-center gap-2 rounded-xl bg-blue-600 px-6 py-2.5 text-xs font-bold text-white shadow-xs transition-all disabled:opacity-40 disabled:cursor-not-allowed hover:bg-blue-700 cursor-pointer">
          <IconSearch class="h-4 w-4" />
          <span>Comparar con MEF</span>
        </button>
      </div>
    </template>

    <div v-if="estaCargando" class="flex flex-col items-center justify-center py-24 space-y-4">
      <div class="h-10 w-10 animate-spin rounded-full border-4 border-blue-600 border-t-transparent" />
      <div class="text-center">
        <p class="text-sm font-bold text-gray-900 dark:text-white">
          Procesando archivos y contrastando datos con el sistema...
        </p>
        <p class="text-2xs text-gray-400 mt-0.5">
          Esto puede tomar unos segundos según el volumen de registros en las planillas.
        </p>
      </div>
    </div>

    <template v-if="resultado && !estaCargando">
      <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-6">
        <TarjetaResumen
          titulo="En Sistema"
          :valor="resultado.resumen.procesados"
          color="blue"
          :icono="IconFileSpreadsheet" />

        <TarjetaResumen
          titulo="En MEF"
          :valor="resultado.resumen.encontrados_mef"
          color="slate"
          :icono="IconFileCheck" />

        <TarjetaResumen
          titulo="Coincidencias"
          :valor="resultado.resumen.ok"
          color="emerald"
          claseValor="text-emerald-700 dark:text-emerald-400"
          :icono="IconCheck" />

        <TarjetaResumen
          titulo="Diferencias"
          :valor="resultado.resumen.diferencias"
          color="amber"
          claseValor="text-amber-700 dark:text-amber-400"
          :icono="IconAlertTriangle" />

        <TarjetaResumen
          titulo="Solo en Sistema"
          :valor="resultado.resumen.no_encontrados"
          color="red"
          claseValor="text-red-700 dark:text-red-400"
          :icono="IconAlertCircle" />

        <TarjetaResumen
          titulo="Solo en MEF"
          :valor="resultado.resumen.no_en_sistema"
          color="purple"
          claseValor="text-purple-700 dark:text-purple-400"
          :icono="IconAlertCircle" />
      </div>

      <div class="rounded-xl border border-gray-200 bg-white p-3.5 shadow-xs dark:border-gray-800 dark:bg-gray-900 space-y-3">
        <div class="flex flex-wrap items-center justify-between gap-3">
          <div class="flex flex-wrap items-center gap-1.5">
            <button
              v-for="f in filtrosEstado"
              :key="f.valor"
              @click="filtroEstado = f.valor"
              :class="[
                'inline-flex items-center gap-1.5 rounded-lg border px-2.5 py-1.5 text-2xs font-bold transition-all cursor-pointer',
                filtroEstado === f.valor
                  ? f.claseActiva
                  : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700',
              ]">
              <component :is="f.icono" class="h-3.5 w-3.5" />
              <span>{{ f.etiqueta }}</span>
              <span class="rounded-full bg-black/10 dark:bg-white/10 px-1.5 py-0.2 text-3xs font-mono">
                {{ conteosPorEstado[f.valor as keyof typeof conteosPorEstado] }}
              </span>
            </button>
          </div>

          <div class="relative w-full sm:w-72">
            <IconSearch class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              v-model="busqueda"
              placeholder="Buscar por DNI, nombre o campo..."
              class="w-full rounded-lg border border-gray-200 bg-gray-50/50 py-1.5 pl-8 pr-7 text-xs text-gray-900 placeholder:text-gray-400 focus:border-blue-500 focus:bg-white focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-700 dark:bg-gray-800 dark:text-white dark:placeholder:text-gray-500" />
            <button
              v-if="busqueda"
              type="button"
              @click="busqueda = ''"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
              <IconX class="h-3 w-3" />
            </button>
          </div>
        </div>

        <div class="flex flex-wrap items-center gap-1.5 pt-1 border-t border-gray-100 dark:border-gray-800">
          <span class="text-2xs font-medium text-gray-400 mr-1">Régimen:</span>
          <button
            v-for="r in filtrosRegimen"
            :key="r.valor"
            @click="filtroRegimen = r.valor"
            :class="[
              'inline-flex items-center gap-1 rounded-md border px-2 py-1 text-2xs font-semibold transition-all cursor-pointer',
              filtroRegimen === r.valor
                ? r.claseActiva
                : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700',
            ]">
            <span>{{ r.etiqueta }}</span>
            <span class="rounded-full bg-black/10 dark:bg-white/10 px-1 py-0.2 text-3xs font-mono">
              {{ conteosPorRegimen[r.valor] ?? 0 }}
            </span>
          </button>
        </div>
      </div>

      <div class="overflow-hidden rounded-xl border border-gray-200 bg-white shadow-xs dark:border-gray-800 dark:bg-gray-900">
        <div class="overflow-x-auto">
          <table class="w-full text-left text-xs">
            <thead>
              <tr class="border-b border-gray-100 bg-gray-50/75 text-2xs font-bold uppercase tracking-wider text-gray-500 dark:border-gray-800 dark:bg-gray-800/50 dark:text-gray-400">
                <th class="py-3 px-3 w-12">#</th>
                <th class="py-3 px-3 w-24">DNI</th>
                <th class="py-3 px-4 min-w-[200px]">Nombre</th>
                <th class="py-3 px-3 w-24">Régimen</th>
                <th class="py-3 px-3">Campo Evaluado</th>
                <th class="py-3 px-3 min-w-[140px]">Valor en Sistema</th>
                <th class="py-3 px-3 min-w-[140px]">Valor en MEF</th>
                <th class="py-3 px-3 w-32 text-right">Resultado</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-800">
              <tr
                v-for="fila in filasPaginadas"
                :key="`${fila.num}-${fila.campo}`"
                class="hover:bg-gray-50/80 dark:hover:bg-gray-800/50 transition-colors">
                <td class="py-2.5 px-3 text-2xs text-gray-400 tabular-nums">{{ fila.num }}</td>
                <td class="py-2.5 px-3 font-mono text-2xs font-semibold text-gray-700 dark:text-gray-300">
                  <RouterLink
                    v-if="fila.resultado !== 'NO_EXISTE_EN_SISTEMA'"
                    :to="`/perfil/${fila.dni}`"
                    class="hover:text-blue-600 dark:hover:text-blue-400">
                    {{ fila.dni }}
                  </RouterLink>
                  <span v-else>{{ fila.dni }}</span>
                </td>
                <td class="py-2.5 px-4 font-medium text-gray-900 dark:text-gray-100">
                  {{ fila.nombre }}
                </td>
                <td class="py-2.5 px-3 whitespace-nowrap">
                  <span
                    :class="claseBadgeRegimen(fila.regimen)"
                    class="inline-flex items-center rounded-md px-2 py-0.5 text-2xs font-semibold">
                    {{ fila.regimen || '—' }}
                  </span>
                </td>
                <td class="py-2.5 px-3 font-medium text-gray-600 dark:text-gray-300">
                  {{ fila.campo }}
                </td>
                <td class="py-2.5 px-3 text-gray-800 dark:text-gray-200">
                  <span :class="fila.resultado === 'DIFERENCIA' ? 'text-amber-700 dark:text-amber-400 font-semibold' : ''">
                    {{ fila.valor_propio || '—' }}
                  </span>
                </td>
                <td class="py-2.5 px-3 text-gray-800 dark:text-gray-200">
                  <span :class="fila.resultado === 'DIFERENCIA' ? 'text-amber-700 dark:text-amber-400 font-semibold' : ''">
                    {{ fila.valor_mef || '—' }}
                  </span>
                </td>
                <td class="py-2.5 px-3 text-right whitespace-nowrap">
                  <span
                    :class="[
                      'inline-flex items-center gap-1 rounded-md border px-2 py-0.5 text-2xs font-bold',
                      fila.resultado === 'OK' && 'bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-950/40 dark:text-emerald-300 dark:border-emerald-800',
                      fila.resultado === 'DIFERENCIA' && 'bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-950/40 dark:text-amber-300 dark:border-amber-800',
                      fila.resultado === 'NO_EXISTE_EN_MEF' && 'bg-red-50 text-red-700 border-red-200 dark:bg-red-950/40 dark:text-red-300 dark:border-red-800',
                      fila.resultado === 'NO_EXISTE_EN_SISTEMA' && 'bg-purple-50 text-purple-700 border-purple-200 dark:bg-purple-950/40 dark:text-purple-300 dark:border-purple-800',
                    ]">
                    <IconCheck v-if="fila.resultado === 'OK'" class="h-3 w-3" />
                    <IconAlertTriangle v-else-if="fila.resultado === 'DIFERENCIA'" class="h-3 w-3" />
                    <IconAlertCircle v-else class="h-3 w-3" />
                    {{ etiquetaResultado(fila.resultado) }}
                  </span>
                </td>
              </tr>
              <tr v-if="filasFiltradas.length === 0">
                <td colspan="8" class="py-10 text-center text-xs text-gray-400">
                  No hay registros para los filtros seleccionados
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="flex items-center justify-between border-t border-gray-100 bg-gray-50/50 px-4 py-3 dark:border-gray-800 dark:bg-gray-800/30 text-2xs text-gray-500 dark:text-gray-400">
          <span>Mostrando {{ inicioRango }}–{{ finRango }} de {{ filasFiltradas.length }} comparaciones</span>
          <div v-if="totalPaginas > 1" class="flex items-center gap-1">
            <button
              type="button"
              @click="paginaActual--"
              :disabled="paginaActual === 1"
              class="inline-flex items-center gap-1 rounded-lg border border-gray-200 bg-white px-2.5 py-1 text-2xs font-semibold text-gray-700 shadow-xs hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 cursor-pointer">
              <IconChevronLeft class="h-3 w-3" />
              <span>Anterior</span>
            </button>
            <span class="px-2 font-mono">{{ paginaActual }} / {{ totalPaginas }}</span>
            <button
              type="button"
              @click="paginaActual++"
              :disabled="paginaActual === totalPaginas"
              class="inline-flex items-center gap-1 rounded-lg border border-gray-200 bg-white px-2.5 py-1 text-2xs font-semibold text-gray-700 shadow-xs hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 cursor-pointer">
              <span>Siguiente</span>
              <IconChevronRight class="h-3 w-3" />
            </button>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
