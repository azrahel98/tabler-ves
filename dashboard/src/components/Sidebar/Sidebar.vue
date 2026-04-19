<template>
  <aside
    :class="sidebarToggle ? 'translate-x-0 lg:w-[90px] px-0' : '-translate-x-full px-5'"
    class="sidebar fixed left-0 top-0 z-9999 flex h-screen w-[260px] flex-col overflow-y-hidden border-r border-gray-100 bg-card dark:border-white/6 dark:bg-gray-950 lg:static lg:translate-x-0">
    <div :class="sidebarToggle ? 'justify-center' : 'justify-start px-4'" class="flex items-center gap-2 pt-8 sidebar-header pb-7">
      <router-link to="/">
        <span class="logo flex items-center gap-2">
          <img class="dark:hidden" :class="sidebarToggle ? 'hidden' : 'block'" src="/logo-icon.svg" alt="Logossdfads" />
          <img class="hidden dark:block" :class="sidebarToggle ? 'hidden' : 'block'" src="/logo-icon.svg" alt="Logo" />

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
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <Users class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Personal</span>
              </router-link>
            </li>

            <li>
              <router-link
                to="/organigrama"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <Network class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Organigrama</span>
              </router-link>
            </li>

            <li v-if="esAdmin">
              <router-link
                to="/sindicato"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <Shield class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Sindicatos</span>
              </router-link>
            </li>

            <li v-if="esAdmin">
              <router-link
                to="/nuevo-vinculo"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <UserPlus class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Nuevo Vínculo</span>
              </router-link>
            </li>

            <li v-if="esAdmin">
              <router-link
                to="/usuarios"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <Settings class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Usuarios</span>
              </router-link>
            </li>

            <li v-if="esAdmin">
              <router-link
                to="/comparacion-mef"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <FileSpreadsheet class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Comparar MEF</span>
              </router-link>
            </li>

            <li v-if="esAdmin">
              <router-link
                to="/carga-masiva"
                class="relative flex items-center gap-2.5 rounded-sm py-2 menu-item-inactive"
                :class="sidebarToggle ? 'justify-center px-0' : 'px-4'"
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
                <FileUp class="size-5" />
                <span class="text-sm" :class="sidebarToggle ? 'lg:hidden' : ''">Carga Masiva</span>
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
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
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
                active-class="bg-primary/10 text-primary dark:bg-primary/20 dark:text-brand-300">
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
  import { LayoutDashboard, Users, KeyRound, Network, Shield, UserPlus, Settings, FileSpreadsheet, FileUp } from 'lucide-vue-next'
  import { storeToRefs } from 'pinia'
  import router from '../../router'
  import { useAutenticacionStore } from '../../stores/auth'

  const configuracionStore = useConfiguracionStore()
  const { menuLateralAbierto: sidebarToggle } = storeToRefs(configuracionStore)

  const user = useAutenticacionStore()
  const { esAdmin } = storeToRefs(user)
</script>
