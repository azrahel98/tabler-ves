import { api, getApiBaseUrl } from './api'

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

export interface DistritoReport {
  distrito: string
  cantidad: number
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

export interface Alerta70 {
  dni: string
  nombre: string
  nacimiento: string
  edad_actual: number
  fecha_70_anos: string
  fecha_limite_mes: string
  fecha_extension_fin_ano: string
  dias_para_70: number
  dias_para_cese_mes: number
  dias_para_cese_extension: number
  estado_alerta: string
  area: string
  cargo: string
  regimen: string
  plaza: string
  avatar?: string
}

export function resolveAvatarUrl(avatar?: string | null): string | null {
  if (!avatar) return null
  if (avatar.startsWith('http://') || avatar.startsWith('https://')) return avatar
  const baseUrl = getApiBaseUrl()
  const cleanPath = avatar.startsWith('/') ? avatar : `/${avatar}`
  return `${baseUrl}${cleanPath}`
}

export async function fetchResumenPersonal(): Promise<ResumenPersonal> {
  return await api<ResumenPersonal>('/api/dash/resumen')
}

export async function fetchAreaReport(): Promise<AreaReport[]> {
  try {
    return await api<AreaReport[]>('/api/dash/areareport')
  } catch {
    return []
  }
}

export async function fetchRangosEdad(): Promise<RangoReport[]> {
  try {
    return await api<RangoReport[]>('/api/dash/rangos_edad')
  } catch {
    return []
  }
}

export async function fetchRangosAntiguedad(): Promise<RangoReport[]> {
  try {
    return await api<RangoReport[]>('/api/dash/rangos_antiguedad')
  } catch {
    return []
  }
}

export async function fetchCumpleanos(): Promise<Cumpleanero[]> {
  try {
    return await api<Cumpleanero[]>('/api/dash/cumpleanos')
  } catch {
    return []
  }
}

export async function fetchTrabajadoresNuevos(): Promise<TrabajadorNuevo[]> {
  try {
    return await api<TrabajadorNuevo[]>('/api/dash/trabajadores_nuevos')
  } catch {
    return []
  }
}

export async function fetchAlertas70(): Promise<Alerta70[]> {
  try {
    return await api<Alerta70[]>('/api/dash/alerta_70')
  } catch {
    return []
  }
}

export interface OrganigramaSubgerencia {
  id: number
  area: string
  jefe?: string | null
  dni?: string | null
}

export interface OrganigramaItem {
  id: number
  area: string
  jefe?: string | null
  dni?: string | null
  subgerencias?: OrganigramaSubgerencia[]
}

export async function fetchOrganigrama(): Promise<OrganigramaItem[]> {
  try {
    return await api<OrganigramaItem[]>('/api/dash/organigrama')
  } catch {
    return []
  }
}
