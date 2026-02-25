<script setup lang="ts">
  import { ref, watch, onUnmounted } from 'vue'
  import { usePersonalStore } from '../../stores/personal'
  import Loading from '../ui/Loading.vue'

  const personalStore = usePersonalStore()
  const searchContainerRef = ref<HTMLElement | null>(null)
  const searchQuery = ref('')
  const searchResults = ref<any[]>([])
  const showDropdown = ref(false)
  const isLoading = ref(false)

  let abortController: AbortController | null = null
  let debounceTimer: ReturnType<typeof setTimeout>

  const handleSearch = async (query: string) => {
    if (abortController) abortController.abort()
    abortController = new AbortController()

    if (!query.trim()) {
      searchResults.value = []
      showDropdown.value = false
      return
    }

    isLoading.value = true
    showDropdown.value = true

    try {
      const results = await personalStore.buscarGlobal(query)
      searchResults.value = results || []
    } catch (e: any) {
      if (e.name === 'AbortError') return
      console.error('Error en búsqueda:', e)
      searchResults.value = []
    } finally {
      isLoading.value = false
    }
  }

  watch(searchQuery, (newVal) => {
    clearTimeout(debounceTimer)

    if (!newVal) {
      searchResults.value = []
      showDropdown.value = false
      return
    }

    debounceTimer = setTimeout(() => {
      handleSearch(newVal)
    }, 350)
  })

  const closeDropdown = (event?: MouseEvent | KeyboardEvent) => {
    if (event instanceof KeyboardEvent && event.key === 'Escape') {
      showDropdown.value = false
    } else if (event instanceof MouseEvent) {
      if (searchContainerRef.value && !searchContainerRef.value.contains(event.target as Node)) {
        showDropdown.value = false
      }
    }
  }

  window.addEventListener('click', closeDropdown)
  window.addEventListener('keydown', closeDropdown)

  onUnmounted(() => {
    window.removeEventListener('click', closeDropdown)
    window.removeEventListener('keydown', closeDropdown)
    if (abortController) abortController.abort()
    clearTimeout(debounceTimer)
  })
</script>

<template>
  <div class="hidden lg:block">
    <form @submit.prevent="handleSearch(searchQuery)">
      <div class="relative" ref="searchContainerRef">
        <span class="absolute top-1/2 left-4 -translate-y-1/2">
          <svg v-if="!isLoading" class="fill-gray-500 dark:fill-gray-400" width="20" height="20" viewBox="0 0 20 20">
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M3.04175 9.37363C3.04175 5.87693 5.87711 3.04199 9.37508 3.04199C12.8731 3.04199 15.7084 5.87693 15.7084 9.37363C15.7084 12.8703 12.8731 15.7053 9.37508 15.7053C5.87711 15.7053 3.04175 12.8703 3.04175 9.37363ZM9.37508 1.54199C5.04902 1.54199 1.54175 5.04817 1.54175 9.37363C1.54175 13.6991 5.04902 17.2053 9.37508 17.2053C11.2674 17.2053 13.003 16.5344 14.357 15.4176L17.177 18.238C17.4699 18.5309 17.9448 18.5309 18.2377 18.238C18.5306 17.9451 18.5306 17.4703 18.2377 17.1774L15.418 14.3573C16.5365 13.0033 17.2084 11.2669 17.2084 9.37363C17.2084 5.04817 13.7011 1.54199 9.37508 1.54199Z" />
          </svg>
          <Loading v-else size="xs" />
        </span>

        <input
          v-model="searchQuery"
          type="text"
          placeholder="Buscar personal por nombre o DNI..."
          id="search-input"
          autocomplete="off"
          class="dark:bg-dark-900 shadow-theme-xs focus:border-brand-300 focus:ring-brand-500/10 h-11 w-full rounded-lg border border-gray-200 bg-transparent py-2.5 pr-14 pl-12 text-sm text-gray-800 placeholder:text-gray-400 focus:ring-3 focus:outline-hidden xl:w-[430px] dark:border-gray-800 dark:text-white/90" />

        <div
          v-if="showDropdown && searchResults.length > 0"
          class="absolute left-0 top-full mt-2 w-full rounded-xl border border-gray-200 bg-white py-2 shadow-theme-lg dark:border-gray-800 dark:bg-gray-900 z-50 max-h-[400px] overflow-y-auto">
          <div v-if="searchResults.length > 0">
            <RouterLink
              v-for="person in searchResults"
              :key="person.dni"
              :to="{ name: 'personal-profile', params: { dni: person.dni } }"
              @click="showDropdown = false"
              class="block px-4 py-3 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors border-b last:border-0 border-gray-100 dark:border-gray-800">
              <p class="text-sm font-semibold text-gray-800 dark:text-white uppercase">
                {{ person.nombre }}
              </p>
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">DNI: {{ person.dni }}</p>
            </RouterLink>
          </div>

          <div v-else-if="!isLoading" class="px-4 py-6 text-sm text-gray-500 dark:text-gray-400 text-center">
            <span class="block text-lg mb-1">🔍</span>
            No se encontraron resultados para "{{ searchQuery }}"
          </div>
        </div>
      </div>
    </form>
  </div>
</template>
