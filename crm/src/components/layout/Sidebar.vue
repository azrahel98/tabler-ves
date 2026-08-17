<script setup lang="ts">
import {
  IconLayoutDashboard,
  IconCalendar,
  IconHourglass,
  IconArrowsDiff,
  IconLogout,
  IconX,
  IconUsers,
  IconUser,
  IconMapPin,
  IconBuildingCommunity,
  IconBriefcase,
} from '@tabler/icons-vue'
import { RouterLink, useRoute } from 'vue-router'
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useAuthStore } from '@/stores/auth'

defineProps<{ isOpen?: boolean }>()
const emit = defineEmits<{ (e: 'close'): void }>()

const route = useRoute()
const authStore = useAuthStore()
const { currentUser } = storeToRefs(authStore)
const logout = authStore.logout

interface ElementoNav {
  nombre: string
  ruta: string
  icono: any
  badge?: string
  badgeColor?: string
}

interface GrupoNav {
  titulo: string
  items: ElementoNav[]
}

const gruposNav = computed<GrupoNav[]>(() => {
  const items: ElementoNav[] = [
    {
      nombre: 'Dashboard',
      ruta: '/dashboard',
      icono: IconLayoutDashboard,
    },
    {
      nombre: 'Calendario',
      ruta: '/calendario',
      icono: IconCalendar,
    },
    {
      nombre: 'Jubilación (70 Años)',
      ruta: '/jubilacion',
      icono: IconHourglass,
    },
    {
      nombre: 'Comparar MEF',
      ruta: '/comparacion-mef',
      icono: IconArrowsDiff,
    },
  ]

  if (route.name === 'perfil' || route.path.startsWith('/perfil/')) {
    items.push({
      nombre: 'Personal',
      ruta: route.fullPath,
      icono: IconUser,
    })
  }

  if (route.name === 'distrito' || route.path.startsWith('/distrito/')) {
    items.push({
      nombre: 'Distrito',
      ruta: route.fullPath,
      icono: IconMapPin,
    })
  }

  if (route.name === 'area' || route.path.startsWith('/area/')) {
    items.push({
      nombre: 'Área',
      ruta: route.fullPath,
      icono: IconBuildingCommunity,
    })
  }

  if (route.name === 'regimen' || route.path.startsWith('/regimen/')) {
    items.push({
      nombre: 'Régimen',
      ruta: route.fullPath,
      icono: IconBriefcase,
    })
  }

  if (route.name === 'sindicato' || route.path.startsWith('/sindicato/')) {
    items.push({
      nombre: 'Sindicato',
      ruta: route.fullPath,
      icono: IconBuildingCommunity,
    })
  }

  if (currentUser.value?.role === 'ADMIN') {
    items.push({
      nombre: 'Usuarios',
      ruta: '/usuarios',
      icono: IconUsers,
    })
  }

  return [
    {
      titulo: 'Principal',
      items,
    },
  ]
})
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-40 bg-gray-900/50 backdrop-blur-xs md:hidden transition-opacity"
    @click="emit('close')" />

  <aside
    :class="[
      'fixed top-0 left-0 z-50 flex h-full min-h-full w-64 flex-col bg-white border-r border-gray-200/80 shadow-xs transition-transform duration-300 ease-in-out dark:bg-gray-900 dark:border-gray-800 md:z-0 md:w-16 xl:w-64 overflow-hidden',
      isOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0',
    ]">
    <div
      class="flex h-20 items-center justify-between border-b border-gray-100 px-4 dark:border-gray-800 md:justify-center xl:justify-between xl:px-5">
      <div class="flex items-center gap-3">
        <div
          class="flex h-8 w-8 items-center justify-center rounded-xl bg-gradient-to-tr from-blue-600 to-indigo-500 shadow-xs text-white flex-shrink-0">
          <svg
            class="h-4 w-4"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round">
            <path d="M12 2L2 7l10 5 10-5-10-5z" />
            <path d="M2 17l10 5 10-5" />
            <path d="M2 12l10 5 10-5" />
          </svg>
        </div>
        <div class="min-w-0 md:hidden xl:flex items-center gap-2">
          <span class="text-xs font-bold uppercase tracking-wider text-gray-900 dark:text-white">
            CRM <span class="text-blue-600 dark:text-blue-400">VES</span>
          </span>
          <span
            class="inline-flex items-center rounded-md bg-blue-50 px-1.5 py-0.5 text-2xs font-bold text-blue-600 dark:bg-blue-900/40 dark:text-blue-300">
            PRO
          </span>
        </div>
      </div>

      <button
        type="button"
        @click="emit('close')"
        class="inline-flex h-7 w-7 items-center justify-center rounded-lg text-gray-400 hover:bg-gray-100 hover:text-gray-600 dark:hover:bg-gray-800 dark:hover:text-gray-200 transition-colors md:hidden">
        <IconX class="h-4 w-4" />
      </button>
    </div>

    <nav class="flex-1 pt-4 space-y-4 overflow-y-auto overflow-x-hidden custom-scrollbar">
      <div v-for="grupo in gruposNav" :key="grupo.titulo" class="space-y-1">
        <p
          v-if="grupo.titulo"
          class="px-4 text-2xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 md:hidden xl:block">
          {{ grupo.titulo }}
        </p>

        <div v-for="item in grupo.items" :key="item.nombre" class="relative">
          <RouterLink
            :to="item.ruta"
            @click="emit('close')"
            class="group relative flex items-center gap-3 py-2.5 px-3 mx-2 rounded-xl text-xs font-medium text-gray-600 hover:bg-gray-100/70 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-800/60 dark:hover:text-gray-200 transition-all duration-150 select-none cursor-pointer [&.sidebar-activo]:bg-blue-50/70 [&.sidebar-activo]:text-blue-600 [&.sidebar-activo]:font-semibold dark:[&.sidebar-activo]:bg-blue-950/30 dark:[&.sidebar-activo]:text-blue-400">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-lg transition-colors shrink-0 bg-gray-100/80 text-gray-500 group-hover:bg-gray-200/80 group-hover:text-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:group-hover:bg-gray-700 dark:group-hover:text-gray-200 group-[.sidebar-activo]:text-blue-600 dark:group-[.sidebar-activo]:text-blue-400">
              <component :is="item.icono" class="h-4 w-4" />
            </div>

            <span class="leading-none flex-1 truncate md:hidden xl:block">
              {{ item.nombre }}
            </span>

            <span
              class="hidden group-[.sidebar-activo]:inline-block group-[.sidebar-activo]:md:hidden group-[.sidebar-activo]:xl:inline-block h-1.5 w-1.5 rounded-full bg-blue-600 dark:bg-blue-400 shrink-0" />

            <span
              v-if="item.badge"
              :class="[
                'inline-flex items-center rounded-md px-1.5 py-0.5 text-2xs font-bold md:hidden xl:inline-flex group-[.sidebar-activo]:hidden',
                item.badgeColor || 'bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400',
              ]">
              {{ item.badge }}
            </span>
          </RouterLink>
        </div>
      </div>
    </nav>

    <!-- Tarjeta de Usuario / Pie de Sidebar -->
    <div class="border-t border-gray-100 p-3 dark:border-gray-800">
      <div
        class="flex items-center justify-between rounded-lg p-2 hover:bg-gray-50 dark:hover:bg-gray-800/60 transition-colors">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="relative shrink-0">
            <div
              class="flex h-7 w-7 items-center justify-center rounded-full bg-blue-600 text-xs font-semibold text-white">
              {{
                currentUser?.full_name?.charAt(0).toUpperCase() || currentUser?.email?.charAt(0).toUpperCase() || 'A'
              }}
            </div>
            <!-- Estado en línea -->
            <span
              class="absolute bottom-0 right-0 h-2.5 w-2.5 rounded-full bg-emerald-500 ring-2 ring-white dark:ring-gray-900" />
          </div>

          <div class="min-w-0 md:hidden xl:block">
            <p class="text-xs font-semibold text-gray-900 dark:text-white truncate leading-tight">
              {{ currentUser?.full_name || 'Administrador' }}
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400 truncate">Online · Sistema VES</p>
          </div>
        </div>

        <button
          type="button"
          @click="logout()"
          class="inline-flex h-7 w-7 items-center justify-center rounded-lg text-gray-400 hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-950/30 dark:hover:text-red-400 transition-colors cursor-pointer md:hidden xl:inline-flex"
          title="Cerrar sesión">
          <IconLogout class="h-4 w-4" />
        </button>
      </div>
    </div>
  </aside>
</template>
