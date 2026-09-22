import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import DashboardLayout from '@/components/DashboardLayout.vue'
const DashboardView = () => import('@/views/DashboardView.vue')
const test = () => import('@/views/dashboard/test.vue')
const DataTablesGalleryView = () => import('@/views/dashboard/DataTablesGalleryView.vue')
const CardsGalleryView = () => import('@/views/dashboard/CardsGalleryView.vue')
const PerfilView = () => import('@/views/PerfilView.vue')
const OrganigramaView = () => import('@/views/OrganigramaView.vue')
const LoginView = () => import('@/views/LoginView.vue')
const NuevoTrabajadorView = () => import('@/views/NuevoTrabajadorView.vue')
const Alerta70View = () => import('@/views/Alerta70View.vue')

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
      {
        path: 'personal/nuevo',
        alias: 'nuevo-trabajador',
        name: 'nuevo-trabajador',
        component: NuevoTrabajadorView,
        meta: { requiresAuth: true },
      },
      {
        path: 'alerta-70',
        alias: ['jubilacion-70', 'personal/alerta-70'],
        name: 'alerta-70',
        component: Alerta70View,
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

  if (to.meta.requiresAuth) {
    if (!authStore.checkAuth()) {
      next({ name: 'iniciar-sesion' })
      return
    }
    next()
    return
  }

  if (to.name === 'iniciar-sesion' || to.path === '/login' || to.path === '/iniciar-sesion') {
    if (authStore.checkAuth()) {
      next({ name: 'panel' })
      return
    }
    next()
    return
  }

  next()
})

