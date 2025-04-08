<template>
  <div class="sidebar-container">
    <!-- Botón hamburguesa para móvil -->
    <button class="mobile-toggle" @click="toggle()" aria-label="Abrir menú">
      <IconMenu2 size="24" />
    </button>

    <aside ref="sidebar" :class="{ sidebar: true, expanded: isExpanded }">
      <div class="sidebar-header">
        <transition name="fade-slide">
          <img v-if="isExpanded" src="https://preview.tabler.io/static/logo-white.svg" alt="Logo" class="logo" />
        </transition>
        <button @click.stop="toggle()" class="toggle-btn" aria-label="Toggle Sidebar">
          <IconChevronLeft v-if="isExpanded" size="18" />
          <IconChevronRight v-else size="18" />
        </button>
      </div>

      <div class="sidebar-content">
        <nav class="sidebar-nav" aria-label="Main Navigation">
          <RouterLink :to="{ path: '/' }" class="nav-item" :class="{ active: $route.path === '/' }" :title="!isExpanded ? 'Dashboard' : ''">
            <div class="nav-icon">
              <IconDashboard size="20" />
            </div>
            <transition name="fade-slide">
              <span v-if="isExpanded">Dashboard</span>
            </transition>
          </RouterLink>

          <RouterLink :to="{ path: '/buscar' }" class="nav-item" :class="{ active: $route.path === '/buscar' }" :title="!isExpanded ? 'Buscar' : ''">
            <div class="nav-icon">
              <IconSearch size="20" />
            </div>
            <transition name="fade-slide">
              <span v-if="isExpanded">Buscar</span>
            </transition>
          </RouterLink>

          <RouterLink :to="{ path: '/boleta' }" class="nav-item" :class="{ active: $route.path === '/boleta' }" :title="!isExpanded ? 'Organigrama' : ''">
            <div class="nav-icon">
              <IconFile3d size="20" />
            </div>
            <transition name="fade-slide">
              <span v-if="isExpanded">Organigrama</span>
            </transition>
          </RouterLink>

          <RouterLink v-if="store.lvl == 1" :to="{ path: '/add' }" class="nav-item" :class="{ active: $route.path === '/add' }" :title="!isExpanded ? 'Añadir' : ''">
            <div class="nav-icon">
              <IconUsersPlus size="20" />
            </div>
            <transition name="fade-slide">
              <span v-if="isExpanded">Añadir</span>
            </transition>
          </RouterLink>

          <div v-if="router.currentRoute.value.name === 'perfil'" class="nav-item active" :title="!isExpanded ? 'Perfil' : ''">
            <div class="nav-icon">
              <IconUsers size="20" />
            </div>
            <transition name="fade-slide">
              <span v-if="isExpanded">Perfil</span>
            </transition>
          </div>
        </nav>
      </div>

      <div class="sidebar-footer">
        <button class="nav-item logout-item" @click="store.logout()" :title="!isExpanded ? 'Salir' : ''">
          <div class="nav-icon">
            <IconLogout size="20" />
          </div>
          <transition name="fade-slide">
            <span v-if="isExpanded">Salir</span>
          </transition>
        </button>
      </div>
    </aside>

    <div class="sidebar-overlay" :class="{ active: isExpanded }" @click="toggle()"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRoute } from 'vue-router'
import { IconChevronLeft, IconChevronRight, IconDashboard, IconUsers, IconSearch, IconFile3d, IconUsersPlus, IconLogout, IconMenu2 } from '@tabler/icons-vue'
import { router } from '@router/router'
import { userStore } from '../store/user'

const store = userStore()
const $route = useRoute()
const sidebar = ref<HTMLElement | null>(null)
const isMobile = ref(false)

const props = defineProps({
  isExpanded: { type: Boolean, required: false, default: false },
  toggle: { type: Function, required: true }
})

const checkMobile = () => {
  isMobile.value = window.innerWidth < 768
}

const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.isExpanded) {
    props.toggle()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('resize', checkMobile)
  checkMobile()
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyDown)
  window.removeEventListener('resize', checkMobile)
})
</script>

<style lang="scss" scoped>
.sidebar-container {
  position: relative;
}

.mobile-toggle {
  position: fixed;
  top: 1rem;
  left: 1rem;
  z-index: 1001;
  background-color: #1a2234;
  color: white;
  border: none;
  border-radius: 4px;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  transition: all 0.2s ease;

  &:hover {
    background-color: #2c3a57;
  }

  &:active {
    transform: scale(0.95);
  }

  @media (min-width: 768px) {
    display: none;
  }
}

.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;
  width: 64px;
  background-color: #1a2234;
  color: #ffffff;
  transition: all 0.2s cubic-bezier(0.25, 1, 0.5, 1);
  display: flex;
  flex-direction: column;
  overflow-x: hidden;
  z-index: 1000;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);

  &.expanded {
    width: 220px;
  }

  @media (max-width: 767px) {
    transform: translateX(-100%);
    width: 220px;

    &.expanded {
      transform: translateX(0);
    }
  }
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  height: 64px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);

  .logo {
    height: 32px;
    width: auto;
    margin-right: auto;
  }

  .toggle-btn {
    background: none;
    border: none;
    color: #ffffff;
    cursor: pointer;
    padding: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    border-radius: 4px;
    margin-left: auto;

    &:hover {
      background-color: rgba(255, 255, 255, 0.1);
    }

    &:focus-visible {
      outline: none;
      box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.5);
    }

    @media (max-width: 767px) {
      display: none;
    }
  }
}

.sidebar-content {
  flex-grow: 1;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.2) transparent;

  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
  }
}

.sidebar-nav {
  padding-top: 0.5rem;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 1rem;
  color: #a1a9b7;
  gap: 1rem;
  text-decoration: none;
  transition: all 0.25s ease;
  border-left: 3px solid transparent;
  margin: 0.25rem 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  position: relative;
  overflow: hidden;

  .nav-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    min-width: 24px;
    transition: transform 0.2s ease;
  }

  &:hover {
    background-color: rgba(255, 255, 255, 0.08);
    color: #ffffff;

    .nav-icon {
      transform: scale(1.1);
    }
  }

  &.active {
    background-color: rgba(32, 107, 196, 0.15);
    color: #ffffff;
    border-left-color: #206bc4;
    font-weight: 500;
  }

  span {
    white-space: nowrap;
    font-size: 0.9rem;
  }
}

.sidebar-footer {
  margin-top: auto;
  border-top: 1px solid rgba(255, 255, 255, 0.08);

  .logout-item {
    width: calc(100% - 1rem);
    background-color: transparent;

    &:hover {
      .nav-icon {
        color: #ff6b6b;
      }
    }

    .nav-icon {
      color: #ff6b6b;
    }

    span {
      font-weight: 500;
    }
  }
}

.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 999;
  visibility: hidden;
  opacity: 0;
  transition: opacity 0.3s ease, visibility 0.3s ease;
  backdrop-filter: blur(2px);

  &.active {
    visibility: visible;
    opacity: 1;
  }
}

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: all 0.3s ease;
  max-width: 150px;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  max-width: 0;
  transform: translateX(-10px);
}
</style>
