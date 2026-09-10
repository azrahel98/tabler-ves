import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/services/api'
import { isTokenValid } from '@/utils/jwt'

export interface AuthUser {
  id: number
  google_sub: string
  email: string
  full_name: string
  picture_url?: string | null
  role: string
  status: 'APPROVED' | 'PENDING' | 'REJECTED' | string
  created_at?: string
  updated_at?: string
}

export interface LoginResponse {
  token: string
  user: AuthUser
}

export interface RegisterResponse {
  id: number
  message: string
}

export interface User extends AuthUser {
  name?: string
  avatar?: string
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('crm_token'))
  const user = ref<User | null>(
    localStorage.getItem('crm_user')
      ? JSON.parse(localStorage.getItem('crm_user') as string)
      : null,
  )

  const isAuthenticated = computed(() => {
    return !!token.value && isTokenValid(token.value)
  })

  function logout() {
    token.value = null
    user.value = null
    localStorage.removeItem('crm_token')
    localStorage.removeItem('crm_user')
  }

  function checkAuth(): boolean {
    if (!token.value || !isTokenValid(token.value)) {
      logout()
      return false
    }
    return true
  }

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

  async function loginWithGoogle(google_sub: string, email: string): Promise<LoginResponse> {
    const res = await api<LoginResponse>('/login/', {
      method: 'POST',
      body: { google_sub, email },
    })
    setAuth(res.token, res.user)
    return res
  }

  async function registerWithGoogle(payload: {
    google_sub: string
    email: string
    full_name: string
    picture_url?: string | null
  }): Promise<RegisterResponse> {
    return await api<RegisterResponse>('/login/register', {
      method: 'POST',
      body: payload,
    })
  }

  if (token.value && !isTokenValid(token.value)) {
    logout()
  }

  return {
    token,
    user,
    isAuthenticated,
    checkAuth,
    setAuth,
    loginWithGoogle,
    registerWithGoogle,
    logout,
  }
})
