<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import { usePerfilStore } from '@/stores/perfil'
import {
  IconMenu2,
  IconMoon,
  IconSun,
  IconSearch,
  IconLogout,
  IconUser,
  IconLoader2,
  IconX,
  IconCommand,
} from '@tabler/icons-vue'
import { useAuthStore } from '@/stores/auth'
import { useInterfazStore } from '@/stores/interfaz'

const emit = defineEmits<{ (e: 'toggleSidebar'): void }>()
const interfazStore = useInterfazStore()

const router = useRouter()
const oscuro = ref(false)
const errorImagen = ref(false)
const busqueda = ref('')
const mostrarDropdown = ref(false)
const busquedaMovilAbierta = ref(false)
const indiceSeleccionado = ref(-1)

const contenedorBusqueda = ref<HTMLElement | null>(null)
const inputBusquedaEscritorio = ref<HTMLInputElement | null>(null)
const inputBusquedaMovil = ref<HTMLInputElement | null>(null)

const authStore = useAuthStore()
const { currentUser } = storeToRefs(authStore)
const logout = authStore.logout

const perfilStore = usePerfilStore()
const { resultadosBusqueda, buscandoSugerencias } = storeToRefs(perfilStore)
const buscarSugerencias = perfilStore.buscarSugerencias

function manejarEntradaBusqueda() {
  const q = busqueda.value.trim()
  indiceSeleccionado.value = -1
  if (q.length >= 2) {
    mostrarDropdown.value = true
    buscarSugerencias(q)
  } else {
    mostrarDropdown.value = false
  }
}

function seleccionarPersona(dni: string) {
  mostrarDropdown.value = false
  busquedaMovilAbierta.value = false
  busqueda.value = ''
  indiceSeleccionado.value = -1
  router.push(`/perfil/${dni}`)
}

function limpiarBusqueda() {
  busqueda.value = ''
  mostrarDropdown.value = false
  busquedaMovilAbierta.value = false
  indiceSeleccionado.value = -1
}

function alternarBusquedaMovil() {
  busquedaMovilAbierta.value = !busquedaMovilAbierta.value
  if (busquedaMovilAbierta.value) {
    nextTick(() => {
      inputBusquedaMovil.value?.focus()
    })
  } else {
    limpiarBusqueda()
  }
}

function manejarTecladoBusqueda(e: KeyboardEvent) {
  if (!mostrarDropdown.value || !resultadosBusqueda.value || resultadosBusqueda.value.length === 0) {
    if (e.key === 'Escape') {
      mostrarDropdown.value = false
    }
    return
  }

  if (e.key === 'ArrowDown') {
    e.preventDefault()
    indiceSeleccionado.value = (indiceSeleccionado.value + 1) % resultadosBusqueda.value.length
  } else if (e.key === 'ArrowUp') {
    e.preventDefault()
    indiceSeleccionado.value =
      indiceSeleccionado.value <= 0 ? resultadosBusqueda.value.length - 1 : indiceSeleccionado.value - 1
  } else if (e.key === 'Enter') {
    e.preventDefault()
    if (indiceSeleccionado.value >= 0 && indiceSeleccionado.value < resultadosBusqueda.value.length) {
      seleccionarPersona(resultadosBusqueda.value[indiceSeleccionado.value].dni)
    }
  } else if (e.key === 'Escape') {
    mostrarDropdown.value = false
  }
}

function manejarAtajoGlobal(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    if (window.innerWidth < 1024) {
      busquedaMovilAbierta.value = true
      nextTick(() => inputBusquedaMovil.value?.focus())
    } else {
      inputBusquedaEscritorio.value?.focus()
    }
  }
}

function manejarClickFuera(e: MouseEvent) {
  if (contenedorBusqueda.value && !contenedorBusqueda.value.contains(e.target as Node)) {
    mostrarDropdown.value = false
  }
}

function aplicarTema(dark: boolean) {
  if (dark) {
    document.documentElement.classList.add('dark')
    localStorage.setItem('tema', 'dark')
  } else {
    document.documentElement.classList.remove('dark')
    localStorage.setItem('tema', 'light')
  }
}

function alternarTema() {
  oscuro.value = !oscuro.value
  aplicarTema(oscuro.value)
}

onMounted(() => {
  const temaGuardado = localStorage.getItem('tema')
  const prefiereOscuro = window.matchMedia('(prefers-color-scheme: dark)').matches
  oscuro.value = temaGuardado === 'dark' || (!temaGuardado && prefiereOscuro)
  aplicarTema(oscuro.value)
  document.addEventListener('click', manejarClickFuera)
  window.addEventListener('keydown', manejarAtajoGlobal)
})

