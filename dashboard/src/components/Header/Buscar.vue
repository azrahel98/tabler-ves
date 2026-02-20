<template>
  <div class="hidden lg:block">
    <form @submit.prevent="handleSearch">
      <div class="relative" ref="searchContainerRef">
        <span class="absolute top-1/2 left-4 -translate-y-1/2">
          <svg class="fill-gray-500 dark:fill-gray-400" width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M3.04175 9.37363C3.04175 5.87693 5.87711 3.04199 9.37508 3.04199C12.8731 3.04199 15.7084 5.87693 15.7084 9.37363C15.7084 12.8703 12.8731 15.7053 9.37508 15.7053C5.87711 15.7053 3.04175 12.8703 3.04175 9.37363ZM9.37508 1.54199C5.04902 1.54199 1.54175 5.04817 1.54175 9.37363C1.54175 13.6991 5.04902 17.2053 9.37508 17.2053C11.2674 17.2053 13.003 16.5344 14.357 15.4176L17.177 18.238C17.4699 18.5309 17.9448 18.5309 18.2377 18.238C18.5306 17.9451 18.5306 17.4703 18.2377 17.1774L15.418 14.3573C16.5365 13.0033 17.2084 11.2669 17.2084 9.37363C17.2084 5.04817 13.7011 1.54199 9.37508 1.54199Z"
              fill="" />
          </svg>
        </span>
        <input
          v-model="searchQuery"
          @input="onSearchInput"
          type="text"
          placeholder="Buscar personal..."
          id="search-input"
          autocomplete="off"
          class="dark:bg-dark-900 shadow-theme-xs focus:border-brand-300 focus:ring-brand-500/10 dark:focus:border-brand-800 h-11 w-full rounded-lg border border-gray-200 bg-transparent py-2.5 pr-14 pl-12 text-sm text-gray-800 placeholder:text-gray-400 focus:ring-3 focus:outline-hidden xl:w-[430px] dark:border-gray-800 dark:bg-gray-900 dark:bg-white/[0.03] dark:text-white/90 dark:placeholder:text-white/30" />

        <div
          v-show="showDropdown && (searchResults.length > 0 || searchQuery)"
          class="absolute left-0 top-full mt-2 w-full rounded-xl border border-gray-200 bg-white py-2 shadow-theme-lg dark:border-gray-800 dark:bg-gray-900 z-50 max-h-[400px] overflow-y-auto">
          <div v-if="searchResults.length > 0">
            <div v-for="person in searchResults" :key="person.dni" class="cursor-pointer px-4 py-2 hover:bg-gray-100 dark:hover:bg-white/5 transition-colors">
              <RouterLink
                :to="{
                  name: 'personal-profile',
                  params: { dni: person.dni },
                }"
                @click="showDropdown = false">
                <p class="text-sm font-medium text-gray-800 dark:text-white">
                  {{ person.nombre }}
                </p>
                <div class="flex items-center justify-between mt-1">
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    {{ person.dni }}
                  </p>
                </div>
              </RouterLink>
            </div>
          </div>
          <div v-else class="px-4 py-3 text-sm text-gray-500 dark:text-gray-400 text-center">No se encontraron resultados</div>
        </div>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue'
  import { usePersonalStore } from '../../stores/personal'

  const personalStore = usePersonalStore()

  const searchContainerRef = ref<HTMLElement | null>(null)
  const searchQuery = ref('')
  const searchResults = ref<any[]>([])
  const showDropdown = ref(false)
  let debounceTimer: any = null

  const onSearchInput = () => {
    if (debounceTimer) clearTimeout(debounceTimer)
    debounceTimer = setTimeout(() => {
      handleSearch()
    }, 300)
  }

  const handleSearch = async () => {
    if (!searchQuery.value.trim()) {
      searchResults.value = []
      showDropdown.value = false
      return
    }
    showDropdown.value = true
    try {
      const results = await personalStore.buscarGlobal(searchQuery.value)
      searchResults.value = results || []
    } catch (e) {
      console.error(e)
      searchResults.value = []
    }
  }

  const handleClickOutside = (event: MouseEvent) => {
    if (searchContainerRef.value && !searchContainerRef.value.contains(event.target as Node)) {
      showDropdown.value = false
    }
  }

  onMounted(() => {
    document.addEventListener('click', handleClickOutside)
  })

  onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
  })
</script>
