import { apiClient } from './client'
import type { DetalleDistritoResponse } from './types'

export const distritoApi = {
	obtenerActivosPorDistrito(distrito: string): Promise<DetalleDistritoResponse> {
		return apiClient.get<DetalleDistritoResponse>(
			`/personal/activos_por_distrito?distrito=${encodeURIComponent(distrito)}`,
		)
	},
}
