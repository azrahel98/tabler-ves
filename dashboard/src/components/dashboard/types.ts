import { getApiBaseUrl } from '@/services/api'

export interface ResumenPersonal {
  total: number
  activos: number
  por_regimen: { cantidad: number; nombre: string }[]
  por_sexo: { cantidad: number; nombre: string }[]
  por_sindicato: { cantidad: number; nombre: string }[]
}

export interface Cumpleanero {
  dni: string
  nombre: string
  nacimiento: string
  edad: number
  avatar?: string
  regimen: string
}

export interface AreaReport {
  cantidad: number
  nombre: string
}

export interface RangoReport {
  cantidad: number
  nombre: string
}

export interface TrabajadorNuevo {
  id: number
  dni: string
  nombre: string
  ingreso: string
  documento: string
  area: string
  cargo: string
  regimen: string
  sueldo: number
  plaza: string
  avatar?: string
}

export interface TrabajadorRenuncia {
  id: number
  dni: string
  nombre: string
  fecha: string
  cargo: string
  area: string
  codigo: string
  avatar?: string
}

export function resolveAvatarUrl(avatar?: string | null): string | null {
  if (!avatar) return null
  if (avatar.startsWith('http://') || avatar.startsWith('https://')) return avatar
  const baseUrl = getApiBaseUrl()
  const cleanPath = avatar.startsWith('/') ? avatar : `/${avatar}`
  return `${baseUrl}${cleanPath}`
}
