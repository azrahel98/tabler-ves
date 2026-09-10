import { api, getApiBaseUrl } from '@/services/api'

export interface Notificacion {
  id: number
  tipo: string
  titulo: string
  mensaje: string
  avatar?: string | null
  enlace?: string | null
  leido: boolean
  metadata?: Record<string, unknown> | null
  created_at: string
}

export interface NotificacionesListResponse {
  notificaciones: Notificacion[]
  no_leidas: number
}

export interface MarcarLeidaResponse {
  status: string
  mensaje: string
  id: number
}

export interface MarcarTodasLeidasResponse {
  status: string
  mensaje: string
  actualizadas: number
}

export async function fetchNotificaciones(limit = 30): Promise<NotificacionesListResponse> {
  return await api<NotificacionesListResponse>('/notificaciones', {
    method: 'GET',
    params: { limit },
  })
}

export async function marcarNotificacionLeida(id: number): Promise<MarcarLeidaResponse> {
  return await api<MarcarLeidaResponse>(`/notificaciones/${id}/leer`, {
    method: 'PUT',
  })
}

export async function marcarTodasNotificacionesLeidas(): Promise<MarcarTodasLeidasResponse> {
  return await api<MarcarTodasLeidasResponse>('/notificaciones/leer-todas', {
    method: 'PUT',
  })
}

export function resolveNotificationAvatar(avatar?: string | null): string {
  if (!avatar) return ''
  if (avatar.startsWith('http://') || avatar.startsWith('https://') || avatar.startsWith('data:')) {
    return avatar
  }
  const baseUrl = getApiBaseUrl()
  const cleanPath = avatar.startsWith('/') ? avatar : `/${avatar}`
  return `${baseUrl}${cleanPath}`
}

export function resolveNotificationLink(enlace?: string | null): string {
  if (!enlace) return ''
  if (enlace.startsWith('/personal/')) {
    return enlace.replace('/personal/', '/perfil/')
  }
  return enlace
}

export function getNotificationStreamUrl(token?: string | null): string {
  const baseUrl = getApiBaseUrl()
  const baseEndpoint = `${baseUrl}/notificaciones/stream`
  if (!token) return baseEndpoint
  const separator = baseEndpoint.includes('?') ? '&' : '?'
  return `${baseEndpoint}${separator}token=${encodeURIComponent(token)}`
}
