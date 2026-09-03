<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  buscarTrabajadores,
  getPersonalAvatarUrl,
  type PersonalSearchResult,
} from '@/services/personal'
import {
  IconSearch,
  IconX,
  IconLoader2,
  IconUser,
  IconChevronRight,
} from '@tabler/icons-vue'

const router = useRouter()

const searchQuery = ref('')
const results = ref<PersonalSearchResult[]>([])
const isLoading = ref(false)
const isOpen = ref(false)
const highlightedIndex = ref(-1)
const hasSearched = ref(false)

const inputRef = ref<HTMLInputElement | null>(null)
const containerRef = ref<HTMLElement | null>(null)

let debounceTimer: ReturnType<typeof setTimeout> | null = null

const executeSearch = async () => {
  const query = searchQuery.value.trim()
  if (!query) {
    results.value = []
    isOpen.value = false
    hasSearched.value = false
    return
  }

  isLoading.value = true
  hasSearched.value = true
  isOpen.value = true
  highlightedIndex.value = -1

  try {
    const data = await buscarTrabajadores(query)
    results.value = data
  } catch {
    results.value = []
  } finally {
    isLoading.value = false
  }
}

watch(searchQuery, (newVal) => {
  if (debounceTimer) clearTimeout(debounceTimer)
  if (!newVal.trim()) {
    results.value = []
    isOpen.value = false
    hasSearched.value = false
    isLoading.value = false
    return
  }
  debounceTimer = setTimeout(() => {
    executeSearch()
  }, 250)
})

const handleFocus = () => {
  if (searchQuery.value.trim() && results.value.length > 0) {
    isOpen.value = true
  }
}

const handleClear = () => {
  searchQuery.value = ''
  results.value = []
  isOpen.value = false
  hasSearched.value = false
  inputRef.value?.focus()
}

const handleSelect = (dni: string) => {
  isOpen.value = false
  searchQuery.value = ''
  results.value = []
  router.push({ name: 'perfil', params: { dni } })
}

const handleKeyDown = (e: KeyboardEvent) => {
  if (!isOpen.value) {
    if (e.key === 'ArrowDown' && results.value.length > 0) {
      isOpen.value = true
      highlightedIndex.value = 0
      e.preventDefault()
    }
    return
  }

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    if (results.value.length > 0) {
      highlightedIndex.value = (highlightedIndex.value + 1) % results.value.length
    }
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    if (results.value.length > 0) {
      highlightedIndex.value =
        (highlightedIndex.value - 1 + results.value.length) % results.value.length
    }
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (highlightedIndex.value >= 0 && highlightedIndex.value < results.value.length) {
      handleSelect(results.value[highlightedIndex.value].dni)
    } else {
      executeSearch()
    }
  } else if (e.key === 'Escape') {
    isOpen.value = false
    inputRef.value?.blur()
  }
}

const handleClickOutside = (e: MouseEvent) => {
  if (containerRef.value && !containerRef.value.contains(e.target as Node)) {
    isOpen.value = false
  }
}

const handleGlobalKeyDown = (e: KeyboardEvent) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    inputRef.value?.focus()
    inputRef.value?.select()
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  window.addEventListener('keydown', handleGlobalKeyDown)
})

onUnmounted(() => {
  if (debounceTimer) clearTimeout(debounceTimer)
  document.removeEventListener('click', handleClickOutside)
  window.removeEventListener('keydown', handleGlobalKeyDown)
})
</script>

