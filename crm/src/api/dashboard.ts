import { apiClient } from './client'
import type {
	ActivosDistritoItem,
	Alerta70AnosItem,
	AreaReportItem,
	ComparacionMefItem,
	CumpleanosItem,
	PersonalActivoReporteItem,
	RangoAntiguedadItem,
	RangoEdadItem,
	RenunciasAnoItem,
	ResultadoComparacionMef,
	ResumenData,
	TrabajadorNuevoItem,
} from './types'

export const dashboardApi = {
	getResumen(): Promise<ResumenData> {
		return apiClient.get<ResumenData>('api/dash/resumen')
	},

	getCumpleanos(): Promise<CumpleanosItem[]> {
		return apiClient.get<CumpleanosItem[]>('api/dash/cumpleanos')
	},

	getAreaReport(): Promise<AreaReportItem[]> {
		return apiClient.get<AreaReportItem[]>('api/dash/areareport')
	},

	getRenunciasAno(): Promise<RenunciasAnoItem[]> {
		return apiClient.get<RenunciasAnoItem[]>('api/dash/renunciasano')
	},

	getTrabajadoresNuevos(): Promise<TrabajadorNuevoItem[]> {
		return apiClient.get<TrabajadorNuevoItem[]>('api/dash/trabajadores_nuevos')
	},

	getRangosEdad(): Promise<RangoEdadItem[]> {
		return apiClient.get<RangoEdadItem[]>('api/dash/rangos_edad')
	},

	getRangosAntiguedad(): Promise<RangoAntiguedadItem[]> {
		return apiClient.get<RangoAntiguedadItem[]>('api/dash/rangos_antiguedad')
	},

	getActivosDistrito(): Promise<ActivosDistritoItem[]> {
		return apiClient.get<ActivosDistritoItem[]>('api/dash/activos/distrito')
	},

	getPersonalActivoArea(params: { area_id?: number; area?: string }): Promise<PersonalActivoReporteItem[]> {
		const query = new URLSearchParams()
		if (params.area_id != null) query.append('area_id', params.area_id.toString())
		if (params.area != null) query.append('area', params.area)
		return apiClient.get<PersonalActivoReporteItem[]>(`api/dash/activos/area?${query.toString()}`)
	},

	getPersonalActivoRegimen(params: { regimen_id?: number; regimen?: string }): Promise<PersonalActivoReporteItem[]> {
		const query = new URLSearchParams()
		if (params.regimen_id != null) query.append('regimen_id', params.regimen_id.toString())
		if (params.regimen != null) query.append('regimen', params.regimen)
		return apiClient.get<PersonalActivoReporteItem[]>(`api/dash/activos/regimen?${query.toString()}`)
	},

	getPersonalActivoSindicato(params: {
		sindicato_id?: number
		sindicato?: string
	}): Promise<PersonalActivoReporteItem[]> {
		const query = new URLSearchParams()
		if (params.sindicato_id != null) query.append('sindicato_id', params.sindicato_id.toString())
		if (params.sindicato != null) query.append('sindicato', params.sindicato)
		return apiClient.get<PersonalActivoReporteItem[]>(`api/dash/activos/sindicato?${query.toString()}`)
	},

	getAlerta70Anos(edadMin?: number): Promise<Alerta70AnosItem[]> {
		const query = new URLSearchParams()
		if (edadMin != null) query.append('edad_min', edadMin.toString())
		const queryString = query.toString()
		return apiClient.get<Alerta70AnosItem[]>(`api/dash/alerta_70${queryString ? `?${queryString}` : ''}`)
	},

	compararMef(archivoCas?: File | null, archivoOtros?: File | null): Promise<ResultadoComparacionMef> {
		const formData = new FormData()
		if (archivoCas) formData.append('file_cas', archivoCas)
		if (archivoOtros) formData.append('file_otros', archivoOtros)
		return apiClient.postFormData<ResultadoComparacionMef>('api/dash/comparar_mef', formData)
	},

	generarExcelMef(comparaciones: ComparacionMefItem[]): Promise<Blob> {
		return apiClient.postBlob('api/dash/generar_mef', { comparaciones })
	},
}


