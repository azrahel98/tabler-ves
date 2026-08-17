import { HSStaticMethods } from 'preline'
import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{
			path: '/login',
			name: 'login',
			component: () => import('@/views/LoginView.vue'),
			meta: { requiresGuest: true },
		},
		{
			path: '/register',
			name: 'register',
			component: () => import('@/views/RegisterView.vue'),
			meta: { requiresGuest: true },
		},
		{
			path: '/',
			component: () => import('@/views/pages/main.vue'),
			meta: { requiresAuth: true },
			children: [
				{
					path: '',
					redirect: '/dashboard',
				},
				{
					path: 'dashboard',
					name: 'dashboard',
					component: () => import('@/views/DashboardView.vue'),
				},
				{
					path: 'perfil/:dni',
					name: 'perfil',
					component: () => import('@/views/PerfilView.vue'),
				},
				{
					path: 'distrito/:nombre',
					name: 'distrito',
					component: () => import('@/views/DistritoView.vue'),
				},
				{
					path: 'area/:id',
					name: 'area',
					component: () => import('@/views/AreaView.vue'),
				},
				{
					path: 'regimen/:id',
					name: 'regimen',
					component: () => import('@/views/RegimenView.vue'),
				},
				{
					path: 'sindicato/:id',
					name: 'sindicato',
					component: () => import('@/views/SindicatoView.vue'),
				},
				{
					path: 'calendario',
					name: 'calendario',
					component: () => import('@/views/CalendarioView.vue'),
				},
				{
					path: 'jubilacion',
					name: 'jubilacion',
					component: () => import('@/views/JubilacionView.vue'),
				},
				{
					path: 'comparacion-mef',
					alias: 'mef',
					name: 'comparacion-mef',
					component: () => import('@/views/ComparacionMefView.vue'),
				},
				{
					path: 'usuarios',
					name: 'usuarios',
					component: () => import('@/views/UsuariosView.vue'),
				},
				{
					path: 'documento/:id',
					alias: 'personal/documento/:id',
					name: 'documento',
					component: () => import('@/views/DocumentoView.vue'),
				},
			],
		},
		{
			path: '/:pathMatch(.*)*',
			redirect: '/dashboard',
		},
	],

	linkActiveClass: 'sidebar-activo',
})

router.beforeEach((to, _from, next) => {
	const authStore = useAuthStore()

	if (to.meta.requiresAuth && !authStore.isAuthenticated) {
		next('/login')
	} else if (to.meta.requiresGuest && authStore.isAuthenticated) {
		next('/dashboard')
	} else {
		next()
	}
})

router.afterEach((_to, _from, failure) => {
	if (!failure) {
		setTimeout(() => {
			HSStaticMethods.autoInit()
		}, 100)
	}
})

export default router
