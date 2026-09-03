import { ofetch } from 'ofetch'
import { router } from '@/router'
import { tokenExpirado } from '@/utils/token'

export function getApiBaseUrl(): string {
  const url = import.meta.env.VITE_API_URL || import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
  return url.replace(/\/+$/, '')
}

function cerrarSesionYRedirigir() {
  localStorage.removeItem('crm_token')
  localStorage.removeItem('crm_user')
  if (router.currentRoute.value?.name !== 'iniciar-sesion') {
    router.push({ name: 'iniciar-sesion' })
  }
}

export const api = ofetch.create({
  baseURL: getApiBaseUrl(),
  onRequest({ options }) {
    const token = localStorage.getItem('crm_token')
    if (token) {
      if (tokenExpirado(token)) {
        cerrarSesionYRedirigir()
        throw new Error('Sesión expirada')
      }
      const headers = new Headers(options.headers)
      headers.set('token', token)
      options.headers = headers
    }
  },
  onResponseError({ response }) {
    if (response.status === 401) {
      cerrarSesionYRedirigir()
    }
    const errorData = response._data
    const message = errorData?.error || errorData?.message || response.statusText || 'Error en la petición'
    throw new Error(message)
  },
})
