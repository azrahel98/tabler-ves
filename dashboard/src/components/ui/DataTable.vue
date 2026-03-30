<template>
  <div class="rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-gray-900 overflow-hidden">
    <!-- Cabecera con título y buscador -->
    <div class="flex flex-wrap items-center justify-between gap-3 border-b border-gray-200 dark:border-gray-800 px-5 py-4">
      <div>
        <h3 v-if="titulo" class="text-base font-semibold text-gray-800 dark:text-white/90">{{ titulo }}</h3>
        <p v-if="subtitulo" class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">{{ subtitulo }}</p>
      </div>
      <div class="flex items-center gap-2">
        <!-- Buscador -->
        <div class="relative">
          <Search class="absolute left-3 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400" />
          <input
            v-model="busqueda"
            type="text"
            :placeholder="placeholderBusqueda"
            class="w-56 rounded-lg border border-gray-200 bg-gray-50 py-2 pl-8 pr-3 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:outline-none focus:ring-2 focus:ring-brand-500/20 dark:border-gray-700 dark:bg-gray-800 dark:text-white dark:placeholder:text-gray-500" />
        </div>
        <!-- Selector de filas por página -->
        <select
          v-model="filasPorPagina"
          class="rounded-lg border border-gray-200 bg-gray-50 py-2 px-2 text-sm text-gray-700 focus:outline-none dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300">
          <option v-for="n in [10, 25, 50]" :key="n" :value="n">{{ n }}</option>
        </select>
      </div>
    </div>

    <!-- Tabla -->
    <div class="overflow-x-auto">
      <table class="w-full text-sm">
        <!-- Encabezados -->
        <thead>
          <tr class="border-b border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-800/50">
            <th
              v-for="col in columnas"
              :key="col.clave"
              @click="ordenarPor(col.clave)"
              class="px-4 py-3 text-left text-[10px] font-bold uppercase tracking-wider text-gray-400 select-none"
              :class="[col.ancho ?? '', col.ordenable !== false ? 'cursor-pointer hover:text-gray-600 dark:hover:text-gray-300' : '']">
              <div class="flex items-center gap-1">
                {{ col.titulo }}
                <template v-if="col.ordenable !== false">
                  <ArrowUpDown v-if="columnaOrden !== col.clave" class="h-3 w-3 opacity-30" />
                  <ArrowUp v-else-if="direccionOrden === 'asc'" class="h-3 w-3 text-brand-500" />
                  <ArrowDown v-else class="h-3 w-3 text-brand-500" />
                </template>
              </div>
            </th>
          </tr>
        </thead>

        <!-- Cuerpo -->
        <tbody v-if="filasPagina.length > 0">
          <tr
            v-for="(fila, idx) in filasPagina"
            :key="idx"
            @click="$emit('click-fila', fila)"
            class="border-b border-gray-100 text-xs dark:border-gray-800 last:border-none transition-colors"
            :class="clickable ? 'cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-800/50' : ''">
            <td v-for="col in columnas" :key="col.clave" class="px-4 py-3 text-gray-700 dark:text-gray-300">
              <!-- Slot personalizado para la celda -->
              <slot :name="`celda-${col.clave}`" :fila="fila" :valor="fila[col.clave]">
                {{ fila[col.clave] ?? '—' }}
              </slot>
            </td>
          </tr>
        </tbody>

        <!-- Sin resultados -->
        <tbody v-else>
          <tr>
            <td :colspan="columnas.length" class="px-4 py-14 text-center">
              <div class="flex flex-col items-center gap-2 text-gray-400 dark:text-gray-500">
                <SearchX class="h-8 w-8" />
                <span class="text-sm font-medium">
                  {{ busqueda ? `Sin resultados para "${busqueda}"` : 'Sin datos' }}
                </span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pie: info de paginación y navegación -->
    <div v-if="totalFilasFiltradas > 0" class="flex flex-wrap items-center justify-between gap-3 border-t border-gray-200 dark:border-gray-800 px-5 py-3">
      <p class="text-xs text-gray-400 dark:text-gray-500">
        Mostrando
        <span class="font-semibold text-gray-600 dark:text-gray-400">{{ inicioPagina }}–{{ finPagina }}</span>
        de
        <span class="font-semibold text-gray-600 dark:text-gray-400">{{ totalFilasFiltradas }}</span>
        resultado{{ totalFilasFiltradas === 1 ? '' : 's' }}
      </p>

      <div class="flex items-center gap-1">
        <button
          @click="paginaActual = 1"
          :disabled="paginaActual === 1"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors"
          title="Primera página">
          <ChevronsLeft class="h-4 w-4" />
        </button>
        <button
          @click="paginaActual--"
          :disabled="paginaActual === 1"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors"
          title="Página anterior">
          <ChevronLeft class="h-4 w-4" />
        </button>

        <div class="flex items-center gap-1">
          <button
            v-for="n in paginasVisibles"
            :key="n"
            @click="typeof n === 'number' && (paginaActual = n)"
            class="min-w-[30px] rounded-lg px-2 py-1 text-xs font-medium transition-colors"
            :class="
              n === paginaActual
                ? 'bg-brand-500 text-white'
                : typeof n === 'number'
                  ? 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 dark:text-gray-400'
                  : 'text-gray-300 cursor-default dark:text-gray-600'
            ">
            {{ n }}
          </button>
        </div>

        <button
          @click="paginaActual++"
          :disabled="paginaActual === totalPaginas"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors"
          title="Página siguiente">
          <ChevronRight class="h-4 w-4" />
        </button>
        <button
          @click="paginaActual = totalPaginas"
          :disabled="paginaActual === totalPaginas"
          class="rounded-lg p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-700 disabled:opacity-30 disabled:cursor-not-allowed dark:hover:bg-gray-800 dark:hover:text-gray-300 transition-colors"
          title="Última página">
          <ChevronsRight class="h-4 w-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import { Search, SearchX, ArrowUp, ArrowDown, ArrowUpDown, ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-vue-next'

  // ─── Tipos ────────────────────────────────────────────────
  interface Columna {
    clave: string
    titulo: string
    ancho?: string
    ordenable?: boolean
    busqueda?: boolean // si false, se excluye de la búsqueda global
  }

  // ─── Props ────────────────────────────────────────────────
  const props = withDefaults(
    defineProps<{
      columnas: Columna[]
      filas: any[]
      titulo?: string
      subtitulo?: string
      placeholderBusqueda?: string
      clickable?: boolean
    }>(),
    {
      titulo: '',
      subtitulo: '',
      placeholderBusqueda: 'Buscar...',
      clickable: true,
    }
  )

  defineEmits<{
    (e: 'click-fila', fila: any): void
  }>()

  // ─── Estado ───────────────────────────────────────────────
  const busqueda = ref('')
  const paginaActual = ref(1)
  const filasPorPagina = ref(15)
  const columnaOrden = ref<string | null>(null)
  const direccionOrden = ref<'asc' | 'desc'>('asc')

  // Reiniciar página al cambiar búsqueda o tamaño
  watch([busqueda, filasPorPagina], () => (paginaActual.value = 1))

  // ─── Lógica ───────────────────────────────────────────────
  function ordenarPor(clave: string) {
    const col = props.columnas.find((c) => c.clave === clave)
    if (col?.ordenable === false) return
    if (columnaOrden.value === clave) {
      direccionOrden.value = direccionOrden.value === 'asc' ? 'desc' : 'asc'
    } else {
      columnaOrden.value = clave
      direccionOrden.value = 'asc'
    }
  }

  const columnasDeBusqueda = computed(() => props.columnas.filter((c) => c.busqueda !== false).map((c) => c.clave))

  const filasFiltradas = computed(() => {
    const termino = busqueda.value.trim().toLowerCase()
    let resultado = props.filas

    if (termino) {
      resultado = resultado.filter((fila) =>
        columnasDeBusqueda.value.some((clave) =>
          String(fila[clave] ?? '')
            .toLowerCase()
            .includes(termino)
        )
      )
    }

    if (columnaOrden.value) {
      const clave = columnaOrden.value
      resultado = [...resultado].sort((a, b) => {
        const va = String(a[clave] ?? '').toLowerCase()
        const vb = String(b[clave] ?? '').toLowerCase()
        const comparacion = va.localeCompare(vb, 'es', { numeric: true })
        return direccionOrden.value === 'asc' ? comparacion : -comparacion
      })
    }

    return resultado
  })

  const totalFilasFiltradas = computed(() => filasFiltradas.value.length)
  const totalPaginas = computed(() => Math.max(1, Math.ceil(totalFilasFiltradas.value / filasPorPagina.value)))
  const inicioPagina = computed(() => Math.min((paginaActual.value - 1) * filasPorPagina.value + 1, totalFilasFiltradas.value))
  const finPagina = computed(() => Math.min(paginaActual.value * filasPorPagina.value, totalFilasFiltradas.value))

  const filasPagina = computed(() => {
    const inicio = (paginaActual.value - 1) * filasPorPagina.value
    return filasFiltradas.value.slice(inicio, inicio + filasPorPagina.value)
  })

  // Páginas visibles en el paginador (con puntos suspensivos)
  const paginasVisibles = computed<(number | '…')[]>(() => {
    const total = totalPaginas.value
    const actual = paginaActual.value
    if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1)

    const paginas: (number | '…')[] = [1]
    if (actual > 3) paginas.push('…')
    for (let i = Math.max(2, actual - 1); i <= Math.min(total - 1, actual + 1); i++) {
      paginas.push(i)
    }
    if (actual < total - 2) paginas.push('…')
    paginas.push(total)
    return paginas
  })
</script>
