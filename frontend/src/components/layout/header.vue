<template>
  <header class="bg-transparent border-b border-gray-100 px-4 md:px-8 py-3 md:py-4">
    <div class="flex items-center justify-between gap-4">
      <button class="md:hidden p-2 -ml-2 text-gray-600 hover:text-primary hover:cursor-pointer rounded-lg transition-colors" @click="$emit('toggle-sidebar')">
        <Menu :size="20" />
      </button>

      <div class="flex-1 max-w-md hidden md:block">
        <div class="relative" ref="searchContainer">
          <Search :size="20" class="lucide lucide-search absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
          <input
            v-model="searchQuery"
            @input="handleSearch"
            @focus="showDropdown = true"
            placeholder="Search"
            class="w-full min-w-[350px] pl-10 pr-12 py-2.5 bg-white border border-gray-200 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-nexus-primary/20 focus:border-nexus-primary transition-all placeholder:text-gray-400"
            type="text"
          />

          <div
            v-if="showDropdown && searchQuery"
            class="absolute top-full w-fit left-0 right-0 mt-2 bg-white rounded-xl shadow-xl border border-gray-100 max-h-96 overflow-y-auto overscroll-contain z-50"
          >
            <div v-if="isLoading" class="p-4 text-center text-gray-500 text-sm">Buscando...</div>
            <div v-else-if="searchResults.length === 0" class="p-4 text-center text-gray-500 text-sm">No se encontraron resultados</div>
            <ul v-else class="py-2">
              <div class="px-4 py-2 bg-gray-50/50 text-xs font-semibold text-gray-500 uppercase tracking-wider border-b border-gray-100">
                Personas encontradas ({{ searchResults.length }})
              </div>
              <RouterLink
                v-for="result in searchResults"
                :key="result.dni"
                :to="{ name: 'personal', params: { dni: result.dni } }"
                @click="selectResult(result)"
                class="flex items-center gap-2 p-2 hover:bg-gray-50 cursor-pointer border-b border-gray-50 last:border-0 transition-colors"
              >
                <div class="relative w-8 h-8 rounded-full overflow-hidden" :class="result.estado === 'activo' ? 'bg-green-500' : 'bg-red-500'">
                  <img v-if="result.avatar" :src="`${SERVER}${result.avatar}`" alt="Avatar" class="w-full h-full object-cover" />

                  <img v-else-if="result.sexo === 'M'" src="/m.png?url" alt="Avatar Default" class="w-full h-full object-cover" />
                  <img v-else src="/f.png?url" alt="Avatar Default" class="w-full h-full object-cover" />
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center justify-between">
                    <p class="text-xs font-semibold text-gray-900 truncate">{{ result.nombre }}</p>
                  </div>
                  <span class="text-xs font-mono text-gray-500 bg-gray-100 px-1.5 py-0.5 rounded">{{ result.dni }}</span>
                </div>
              </RouterLink>
            </ul>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <!-- <button class="p-2.5 bg-white rounded-xl border border-gray-200 text-gray-500 hover:text-gray-900 transition-colors relative cursor-pointer">
          <Bell :size="20" class="lucide lucide-bell w-5 h-5" />
          <span class="absolute top-2 right-2.5 w-2 h-2 bg-red-500 rounded-full border-2 border-white"></span>
        </button> -->
        <div class="relative group outline-none z-50" tabindex="-1" v-if="router.currentRoute.value.name === 'personal'">
          <button
            class="flex items-center justify-center w-10 h-10 bg-white rounded-xl border border-gray-200 text-gray-500 hover:text-[#1a1a1a] hover:border-gray-300 hover:shadow-md hover:-translate-y-0.5 group-focus-within:bg-[#5347ce] group-focus-within:text-white group-focus-within:border-[#5347ce] group-focus-within:shadow-[#5347ce]/30 group-focus-within:shadow-lg transition-all duration-300 cubic-bezier(0.4, 0, 0.2, 1) cursor-pointer"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              class="transition-transform duration-300 group-focus-within:rotate-90"
            >
              <path d="M5 12h14" />
              <path d="M12 5v14" />
            </svg>
          </button>

          <div
            class="absolute right-0 top-full py-5 w-56 bg-white p-1.5 rounded-xl border border-gray-100 shadow-xl shadow-gray-200/50 invisible opacity-0 translate-y-2 scale-95 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-0 group-focus-within:scale-100 transition-all duration-200 ease-out origin-top-right overflow-hidden"
          >
            <div class="flex flex-col gap-1">
              <div class="px-3 py-2 text-[10px] font-bold text-gray-400 uppercase tracking-wider border-b border-gray-50 mb-1">Acciones</div>
              <Agregar_banco />
            </div>
          </div>
        </div>

        <div class="relative group outline-none z-50 cursor-pointer" tabindex="-1">
          <div
            class="flex items-center gap-3 hover:bg-gray-50 p-1.5 pl-2 pr-3 rounded-xl transition-all border border-transparent hover:border-gray-100 hover:shadow-sm group-focus-within:bg-gray-50 group-focus-within:border-gray-100 group-focus-within:shadow-sm"
          >
            <div class="w-9 h-9 rounded-full bg-gray-200 overflow-hidden border border-gray-200 shadow-sm relative shrink-0">
              <img src="/m.png?url" alt="User Avatar" class="w-full h-full object-cover" />
            </div>

            <div class="hidden md:block text-left mr-1">
              <p class="text-sm font-semibold text-gray-900 leading-tight">{{ store.nombre }}</p>
              <p class="text-[10px] text-gray-500 font-medium uppercase tracking-wide">{{ store.isAdmin ? 'Administrador' : 'Usuario' }}</p>
            </div>

            <ChevronDown class="w-3.5 h-3.5 text-gray-400 group-focus-within:rotate-180 transition-transform duration-300" />
          </div>

          <div
            class="absolute right-0 top-full mt-2 w-52 bg-white p-1.5 rounded-xl border border-gray-100 shadow-xl shadow-gray-200/50 invisible opacity-0 translate-y-2 scale-95 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-0 group-focus-within:scale-100 transition-all duration-200 ease-out origin-top-right overflow-hidden"
          >
            <div class="px-2 py-1.5 mb-1 bg-gray-50 rounded-lg">
              <p class="text-[10px] font-bold text-gray-400 uppercase tracking-wider">Cuenta</p>
              <p class="text-xs font-medium text-gray-700 truncate">{{ store.nombre }}</p>
            </div>

            <div class="flex flex-col gap-0.5">
              <button
                class="flex items-center gap-2.5 w-full px-2.5 py-2 text-xs font-medium text-gray-600 rounded-lg hover:bg-gray-50 hover:text-gray-900 transition-colors text-left"
                @click="store.logout()"
              >
                <Users class="w-3.5 h-3.5" />
                Cambiar de cuenta
              </button>
              <div class="h-px bg-gray-50 my-0.5"></div>
              <button class="flex items-center gap-2.5 w-full px-2.5 py-2 text-xs font-medium text-red-600 rounded-lg hover:bg-red-50 transition-colors text-left" @click="store.logout()">
                <LogOut class="w-3.5 h-3.5" />
                Cerrar sesión
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { userStore } from '@store/user'
import { api, SERVER } from '@api/axios'
import { Menu, Search, LogOut, Users, ChevronDown } from 'lucide-vue-next'
import Agregar_banco from '@comp/personal/modal/agregar_banco.vue'
import { router } from '@router/router'

defineEmits(['toggle-sidebar'])

const store = userStore()
const searchQuery = ref('')
const searchResults = ref<any[]>([])
const isLoading = ref(false)
const showDropdown = ref(false)
const searchContainer = ref<HTMLElement | null>(null)
let searchTimeout: ReturnType<typeof setTimeout>

const handleSearch = () => {
  showDropdown.value = true
  clearTimeout(searchTimeout)

  if (!searchQuery.value.trim()) {
    searchResults.value = []
    return
  }

  isLoading.value = true
  searchTimeout = setTimeout(async () => {
    try {
      const response = await api.post('/personal/buscar', { nombre: searchQuery.value })
      searchResults.value = response.data
    } catch (error) {
      console.error('Search error:', error)
      searchResults.value = []
    } finally {
      isLoading.value = false
    }
  }, 300)
}

const selectResult = async (_result: any) => {
  searchQuery.value = ''
  showDropdown.value = false
}

const handleClickOutside = (event: MouseEvent) => {
  if (searchContainer.value && !searchContainer.value.contains(event.target as Node)) {
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
