<script setup lang="ts">
  import { ref } from 'vue'
  import { Search, SlidersHorizontal } from 'lucide-vue-next'
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

  watchDebounced(searchQuery, (val) => handleSearch(val), { debounce: 350 })
</script>

<template>
  <div>
    <div class="flex flex-wrap items-center justify-between gap-3 mb-6">
      <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Personal</h3>
    </div>

    <div class="overflow-hidden rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
      <div class="flex flex-col justify-between gap-5 border-b border-gray-200 px-5 py-4 sm:flex-row sm:items-center dark:border-gray-800">
        <div>
          <h3 class="text-lg font-semibold text-gray-800 dark:text-white/90">Lista de Personal</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400">Busque al personal por su nombre</p>
        </div>
        <div class="flex gap-3">
          <button
            class="shadow-theme-xs inline-flex items-center justify-center gap-2 rounded-lg bg-white px-4 py-3 text-sm font-medium text-gray-700 ring-1 ring-gray-300 transition hover:bg-gray-50 dark:bg-gray-800 dark:text-gray-400 dark:ring-gray-700 dark:hover:bg-white/[0.03]">
            Export
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none">
              <path
                d="M16.667 13.3333V15.4166C16.667 16.1069 16.1074 16.6666 15.417 16.6666H4.58295C3.89259 16.6666 3.33295 16.1069 3.33295 15.4166V13.3333M10.0013 13.3333L10.0013 3.33325M6.14547 9.47942L9.99951 13.331L13.8538 9.47942"
                stroke="currentColor"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round" />
            </svg>
          </button>
          <a
            href="/add-product"
            class="bg-brand-500 shadow-theme-xs hover:bg-brand-600 inline-flex items-center justify-center gap-2 rounded-lg px-4 py-3 text-sm font-medium text-white transition">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none">
              <path d="M5 10.0002H15.0006M10.0002 5V15.0006" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            Add Product
          </a>
        </div>
      </div>
      <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
        <div class="flex gap-3 sm:justify-between">
          <div class="relative flex-1 sm:flex-auto">
            <span class="absolute top-1/2 left-4 -translate-y-1/2 text-gray-500 dark:text-gray-400">
              <Search v-if="!loading" />
              <Loading v-else size="xs" />
            </span>
            <input
              type="text"
              v-model="searchQuery"
              placeholder="Buscar por nombre o DNI..."
              autocomplete="off"
              class="dark:bg-dark-900 shadow-theme-xs focus:border-brand-300 focus:ring-brand-500/10 dark:focus:border-brand-800 h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pr-4 pl-11 text-sm text-gray-800 placeholder:text-gray-400 focus:ring-3 focus:outline-hidden sm:w-[300px] sm:min-w-[300px] dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 dark:placeholder:text-white/30" />
          </div>
          <div class="relative">
            <button
              class="shadow-theme-xs flex h-11 w-full items-center justify-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 sm:w-auto sm:min-w-[100px] dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400"
              type="button">
              <SlidersHorizontal />
              Filter
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="flex flex-col gap-8 pt-2">
      <template v-if="loading">
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6">
          <div v-for="i in 12" :key="i" class="rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 p-4 animate-pulse">
            <div class="mx-auto mb-3 h-14 w-14 rounded-full bg-gray-200 dark:bg-gray-700"></div>
            <div class="h-3 w-3/4 mx-auto rounded bg-gray-200 dark:bg-gray-700 mb-2"></div>
            <div class="h-2.5 w-1/2 mx-auto rounded bg-gray-100 dark:bg-gray-800"></div>
          </div>
        </div>
      </template>

      <div v-else-if="hasSearched && resultadosBusqueda.length === 0" class="rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3 p-12 text-center">
        <div class="mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-gray-100 dark:bg-gray-800">
          <svg class="h-10 w-10 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>
        <h3 class="mb-2 text-xl font-bold text-gray-800 dark:text-white">No se encontraron resultados</h3>
        <p class="text-gray-500 dark:text-gray-400">
          No hay trabajadores que coincidan con "<span class="font-medium text-gray-700 dark:text-gray-300">{{ searchQuery }}</span
          >". Intenta con otros términos.
        </p>
      </div>

      <template v-else-if="resultadosBusqueda.length > 0">
        <div class="flex items-center justify-between">
          <h4 class="text-md font-medium text-gray-800 dark:text-white/90">Resultados de la búsqueda</h4>
          <span class="rounded-full bg-primary/10 px-3 py-1 text-sm font-medium text-primary">
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
