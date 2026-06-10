<script setup lang="ts">
  import { ref } from 'vue'
  import { Search, Download, SearchX, UserRoundSearch } from 'lucide-vue-next'
  import { watchDebounced } from '@vueuse/core'
  import CardSearch from '../../components/search/card.vue'
  import Loading from '../../components/ui/Loading.vue'
  import api from '../../services/api'

  const searchQuery = ref('')
  const loading = ref(false)
  const hasSearched = ref(false)
  const resultadosBusqueda = ref<any[]>([])

  let abortController: AbortController | null = null

  async function handleSearch(query: string) {
    const normalized = query.trim().toLowerCase().replace(/\s+/g, ' ')

    if (!normalized) {
      resultadosBusqueda.value = []
      hasSearched.value = false
      return
    }

    abortController?.abort()
    abortController = new AbortController()

    loading.value = true
    hasSearched.value = false

    try {
      const res = await api.post('/personal/buscar', { nombre: normalized }, { signal: abortController.signal })
      resultadosBusqueda.value = res.data || []
      hasSearched.value = true
    } catch (e: any) {
      if (e.name === 'AbortError' || e.code === 'ERR_CANCELED') return
      console.error('Error en búsqueda:', e)
      resultadosBusqueda.value = []
      hasSearched.value = true
    } finally {
      loading.value = false
    }
  }

  const exportando = ref(false)

  async function exportarExcel() {
    exportando.value = true
    try {
      const respuesta = await api.post('/dash/exportar_excel', {}, { responseType: 'blob' })
      const url = window.URL.createObjectURL(new Blob([respuesta.data]))
      const enlace = document.createElement('a')
      enlace.href = url
      enlace.setAttribute('download', 'reporte.xlsx')
      document.body.appendChild(enlace)
      enlace.click()
      document.body.removeChild(enlace)
      window.URL.revokeObjectURL(url)
    } catch (error) {
      console.error('Error al exportar:', error)
    } finally {
      exportando.value = false
    }
  }

  watchDebounced(searchQuery, (val) => handleSearch(val), { debounce: 350 })
</script>

<template>
  <div>
    <div class="flex flex-col justify-between gap-4 border-b border-gray-100 pb-5 sm:flex-row sm:items-center dark:border-white/6 mb-6">
      <div>
        <h1 class="text-title-xl font-bold leading-tight text-gray-900 dark:text-white tracking-tight">Personal</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Busque al personal de la municipalidad por su nombre, apellido o DNI</p>
      </div>
      <div class="flex flex-wrap items-center gap-3">
        <div class="relative">
          <span class="absolute top-1/2 left-4 -translate-y-1/2 text-gray-400 dark:text-gray-500">
            <Search v-if="!loading" class="h-4 w-4" />
            <Loading v-else size="xs" />
          </span>
          <input
            type="text"
            v-model="searchQuery"
            placeholder="Buscar por nombre o DNI..."
            autocomplete="off"
            class="w-full h-11 pl-11 pr-4 text-sm text-gray-900 bg-card border-[1.5px] border-gray-100 rounded-xl shadow-theme-xs outline-none transition-all duration-200 placeholder:text-gray-400 focus:border-primary/40 focus:ring-4 focus:ring-primary/10 dark:bg-white/3 dark:border-white/6 dark:text-white/90 dark:placeholder:text-white/25 dark:focus:border-primary/60 dark:focus:ring-primary/15 sm:w-[280px]" />
        </div>
        <button
          @click="exportarExcel"
          :disabled="exportando"
          class="shadow-theme-xs inline-flex items-center justify-center gap-2 rounded-xl bg-card px-4 py-2.5 text-sm font-semibold text-gray-700 ring-1 ring-gray-100 transition hover:bg-primary/5 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-white/5 dark:text-gray-300 dark:ring-white/10 dark:hover:bg-white/10">
          <Download class="h-4 w-4" :class="{ 'animate-bounce': exportando }" />
          {{ exportando ? 'Exportando...' : 'Exportar Excel' }}
        </button>
      </div>
    </div>

    <div class="flex flex-col gap-8 pt-2">
      <template v-if="loading">
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6">
          <div v-for="i in 12" :key="i" class="rounded-xl border border-gray-100 dark:border-white/6 bg-card dark:bg-white/3 p-4 animate-pulse">
            <div class="mx-auto mb-3 h-14 w-14 rounded-full bg-gray-100 dark:bg-white/10"></div>
            <div class="h-3 w-3/4 mx-auto rounded bg-gray-100 dark:bg-white/10 mb-2"></div>
            <div class="h-2.5 w-1/2 mx-auto rounded bg-surface dark:bg-white/5"></div>
          </div>
        </div>
      </template>

      <div v-else-if="!hasSearched" class="flex flex-col items-center justify-center py-24 text-center rounded-2xl border border-dashed border-gray-200 bg-transparent dark:border-gray-800">
        <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
          <UserRoundSearch class="h-6 w-6" />
        </div>
        <h3 class="mb-1 text-base font-semibold text-gray-800 dark:text-white">Buscar Personal</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 max-w-sm px-4">
          Ingrese el nombre o número de DNI del trabajador en el campo de búsqueda de arriba para comenzar a buscar.
        </p>
      </div>

      <div v-else-if="hasSearched && resultadosBusqueda.length === 0" class="rounded-2xl border border-gray-100 bg-transparent dark:border-white/6 p-16 text-center">
        <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-red-50 text-red-500 dark:bg-red-500/10">
          <SearchX class="h-6 w-6" />
        </div>
        <h3 class="mb-1 text-base font-semibold text-gray-800 dark:text-white">No se encontraron resultados</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 max-w-sm mx-auto px-4">
          No hay trabajadores que coincidan con "<span class="font-semibold text-gray-700 dark:text-gray-300">{{ searchQuery }}</span>". Intente con otros términos de búsqueda.
        </p>
      </div>

      <template v-else-if="resultadosBusqueda.length > 0">
        <div class="flex items-center justify-between">
          <h3 class="text-title-sm font-semibold text-gray-800 dark:text-white/90">Resultados de la búsqueda</h3>
          <span class="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-bold uppercase tracking-wider bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300">
            {{ resultadosBusqueda.length }} {{ resultadosBusqueda.length === 1 ? 'trabajador' : 'trabajadores' }}
          </span>
        </div>
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6">
          <CardSearch v-for="person in resultadosBusqueda" :key="person.dni" :person="person" />
        </div>
      </template>
    </div>
  </div>
</template>
