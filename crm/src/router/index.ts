import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import DashboardLayout from '@/components/DashboardLayout.vue'
import DashboardView from '@/views/DashboardView.vue'
import test from '@/views/dashboard/test.vue'
import DataTablesGalleryView from '@/views/dashboard/DataTablesGalleryView.vue'
import CardsGalleryView from '@/views/dashboard/CardsGalleryView.vue'
import PerfilView from '@/views/PerfilView.vue'
import OrganigramaView from '@/views/OrganigramaView.vue'
import LoginView from '@/views/LoginView.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/iniciar-sesion',
    alias: '/login',
    name: 'iniciar-sesion',
    component: LoginView,
  },
  {
    path: '/',
    component: DashboardLayout,
    children: [
      {
        path: '',
        redirect: '/panel',
      },
      {
        path: 'panel',
        alias: 'dashboard',
        name: 'panel',
        component: DashboardView,
        meta: { requiresAuth: true },
      },
      {
        path: 'pruebas',
        alias: 'test',
        name: 'pruebas',
        component: test,
        meta: { requiresAuth: true },
      },
      {
        path: 'tablas',
        alias: 'datatables',
        name: 'tablas',
        component: DataTablesGalleryView,
        meta: { requiresAuth: true },
      },
      {
        path: 'tarjetas',
        alias: 'cards',
        name: 'tarjetas',
        component: CardsGalleryView,
        meta: { requiresAuth: true },
      },
      {
        path: 'perfil/:dni?',
        alias: ['perfilview', 'perfilview/:dni?'],
        name: 'perfil',
        component: PerfilView,
        meta: { requiresAuth: true },
      },
      {
        path: 'organigrama',
        name: 'organigrama',
        component: OrganigramaView,
        meta: { requiresAuth: true },
      },
    ],
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/panel',
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
  linkActiveClass: 'bg-sidebar-nav-active',
  sensitive: true,
})

router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore()

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next({ name: 'iniciar-sesion' })
  } else if ((to.name === 'iniciar-sesion' || to.path === '/login' || to.path === '/iniciar-sesion') && authStore.isAuthenticated) {
    next({ name: 'panel' })
  } else {
    next()
  }
})

export default router
