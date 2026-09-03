import { ofetch } from 'ofetch'

export function getApiBaseUrl(): string {
  const url = import.meta.env.VITE_API_URL || import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
  return url.replace(/\/+$/, '')
}

export const api = ofetch.create({
  baseURL: getApiBaseUrl(),
  onRequest({ options }) {
    const token = localStorage.getItem('crm_token')
    if (token) {
      const headers = new Headers(options.headers)
      headers.set('token', token)
      options.headers = headers
    }
  },
  onResponseError({ response }) {
    const errorData = response._data
    const message = errorData?.error || errorData?.message || response.statusText || 'Error en la petición'
    throw new Error(message)
  },
})
