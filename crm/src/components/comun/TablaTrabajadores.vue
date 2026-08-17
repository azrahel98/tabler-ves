<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { formatearFecha } from '@/utils/fechas'
import {
  IconSearch,
  IconX,
  IconChevronLeft,
  IconChevronRight,
  IconArrowsSort,
  IconSortAscending,
  IconSortDescending,
  IconAdjustmentsHorizontal,
  IconDownload,
  IconExternalLink,
  IconCheck,
  IconAlertTriangle,
  IconBuildingCommunity,
  IconBriefcase,
} from '@tabler/icons-vue'

export interface TrabajadorGenerico {
  dni: string
  nombre?: string | null
  avatar?: string | null
  ingreso?: string | null
  renuncia?: string | null
  direccion?: string | null
  cargo?: string | { id?: number; nombre: string } | null
  area?: string | { id?: number; nombre: string } | null
  regimen?: string | { id?: number; nombre: string } | null
  sindicato?: string | { id?: number; nombre: string } | null
  edad?: number | null
}

type TipoColumnaOrden = 'nombre' | 'area' | 'cargo' | 'regimen' | 'ingreso' | 'direccion'
type TipoPestañaFiltro = 'todos' | 'activos' | 'cesados'

const props = withDefaults(
  defineProps<{
    trabajadores: TrabajadorGenerico[]
    titulo?: string
    subtitulo?: string
    mostrarPestanas?: boolean
    mostrarCheckboxes?: boolean
    mostrarBuscador?: boolean
    mostrarExportar?: boolean
    mostrarFiltros?: boolean
    mostrarFiltroArea?: boolean
    mostrarFiltroRegimen?: boolean
    mostrarColumnaArea?: boolean
    mostrarColumnaRegimen?: boolean
    mostrarColumnaCargo?: boolean
    mostrarColumnaIngreso?: boolean
    mostrarColumnaDireccion?: boolean
    mostrarColumnaEstado?: boolean
    mostrarColumnaAccion?: boolean
    elementosPorPaginaInicial?: number
  }>(),
  {
    titulo: 'Padrón de Trabajadores',
    subtitulo: '',
    mostrarPestanas: true,
    mostrarCheckboxes: true,
    mostrarBuscador: true,
    mostrarExportar: true,
    mostrarFiltros: true,
    mostrarFiltroArea: true,
    mostrarFiltroRegimen: true,
    mostrarColumnaArea: true,
    mostrarColumnaRegimen: true,
    mostrarColumnaCargo: true,
    mostrarColumnaIngreso: true,
    mostrarColumnaDireccion: true,
    mostrarColumnaEstado: true,
    mostrarColumnaAccion: true,
    elementosPorPaginaInicial: 10,
  },
)

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
const router = useRouter()
const terminoBusqueda = ref('')
const pestanaActiva = ref<TipoPestañaFiltro>('todos')
const mostrarPanelFiltros = ref(false)
const filtroAreaSeleccionada = ref('todas')
const filtroRegimenSeleccionado = ref('todos')
const paginaActual = ref(1)
const elementosPorPagina = ref(props.elementosPorPaginaInicial)
const columnaOrden = ref<TipoColumnaOrden | null>(null)
const direccionOrden = ref<'asc' | 'desc'>('asc')
const erroresAvatar = ref(new Set<string>())
const elementosSeleccionados = ref(new Set<string>())

function extraerTexto(valor: any): string {
  if (!valor) return ''
  if (typeof valor === 'string') return valor.trim()
  if (typeof valor === 'object' && valor.nombre) return String(valor.nombre).trim()
  return String(valor).trim()
}

function esActivo(t: TrabajadorGenerico): boolean {
  if (t.renuncia && t.renuncia.trim() !== '') return false
  return true
}

function obtenerIniciales(nombre?: string | null): string {
  if (!nombre) return 'T'
  const partes = nombre.trim().split(/\s+/)
  if (partes.length === 1) return partes[0].charAt(0).toUpperCase()
  return (partes[0].charAt(0) + partes[1].charAt(0)).toUpperCase()
}

function obtenerAvatarUrl(dni: string): string {
  return `${apiBaseUrl.replace(/\/$/, '')}/personal/avatar/${dni}`
}

function manejarErrorAvatar(dni: string) {
  erroresAvatar.value.add(dni)
}

