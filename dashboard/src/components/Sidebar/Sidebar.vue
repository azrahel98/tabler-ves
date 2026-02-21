<template>
  <aside
    :class="sidebarToggle ? 'translate-x-0 lg:w-[90px] px-0' : '-translate-x-full px-5'"
    class="sidebar fixed left-0 top-0 z-9999 flex h-screen w-[260px] flex-col overflow-y-hidden border-r border-gray-200 bg-white dark:border-gray-800 dark:bg-black lg:static lg:translate-x-0 transition-all duration-300">
    <div :class="sidebarToggle ? 'justify-center' : 'justify-start px-4'" class="flex items-center gap-2 pt-8 sidebar-header pb-7">
      <router-link to="/">
        <span class="logo flex items-center gap-2">
          <img class="dark:hidden" :class="sidebarToggle ? 'hidden' : 'block'" src="/logo.svg" alt="Logossdfads" />
          <img class="hidden dark:block" :class="sidebarToggle ? 'hidden' : 'block'" src="/logo-dark.svg" alt="Logo" />

          <img class="logo-icon w-8 h-8" :class="sidebarToggle ? 'block' : 'hidden'" src="/logo-icon.svg" alt="Logo" />
        </span>
      </router-link>
    </div>

    <div class="flex flex-col overflow-y-auto duration-300 mx-0 px-3 ease-linear no-scrollbar">
      <nav class="mt-5 py-2 lg:mt-9 px-0">
        <div>
          <h3 class="mb-4 text-xs font-semibold text-gray-500 uppercase tracking-wider" :class="sidebarToggle ? 'lg:hidden' : ''">MENU</h3>

          <ul class="mb-6 flex flex-col gap-1.5">
            <li>
              <router-link to="/dash" class="flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive" :class="sidebarToggle ? 'justify-center px-0' : 'px-4'">
                <LayoutDashboard class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Dashboard</span>
              </router-link>
            </li>

            <li>
              <router-link
                to="/personal"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-gray-100 dark:bg-gray-700 text-primary dark:text-white">
                <Users class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Personal</span>
              </router-link>
            </li>

            <li>
              <router-link
                v-if="router.currentRoute.value.name == 'personal-profile'"
                :to="{
                  name: 'personal-profile',
                  params: {
                    dni: `${router.currentRoute.value.params.dni}`,
                  },
                }"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-gray-100 dark:bg-gray-700 text-primary dark:text-white">
                <Users class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Perfil</span>
              </router-link>
            </li>
          </ul>
        </div>

        <div>
          <h3 class="mb-4 text-xs font-semibold text-gray-500 uppercase tracking-wider" :class="sidebarToggle ? 'lg:hidden' : ''">CONFIGURACIÓN</h3>

          <ul class="mb-6 flex flex-col gap-1.5">
            <li>
              <router-link
                to="/change-password"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-gray-100 dark:bg-gray-700 text-primary dark:text-white">
                <KeyRound class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Contraseña</span>
              </router-link>
            </li>
          </ul>
        </div>
      </nav>
    </div>
  </aside>
</template>

<script setup lang="ts">
  import { useConfiguracionStore } from '../../stores/layout'
  import { LayoutDashboard, Users, KeyRound } from 'lucide-vue-next'
  import { storeToRefs } from 'pinia'
  import router from '../../router'

  const configuracionStore = useConfiguracionStore()
  const { menuLateralAbierto: sidebarToggle } = storeToRefs(configuracionStore)
</script>
