import { api, getApiBaseUrl } from './api'

export interface PersonalPerfil {
  dni: string
  nombre: string
  telf: string | null
  direccion: string | null
  email: string | null
  ruc: string | null
  nacimiento: string | null
  sexo: string | null
  region: string | null
  distrito: string | null
}

export interface PersonalBanco {
  id: number
  numero_cuenta: string
  tipo_cuenta: string
  cci: string
  banco: string
  estado: number
  dni?: string
}

export interface PersonalGrado {
  id: number
  profesion: string
  universidad: string
  nivel_academico: string
  abrv: string
  dni: string
  fecha: string
}

export interface PersonalContacto {
  persona_dni: string
  nombre: string
  relacion: string
  telefono: string
}

export interface PersonalVinculo {
  id: number
  dni: string
  area: string
  cargo: string
  regimen: string
  sueldo: number
  codigo: string
  estado: string
  fecha_ingreso: string
  fecha_salida: string | null
  sindicato: string | null
  tipo_evento: string | null
  estado_evento: string | null
  id_evento: number | null
  doc_ingreso_id?: number | null
  doc_ingreso?: string | null
  numero_doc_ingreso?: string | null
  descrip_ingreso?: string | null
  cargo_estructural?: string | null
  grupo_ocupacional?: string | null
  doc_salida_id?: number | null
  doc_salida?: string | null
  descrip_salida?: string | null
  numero_doc_salida?: string | null
  doc_evento_id?: number | null
  doc_evento_tipo?: string | null
  numero_doc_evento?: string | null
  fecha_evento?: string | null
}

export interface PersonalArchivo {
  id: number
  documento_id: number | null
  dni_asociado: string
  original_name: string
  file_hash: string
  extension: string
  external_url: string | null
  usuario_subida: string
  fecha_subida: string
  size?: string
  access?: string
  access_members?: number
}

export interface PersonalDocumento {
  id: number
  sigla: string
  fecha: string
  descripcion: string
}

export interface PersonalSearchResult {
  nombre: string
  dni: string
  estado: string
  sexo: string
}

export function getFileDownloadUrl(hash: string): string {
  const baseUrl = getApiBaseUrl()
  return `${baseUrl}/fileserver/${hash}`
}

export async function fetchPersonalPerfil(dni: string): Promise<PersonalPerfil> {
  return await api<PersonalPerfil>(`/personal/perfil/${dni}`)
}

export async function fetchPersonalBanco(dni: string): Promise<PersonalBanco | null> {
  try {
    return await api<PersonalBanco | null>(`/personal/banco/${dni}`)
  } catch {
    return null
  }
}

export async function fetchPersonalGrados(dni: string): Promise<PersonalGrado[]> {
  try {
    return await api<PersonalGrado[]>(`/personal/grado/${dni}`)
  } catch {
    return []
  }
}

export async function fetchPersonalContacto(dni: string): Promise<PersonalContacto | null> {
  try {
    return await api<PersonalContacto | null>(`/personal/contacto/${dni}`)
  } catch {
    return null
  }
}

export async function fetchPersonalVinculos(dni: string): Promise<PersonalVinculo[]> {
  try {
    return await api<PersonalVinculo[]>(`/personal/vinculos/${dni}`)
  } catch {
    return []
  }
}

export async function fetchPersonalArchivos(dni: string): Promise<PersonalArchivo[]> {
  try {
    return await api<PersonalArchivo[]>(`/fileserver/archivos_por_dni/${dni}`)
  } catch {
    return []
  }
}

export async function fetchPersonalDocumentos(dni: string): Promise<PersonalDocumento[]> {
  try {
    return await api<PersonalDocumento[]>(`/fileserver/documentos/${dni}`)
  } catch {
    return []
  }
}

export async function buscarTrabajadores(nombre: string): Promise<PersonalSearchResult[]> {
  try {
    return await api<PersonalSearchResult[]>('/personal/buscar', {
      query: { nombre },
    })
  } catch {
    return []
  }
}

export async function updatePersonalPerfil(perfil: PersonalPerfil): Promise<boolean> {
  await api('/personal/editar_por_dni', {
    method: 'PUT',
    body: perfil,
  })
  return true
}

export function getPersonalAvatarUrl(dni: string): string {
  const baseUrl = getApiBaseUrl()
  return `${baseUrl}/personal/avatar/${dni}`
}

export function formatMoneda(val: number | null | undefined): string {
  if (val === null || val === undefined) return 'S/ 0.00'
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(val)
}

export type VinculoStatusType = 'success' | 'warning' | 'normal'

export function getVinculoStatusType(v: PersonalVinculo | null | undefined): VinculoStatusType {
  if (!v) return 'normal'
  if (v.estado?.toLowerCase() === 'activo') return 'success'
  const hasDocSalida = Boolean(
    (v.doc_salida && v.doc_salida.trim()) ||
    (v.numero_doc_salida && v.numero_doc_salida.trim())
  )
  return hasDocSalida ? 'normal' : 'warning'
}
