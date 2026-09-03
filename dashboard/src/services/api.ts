import { ofetch } from 'ofetch'
import { isTokenValid } from '@/utils/jwt'

export function getApiBaseUrl(): string {
  const url = import.meta.env.VITE_API_URL || import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
  return url.replace(/\/+$/, '')
}

function handleUnauthorized() {
  localStorage.removeItem('crm_token')
  localStorage.removeItem('crm_user')
  if (typeof window !== 'undefined') {
    const path = window.location.pathname
    if (path !== '/iniciar-sesion' && path !== '/login') {
      window.location.href = '/iniciar-sesion'
    }
  }
}

export const api = ofetch.create({
  baseURL: getApiBaseUrl(),
  onRequest({ options }) {
    const token = localStorage.getItem('crm_token')
    if (token) {
      if (!isTokenValid(token)) {
        handleUnauthorized()
        return
      }
      const headers = new Headers(options.headers)
      headers.set('token', token)
      options.headers = headers
    }
  },
  onResponseError({ response }) {
    const errorData = response._data
    if (response.status === 401 || response.status === 403 || errorData?.code === 401) {
      handleUnauthorized()
    }
    const message = errorData?.error || errorData?.message || response.statusText || 'Error en la petición'
    throw new Error(message)
  },
})

