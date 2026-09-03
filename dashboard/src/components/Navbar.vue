<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import NavbarSearch from '@/components/NavbarSearch.vue'
import {
  IconMenu2,
  IconLayoutSidebarLeftCollapse,
  IconSun,
  IconMoon,
  IconBell,
  IconChevronDown,
  IconLayoutDashboard,
  IconUsers,
  IconUser,
  IconLogout,
} from '@tabler/icons-vue'

const emit = defineEmits<{
  (e: 'toggleSidebar'): void
  (e: 'toggleCollapse'): void
}>()

const router = useRouter()
const authStore = useAuthStore()

const isUserMenuOpen = ref(false)
const isDarkMode = ref(document.documentElement.classList.contains('dark'))

const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark')
    localStorage.setItem('hs_theme', 'dark')
  } else {
    document.documentElement.classList.remove('dark')
    localStorage.setItem('hs_theme', 'light')
  }
}

const handleLogout = () => {
  authStore.logout()
  router.push('/iniciar-sesion')
}
</script>

<template>
  <header class="sticky top-0 z-30 h-16 bg-navbar border-b border-navbar-line flex items-center justify-between px-4 sm:px-6">
    <div class="flex items-center gap-3">
      <button
        type="button"
        class="lg:hidden p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden"
        aria-label="Abrir barra lateral"
        @click="emit('toggleSidebar')"
      >
        <IconMenu2 class="size-5" :stroke-width="2" />
      </button>

      <button
        type="button"
        class="hidden lg:flex p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition"
        title="Alternar barra lateral"
        aria-label="Alternar barra lateral"
        @click="emit('toggleCollapse')"
      >
        <IconLayoutSidebarLeftCollapse class="size-5" :stroke-width="2" />
      </button>

      <div class="hidden sm:block">
        <NavbarSearch />
      </div>
    </div>

    <div class="flex items-center gap-2 sm:gap-3">
      <button
        type="button"
        class="size-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition"
        title="Modo Oscuro / Claro"
        @click="toggleDarkMode"
      >
        <IconSun v-if="isDarkMode" class="size-4" :stroke-width="2" />
        <IconMoon v-else class="size-4" :stroke-width="2" />
      </button>

      <button
        type="button"
        class="relative size-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition"
        title="Notificaciones"
      >
        <IconBell class="size-4" :stroke-width="2" />
        <span class="absolute top-2 inset-e-2 size-2 rounded-full bg-rose-500 ring-2 ring-navbar"></span>
      </button>

      <div class="h-6 w-px bg-border mx-1"></div>

      <div class="relative">
        <button
          type="button"
          class="flex items-center gap-2 p-1 rounded-lg hover:bg-muted focus:outline-hidden transition"
          @click="isUserMenuOpen = !isUserMenuOpen"
        >
          <img
            :src="authStore.user?.avatar || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=256&auto=format&fit=crop'"
            alt="Avatar de usuario"
            class="size-8 rounded-full object-cover border border-border"
          />
          <div class="hidden md:flex flex-col text-left">
            <span class="text-xs font-semibold text-foreground leading-tight">
              {{ authStore.user?.name || 'Administrador' }}
            </span>
            <span class="text-[10px] text-muted-foreground leading-tight">
              {{ authStore.user?.role || 'Super Admin' }}
            </span>
          </div>
          <IconChevronDown class="size-3.5 text-muted-foreground hidden md:block" :stroke-width="2" />
        </button>

        <div
          v-if="isUserMenuOpen"
          class="fixed inset-0 z-40"
          @click="isUserMenuOpen = false"
        ></div>

        <div
          v-if="isUserMenuOpen"
          class="absolute inset-e-0 mt-2 w-56 bg-card border border-border rounded-xl shadow-lg py-1.5 z-50 text-xs"
        >
          <div class="px-4 py-2 border-b border-border">
            <p class="font-semibold text-foreground">{{ authStore.user?.name || 'Administrador' }}</p>
            <p class="text-muted-foreground truncate">{{ authStore.user?.email || 'admin@crmpulse.com' }}</p>
          </div>

          <div class="py-1">
            <router-link
              to="/perfil"
              class="flex items-center gap-2.5 px-4 py-2 text-foreground hover:bg-muted transition"
              @click="isUserMenuOpen = false"
            >
              <IconUser class="size-4 text-muted-foreground" :stroke-width="2" />
              <span>Mi Perfil</span>
            </router-link>

            <router-link
              to="/panel"
              class="flex items-center gap-2.5 px-4 py-2 text-foreground hover:bg-muted transition"
              @click="isUserMenuOpen = false"
            >
              <IconLayoutDashboard class="size-4 text-muted-foreground" :stroke-width="2" />
              <span>Panel Principal</span>
            </router-link>

            <router-link
              to="/customers"
              class="flex items-center gap-2.5 px-4 py-2 text-foreground hover:bg-muted transition"
              @click="isUserMenuOpen = false"
            >
              <IconUsers class="size-4 text-muted-foreground" :stroke-width="2" />
              <span>Cartera de Clientes</span>
            </router-link>
          </div>

          <div class="border-t border-border pt-1">
            <button
              type="button"
              class="w-full flex items-center gap-2.5 px-4 py-2 text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-950/30 transition text-left"
              @click="handleLogout"
            >
              <IconLogout class="size-4" :stroke-width="2" />
              <span>Cerrar sesión</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
