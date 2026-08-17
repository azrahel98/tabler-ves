export interface Persona {
  dni: string
  nombre: string | null
  apaterno: string | null
  amaterno: string | null
  telf: string | null
  email: string | null
  direccion: string | null
  region: string | null
  distrito: string | null
  ruc: string | null
  nacimiento: string | null
  sexo: string | null
  estado?: string | null
}

export interface Vinculo {
  id: number
  dni: string
  area: string | null
  area_id: number | null
  cargo: string | null
  cargo_id: number | null
  codigo: string | null
  regimen: string | null
  sueldo: number | null
  estado: string
  sindicato: string | null
  tipo_evento: string | null
  estado_evento: string | null
  fecha_evento: string | null
  id_evento: number | null
  doc_evento_tipo: string | null
  numero_doc_ingreso: string | null
  numero_doc_evento: string | null
  doc_ingreso: string | null
  fecha_ingreso: string | number | Date
  descrip_ingreso: string | null
  doc_salida: string | null
  numero_doc_salida: string | null
  fecha_salida: string | number | Date
  descrip_salida: string | null
  cargo_estructural: string | null
  grupo_ocupacional: string | null
}

export interface InfoBancaria {
  id: number
  numero_cuenta: string
  tipo_cuenta: string
  cci: string
  banco: string
  estado: number
}

export interface GradoAcademico {
  id: number | null
  profesion: string
  universidad: string
  colegiatura: string
  nivel_academico: string
  abrv: string
}

export interface ContactoEmergencia {
  id: number
  nombre: string
  relacion: string
  telefono: string
}

export interface Resumen {
  total: number
  activos: number
  por_regimen: { nombre: string; cantidad: number }[]
  por_sexo: { nombre: string; cantidad: number }[]
  por_sindicato: { nombre: string; cantidad: number }[]
}

export interface ReporteArea {
  id: number
  area: string
  cantidad: number
}

export interface Cumpleano {
  dni: string
  nombre: string
  nacimiento: string
  area: string
}

export interface PersonalActivo {
  dni: string
  nombre: string
  area: string | null
  cargo: string | null
  regimen: string | null
  sindicato: string | null
  ingreso: string | null
}

export interface Renuncia {
  dni: string
  nombre: string
  area: string | null
  fecha: string
}

export interface NodoOrganigrama {
  id: number
  area: string
  jefe: string | null
  dni: string | null
  subgerencias: NodoOrganigrama[]
}

export interface Documento {
  id: number
  sigla: string
  nombre: string
}

export interface Banco {
  id: number
  nombre: string
}

export interface Legajo {
  id: number
  estado: string
  responsable: string | null
}

export interface NuevoTrabajador {
  id: number
  dni: string
  nombre: string
  ingreso: string | null
  documento: string | null
  area: string
  cargo: string
  regimen: string
  sueldo: number | null
  plaza: string | null
}

export interface EventoVinculo {
  id: number
  tipo_evento: string
  estado: string | null
  nombre: string
  dni: string
  area_original: string
  area_nueva: string | null
  cargo: string
  fecha_inicio: string | null
  descripcion_inicio: string | null
  fecha_salida: string | null
  descripcion_salida: string | null
}

export interface Area {
  id: number
  nombre: string
}

export interface Cargo {
  id: number
  nombre: string
}

export interface DocumentoRespaldo {
  tipoDocumento?: string | null
  numeroDocumento?: number | null
  añoDocumento?: number | null
  fecha: string
  descripcion: string
}

export interface CambioAreaPayload {
  vinculo_id: number
  nueva_area_id: number
  fecha_cambio: string
  documento: DocumentoRespaldo
}

export interface TrabajadorPorDistrito {
  dni: string
  nombre: string
  ingreso: string | null
  renuncia: string | null
  area: string | null
  cargo: string | null
  sindicato: string | null
  regimen: string | null
  distrito: string
}

export interface RegistrarUrlPayload {
  dni_asociado: string
  original_name: string
  external_url: string
  documento_id?: number
}