onUnmounted(() => {
  document.removeEventListener('click', manejarClickFuera)
  window.removeEventListener('keydown', manejarAtajoGlobal)
})
</script>

<template>
  <header
    class="sticky top-0 z-40 flex flex-col bg-white/90 backdrop-blur-xl border-b border-gray-100 dark:bg-gray-900/90 dark:border-gray-800/80 transition-colors">
    <div class="flex h-20 items-center justify-between gap-4 px-4 lg:px-6">
      <div class="flex items-center gap-3">
        <button
          type="button"
          @click="interfazStore.alternarMenuLateral(); emit('toggleSidebar')"
          class="inline-flex items-center justify-center h-9 w-9 rounded-xl text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-200 transition-colors cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500/20"
          :aria-label="interfazStore.menuLateralColapsado ? 'Expandir menú lateral' : 'Colapsar menú lateral'">
          <IconMenu2 class="h-5 w-5" />
        </button>

        <div ref="contenedorBusqueda" class="relative hidden lg:block w-72 xl:w-96">
          <div class="relative w-full">
            <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5">
              <IconLoader2 v-if="buscandoSugerencias" class="h-4 w-4 animate-spin text-blue-600 dark:text-blue-400" />
              <IconSearch v-else class="h-4 w-4 text-gray-400" />
            </div>
            <input
              ref="inputBusquedaEscritorio"
              v-model="busqueda"
              @input="manejarEntradaBusqueda"
              @focus="manejarEntradaBusqueda"
              @keydown="manejarTecladoBusqueda"
              type="text"
              placeholder="Buscar trabajador por DNI o Nombre..."
              class="block w-full rounded-xl border border-gray-200/90 bg-gray-50/80 py-2.5 pl-10 pr-16 text-xs text-gray-900 placeholder:text-gray-400 focus:border-blue-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500/20 dark:border-gray-700/80 dark:bg-gray-800/90 dark:text-gray-100 dark:placeholder:text-gray-500 dark:focus:border-blue-400 transition-all" />

            <div class="absolute inset-y-0 right-0 flex items-center pr-2.5 gap-1">
              <button
                v-if="busqueda"
                type="button"
                @click="limpiarBusqueda()"
                class="p-0.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors cursor-pointer rounded">
                <IconX class="h-3.5 w-3.5" />
              </button>
              <kbd
                v-else
                class="hidden xl:inline-flex items-center gap-0.5 rounded border border-gray-200 bg-white px-1.5 py-0.5 text-[10px] font-semibold font-mono text-gray-400 shadow-2xs dark:border-gray-700 dark:bg-gray-800 dark:text-gray-500">
                <IconCommand class="h-2.5 w-2.5" />K
              </kbd>
            </div>
          </div>

          <div
            v-if="mostrarDropdown && busqueda.trim().length >= 2"
            class="absolute left-0 right-0 top-full mt-1.5 z-50 rounded-xl border border-gray-100 bg-white shadow-xl dark:border-gray-700/80 dark:bg-gray-800 overflow-hidden max-h-80 overflow-y-auto custom-scrollbar animate-in fade-in slide-in-from-top-1 duration-150">
            <div
              v-if="buscandoSugerencias"
              class="flex items-center justify-center gap-2 p-4 text-2xs font-medium text-gray-400">
              <IconLoader2 class="h-4 w-4 animate-spin text-blue-600 dark:text-blue-400" />
              <span>Buscando trabajadores...</span>
            </div>

            <div
              v-else-if="resultadosBusqueda && resultadosBusqueda.length > 0"
              class="divide-y divide-gray-100 dark:divide-gray-700/50">
              <div
                v-for="(persona, idx) in resultadosBusqueda"
                :key="persona.dni"
                @click="seleccionarPersona(persona.dni)"
                :class="[
                  'flex items-center gap-3 px-3.5 py-2.5 cursor-pointer transition-colors group',
                  indiceSeleccionado === idx
                    ? 'bg-blue-50 dark:bg-blue-900/30'
                    : 'hover:bg-blue-50/70 dark:hover:bg-gray-700/60',
                ]">
                <div class="relative h-7 w-7 shrink-0 overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
                  <img
                    v-if="persona.foto"
                    :src="persona.foto"
                    :alt="persona.nombre"
                    class="h-full w-full object-cover" />
                  <div v-else class="flex h-full w-full items-center justify-center text-gray-400 dark:text-gray-500">
                    <IconUser class="h-4 w-4" />
                  </div>
                </div>

                <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between gap-1">
                    <p
                      class="text-xs font-semibold text-gray-900 dark:text-white truncate group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                      {{ persona.nombre }}
                    </p>
                    <span
                      class="text-2xs font-mono font-bold text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/30 px-1.5 py-0.5 rounded">
                      {{ persona.dni }}
                    </span>
                  </div>
                  <p v-if="persona.cargo || persona.area" class="text-2xs font-medium text-gray-400 truncate">
                    {{ persona.cargo || 'Sin cargo' }} · {{ persona.area || 'General' }}
                  </p>
                </div>
              </div>
            </div>

            <div v-else class="p-4 text-center text-2xs font-medium text-gray-400 dark:text-gray-500">
              No se encontraron resultados para "{{ busqueda }}"
            </div>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          @click="alternarBusquedaMovil"
          class="inline-flex items-center justify-center h-9 w-9 rounded-xl text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-200 transition-colors cursor-pointer lg:hidden focus:outline-none focus:ring-2 focus:ring-blue-500/20"
          aria-label="Buscar trabajador">
          <IconSearch class="h-4.5 w-4.5" />
        </button>

        <button
          type="button"
          @click="alternarTema"
          class="inline-flex items-center justify-center h-9 w-9 rounded-xl text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-200 transition-all cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-500/20"
          :title="oscuro ? 'Cambiar a modo claro' : 'Cambiar a modo oscuro'"
          :aria-label="oscuro ? 'Cambiar a modo claro' : 'Cambiar a modo oscuro'">
          <IconMoon v-if="!oscuro" class="h-4.5 w-4.5 transition-transform duration-200 hover:rotate-12" />
          <IconSun v-else class="h-4.5 w-4.5 text-amber-400 transition-transform duration-200 hover:rotate-45" />
        </button>

        <div class="mx-1 h-5 w-px bg-gray-200 dark:bg-gray-700 hidden sm:block" />

        <div class="hs-dropdown [--placement:bottom-right] [--auto-close:inside] relative inline-flex">
          <button
            id="nav-perfil"
            type="button"
            class="hs-dropdown-toggle relative inline-flex items-center justify-center cursor-pointer focus:outline-none rounded-full ring-2 ring-transparent hover:ring-blue-500/25 focus:ring-blue-500/30 transition-all"
            aria-expanded="false"
            aria-haspopup="menu"
            aria-label="Menú de cuenta">
            <img
              v-if="currentUser?.picture_url && !errorImagen"
              class="h-8.5 w-8.5 rounded-full object-cover"
              :src="currentUser.picture_url"
              loading="lazy"
              referrerpolicy="no-referrer"
              @error="errorImagen = true"
              alt="Avatar de usuario" />
            <div
              v-else
              class="flex h-8.5 w-8.5 items-center justify-center rounded-full bg-blue-600 text-xs font-bold text-white select-none">
              {{
                currentUser?.full_name?.charAt(0).toUpperCase() || currentUser?.email?.charAt(0).toUpperCase() || 'U'
              }}
            </div>
            <span
              class="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-emerald-500 ring-2 ring-white dark:ring-gray-900" />
          </button>

          <div
            id="dd-perfil"
            class="hs-dropdown-menu hs-dropdown-open:opacity-100 hidden w-64 rounded-xl border border-gray-100 bg-white opacity-0 shadow-xl transition-[opacity,margin] duration-200 dark:border-gray-700 dark:bg-gray-800"
            role="menu"
            aria-orientation="vertical"
            aria-labelledby="nav-perfil">
            <div class="flex items-center gap-3 px-4 py-3 border-b border-gray-100 dark:border-gray-700">
              <img
                v-if="currentUser?.picture_url && !errorImagen"
                class="h-9 w-9 rounded-full object-cover shrink-0"
                :src="currentUser.picture_url"
                loading="lazy"
                referrerpolicy="no-referrer"
                @error="errorImagen = true"
                alt="Avatar" />
              <div
                v-else
                class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-blue-600 text-xs font-bold text-white">
                {{ currentUser?.full_name?.charAt(0).toUpperCase() || 'U' }}
              </div>
              <div class="min-w-0 flex-1">
                <div class="flex items-center justify-between gap-1">
                  <p class="text-xs font-bold text-gray-900 dark:text-white truncate">
                    {{ currentUser?.full_name || 'Usuario' }}
                  </p>
                  <span
                    :class="[
                      'rounded-md px-1.5 py-0.2 text-[9px] font-bold uppercase tracking-wider',
                      currentUser?.role === 'ADMIN'
                        ? 'bg-purple-50 text-purple-700 dark:bg-purple-900/40 dark:text-purple-300'
                        : 'bg-blue-50 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300',
                    ]">
                    {{ currentUser?.role || 'USER' }}
                  </span>
                </div>
                <p class="text-2xs text-gray-400 truncate">{{ currentUser?.email }}</p>
              </div>
            </div>

            <div class="p-1.5">
              <button
                type="button"
                @click="logout()"
                class="flex w-full items-center gap-2.5 rounded-lg px-3 py-2 text-xs font-semibold text-red-600 hover:bg-red-50 dark:text-red-400 dark:hover:bg-red-950/30 transition-colors cursor-pointer"
                role="menuitem">
                <IconLogout class="h-4 w-4 shrink-0" />
                Cerrar sesión
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div
      v-if="busquedaMovilAbierta"
      class="border-t border-gray-100 p-3 dark:border-gray-800 lg:hidden bg-white/95 dark:bg-gray-900/95 shadow-md backdrop-blur-md animate-in fade-in slide-in-from-top-2 duration-150">
      <div class="relative w-full">
        <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
          <IconLoader2 v-if="buscandoSugerencias" class="h-4 w-4 animate-spin text-blue-600 dark:text-blue-400" />
          <IconSearch v-else class="h-4 w-4 text-gray-400" />
        </div>
        <input
          ref="inputBusquedaMovil"
          v-model="busqueda"
          @input="manejarEntradaBusqueda"
          @keydown="manejarTecladoBusqueda"
          type="text"
          placeholder="Buscar trabajador por DNI o Nombre..."
          class="block w-full rounded-xl border border-gray-200 bg-gray-50 py-2.5 pl-9 pr-8 text-xs text-gray-900 placeholder:text-gray-400 focus:border-blue-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500/20 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100 transition-all" />
        <button
          type="button"
          @click="limpiarBusqueda()"
          class="absolute inset-y-0 right-0 flex items-center pr-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 cursor-pointer">
          <IconX class="h-4 w-4" />
        </button>
      </div>

      <div
        v-if="busqueda.trim().length >= 2"
        class="mt-2 rounded-xl border border-gray-100 bg-white dark:border-gray-700 dark:bg-gray-800 overflow-hidden max-h-60 overflow-y-auto shadow-md">
        <div v-if="buscandoSugerencias" class="flex items-center justify-center gap-2 p-3 text-2xs text-gray-400">
          <IconLoader2 class="h-4 w-4 animate-spin text-blue-600 dark:text-blue-400" />
          <span>Buscando trabajadores...</span>
        </div>

        <div
          v-else-if="resultadosBusqueda && resultadosBusqueda.length > 0"
          class="divide-y divide-gray-100 dark:divide-gray-700/50">
          <div
            v-for="(persona, idx) in resultadosBusqueda"
            :key="persona.dni"
            @click="seleccionarPersona(persona.dni)"
            :class="[
              'flex items-center gap-3 px-3 py-2 cursor-pointer transition-colors',
              indiceSeleccionado === idx
                ? 'bg-blue-50 dark:bg-blue-900/30'
                : 'hover:bg-blue-50/70 dark:hover:bg-gray-700/60',
            ]">
            <div class="relative h-7 w-7 shrink-0 overflow-hidden rounded-full bg-gray-100 dark:bg-gray-700">
              <img v-if="persona.foto" :src="persona.foto" :alt="persona.nombre" class="h-full w-full object-cover" />
              <div v-else class="flex h-full w-full items-center justify-center text-gray-400">
                <IconUser class="h-3.5 w-3.5" />
              </div>
            </div>
            <div class="min-w-0 flex-1">
              <p class="text-xs font-semibold text-gray-900 dark:text-white truncate">{{ persona.nombre }}</p>
              <p class="text-2xs text-gray-400 font-mono">{{ persona.dni }}</p>
            </div>
          </div>
        </div>

        <div v-else class="p-3 text-center text-2xs text-gray-400">
          No se encontraron resultados para "{{ busqueda }}"
        </div>
      </div>
    </div>
  </header>
</template>