<template>
  <div ref="containerRef" class="relative w-64 sm:w-72 md:w-84">
    <div class="relative">
      <div class="absolute inset-y-0 inset-s-0 flex items-center ps-3 pointer-events-none text-muted-foreground">
        <IconLoader2 v-if="isLoading" class="size-4 animate-spin text-primary" />
        <IconSearch v-else class="size-4" :stroke-width="2" />
      </div>

      <input ref="inputRef" v-model="searchQuery" type="text" placeholder="Buscar por DNI o nombres..."
        aria-label="Buscar servidor público"
        class="w-full h-9 ps-9 pe-14 text-xs rounded-lg border border-border bg-background-1 text-foreground placeholder:text-muted-foreground focus:outline-hidden focus:border-primary focus:ring-1 focus:ring-primary transition"
        @focus="handleFocus" @keydown="handleKeyDown" />

      <div class="absolute inset-y-0 inset-e-0 flex items-center pe-2 gap-1">
        <button v-if="searchQuery" type="button"
          class="p-0.5 rounded text-muted-foreground hover:text-foreground cursor-pointer transition"
          aria-label="Limpiar búsqueda" @click="handleClear">
          <IconX class="size-3.5" />
        </button>
        <kbd
          class="hidden sm:inline-block px-1.5 py-0.5 text-[10px] font-semibold text-muted-foreground bg-muted border border-border rounded pointer-events-none select-none">
          ⌘K
        </kbd>
      </div>
    </div>

    <transition enter-active-class="transition duration-150 ease-out"
      enter-from-class="transform -translate-y-1 opacity-0" enter-to-class="transform translate-y-0 opacity-100"
      leave-active-class="transition duration-100 ease-in" leave-from-class="transform translate-y-0 opacity-100"
      leave-to-class="transform -translate-y-1 opacity-0">
      <div v-if="isOpen"
        class="absolute top-full start-0 mt-1.5 w-full sm:w-96 max-w-[calc(100vw-2rem)] bg-card border border-border rounded-xl shadow-xl z-50 overflow-hidden text-xs">
        <div
          class="px-3.5 py-2 border-b border-border/80 bg-muted/20 flex items-center justify-between text-[11px] text-muted-foreground">
          <span class="font-medium">
            {{ isLoading ? 'Buscando servidores...' : results.length > 0 ? `${results.length} coincidencias encontradas`
              : 'Búsqueda de personal' }}
          </span>
          <span class="text-[10px] font-mono">Presiona Esc para salir</span>
        </div>

        <div v-if="isLoading" class="p-6 text-center text-muted-foreground space-y-2">
          <IconLoader2 class="size-5 mx-auto animate-spin text-primary" />
          <p class="text-xs">Consultando base de datos de personal...</p>
        </div>

        <div v-else-if="results.length > 0" class="max-h-72 overflow-y-auto divide-y divide-border/60">
          <router-link v-for="(item, idx) in results" :key="item.dni"
            :to="{ name: 'perfil', params: { dni: item.dni } }"
            class="w-full p-3 flex items-center justify-between gap-3 text-left transition cursor-pointer"
            :class="highlightedIndex === idx ? 'bg-muted/70 text-foreground' : 'hover:bg-muted/40 text-foreground'"
            @click="handleSelect(item.dni)">
            <div class="flex items-center gap-2.5 min-w-0 flex-1">
              <div
                class="size-9 rounded-lg bg-primary/10 border border-primary/20 text-primary flex items-center justify-center shrink-0 text-xs font-bold overflow-hidden shadow-2xs">
                <img :src="getPersonalAvatarUrl(item.dni)" :alt="item.nombre" class="size-full object-cover"
                  @error="($event.target as HTMLElement).style.display = 'none'" />

              </div>

              <div class="min-w-0 flex-1">
                <p class="font-semibold text-foreground truncate text-xs">
                  {{ item.nombre }}
                </p>
                <p class="font-mono text-muted-foreground text-[11px]">
                  DNI: {{ item.dni }}
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              <Badge :variant="item.estado.toLowerCase() === 'activo' ? 'success' : 'secondary'" size="xs"
                class="gap-1.5">
                <span v-if="item.estado.toLowerCase() === 'activo'" class="relative flex size-1.5 shrink-0">
                  <span
                    class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
                  <span class="relative inline-flex size-1.5 rounded-full bg-emerald-500"></span>
                </span>
                <span v-else class="inline-flex size-1.5 rounded-full bg-muted-foreground/50 shrink-0"></span>
                {{ item.estado }}
              </Badge>
              <IconChevronRight class="size-3.5 text-muted-foreground" />
            </div>
          </router-link>
        </div>

        <div v-else-if="hasSearched" class="p-6 text-center text-muted-foreground space-y-2">
          <IconUser class="size-8 mx-auto text-muted-foreground/40" />
          <p class="font-semibold text-foreground text-xs">Sin coincidencias registradas</p>
          <p class="text-[11px]">No se encontraron servidores públicos con el término "{{ searchQuery }}"</p>
        </div>
      </div>
    </transition>
  </div>
</template>
