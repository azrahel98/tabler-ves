import { createRouter, createWebHistory, type NavigationGuardNext, type RouteLocationNormalized } from 'vue-router'
import LoginView from '../views/LoginView.vue'
import MainLayout from '../layouts/MainLayout.vue'
import { decodificar, useAutenticacionStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  linkActiveClass: 'menu-item-active',
  linkExactActiveClass: 'menu-item-active',
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/',
      component: MainLayout,
      meta: { requiresAuth: true },
      beforeEnter: (to, from, next) => middleware(to, from, next),
      children: [
        {
          path: '',
          redirect: '/dash',
        },
        {
          path: 'dash',
          name: 'dashboard',
          component: () => import('../views/DashboardView.vue'),
        },
        {
          path: 'personal',
          name: 'personal',
          component: () => import('../views/Personal/PersonalSearch.vue'),
        },
        {
          path: 'personal/:dni',
          name: 'personal-profile',
          component: () => import('../views/Personal/PersonalProfile.vue'),
        },
        {
          path: 'change-password',
          name: 'change-password',
          component: () => import('../views/ChangePassword.vue'),
        },
        {
          path: 'organigrama',
          name: 'organigrama',
          component: () => import('../views/OrganigramaView.vue'),
        },
        {
          path: 'sindicato',
          name: 'sindicato',
          component: () => import('../views/SindicatoView.vue'),
        },
        {
          path: 'nuevo-vinculo',
          name: 'nuevo-vinculo',
          component: () => import('../views/NuevoVinculoView.vue'),
        },
        {
          path: 'area/:id',
          name: 'area-personal',
          component: () => import('../views/reporte/AreaView.vue'),
        },
        {
          path: 'sindicato/:nombre',
          name: 'sindicato-personal',
          component: () => import('../views/reporte/SindicatoPersonalView.vue'),
        },
        {
          path: '/carga-masiva',
          name: 'carga-masiva',
          component: () => import('../views/CargaMasivaView.vue'),
          meta: { requiresAuth: true }
        },
        {
          path: '/usuarios',

          name: 'usuarios',
          component: () => import('../views/UsuariosView.vue'),
        },
        {
          path: 'comparacion-mef',
          name: 'comparacion-mef',
          component: () => import('../views/ComparacionMefView.vue'),
        },
      ],
    },
  ],
})

const middleware = async (_to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
  const store = useAutenticacionStore()
  const token = localStorage.getItem('token')
  if (token == null) {
    return next({ name: 'login' })
  }

  store.usuario = decodificar(token)

  if (!store.verificarToken()) {
    return next({ name: 'login' })
  }

  return next()
}

export default router