function navegarAlPerfil(dni: string) {
  if (dni) {
    router.push(`/perfil/${dni}`)
  }
}

function alternarOrden(columna: TipoColumnaOrden) {
  if (columnaOrden.value === columna) {
    if (direccionOrden.value === 'asc') {
      direccionOrden.value = 'desc'
    } else {
      columnaOrden.value = null
      direccionOrden.value = 'asc'
    }
  } else {
    columnaOrden.value = columna
    direccionOrden.value = 'asc'
  }
  paginaActual.value = 1
}

const conteoPestanas = computed(() => {
  let total = props.trabajadores.length
  let activos = 0
  let cesados = 0
  for (const t of props.trabajadores) {
    if (esActivo(t)) activos++
    else cesados++
  }
  return { total, activos, cesados }
})

const listaOpcionesAreas = computed(() => {
  const conjunto = new Set<string>()
  for (const t of props.trabajadores) {
    const area = extraerTexto(t.area)
    if (area) conjunto.add(area)
  }
  return Array.from(conjunto).sort()
})

const listaOpcionesRegimenes = computed(() => {
  const conjunto = new Set<string>()
  for (const t of props.trabajadores) {
    const reg = extraerTexto(t.regimen)
    if (reg) conjunto.add(reg)
  }
  return Array.from(conjunto).sort()
})

const trabajadoresFiltrados = computed(() => {
  let resultado = props.trabajadores

  if (pestanaActiva.value === 'activos') {
    resultado = resultado.filter((t) => esActivo(t))
  } else if (pestanaActiva.value === 'cesados') {
    resultado = resultado.filter((t) => !esActivo(t))
  }

  if (filtroAreaSeleccionada.value !== 'todas') {
    resultado = resultado.filter(
      (t) => extraerTexto(t.area).toLowerCase() === filtroAreaSeleccionada.value.toLowerCase(),
    )
  }

  if (filtroRegimenSeleccionado.value !== 'todos') {
    resultado = resultado.filter(
      (t) => extraerTexto(t.regimen).toLowerCase() === filtroRegimenSeleccionado.value.toLowerCase(),
    )
  }

  if (terminoBusqueda.value.trim()) {
    const termino = terminoBusqueda.value.trim().toLowerCase()
    resultado = resultado.filter((t) => {
      const nombre = (t.nombre || '').toLowerCase()
      const dni = t.dni || ''
      const cargo = extraerTexto(t.cargo).toLowerCase()
      const area = extraerTexto(t.area).toLowerCase()
      const regimen = extraerTexto(t.regimen).toLowerCase()
      const direccion = (t.direccion || '').toLowerCase()
      return (
        nombre.includes(termino) ||
        dni.includes(termino) ||
        cargo.includes(termino) ||
        area.includes(termino) ||
        regimen.includes(termino) ||
        direccion.includes(termino)
      )
    })
  }

  if (columnaOrden.value) {
    const col = columnaOrden.value
    const dir = direccionOrden.value === 'asc' ? 1 : -1
    resultado = [...resultado].sort((a, b) => {
      let valA = ''
      let valB = ''
      if (col === 'nombre') {
        valA = a.nombre || ''
        valB = b.nombre || ''
      } else if (col === 'area') {
        valA = extraerTexto(a.area)
        valB = extraerTexto(b.area)
      } else if (col === 'cargo') {
        valA = extraerTexto(a.cargo)
        valB = extraerTexto(b.cargo)
      } else if (col === 'regimen') {
        valA = extraerTexto(a.regimen)
        valB = extraerTexto(b.regimen)
      } else if (col === 'ingreso') {
        valA = a.ingreso || ''
        valB = b.ingreso || ''
      } else if (col === 'direccion') {
        valA = a.direccion || ''
        valB = b.direccion || ''
      }
      return valA.localeCompare(valB, 'es', { numeric: true, sensitivity: 'base' }) * dir
    })
  }

  return resultado
})

const totalPaginas = computed(() => {
  return Math.max(1, Math.ceil(trabajadoresFiltrados.value.length / elementosPorPagina.value))
})

const trabajadoresPaginados = computed(() => {
  const inicio = (paginaActual.value - 1) * elementosPorPagina.value
  return trabajadoresFiltrados.value.slice(inicio, inicio + elementosPorPagina.value)
})

