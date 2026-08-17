import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { apiClient } from '@/api/client'
import type { GoogleRegisterPayload, LoginResponse, RegisterResponse, User } from '@/api/types'
import router from '@/router'

export const useAuthStore = defineStore('auth', () => {
	const token = ref<string | null>(localStorage.getItem('auth_token'))
	const currentUser = ref<User | null>(
		localStorage.getItem('auth_user') ? JSON.parse(localStorage.getItem('auth_user')!) : null,
	)
	const isLoading = ref<boolean>(false)
	const error = ref<string | null>(null)

	const isAuthenticated = computed(() => !!token.value)

	async function loginWithGoogle(googleSub: string, email: string): Promise<LoginResponse> {
		isLoading.value = true
		error.value = null

		try {
			const res = await apiClient.post<LoginResponse>('/login/', {
				google_sub: googleSub,
				email,
			})

			if (res.user.status === 'APPROVED') {
				token.value = res.token
				currentUser.value = res.user
				localStorage.setItem('auth_token', res.token)
				localStorage.setItem('auth_user', JSON.stringify(res.user))
				router.push('/dashboard')
			}

			return res
		} catch (err: any) {
			error.value = err.error || 'Error al iniciar sesión'
			throw err
		} finally {
			isLoading.value = false
		}
	}

	async function registerWithGoogle(payload: GoogleRegisterPayload): Promise<RegisterResponse> {
		isLoading.value = true
		error.value = null

		try {
			return await apiClient.post<RegisterResponse>('/login/register', payload)
		} catch (err: any) {
			error.value = err.error || 'Error en el registro'
			throw err
		} finally {
			isLoading.value = false
		}
	}

	function logout() {
		token.value = null
		currentUser.value = null
		localStorage.removeItem('auth_token')
		localStorage.removeItem('auth_user')
		router.push({ name: 'login' })
	}

	return {
		token,
		currentUser,
		isLoading,
		error,
		isAuthenticated,
		loginWithGoogle,
		registerWithGoogle,
		logout,
	}
})
