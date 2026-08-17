export class ApiError extends Error {
	code: number
	error: string

	constructor(code: number, error: string) {
		super(error)
		this.name = 'ApiError'
		this.code = code
		this.error = error
	}
}

export interface ApiErrorResponse {
	code: number
	error: string
}

export interface User {
	id: number
	google_sub: string
	email: string
	full_name: string
	picture_url: string
	role: 'ADMIN' | 'USER'
	status: 'PENDING' | 'APPROVED' | 'REJECTED'
	created_at: string
	updated_at: string
}

export interface LoginResponse {
	token: string
	user: User
}

export interface RegisterResponse {
	id: number
	message: string
}

export interface GoogleLoginPayload {
	google_sub: string
	email: string
}

export interface GoogleRegisterPayload {
	google_sub: string
	email: string
	full_name: string
	picture_url: string
}

export interface CumpleanosItem {
	dni: string
	nombre: string
	nacimiento: string
	edad: number
	avatar: string | null
	regimen?: string | null
}

export interface CantidadNombre {
	cantidad: number
	nombre: string
}

export interface ResumenData {
	total: number
	activos: number
	por_regimen: CantidadNombre[]
	por_sexo: CantidadNombre[]
	por_sindicato: CantidadNombre[]
}

export interface AreaReportItem {
	cantidad: number
	nombre: string
}

export interface RenunciasAnoItem {
	cantidad: number
	nombre: string
}

export interface TrabajadorNuevoItem {
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
	avatar: string | null
}

export interface RangoEdadItem {
	cantidad: number
	nombre: string
}

export interface RangoAntiguedadItem {
	cantidad: number
	nombre: string
}

export interface ActivosDistritoItem {
	distrito: string
	cantidad: number
}

export interface PersonaDistritoItem {
	dni: string
	nombre: string
	ingreso: string | null
	direccion: string | null
	area: {
		id: number
		nombre: string
	}
	cargo: {
		id: number
		nombre: string
	}
	regimen: {
		id: number
		nombre: string
	}
	sindicato: string | null
	distrito: string
	avatar: string | null
}

export interface AreaDistritoItem {
	id: number
	nombre: string
	cantidad: number
}

export interface RangoEdadDistritoItem {
	nombre: string
	cantidad: number
}

export interface DetalleDistritoResponse {
	distrito: string
	total: number
	areas: AreaDistritoItem[]
	rangos_edad: RangoEdadDistritoItem[]
	personas: PersonaDistritoItem[]
}

export interface PersonalActivoReporteItem {
	dni: string
	nombre: string
	ingreso: string
	renuncia: string | null
	area: string
	cargo: string
	sindicato: string | null
	regimen: string
}

export type EstadoAlerta70 = 'PROXIMO_A_CUMPLIR' | 'CUMPLE_ESTE_MES' | 'EN_PERIODO_EXTENSION' | 'LIMITE_SUPERADO'

export interface Alerta70AnosItem {
	dni: string
	nombre: string | null
	nacimiento: string
	edad_actual: number
	fecha_70_anos: string
	fecha_limite_mes: string
	fecha_extension_fin_ano: string
	dias_para_70: number
	dias_para_cese_mes: number
	dias_para_cese_extension: number
	estado_alerta: EstadoAlerta70 | string
	area: string
	cargo: string
	regimen: string | null
	plaza: string | null
	avatar: string | null
}

export type ResultadoComparacion = 'OK' | 'DIFERENCIA' | 'NO_EXISTE_EN_MEF' | 'NO_EXISTE_EN_SISTEMA' | string

export interface ComparacionMefItem {
	num: number
	dni: string
	nombre: string
	regimen: string
	regimen_mef: string | null
	codigo_registro: string | null
	codigo_puesto_cpe: string
	campo: string
	valor_propio: string
	valor_mef: string
	resultado: ResultadoComparacion
}

export interface ResumenMefData {
	procesados: number
	encontrados_mef: number
	ok: number
	diferencias: number
	no_encontrados: number
	no_en_sistema: number
	fecha_comparacion: string
}

export interface ResultadoComparacionMef {
	resumen: ResumenMefData
	comparaciones: ComparacionMefItem[]
}


