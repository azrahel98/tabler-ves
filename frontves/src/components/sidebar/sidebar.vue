<template>
  <header class="navbar navbar-expand-md d-print-none">
    <div class="container-xl">
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbar-menu"
        aria-controls="navbar-menu"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <IconMenu4 class="navbar-toggler-icon" />
      </button>

      <div class="navbar-brand navbar-brand-autodark d-none-navbar-horizontal pe-0 pe-md-3">
        <a href="." aria-label="Tabler"
          ><svg width="64" height="64" viewBox="0 0 100 100" class="navbar-brand-image" xmlns="http://www.w3.org/2000/svg">
            <circle cx="50" cy="50" r="48" fill="#0d6efd" />
            <path d="M35 60 L50 40 L65 60 V75 H35 Z" fill="white" />
            <g transform="translate(50,50) rotate(-30)">
              <line x1="0" y1="-30" x2="0" y2="-20" stroke="#28a745" stroke-width="3" />
              <line x1="0" y1="30" x2="0" y2="20" stroke="#28a745" stroke-width="3" />
              <line x1="30" y1="0" x2="20" y2="0" stroke="#28a745" stroke-width="3" />
              <line x1="-30" y1="0" x2="-20" y2="0" stroke="#28a745" stroke-width="3" />
              <line x1="21" y1="-21" x2="14" y2="-14" stroke="#28a745" stroke-width="3" />
              <line x1="-21" y1="-21" x2="-14" y2="-14" stroke="#28a745" stroke-width="3" />
            </g></svg
        ></a>
      </div>
      <div class="navbar-nav flex-row order-md-last">
        <Notificaciones v-if="false" />
        <div class="nav-item dropdown">
          <a href="#" class="nav-link d-flex lh-1 p-0 px-2" data-bs-toggle="dropdown" aria-label="Open user menu" aria-expanded="false">
            <span class="avatar avatar-sm"> </span>
            <div class="d-none d-xl-block ps-2">
              <div>Raul Chercca Lopez</div>
              <div class="mt-1 small text-secondary">Usuario</div>
            </div>
          </a>
          <div class="dropdown-menu dropdown-menu-end dropdown-menu-arrow">
            <a class="dropdown-item">Profile</a>
            <a class="dropdown-item">Feedback</a>
            <div class="dropdown-divider"></div>
            <a class="dropdown-item" @click="logout">Logout</a>
          </div>
        </div>
      </div>
      <div class="collapse navbar-collapse" id="navbar-menu">
        <ul class="navbar-nav">
          <RouterLink to="/" class="nav-item">
            <div class="nav-link">
              <span class="nav-link-icon d-md-none d-lg-inline-block">
                <IconHome class="icon" />
              </span>
              <span class="nav-link-title"> Dashboard </span>
            </div>
          </RouterLink>

          <RouterLink to="/buscar" class="nav-item">
            <div class="nav-link">
              <span class="nav-link-icon d-md-none d-lg-inline-block">
                <IconMoodSearch class="icon" />
              </span>
              <span class="nav-link-title fw-medium"> Buscar </span>
            </div>
          </RouterLink>

          <li class="nav-item">
            <RouterLink to="/organigrama" class="nav-link">
              <span class="nav-link-icon d-md-none d-lg-inline-block">
                <IconBuildingArch class="icon" />
              </span>
              <span class="nav-link-title fw-medium"> Organigrama </span>
            </RouterLink>
          </li>

          <li class="nav-item dropdown">
            <a class="nav-link dropdown-toggle" href="#navbar-help" data-bs-toggle="dropdown" data-bs-auto-close="outside" role="button" aria-expanded="false">
              <span class="nav-link-icon d-md-none d-lg-inline-block">
                <IconUsersPlus class="icon" />
              </span>
              <span class="nav-link-title"> Nuevo </span>
            </a>
            <div class="dropdown-menu">
              <a class="dropdown-item" target="_blank" rel="noopener">Decreto Legislativo 1057 - inde</a>
              <a class="dropdown-item">Decreto Legislativo 1057 - tran</a>
              <a class="dropdown-item">Decreto Legislativo 728</a>
              <a class="dropdown-item">Decreto Legislativo 276</a>
              <a class="dropdown-item">Ley de Servicio Civil</a>
              <a class="dropdown-item">Practicante</a>
            </div>
          </li>
        </ul>
      </div>
    </div>
  </header>
</template>
<script setup lang="ts">
import { IconBuildingArch, IconHome, IconMenu4, IconMoodSearch, IconUsersPlus } from '@tabler/icons-vue'
import Notificaciones from './notificaciones.vue'
import { router } from '@router/router'

const logout = () => {
  localStorage.clear()
  router.go(0)
}
</script>
