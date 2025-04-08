import {
  createRouter,
  createWebHistory,
  type NavigationGuardNext,
  type RouteLocationNormalized,
  type RouteRecordRaw
} from 'vue-router'


import { userStore } from '../store/user'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@views/login.vue'),
  },
  {
    path: '/',
    name: 'home',
    component: () => import('@views/dashboard.vue'),
    beforeEnter: (to, from, next) => middleware(to, from, next),
    children: [
      {
        path: '/',
        name: 'dashboard',
        component: () => import('@views/dash/dash.vue')
      },
      {
        path: '/buscar',
        name: 'buscar',
        component: () => import('@views/dash/buscar.vue')
      },
   
    ]
  },
]


const middleware = async (
  _to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) => {
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
  linkExactActiveClass: 'active',
  linkActiveClass: 'active'
})