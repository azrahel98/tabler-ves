import { ofetch } from 'ofetch'
import { isTokenValid } from '@/utils/jwt'

export function getApiBaseUrl(): string {
  const url = import.meta.env.VITE_API_URL || import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
  return url.replace(/\/+$/, '')
}

export function getApiToken(): string | null {
  if (typeof window === 'undefined') return null
  const token = localStorage.getItem('crm_token')
  if (!token || !isTokenValid(token)) return null
  return token
}

const imageBlobCache = new Map<string, string>()

export async function fetchAuthBlob(url: string): Promise<Blob> {
  const token = getApiToken()
  const headers: Record<string, string> = {}
  if (token) {
    headers['token'] = token
  }
  const res = await fetch(url, { headers })
  if (!res.ok) {
    if (res.status === 401 || res.status === 403) {
      handleUnauthorized()
    }
    throw new Error(`Error HTTP ${res.status}`)
  }
  return await res.blob()
}

export async function fetchAuthBlobUrl(url: string): Promise<string> {
  if (imageBlobCache.has(url)) {
    return imageBlobCache.get(url)!
  }
  const blob = await fetchAuthBlob(url)
  const blobUrl = URL.createObjectURL(blob)
  imageBlobCache.set(url, blobUrl)
  return blobUrl
}

export async function openProtectedFile(
  url: string,
  filename?: string,
  mode: '_blank' | 'download' = '_blank'
): Promise<void> {
  const blob = await fetchAuthBlob(url)
  const isPdf =
    (filename && filename.toLowerCase().endsWith('.pdf')) ||
    !blob.type ||
    blob.type === 'application/octet-stream'
  const finalBlob =
    isPdf && blob.type !== 'application/pdf'
      ? new Blob([blob], { type: 'application/pdf' })
      : blob
  const blobUrl = URL.createObjectURL(finalBlob)

  if (mode === 'download') {
    const a = document.createElement('a')
    a.href = blobUrl
    a.download = filename || 'documento.pdf'
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    setTimeout(() => URL.revokeObjectURL(blobUrl), 30000)
  } else {
    window.open(blobUrl, '_blank')
    setTimeout(() => URL.revokeObjectURL(blobUrl), 60000)
  }
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

