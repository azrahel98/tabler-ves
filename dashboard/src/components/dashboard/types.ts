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

export interface EventoVinculo {
  id: number
  tipo_evento: string
  estado: 'activo' | 'desactivado' | null
  nombre: string
  dni: string
  area_original: string
  area_nueva: string | null
  cargo: string
  cargo_nuevo: string | null
  fecha_inicio: string | null
  fecha_salida: string | null
  fecha_desde: string | null
  fecha_hasta: string | null
  descripcion_inicio?: string | null
  descripcion_salida?: string | null
  avatar?: string | null
}

export function resolveAvatarUrl(dni?: string | null): string | null {
  if (!dni) return null
  const baseUrl = getApiBaseUrl()
 return `${baseUrl}/personal/avatar/${dni}`
}