const todosPaginadosSeleccionados = computed(() => {
  if (trabajadoresPaginados.value.length === 0) return false
  return trabajadoresPaginados.value.every((t) => elementosSeleccionados.value.has(t.dni))
})

function alternarSeleccionarTodos() {
  if (todosPaginadosSeleccionados.value) {
    for (const t of trabajadoresPaginados.value) {
      elementosSeleccionados.value.delete(t.dni)
    }
  } else {
    for (const t of trabajadoresPaginados.value) {
      elementosSeleccionados.value.add(t.dni)
    }
  }
}

function alternarSeleccionIndividual(dni: string) {
  if (elementosSeleccionados.value.has(dni)) {
    elementosSeleccionados.value.delete(dni)
  } else {
    elementosSeleccionados.value.add(dni)
  }
}

function cambiarPagina(pagina: number) {
  if (pagina >= 1 && pagina <= totalPaginas.value) {
    paginaActual.value = pagina
  }
}

function restablecerFiltros() {
  terminoBusqueda.value = ''
  pestanaActiva.value = 'todos'
  filtroAreaSeleccionada.value = 'todas'
  filtroRegimenSeleccionado.value = 'todos'
  columnaOrden.value = null
  direccionOrden.value = 'asc'
  paginaActual.value = 1
}

const contadorFiltrosActivos = computed(() => {
  let contador = 0
  if (filtroAreaSeleccionada.value !== 'todas') contador++
  if (filtroRegimenSeleccionado.value !== 'todos') contador++
  if (terminoBusqueda.value.trim() !== '') contador++
  if (pestanaActiva.value !== 'todos') contador++
  return contador
})

function cambiarPestana(p: 'todos' | 'activos' | 'cesados') {
  pestanaActiva.value = p
  paginaActual.value = 1
}

function exportarCSV() {
  const encabezados = ['DNI', 'Nombre', 'Área', 'Cargo', 'Régimen', 'Fecha Ingreso', 'Fecha Salida', 'Dirección']
  const filas = trabajadoresFiltrados.value.map((t) => [
    `"${t.dni}"`,
    `"${(t.nombre || '').replace(/"/g, '""')}"`,
    `"${extraerTexto(t.area).replace(/"/g, '""')}"`,
    `"${extraerTexto(t.cargo).replace(/"/g, '""')}"`,
    `"${extraerTexto(t.regimen).replace(/"/g, '""')}"`,
    `"${t.ingreso || ''}"`,
    `"${t.renuncia || ''}"`,
    `"${(t.direccion || '').replace(/"/g, '""')}"`,
  ])

  const contenido = [encabezados.join(','), ...filas.map((f) => f.join(','))].join('\n')
  const blob = new Blob(['\uFEFF' + contenido], { type: 'text/csv;charset=utf-8;' })
  const enlace = document.createElement('a')
  enlace.href = URL.createObjectURL(blob)
  enlace.setAttribute('download', `padron_trabajadores_${new Date().toISOString().slice(0, 10)}.csv`)
  document.body.appendChild(enlace)
  enlace.click()
  document.body.removeChild(enlace)
}
</script>

