import { createRouter, createWebHistory, type NavigationGuardNext, type RouteLocationNormalized, type RouteRecordRaw } from 'vue-router'

import { userStore } from '@store/user'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@views/login.vue')
  },
  {
    path: '/',
    name: 'home',
    component: () => import('@views/dash.vue'),
    beforeEnter: (to, from, next) => middleware(to, from, next),
    children: [
      {
        path: '/',
        name: 'dashboard',
        component: () => import('@views/app/main.vue')
      },
      {
        path: '/chat',
        name: 'chat',
        component: () => import('@views/app/chat.vue')
      },
      // {
      //   path: '/organigrama',
      //   name: 'organigrama',
      //   component: () => import('@views/dash/organigrama.vue')
      // },
      {
        path: '/personal/:dni',
        name: 'personal',
        component: () => import('@views/app/personal.vue')
      },
      {
        path: '/nuevo',
        name: 'nuevo-usuario',
        component: () => import('@views/app/nuevo_usuario.vue')
      }
    ]
  }
]

const middleware = async (_to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
  const store = userStore()
  const token = localStorage.getItem('jwt')
  if (token == null) {
    return next({
      name: 'login'
    })
  }
  store.create(token)

  return next()
}

export const router = createRouter({
  history: createWebHistory(),
  routes,
  linkExactActiveClass: 'bg-primary/10 text-primary font-bold underline',
  linkActiveClass: ' '
})
