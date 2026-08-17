import { apiClient } from './client'
import type { User } from './types'

export interface EditarUsuarioPayload {
	id: number
	role: 'ADMIN' | 'USER'
	status: 'PENDING' | 'APPROVED' | 'REJECTED'
}

export interface CrearUsuarioPayload {
	google_sub: string
	email: string
	full_name: string
	picture_url?: string | null
}

export async function listarUsuarios(): Promise<User[]> {
	return await apiClient.get<User[]>('/usuarios/listar')
}

export async function crearUsuario(datos: CrearUsuarioPayload): Promise<string> {
	return await apiClient.post<string>('/usuarios/crear', datos)
}

export async function editarUsuario(datos: EditarUsuarioPayload): Promise<string> {
	return await apiClient.put<string>('/usuarios/editar', datos)
}

export async function eliminarUsuario(id: number): Promise<string> {
	return await apiClient.delete<string>(`/usuarios/eliminar/${id}`)
}