<template>
  <div
    class="rounded-2xl border border-slate-200/80 bg-white shadow-xs dark:border-navy-700/80 dark:bg-navy-800 overflow-hidden w-full transition-all">
    <div
      v-if="mostrarPestanas"
      class="px-5 pt-4 border-b border-slate-100 dark:border-navy-700/80 flex items-center justify-between">
      <div class="flex items-center space-x-6 text-xs">
        <button
          type="button"
          @click="cambiarPestana('todos')"
          :class="[
            'pb-3 font-semibold transition-all relative cursor-pointer',
            pestanaActiva === 'todos'
              ? 'text-slate-900 dark:text-white border-b-2 border-slate-900 dark:border-white font-bold'
              : 'text-slate-400 hover:text-slate-600 dark:text-slate-400 dark:hover:text-slate-200',
          ]">
          <span>Todos</span>
          <span class="ml-1 text-2xs text-slate-400 font-normal font-mono">({{ conteoPestanas.total }})</span>
        </button>

        <button
          type="button"
          @click="cambiarPestana('activos')"
          :class="[
            'pb-3 font-semibold transition-all relative cursor-pointer',
            pestanaActiva === 'activos'
              ? 'text-slate-900 dark:text-white border-b-2 border-slate-900 dark:border-white font-bold'
              : 'text-slate-400 hover:text-slate-600 dark:text-slate-400 dark:hover:text-slate-200',
          ]">
          <span>Activos</span>
          <span class="ml-1 text-2xs text-slate-400 font-normal font-mono">({{ conteoPestanas.activos }})</span>
        </button>

        <button
          type="button"
          @click="cambiarPestana('cesados')"
          :class="[
            'pb-3 font-semibold transition-all relative cursor-pointer',
            pestanaActiva === 'cesados'
              ? 'text-slate-900 dark:text-white border-b-2 border-slate-900 dark:border-white font-bold'
              : 'text-slate-400 hover:text-slate-600 dark:text-slate-400 dark:hover:text-slate-200',
          ]">
          <span>Cesados / Inactivos</span>
          <span class="ml-1 text-2xs text-slate-400 font-normal font-mono">({{ conteoPestanas.cesados }})</span>
        </button>
      </div>
    </div>

    <div
      v-if="mostrarBuscador || mostrarExportar || mostrarFiltros"
      class="p-3.5 sm:p-4 border-b border-slate-100 dark:border-navy-700/80 space-y-3">
      <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-3">
        <div v-if="mostrarBuscador" class="relative flex-1 max-w-xl">
          <IconSearch class="absolute left-3 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-slate-400 pointer-events-none" />
          <input
            v-model="terminoBusqueda"
            type="text"
            placeholder="Buscar..."
            class="w-full pl-9 pr-8 py-1.5 rounded-xl border border-slate-200/80 bg-slate-50/60 text-xs text-slate-800 placeholder:text-slate-400 focus:bg-white focus:border-slate-400 focus:outline-none focus:ring-1 focus:ring-slate-300 dark:border-navy-700 dark:bg-navy-900/50 dark:text-white dark:placeholder:text-slate-500 transition-all" />
          <button
            v-if="terminoBusqueda"
            type="button"
            @click="terminoBusqueda = ''"
            class="absolute right-2.5 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 cursor-pointer">
            <IconX class="h-3.5 w-3.5" />
          </button>
        </div>

        <slot name="herramientas-extra" />

        <div class="flex items-center gap-2 self-end sm:self-auto shrink-0">
          <button
            v-if="mostrarExportar"
            type="button"
            @click="exportarCSV"
            class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl border border-slate-200/80 bg-white hover:bg-slate-50 dark:border-navy-700 dark:bg-navy-800 dark:hover:bg-navy-700/60 text-xs font-semibold text-slate-700 dark:text-slate-200 transition-colors shadow-2xs cursor-pointer">
            <IconDownload class="h-3.5 w-3.5 text-slate-500" />
            <span>Exportar</span>
          </button>

          <button
            v-if="mostrarFiltros && (mostrarFiltroArea || mostrarFiltroRegimen)"
            type="button"
            @click="mostrarPanelFiltros = !mostrarPanelFiltros"
            :class="[
              'inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl border text-xs font-semibold transition-colors shadow-2xs cursor-pointer',
              mostrarPanelFiltros || contadorFiltrosActivos > 0
                ? 'bg-slate-900 border-slate-900 text-white dark:bg-white dark:border-white dark:text-slate-900'
                : 'border-slate-200/80 bg-white hover:bg-slate-50 text-slate-700 dark:border-navy-700 dark:bg-navy-800 dark:text-slate-200 dark:hover:bg-navy-700/60',
            ]">
            <IconAdjustmentsHorizontal class="h-3.5 w-3.5" />
            <span>Filtros</span>
            <span
              v-if="contadorFiltrosActivos > 0"
              class="ml-0.5 flex h-4 w-4 items-center justify-center rounded-full bg-emerald-500 text-[10px] font-bold text-white">
              {{ contadorFiltrosActivos }}
            </span>
          </button>
        </div>
      </div>

      <!-- Panel Desplegable de Filtros -->
      <div
        v-if="mostrarPanelFiltros"
        class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2.5 p-3 rounded-xl bg-slate-50/70 dark:bg-navy-900/40 border border-slate-200/60 dark:border-navy-700/50">
        <div v-if="mostrarFiltroArea" class="relative space-y-1">
          <label class="text-3xs uppercase font-semibold text-slate-400 block">Área</label>
          <div class="relative">
            <IconBuildingCommunity
              class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-slate-400 pointer-events-none" />
            <select
              v-model="filtroAreaSeleccionada"
              class="w-full pl-8 pr-3 py-1.5 rounded-lg border border-slate-200/80 bg-white text-xs text-slate-800 focus:outline-none dark:border-navy-700 dark:bg-navy-800 dark:text-white">
              <option value="todas">Todas las Áreas</option>
              <option v-for="area in listaOpcionesAreas" :key="area" :value="area">{{ area }}</option>
            </select>
          </div>
        </div>

        <div v-if="mostrarFiltroRegimen" class="relative space-y-1">
          <label class="text-3xs uppercase font-semibold text-slate-400 block">Régimen</label>
          <div class="relative">
            <IconBriefcase
              class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-slate-400 pointer-events-none" />
            <select
              v-model="filtroRegimenSeleccionado"
              class="w-full pl-8 pr-3 py-1.5 rounded-lg border border-slate-200/80 bg-white text-xs text-slate-800 focus:outline-none dark:border-navy-700 dark:bg-navy-800 dark:text-white">
              <option value="todos">Todos los Regímenes</option>
              <option v-for="reg in listaOpcionesRegimenes" :key="reg" :value="reg">{{ reg }}</option>
            </select>
          </div>
        </div>

        <div class="flex items-end">
          <button
            type="button"
            @click="restablecerFiltros"
            class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-2xs font-semibold text-slate-600 hover:text-slate-900 dark:text-slate-300 dark:hover:text-white transition-colors cursor-pointer">
            <IconX class="h-3 w-3" />
            <span>Restablecer todo</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Tabla Principal -->
    <div class="w-full overflow-x-auto">
      <table class="w-full text-left border-collapse">
        <thead class="bg-white dark:bg-navy-800 border-b border-slate-100 dark:border-navy-700/80">
          <tr class="text-2xs font-medium text-slate-400 dark:text-slate-400 select-none">
            <th v-if="mostrarCheckboxes" scope="col" class="w-10 px-4 py-3 text-center">
              <input
                type="checkbox"
                :checked="todosPaginadosSeleccionados"
                @change="alternarSeleccionarTodos"
                class="h-3.5 w-3.5 rounded border-slate-300 text-slate-900 focus:ring-0 dark:border-navy-600 dark:bg-navy-700 cursor-pointer" />
            </th>

            <th
              scope="col"
              @click="alternarOrden('nombre')"
              class="px-4 py-3 min-w-[220px] cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors">
              <div class="flex items-center gap-1">
                <span>Nombre</span>
                <component
                  :is="
                    columnaOrden === 'nombre'
                      ? direccionOrden === 'asc'
                        ? IconSortAscending
                        : IconSortDescending
                      : IconArrowsSort
                  "
                  class="h-3 w-3 text-slate-400 opacity-60" />
              </div>
            </th>

            <th
              v-if="mostrarColumnaArea || mostrarColumnaDireccion"
              scope="col"
              @click="alternarOrden(mostrarColumnaDireccion ? 'direccion' : 'area')"
              class="px-4 py-3 min-w-[240px] cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors">
              <div class="flex items-center gap-1">
                <span>{{ mostrarColumnaDireccion ? 'Dirección' : 'Área' }}</span>
                <component
                  :is="
                    columnaOrden === (mostrarColumnaDireccion ? 'direccion' : 'area')
                      ? direccionOrden === 'asc'
                        ? IconSortAscending
                        : IconSortDescending
                      : IconArrowsSort
                  "
                  class="h-3 w-3 text-slate-400 opacity-60" />
              </div>
            </th>

            <th v-if="mostrarColumnaEstado" scope="col" class="px-4 py-3 min-w-[130px]">
              <div class="flex items-center gap-1">
                <span>Estado</span>
                <IconArrowsSort class="h-3 w-3 text-slate-400 opacity-60" />
              </div>
            </th>

            <th
              v-if="mostrarColumnaCargo || mostrarColumnaRegimen"
              scope="col"
              @click="alternarOrden(mostrarColumnaCargo ? 'cargo' : 'regimen')"
              class="px-4 py-3 min-w-[160px] cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors">
              <div class="flex items-center gap-1">
                <span>{{ mostrarColumnaCargo ? 'Cargo' : 'Régimen' }}</span>
                <component
                  :is="
                    columnaOrden === (mostrarColumnaCargo ? 'cargo' : 'regimen')
                      ? direccionOrden === 'asc'
                        ? IconSortAscending
                        : IconSortDescending
                      : IconArrowsSort
                  "
                  class="h-3 w-3 text-slate-400 opacity-60" />
              </div>
            </th>

            <th
              v-if="mostrarColumnaIngreso"
              scope="col"
              @click="alternarOrden('ingreso')"
              class="px-4 py-3 min-w-[120px] cursor-pointer hover:text-slate-700 dark:hover:text-slate-200 transition-colors">
              <div class="flex items-center gap-1">
                <span>Fecha Ingreso</span>
                <component
                  :is="
                    columnaOrden === 'ingreso'
                      ? direccionOrden === 'asc'
                        ? IconSortAscending
                        : IconSortDescending
                      : IconArrowsSort
                  "
                  class="h-3 w-3 text-slate-400 opacity-60" />
              </div>
            </th>

            <slot name="encabezados-extra" />

            <th v-if="mostrarColumnaAccion" scope="col" class="w-12 px-4 py-3 text-right"></th>
          </tr>
        </thead>

        <tbody class="divide-y divide-slate-100 dark:divide-navy-700/60 text-xs">
          <tr v-if="trabajadoresFiltrados.length === 0" class="bg-white dark:bg-navy-800">
            <td colspan="12" class="px-4 py-12 text-center text-slate-400 text-xs font-medium">
              No se encontraron trabajadores que coincidan con los criterios de búsqueda.
            </td>
          </tr>

          <tr
            v-for="persona in trabajadoresPaginados"
            :key="persona.dni"
            @click="navegarAlPerfil(persona.dni)"
            class="hover:bg-slate-50/70 dark:hover:bg-navy-900/40 transition-colors cursor-pointer group">
            <td v-if="mostrarCheckboxes" class="px-4 py-3 text-center" @click.stop>
              <input
                type="checkbox"
                :checked="elementosSeleccionados.has(persona.dni)"
                @change="alternarSeleccionIndividual(persona.dni)"
                class="h-3.5 w-3.5 rounded border-slate-300 text-slate-900 focus:ring-0 dark:border-navy-600 dark:bg-navy-700 cursor-pointer" />
            </td>

            <td class="px-4 py-3">
              <div class="flex items-center justify-between gap-2">
                <div class="flex items-center gap-2.5 min-w-0">
                  <div
                    class="relative flex h-7 w-7 shrink-0 items-center justify-center overflow-hidden rounded-full border border-slate-200/80 bg-slate-100 dark:border-navy-600 dark:bg-navy-700 shadow-2xs">
                    <img
                      v-if="persona.dni && !erroresAvatar.has(persona.dni)"
                      :src="obtenerAvatarUrl(persona.dni)"
                      :alt="persona.nombre || 'Personal'"
                      class="h-full w-full object-cover"
                      loading="lazy"
                      @error="manejarErrorAvatar(persona.dni)" />
                    <span
                      v-else
                      class="flex h-full w-full items-center justify-center text-3xs font-bold text-slate-600 dark:text-slate-200 uppercase">
                      {{ obtenerIniciales(persona.nombre) }}
                    </span>
                  </div>

                  <div class="min-w-0">
                    <span
                      class="font-semibold text-xs text-slate-800 dark:text-white group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors truncate block">
                      {{ persona.nombre }}
                    </span>
                    <span class="text-3xs font-mono text-slate-400 block"> DNI: {{ persona.dni }} </span>
                  </div>
                </div>

                <button
                  type="button"
                  @click.stop="navegarAlPerfil(persona.dni)"
                  class="opacity-0 group-hover:opacity-100 inline-flex items-center gap-1 px-2 py-0.5 rounded-md border border-slate-200 bg-white hover:bg-slate-50 dark:border-navy-600 dark:bg-navy-700 text-3xs font-medium text-slate-600 dark:text-slate-200 shadow-2xs transition-all shrink-0">
                  <IconExternalLink class="h-2.5 w-2.5" />
                  <span>Abrir</span>
                </button>
              </div>
            </td>

            <td v-if="mostrarColumnaArea || mostrarColumnaDireccion" class="px-4 py-3">
              <span class="text-xs text-slate-600 dark:text-slate-300 truncate block max-w-xs">
                {{ persona.direccion || extraerTexto(persona.area) || '-' }}
              </span>
            </td>

            <td v-if="mostrarColumnaEstado" class="px-4 py-3">
              <span
                v-if="esActivo(persona)"
                class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-slate-100 dark:bg-navy-700/80 text-slate-700 dark:text-slate-200 text-2xs font-medium border border-slate-200/50 dark:border-navy-600/50">
                <IconCheck class="h-2.5 w-2.5 text-slate-600 dark:text-slate-300 stroke-[3]" />
                <span>Activo</span>
              </span>
              <span
                v-else
                class="inline-flex items-center gap-1 px-2.5 py-0.5 rounded-full bg-amber-50 dark:bg-amber-950/30 text-amber-700 dark:text-amber-300 text-3xs font-medium border border-amber-200/60 dark:border-amber-800/40">
                <IconAlertTriangle class="h-2.5 w-2.5 text-amber-500" />
                <span>Cesado</span>
              </span>
            </td>

            <td v-if="mostrarColumnaCargo || mostrarColumnaRegimen" class="px-4 py-3">
              <div class="truncate max-w-[200px]">
                <span class="text-xs font-medium text-slate-700 dark:text-slate-300 block truncate">
                  {{ extraerTexto(persona.cargo) || '-' }}
                </span>
                <RouterLink
                  v-if="extraerTexto(persona.regimen)"
                  :to="`/regimen/${encodeURIComponent(extraerTexto(persona.regimen))}`"
                  @click.stop
                  class="text-2xs text-slate-400 hover:text-blue-500 dark:text-slate-400 truncate block">
                  {{ extraerTexto(persona.regimen) }}
                </RouterLink>
              </div>
            </td>

            <td
              v-if="mostrarColumnaIngreso"
              class="px-4 py-3 text-xs font-mono text-slate-500 dark:text-slate-400 whitespace-nowrap">
              {{ formatearFecha(persona.ingreso) || '-' }}
            </td>

            <slot name="celdas-extra" :trabajador="persona" />

            <td v-if="mostrarColumnaAccion" class="px-4 py-3 text-right" @click.stop>
              <slot name="acciones" :trabajador="persona">
                <button
                  type="button"
                  @click="navegarAlPerfil(persona.dni)"
                  class="p-1 rounded-lg border border-slate-200/80 dark:border-navy-700 bg-white dark:bg-navy-800 hover:bg-slate-50 dark:hover:bg-navy-700 text-slate-400 hover:text-slate-700 dark:hover:text-white transition-colors shadow-2xs cursor-pointer"
                  title="Ver perfil completo">
                  <IconChevronRight class="h-3.5 w-3.5" />
                </button>
              </slot>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Paginación Inferior -->
    <div
      class="flex flex-wrap items-center justify-between gap-3 border-t border-slate-100 px-5 py-3 dark:border-navy-700/80 bg-white dark:bg-navy-800">
      <span class="text-2xs text-slate-400">
        Mostrando {{ Math.min(trabajadoresFiltrados.length, (paginaActual - 1) * elementosPorPagina + 1) }} -
        {{ Math.min(trabajadoresFiltrados.length, paginaActual * elementosPorPagina) }} de
        {{ trabajadoresFiltrados.length }} trabajadores
      </span>

      <div class="flex items-center gap-1.5">
        <button
          type="button"
          @click="cambiarPagina(paginaActual - 1)"
          :disabled="paginaActual === 1"
          class="inline-flex h-7 w-7 items-center justify-center rounded-lg border border-slate-200/80 bg-white text-slate-600 hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-navy-700 dark:bg-navy-800 dark:text-slate-300 cursor-pointer transition-colors shadow-2xs">
          <IconChevronLeft class="h-3.5 w-3.5" />
        </button>

        <span class="text-2xs font-mono text-slate-500 px-1"> {{ paginaActual }} / {{ totalPaginas }} </span>

        <button
          type="button"
          @click="cambiarPagina(paginaActual + 1)"
          :disabled="paginaActual === totalPaginas"
          class="inline-flex h-7 w-7 items-center justify-center rounded-lg border border-slate-200/80 bg-white text-slate-600 hover:bg-slate-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-navy-700 dark:bg-navy-800 dark:text-slate-300 cursor-pointer transition-colors shadow-2xs">
          <IconChevronRight class="h-3.5 w-3.5" />
        </button>
      </div>
    </div>
  </div>
</template>
