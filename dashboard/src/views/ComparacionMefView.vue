<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-title-lg font-bold leading-tight text-gray-900 dark:text-white tracking-tight">Comparación MEF</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Contraste los datos del sistema con los archivos Excel del MEF</p>
        </div>
        <div v-if="resultado" class="text-xs text-gray-400 dark:text-gray-500">
          Procesado el {{ resultado.resumen.fecha_comparacion }}
        </div>
      </div>

      
      <div v-if="error" class="mb-4 flex items-center gap-3 rounded-xl border border-red-200 bg-red-50 px-4 py-3 dark:border-red-800/40 dark:bg-red-900/20">
        <AlertCircle class="h-5 w-5 shrink-0 text-red-500" />
        <p class="text-sm text-red-700 dark:text-red-400">{{ error }}</p>
        <button class="ml-auto text-red-400 hover:text-red-600" @click="error = null">
          <X class="h-4 w-4" />
        </button>
      </div>

      
      <template v-if="!resultado && !cargando">
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 mb-4">
          
          <div
            class="flex flex-col items-center justify-center rounded-2xl border-2 border-dashed border-gray-200 dark:border-white/10 bg-card py-10 text-center cursor-pointer transition-colors hover:border-blue-400/60 hover:bg-blue-50/30 dark:hover:bg-white/5"
            :class="archivoCas ? 'border-blue-400 bg-blue-50/40 dark:bg-blue-900/10' : ''"
            @click="inputCas?.click()"
            @dragover.prevent
            @drop.prevent="(e) => onDrop(e, 'cas')">
            <FileSpreadsheet class="h-10 w-10 mb-3" :class="archivoCas ? 'text-blue-500' : 'text-gray-300 dark:text-gray-600'" />
            <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              Archivo <span class="text-blue-600 dark:text-blue-400">CAS</span>
            </p>
            <p v-if="archivoCas" class="mt-1 text-xs text-blue-600 dark:text-blue-400 font-medium truncate max-w-[200px]">
              {{ archivoCas.name }}
            </p>
            <p v-else class="mt-1 text-xs text-gray-400 dark:text-gray-500">Arrastre o haga clic (.xlsx)</p>
            <input ref="inputCas" type="file" accept=".xlsx,.xls" class="hidden" @change="(e) => onArchivoSeleccionado(e, 'cas')" />
          </div>

          
          <div
            class="flex flex-col items-center justify-center rounded-2xl border-2 border-dashed border-gray-200 dark:border-white/10 bg-card py-10 text-center cursor-pointer transition-colors hover:border-violet-400/60 hover:bg-violet-50/30 dark:hover:bg-white/5"
            :class="archivoOtros ? 'border-violet-400 bg-violet-50/40 dark:bg-violet-900/10' : ''"
            @click="inputOtros?.click()"
            @dragover.prevent
            @drop.prevent="(e) => onDrop(e, 'otros')">
            <FileSpreadsheet class="h-10 w-10 mb-3" :class="archivoOtros ? 'text-violet-500' : 'text-gray-300 dark:text-gray-600'" />
            <p class="text-sm font-semibold text-gray-700 dark:text-gray-300">
              Archivo <span class="text-violet-600 dark:text-violet-400">276 / 728</span>
            </p>
            <p v-if="archivoOtros" class="mt-1 text-xs text-violet-600 dark:text-violet-400 font-medium truncate max-w-[200px]">
              {{ archivoOtros.name }}
            </p>
            <p v-else class="mt-1 text-xs text-gray-400 dark:text-gray-500">Arrastre o haga clic (.xlsx)</p>
            <input ref="inputOtros" type="file" accept=".xlsx,.xls" class="hidden" @change="(e) => onArchivoSeleccionado(e, 'otros')" />
          </div>
        </div>

        
        <div class="flex justify-center">
          <button
            @click="procesarArchivos"
            :disabled="!archivoCas && !archivoOtros"
            class="inline-flex items-center gap-2 rounded-xl bg-primary px-6 py-2.5 text-sm font-semibold text-white shadow-sm transition-opacity disabled:opacity-40 disabled:cursor-not-allowed hover:opacity-90">
            <Search class="h-4 w-4" />
            Comparar con MEF
          </button>
        </div>
      </template>

      
      <div v-if="cargando" class="flex flex-col items-center gap-3 py-20">
        <div class="h-10 w-10 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
        <span class="text-sm text-gray-500 dark:text-gray-400">Procesando archivos, por favor espere…</span>
      </div>

      
      <template v-if="resultado">
        
        <div class="mb-6 grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-6">
          <div class="rounded-xl border border-gray-100 bg-card px-4 py-3 shadow-theme-xs dark:border-white/6">
            <p class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">En sistema</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-gray-900 dark:text-white">{{ resultado.resumen.procesados }}</p>
          </div>
          <div class="rounded-xl border border-gray-100 bg-card px-4 py-3 shadow-theme-xs dark:border-white/6">
            <p class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">En MEF</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-gray-900 dark:text-white">{{ resultado.resumen.encontrados_mef }}</p>
          </div>
          <div class="rounded-xl border border-green-100 bg-green-50 px-4 py-3 shadow-theme-xs dark:border-green-800/30 dark:bg-green-900/10">
            <p class="text-xs font-medium text-green-600 dark:text-green-400 uppercase tracking-wider">Coincidencias</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-green-700 dark:text-green-400">{{ resultado.resumen.ok }}</p>
          </div>
          <div class="rounded-xl border border-orange-100 bg-orange-50 px-4 py-3 shadow-theme-xs dark:border-orange-800/30 dark:bg-orange-900/10">
            <p class="text-xs font-medium text-orange-600 dark:text-orange-400 uppercase tracking-wider">Diferencias</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-orange-700 dark:text-orange-400">{{ resultado.resumen.diferencias }}</p>
          </div>
          <div class="rounded-xl border border-red-100 bg-red-50 px-4 py-3 shadow-theme-xs dark:border-red-800/30 dark:bg-red-900/10">
            <p class="text-xs font-medium text-red-600 dark:text-red-400 uppercase tracking-wider">Solo en sistema</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-red-700 dark:text-red-400">{{ resultado.resumen.no_encontrados }}</p>
          </div>
          <div class="rounded-xl border border-purple-100 bg-purple-50 px-4 py-3 shadow-theme-xs dark:border-purple-800/30 dark:bg-purple-900/10">
            <p class="text-xs font-medium text-purple-600 dark:text-purple-400 uppercase tracking-wider">Solo en MEF</p>
            <p class="mt-1 text-title-lg font-bold leading-tight text-purple-700 dark:text-purple-400">{{ resultado.resumen.no_en_sistema }}</p>
          </div>
        </div>

        
        <div class="mb-4 flex flex-wrap items-center gap-2">
          
          <div class="flex gap-1.5">
            <button
              v-for="f in filtrosEstado"
              :key="f.valor"
              @click="filtroEstado = f.valor"
              :class="filtroEstado === f.valor ? f.claseActiva : 'border-gray-200 bg-card text-gray-600 dark:border-white/10 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-white/5'"
              class="inline-flex items-center gap-1.5 rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors">
              <component :is="f.icono" class="h-3.5 w-3.5" />
              {{ f.etiqueta }}
              <span class="ml-0.5 rounded-full bg-black/10 dark:bg-white/10 px-1.5 py-0.5 text-xs font-semibold">{{ conteosPorEstado[f.valor as keyof typeof conteosPorEstado] }}</span>
            </button>
          </div>

          
          <div class="h-5 w-px bg-gray-200 dark:bg-white/10 mx-1"></div>

          
          <div class="flex gap-1.5">
            <button
              v-for="r in filtrosRegimen"
              :key="r.valor"
              @click="filtroRegimen = r.valor"
              :class="filtroRegimen === r.valor ? r.claseActiva : 'border-gray-200 bg-card text-gray-600 dark:border-white/10 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-white/5'"
              class="inline-flex items-center gap-1 rounded-lg border px-2.5 py-1.5 text-xs font-medium transition-colors">
              {{ r.etiqueta }}
              <span class="ml-0.5 rounded-full bg-black/10 dark:bg-white/10 px-1.5 py-0.5 text-xs font-semibold">{{ conteosPorRegimen[r.valor] ?? 0 }}</span>
            </button>
          </div>

          <input
            v-model="busqueda"
            type="text"
            placeholder="Buscar por DNI, nombre o campo…"
            class="ml-auto w-60 rounded-lg border border-gray-200 bg-card px-3 py-1.5 text-sm text-gray-700 placeholder-gray-400 focus:border-primary focus:outline-none dark:border-white/10 dark:bg-white/5 dark:text-gray-300 dark:placeholder-gray-500" />

          <button
            @click="exportarExcel"
            :disabled="exportando"
            class="inline-flex items-center gap-2 rounded-lg border border-green-200 bg-green-50 px-3 py-1.5 text-sm font-medium text-green-700 hover:bg-green-100 dark:border-green-800/40 dark:bg-green-900/20 dark:text-green-400 dark:hover:bg-green-900/30 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
            <Download v-if="!exportando" class="h-4 w-4" />
            <div v-else class="h-4 w-4 animate-spin rounded-full border-2 border-green-600 border-t-transparent"></div>
            Excel
          </button>

          <button
            @click="reiniciar"
            class="inline-flex items-center gap-2 rounded-lg border border-gray-200 bg-card px-3 py-1.5 text-sm font-medium text-gray-600 hover:bg-gray-50 dark:border-white/10 dark:bg-white/5 dark:text-gray-300 dark:hover:bg-white/10 transition-colors">
            <RefreshCw class="h-4 w-4" />
            Nueva
          </button>
        </div>

        
        <div class="overflow-x-auto rounded-xl border border-gray-100 bg-card shadow-theme-xs dark:border-white/6">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-100 dark:border-white/6">
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider w-12">#</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider w-24">DNI</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider min-w-[160px]">Nombre</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider w-20">Régimen</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">Campo</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider min-w-[130px]">Sistema</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider min-w-[130px]">MEF</th>
                <th class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider w-32">Estado</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-50 dark:divide-white/4">
              <tr
                v-for="(fila, i) in filasPaginadas"
                :key="i"
                class="hover:bg-gray-50/60 dark:hover:bg-white/3 transition-colors">
                <td class="px-4 py-2.5 text-xs text-gray-400 tabular-nums">{{ fila.num }}</td>
                <td class="px-4 py-2.5 font-mono text-xs text-gray-600 dark:text-gray-300">{{ fila.dni }}</td>
                <td class="px-4 py-2.5 text-xs text-gray-700 dark:text-gray-200">{{ fila.nombre }}</td>
                <td class="px-4 py-2.5">
                  <span
                    :class="badgeRegimen(fila.regimen)"
                    class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold">
                    {{ fila.regimen || '—' }}
                  </span>
                </td>
                <td class="px-4 py-2.5 text-xs font-medium text-gray-500 dark:text-gray-400">{{ fila.campo }}</td>
                <td class="px-4 py-2.5 text-xs text-gray-700 dark:text-gray-200">
                  <span :class="fila.resultado === 'DIFERENCIA' ? 'text-orange-600 dark:text-orange-400 font-medium' : ''">
                    {{ fila.valor_propio || '—' }}
                  </span>
                </td>
                <td class="px-4 py-2.5 text-xs text-gray-700 dark:text-gray-200">
                  <span :class="fila.resultado === 'DIFERENCIA' ? 'text-orange-600 dark:text-orange-400 font-medium' : ''">
                    {{ fila.valor_mef || '—' }}
                  </span>
                </td>
                <td class="px-4 py-2.5">
                  <span
                    :class="{
                      'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400': fila.resultado === 'OK',
                      'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400': fila.resultado === 'DIFERENCIA',
                      'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400': fila.resultado === 'NO_EXISTE_EN_MEF',
                      'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400': fila.resultado === 'NO_EXISTE_EN_SISTEMA',
                    }"
                    class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium">
                    <CheckCircle2 v-if="fila.resultado === 'OK'" class="h-3 w-3" />
                    <AlertTriangle v-else-if="fila.resultado === 'DIFERENCIA'" class="h-3 w-3" />
                    <XCircle v-else class="h-3 w-3" />
                    {{ etiquetaResultado(fila.resultado) }}
                  </span>
                </td>
              </tr>
              <tr v-if="filasFiltradas.length === 0">
                <td colspan="8" class="px-4 py-12 text-center text-sm text-gray-400 dark:text-gray-500">
                  No hay registros para los filtros seleccionados
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        
        <div v-if="totalPaginas > 1" class="mt-4 flex items-center justify-between text-sm text-gray-500 dark:text-gray-400">
          <span>Mostrando {{ inicioRango }}–{{ finRango }} de {{ filasFiltradas.length }}</span>
          <div class="flex items-center gap-1">
            <button
              @click="pagina--"
              :disabled="pagina === 1"
              class="rounded-lg border border-gray-200 bg-card px-3 py-1.5 text-xs disabled:opacity-40 hover:bg-gray-50 dark:border-white/10 dark:bg-white/5 dark:hover:bg-white/10 transition-colors">
              Anterior
            </button>
            <span class="px-2">{{ pagina }} / {{ totalPaginas }}</span>
            <button
              @click="pagina++"
              :disabled="pagina === totalPaginas"
              class="rounded-lg border border-gray-200 bg-card px-3 py-1.5 text-xs disabled:opacity-40 hover:bg-gray-50 dark:border-white/10 dark:bg-white/5 dark:hover:bg-white/10 transition-colors">
              Siguiente
            </button>
          </div>
        </div>
      </template>
    </div>
  </main>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import {
    FileSpreadsheet, AlertCircle, X, RefreshCw, Search,
    CheckCircle2, AlertTriangle, XCircle, Download,
  } from 'lucide-vue-next'
  import api from '../services/api'

  interface Comparacion {
    num: number
    dni: string
    nombre: string
    regimen: string
    regimen_mef: string | null
    codigo_registro: string | null
    codigo_puesto_cpe: string
    campo: string
    valor_propio: string
    valor_mef: string
    resultado: 'OK' | 'DIFERENCIA' | 'NO_EXISTE_EN_MEF' | 'NO_EXISTE_EN_SISTEMA'
  }

  interface Resumen {
    procesados: number
    encontrados_mef: number
    ok: number
    diferencias: number
    no_encontrados: number
    no_en_sistema: number
    fecha_comparacion: string
  }

  interface Resultado {
    resumen: Resumen
    comparaciones: Comparacion[]
  }

  const inputCas   = ref<HTMLInputElement | null>(null)
  const inputOtros = ref<HTMLInputElement | null>(null)
  const archivoCas   = ref<File | null>(null)
  const archivoOtros = ref<File | null>(null)
  const cargando = ref(false)
  const exportando = ref(false)
  const error = ref<string | null>(null)
  const resultado = ref<Resultado | null>(null)
  const filtroEstado = ref<string>('DIFERENCIA')
  const filtroRegimen = ref<string>('TODOS')
  const busqueda = ref('')
  const pagina = ref(1)
  const POR_PAGINA = 50

  const filtrosEstado = [
    {
      valor: 'TODOS',
      etiqueta: 'Todos',
      icono: FileSpreadsheet,
      claseActiva: 'border-gray-400 bg-gray-100 text-gray-700 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200',
    },
    {
      valor: 'DIFERENCIA',
      etiqueta: 'Diferencias',
      icono: AlertTriangle,
      claseActiva: 'border-orange-300 bg-orange-50 text-orange-700 dark:border-orange-700 dark:bg-orange-900/20 dark:text-orange-400',
    },
    {
      valor: 'NO_EXISTE_EN_MEF',
      etiqueta: 'Solo en sistema',
      icono: XCircle,
      claseActiva: 'border-red-300 bg-red-50 text-red-700 dark:border-red-700 dark:bg-red-900/20 dark:text-red-400',
    },
    {
      valor: 'NO_EXISTE_EN_SISTEMA',
      etiqueta: 'Solo en MEF',
      icono: XCircle,
      claseActiva: 'border-purple-300 bg-purple-50 text-purple-700 dark:border-purple-700 dark:bg-purple-900/20 dark:text-purple-400',
    },
    {
      valor: 'OK',
      etiqueta: 'Correctos',
      icono: CheckCircle2,
      claseActiva: 'border-green-300 bg-green-50 text-green-700 dark:border-green-700 dark:bg-green-900/20 dark:text-green-400',
    },
  ]

  const filtrosRegimen = computed(() => {
    const lista = resultado.value?.comparaciones ?? []
    const regimenes = new Set(lista.map(c => c.regimen).filter(Boolean))
    const opciones = [{ valor: 'TODOS', etiqueta: 'Todos los regímenes', claseActiva: 'border-gray-400 bg-gray-100 text-gray-700 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200' }]
    const colores: Record<string, string> = {
      CAS:     'border-blue-300 bg-blue-50 text-blue-700 dark:border-blue-700 dark:bg-blue-900/20 dark:text-blue-400',
      '276':   'border-violet-300 bg-violet-50 text-violet-700 dark:border-violet-700 dark:bg-violet-900/20 dark:text-violet-400',
      '728':   'border-teal-300 bg-teal-50 text-teal-700 dark:border-teal-700 dark:bg-teal-900/20 dark:text-teal-400',
    }
    for (const r of ['CAS', '276', '728', ...regimenes]) {
      if (r !== 'TODOS' && regimenes.has(r)) {
        opciones.push({
          valor: r,
          etiqueta: r,
          claseActiva: colores[r] ?? 'border-gray-400 bg-gray-100 text-gray-700 dark:border-gray-500 dark:bg-gray-800 dark:text-gray-200',
        })
      }
    }
    return opciones
  })

  const conteosPorEstado = computed(() => {
    const lista = resultado.value?.comparaciones ?? []
    return {
      TODOS: lista.length,
      OK: lista.filter(c => c.resultado === 'OK').length,
      DIFERENCIA: lista.filter(c => c.resultado === 'DIFERENCIA').length,
      NO_EXISTE_EN_MEF: lista.filter(c => c.resultado === 'NO_EXISTE_EN_MEF').length,
      NO_EXISTE_EN_SISTEMA: lista.filter(c => c.resultado === 'NO_EXISTE_EN_SISTEMA').length,
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
      lista = lista.filter(c => c.resultado === filtroEstado.value)
    }
    if (filtroRegimen.value !== 'TODOS') {
      lista = lista.filter(c => c.regimen === filtroRegimen.value)
    }
    if (busqueda.value.trim()) {
      const q = busqueda.value.trim().toLowerCase()
      lista = lista.filter(
        c =>
          c.dni.toLowerCase().includes(q) ||
          c.nombre.toLowerCase().includes(q) ||
          c.campo.toLowerCase().includes(q),
      )
    }
    return lista
  })

  const totalPaginas = computed(() => Math.max(1, Math.ceil(filasFiltradas.value.length / POR_PAGINA)))
  const inicioRango  = computed(() => (pagina.value - 1) * POR_PAGINA + 1)
  const finRango     = computed(() => Math.min(pagina.value * POR_PAGINA, filasFiltradas.value.length))
  const filasPaginadas = computed(() => filasFiltradas.value.slice((pagina.value - 1) * POR_PAGINA, pagina.value * POR_PAGINA))

  watch([filtroEstado, filtroRegimen, busqueda], () => { pagina.value = 1 })

  function onArchivoSeleccionado(e: Event, tipo: 'cas' | 'otros') {
    const archivo = (e.target as HTMLInputElement).files?.[0]
    if (!archivo) return
    if (tipo === 'cas') archivoCas.value = archivo
    else archivoOtros.value = archivo
  }

  function onDrop(e: DragEvent, tipo: 'cas' | 'otros') {
    const archivo = e.dataTransfer?.files?.[0]
    if (!archivo) return
    if (tipo === 'cas') archivoCas.value = archivo
    else archivoOtros.value = archivo
  }

  async function procesarArchivos() {
    if (!archivoCas.value && !archivoOtros.value) return
    error.value = null
    cargando.value = true
    try {
      const form = new FormData()
      if (archivoCas.value)   form.append('archivo_cas',   archivoCas.value)
      if (archivoOtros.value) form.append('archivo_otros', archivoOtros.value)
      const { data } = await api.post('/dash/comparar_mef', form, {
        headers: { 'Content-Type': 'multipart/form-data' },
      })
      resultado.value = data
      filtroEstado.value = 'DIFERENCIA'
      filtroRegimen.value = 'TODOS'
    } catch (e: any) {
      error.value = e?.response?.data?.message ?? e?.message ?? 'Error al procesar los archivos'
    } finally {
      cargando.value = false
    }
  }

  async function exportarExcel() {
    if (!resultado.value) return
    exportando.value = true
    try {
      const response = await api.post(
        '/dash/exportar_comparacion_mef',
        { comparaciones: resultado.value.comparaciones },
        { responseType: 'blob' },
      )
      const url = URL.createObjectURL(new Blob([response.data]))
      const a = document.createElement('a')
      a.href = url
      const fecha = resultado.value.resumen.fecha_comparacion.replace(/[/:]/g, '-').replace(/ /g, '_')
      a.download = `comparacion_mef_${fecha}.xlsx`
      a.click()
      URL.revokeObjectURL(url)
    } catch (e: any) {
      error.value = e?.response?.data?.message ?? e?.message ?? 'Error al exportar'
    } finally {
      exportando.value = false
    }
  }

  function reiniciar() {
    resultado.value = null
    error.value = null
    busqueda.value = ''
    pagina.value = 1
    archivoCas.value = null
    archivoOtros.value = null
    if (inputCas.value)   inputCas.value.value = ''
    if (inputOtros.value) inputOtros.value.value = ''
  }

  function etiquetaResultado(r: string) {
    if (r === 'OK') return 'OK'
    if (r === 'DIFERENCIA') return 'Diferencia'
    if (r === 'NO_EXISTE_EN_MEF') return 'Solo en sistema'
    if (r === 'NO_EXISTE_EN_SISTEMA') return 'Solo en MEF'
    return r
  }

  function badgeRegimen(r: string): string {
    if (r === 'CAS')   return 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
    if (r === '276')   return 'bg-violet-100 text-violet-700 dark:bg-violet-900/30 dark:text-violet-400'
    if (r === '728')   return 'bg-teal-100 text-teal-700 dark:bg-teal-900/30 dark:text-teal-400'
    return 'bg-gray-100 text-gray-600 dark:bg-white/10 dark:text-gray-400'
  }
</script>
