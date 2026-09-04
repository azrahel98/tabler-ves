<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import {
  IconStack2,
  IconX,
  IconLayoutDashboard,
  IconLayoutSidebarLeftCollapse,
  IconLayoutSidebarLeftExpand,
  IconComponents,
  IconTable,
  IconCards,
  IconUser,
  IconSitemap,
} from '@tabler/icons-vue'

withDefaults(
  defineProps<{
    isOpen: boolean
    isCollapsed?: boolean
  }>(),
  {
    isCollapsed: false,
  }
)

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'toggleCollapse'): void
}>()

const route = useRoute()
const authStore = useAuthStore()
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-40 bg-neutral-900/50 backdrop-blur-xs lg:hidden transition-opacity"
    @click="emit('close')"></div>

  <aside
    class="fixed inset-y-0 inset-s-0 z-50 bg-sidebar border-e border-sidebar-line flex flex-col transition-all duration-200 ease-in-out lg:translate-x-0"
    :class="[
      isOpen ? 'translate-x-0' : '-translate-x-full',
      isCollapsed ? 'lg:w-20 w-64' : 'w-64',
    ]" aria-label="Sidebar">
    <div class="h-16 px-4 flex items-center justify-between shrink-0 border-b border-sidebar-line/40"
      :class="isCollapsed ? 'lg:px-0 lg:justify-center' : ''">
      <router-link to="/panel" class="flex items-center gap-3 overflow-hidden"
        :class="isCollapsed ? 'lg:justify-center' : ''" @click="emit('close')">
        <div
          class="size-9 rounded-xl bg-primary text-primary-foreground flex items-center justify-center font-black shrink-0 transition-opacity hover:opacity-90">
          <IconStack2 class="size-5" :stroke-width="2.2" />
        </div>
        <div v-if="!isCollapsed" class="flex flex-col min-w-0">
          <span class="text-sm font-bold tracking-tight text-foreground">CRM Pulse</span>
          <span class="text-[10px] font-medium text-muted-foreground -mt-0.5 tracking-wider uppercase">Admin
            Suite</span>
        </div>
      </router-link>

      <button v-if="!isCollapsed" type="button"
        class="hidden lg:flex p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition"
        title="Colapsar barra lateral" aria-label="Colapsar barra lateral" @click="emit('toggleCollapse')">
        <IconLayoutSidebarLeftCollapse class="size-4" :stroke-width="2" />
      </button>

      <button type="button"
        class="lg:hidden p-1.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted focus:outline-hidden"
        aria-label="Cerrar navegación" @click="emit('close')">
        <IconX class="size-5" :stroke-width="2" />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto p-3 space-y-4">
      <div v-if="isCollapsed" class="hidden lg:flex justify-center mb-1">
        <button type="button"
          class="p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition"
          title="Expandir barra lateral" aria-label="Expandir barra lateral" @click="emit('toggleCollapse')">
          <IconLayoutSidebarLeftExpand class="size-4" :stroke-width="2" />
        </button>
      </div>

      <nav class="space-y-1">
        <router-link to="/panel"
          class="flex items-center rounded-lg text-sm  transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'Dashboard' : undefined" @click="emit('close')">
          <IconLayoutDashboard class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">Dashboard</span>
        </router-link>

        <!-- <router-link to="/pruebas"
          class="flex items-center rounded-lg text-sm  transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'Componentes UI' : undefined" @click="emit('close')">
          <IconComponents class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">Componentes UI</span>
        </router-link>

        <router-link to="/tablas"
          class="flex items-center rounded-lg text-sm transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'DataTables' : undefined" @click="emit('close')">
          <IconTable class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">DataTables</span>
        </router-link>

        <router-link to="/tarjetas"
          class="flex items-center rounded-lg text-sm  transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'Cards' : undefined" @click="emit('close')">
          <IconCards class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">Cards</span>
        </router-link> -->

        <router-link to="/organigrama"
          class="flex items-center rounded-lg text-sm  transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'Organigrama' : undefined" @click="emit('close')">
          <IconSitemap class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">Organigrama</span>
        </router-link>

        <router-link :to="route.fullPath" v-if="route.path.startsWith('/perfil')"
          class="flex items-center rounded-lg text-sm  transition-colors text-sidebar-nav-foreground hover:bg-sidebar-nav-hover"
          :class="isCollapsed ? 'lg:justify-center lg:px-0 px-3 py-2.5' : 'px-3 py-2 gap-3'"
          :title="isCollapsed ? 'Perfil' : undefined" @click="emit('close')">
          <IconUser class="size-5 shrink-0" :stroke-width="1.3" />
          <span :class="isCollapsed ? 'lg:hidden truncate' : 'truncate'">Perfil</span>
        </router-link>
      </nav>

    </div>

    <div class="p-3 border-t border-sidebar-line shrink-0">
      <div class="flex items-center" :class="isCollapsed ? 'lg:justify-center' : 'gap-3'"
        :title="isCollapsed ? (authStore.user?.name || 'Administrador') : undefined">
        <img
          :src="authStore.user?.avatar || 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?q=80&w=256&auto=format&fit=crop'"
          alt="Usuario" class="size-8 rounded-full object-cover border border-border shrink-0" />
        <div :class="isCollapsed ? 'lg:hidden flex-1 min-w-0' : 'flex-1 min-w-0'">
          <p class="text-xs font-semibold text-foreground wrap-break-word">
            {{ authStore.user?.name || 'Administrador' }}
          </p>
          <p class="text-[11px] text-muted-foreground truncate">
            {{ authStore.user?.email || 'admin@crmpulse.com' }}
          </p>
        </div>
      </div>
    </div>
  </aside>
</template>
