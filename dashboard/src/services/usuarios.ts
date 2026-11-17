import { api } from './api'
import type { AuthUser } from '@/stores/auth'

export type RolUsuario = 'ADMIN' | 'EDITOR' | 'USER'
export type EstadoUsuario = 'PENDING' | 'APPROVED' | 'REJECTED'

export type Usuario = AuthUser

export async function listarUsuarios(): Promise<Usuario[]> {
  return await api<Usuario[]>('/usuarios/listar')
}

export async function editarUsuario(payload: {
  id: number
  role: RolUsuario | string
  status: EstadoUsuario | string
}): Promise<string> {
  return await api<string>('/usuarios/editar', { method: 'PUT', body: payload })
}

export async function eliminarUsuario(id: number): Promise<string> {
  return await api<string>(`/usuarios/eliminar/${id}`, { method: 'DELETE' })
}
