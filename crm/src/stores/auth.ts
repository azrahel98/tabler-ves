import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  loginWithGoogleApi,
  registerWithGoogleApi,
  type AuthUser,
} from '@/services/auth'
import { tokenExpirado } from '@/utils/token'

export interface User extends AuthUser {
  name?: string
  avatar?: string
}

export const useAuthStore = defineStore('auth', () => {
  const storedToken = localStorage.getItem('crm_token')
  const initialToken = tokenExpirado(storedToken) ? null : storedToken

  if (storedToken && !initialToken) {
    localStorage.removeItem('crm_token')
    localStorage.removeItem('crm_user')
  }

  const token = ref<string | null>(initialToken)
  const user = ref<User | null>(
    initialToken && localStorage.getItem('crm_user')
      ? JSON.parse(localStorage.getItem('crm_user') as string)
      : null,
  )

  const isAuthenticated = computed(() => {
    if (!token.value) return false
    if (tokenExpirado(token.value)) {
      logout()
      return false
    }
    return true
  })

  const currentUser = computed(() => user.value)

  function setAuth(newToken: string, newUser: AuthUser) {
    const normalizedUser: User = {
      ...newUser,
      name: newUser.full_name,
      avatar: newUser.picture_url || '',
    }
    token.value = newToken
    user.value = normalizedUser

    localStorage.setItem('crm_token', newToken)
    localStorage.setItem('crm_user', JSON.stringify(normalizedUser))
  }

  async function loginWithGoogle(google_sub: string, email: string) {
    const res = await loginWithGoogleApi(google_sub, email)
    setAuth(res.token, res.user)
    return res
  }

  async function registerWithGoogle(payload: {
    google_sub: string
    email: string
    full_name: string
    picture_url?: string | null
  }) {
    return await registerWithGoogleApi(payload)
  }

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('crm_token')
    localStorage.removeItem('crm_user')
  }

  return {
    token,
    user,
    currentUser,
    isAuthenticated,
    setAuth,
    loginWithGoogle,
    registerWithGoogle,
    logout,
  }
})
