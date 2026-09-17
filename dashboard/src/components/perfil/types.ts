import { api, getApiBaseUrl } from '@/services/api'

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
  origen?: string
  eventos?: EventoVinculoDetalle[]
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

export type VinculoStatusType = 'success' | 'warning' | 'normal'

export function getFileDownloadUrl(hash: string): string {
  const baseUrl = getApiBaseUrl()
  return `${baseUrl}/fileserver/${hash}`
}

export function getPersonalAvatarUrl(dni: string): string {
  const baseUrl = getApiBaseUrl()
  return `${baseUrl}/personal/avatar/${dni}`
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

export function formatMoneda(val: number | null | undefined): string {
  if (val === null || val === undefined) return 'S/ 0.00'
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(val)
}

export function getVinculoStatusType(v: PersonalVinculo | null | undefined): VinculoStatusType {
  if (!v) return 'normal'
  if (v.estado?.toLowerCase() === 'activo') return 'success'
  const hasDocSalida = Boolean(
    (v.doc_salida && v.doc_salida.trim()) ||
    (v.numero_doc_salida && v.numero_doc_salida.trim())
  )
  return hasDocSalida ? 'normal' : 'warning'
}

export function getTipoEventoLabel(tipo: string | null | undefined): string {
  if (!tipo) return '-'
  const map: Record<string, string> = {
    rotacion: 'Rotación',
    destaque: 'Destaque',
    encargo_puesto: 'Encargo de Puesto',
    encargo_funciones: 'Encargo de Funciones',
    abandono: 'Abandono de Cargo',
    suspension: 'Suspensión',
    licencia: 'Licencia',
    otro: 'Otro Evento',
  }
  return map[tipo.toLowerCase()] || tipo
}

export interface RenunciaPayload {
  id: number
  tipoDocumento: string
  numeroDocumento?: number | null
  añoDocumento?: number | null
  fecha: string
  fechaValida?: string | null
  descripcion: string
}

export interface RenunciaResponse {
  dni: string
  nombre: string
  estado: string
  fecha: string
  descripcion: string
  documento: string
}

export interface TipoDocumentoOption {
  id: number
  nombre: string
  sigla: string
}

export interface RegistrarUrlPayload {
  dni_asociado: string
  original_name: string
  external_url: string
  documento_id?: number | null
}

export interface RegistrarUrlResponse {
  id: number
  original_name: string
  file_hash: string
  external_url: string
  extension: string
}

export interface UploadArchivoResponse {
  id: number
  original_name: string
  file_hash: string
  extension: string
}

export type TipoDocumentoVinculo = 'ingreso' | 'salida' | 'evento'

export interface DocumentoVinculoInfo {
  tipo: TipoDocumentoVinculo
  titulo: string
  subtitulo?: string
  tipoDocumentoNombre?: string | null
  numeroDocumento?: string | null
  documentoId?: number | null
  descripcion?: string | null
  fecha?: string | null
  eventoId?: number | null
  tipoEvento?: string | null
  estadoEvento?: string | null
  vinculo: PersonalVinculo
}

export interface CargoOption {
  id: number
  nombre: string
  activo: boolean
}

export interface EventoVinculoDetalle {
  id: number
  vinculo_id: number
  tipo_evento: string
  estado?: string | null
  nueva_area_id?: number | null
  nueva_area?: string | null
  nuevo_cargo_id?: number | null
  nuevo_cargo?: string | null
  doc_inicio_id?: number | null
  tipo_doc_inicio?: string | null
  numero_doc_inicio?: string | null
  fecha_inicio?: string | null
  fecha_valida_inicio?: string | null
  descrip_inicio?: string | null
  doc_salida_id?: number | null
  tipo_doc_salida?: string | null
  numero_doc_salida?: string | null
  fecha_salida?: string | null
  fecha_valida_salida?: string | null
  descrip_salida?: string | null
}

export interface EventoVinculoPayload {
  id?: number | null
  vinculo_id: number
  tipo_evento: string
  nueva_area_id?: number | null
  nuevo_cargo_id?: number | null
  documento_inicio?: {
    tipoDocumento: string
    areaId: number | null
    numeroDocumento: number
    añoDocumento: number
    fecha: string
    fechaValida?: string | null
    descripcion: string
  } | null
  documento_salida?: {
    tipoDocumento: string
    areaId: number | null
    numeroDocumento: number
    añoDocumento: number
    fecha: string
    fechaValida?: string | null
    descripcion: string
  } | null
  mismo_documento?: boolean | null
  estado?: string | null
}

