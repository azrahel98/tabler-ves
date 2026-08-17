import { apiClient } from './client'

export interface PerfilPersona {
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

export interface VinculoLaboral {
	id: number
	dni: string
	doc_ingreso_id?: number | null
	doc_ingreso?: string | null
	numero_doc_ingreso?: string | null
	descrip_ingreso?: string | null
	fecha_ingreso: string
	area?: string | null
	cargo: string
	regimen?: string | null
	sueldo?: number | null
	codigo?: string | null
	cargo_estructural?: string | null
	grupo_ocupacional?: string | null
	estado: string
	doc_salida_id?: number | null
	doc_salida?: string | null
	descrip_salida?: string | null
	fecha_salida?: string | null
	numero_doc_salida?: string | null
	sindicato?: string | null
	id_evento?: number | null
	tipo_evento?: string | null
	estado_evento?: string | null
	doc_evento_id?: number | null
	doc_evento_tipo?: string | null
	numero_doc_evento?: string | number | null
	fecha_evento?: string | null
	documento_id?: number | null
}

export interface DocumentoDetalle {
	id?: number
	tipoDocumento?: string | null
	tipo?: string | null
	areaId?: number | null
	area?: string | null
	sigla?: string | null
	numeroDocumento?: number | null
	numero?: number | null
	añoDocumento?: number | null
	año?: number | null
	fecha: string
	fechaValida?: string | null
	conv?: number | null
	descripcion: string
	funcion?: number | null
}

export interface InfoBancaria {
	id: number
	numero_cuenta: string
	tipo_cuenta: string
	cci: string
	banco: string
	estado: number
}

export interface ContactoEmergencia {
	persona_dni: string
	nombre: string
	relacion: string
	telefono: string
}

export interface GradoAcademico {
	id: number
	profesion: string
	universidad: string
	nivel_academico: string
	abrv: string
	dni: string
	fecha: string
}

export interface ArchivoLegajo {
	id: number
	documento_id: number | null
	dni_asociado: string
	original_name: string
	file_hash: string
	extension: string
	external_url: string | null
	usuario_subida: string | null
	fecha_subida: string | null
}

export interface PersonaBusqueda {
	nombre: string
	dni: string
	estado: string
	sexo: string | null
	foto?: string | null
	cargo?: string | null
	area?: string | null
}

export const perfilApi = {
	getPerfil(dni: string): Promise<PerfilPersona> {
		return apiClient.get<PerfilPersona>(`/personal/perfil/${dni}`)
	},

	getVinculos(dni: string): Promise<VinculoLaboral[]> {
		return apiClient.get<VinculoLaboral[]>(`/personal/vinculos/${dni}`)
	},

	getBanco(dni: string): Promise<InfoBancaria> {
		return apiClient.get<InfoBancaria>(`/personal/banco/${dni}`)
	},

	getContacto(dni: string): Promise<ContactoEmergencia> {
		return apiClient.get<ContactoEmergencia>(`/personal/contacto/${dni}`)
	},

	getGrados(dni: string): Promise<GradoAcademico[]> {
		return apiClient.get<GradoAcademico[]>(`/personal/grado/${dni}`)
	},

	getDocumentosLegajo(dni: string): Promise<ArchivoLegajo[]> {
		return apiClient.get<ArchivoLegajo[]>(`/fileserver/archivos_por_dni/${dni}`)
	},

	getDocumento(id: number | string): Promise<DocumentoDetalle> {
		return apiClient.get<DocumentoDetalle>(`/personal/documento/${id}`)
	},

	buscarPersonas(nombre: string): Promise<PersonaBusqueda[]> {
		return apiClient.get<PersonaBusqueda[]>(`/personal/buscar?nombre=${encodeURIComponent(nombre)}`)
	},
}
