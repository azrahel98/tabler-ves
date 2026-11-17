<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import NavbarSearch from '@/components/NavbarSearch.vue'
import NavbarNotifications from '@/components/NavbarNotifications.vue'
import {
  IconMenu2,
  IconLayoutSidebarLeftCollapse,
  IconSun,
  IconMoon,
  IconChevronDown,
  IconLayoutDashboard,
  IconLogout,
} from '@tabler/icons-vue'
import { useNotificacionesStore } from '@/stores/notificaciones'

const emit = defineEmits<{
  (e: 'toggleSidebar'): void
  (e: 'toggleCollapse'): void
}>()

const router = useRouter()
const authStore = useAuthStore()
const themeStore = useThemeStore()
const notifStore = useNotificacionesStore()

const handleLogout = () => {
  notifStore.resetear()
  authStore.logout()
  router.push('/iniciar-sesion')
}
</script>

<template>
  <header
    class="sticky top-0 z-30 h-13 sm:h-14 2xl:h-16 bg-navbar border-b border-navbar-line flex items-center justify-between px-3 sm:px-6">
    <div class="flex items-center gap-2.5 sm:gap-3">
      <button type="button"
        class="lg:hidden p-1.5 sm:p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden cursor-pointer"
        aria-label="Abrir barra lateral" @click="emit('toggleSidebar')">
        <IconMenu2 class="size-5" :stroke-width="2" />
      </button>

      <button type="button"
        class="hidden lg:flex p-1.5 sm:p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition cursor-pointer"
        title="Alternar barra lateral" aria-label="Alternar barra lateral" @click="emit('toggleCollapse')">
        <IconLayoutSidebarLeftCollapse class="size-5" :stroke-width="2" />
      </button>

      <NavbarSearch />
    </div>

    <div class="flex items-center gap-1.5 sm:gap-3">
      <button type="button"
        class="size-8 sm:size-9 flex items-center justify-center rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden transition cursor-pointer"
        title="Modo Oscuro / Claro" @click="themeStore.toggleTheme">
        <IconSun v-if="themeStore.isDark" class="size-4" :stroke-width="2" />
        <IconMoon v-else class="size-4" :stroke-width="2" />
      </button>

      <NavbarNotifications />

      <div class="h-5 sm:h-6 w-px bg-border mx-0.5 sm:mx-1"></div>

      <div class="hs-dropdown [--placement:bottom-right] relative inline-flex">
        <button id="user-menu-btn" type="button" aria-haspopup="menu" aria-expanded="false" aria-label="Menú de usuario"
          class="hs-dropdown-toggle flex items-center gap-1.5 p-1 rounded-lg hover:bg-muted focus:outline-hidden transition cursor-pointer"
          >
          <div class="flex flex-col items-center">
            <img
              :src="authStore.user?.avatar || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=256&auto=format&fit=crop'"
              alt="Avatar de usuario" class="size-7 rounded-full object-cover border border-border" />
            <span class="text-nano font-semibold uppercase tracking-wider text-muted-foreground leading-none mt-0.5">
              {{ authStore.user?.role || 'Super Admin' }}
            </span>
          </div>
          <IconChevronDown class="size-3.5 text-muted-foreground hidden md:block" :stroke-width="2" />
        </button>

        <div role="menu" aria-orientation="vertical" aria-labelledby="user-menu-btn"
          class="hs-dropdown-menu hs-dropdown-open:opacity-100 transition-[opacity,margin] duration hidden opacity-0 w-48 bg-card border border-border rounded-xl shadow-lg py-1 z-50 text-meta" >
          <div class="px-3 py-1.5 border-b border-border">
            <p class="font-semibold text-foreground truncate" :title="authStore.user?.name || 'Administrador'">{{ authStore.user?.name || 'Administrador' }}</p>
            <p class="text-micro text-muted-foreground truncate">{{ authStore.user?.email || 'admin@crmpulse.com' }}</p>
          </div>

          <div class="py-0.5">
            <router-link to="/panel"
              class="flex items-center gap-2 px-3 py-1.5 text-foreground hover:bg-muted transition">
              <IconLayoutDashboard class="size-3.5 text-muted-foreground shrink-0" :stroke-width="2" />
              <span>Panel Principal</span>
            </router-link>
          </div>

          <div class="border-t border-border pt-0.5">
            <button type="button"
              class="w-full flex items-center gap-2 px-3 py-1.5 text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-950/30 transition text-left cursor-pointer"
              @click="handleLogout">
              <IconLogout class="size-3.5 shrink-0" :stroke-width="2" />
              <span>Cerrar sesión</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>
