import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import type { User } from '@/api/types'
import {
	type CrearUsuarioPayload,
	crearUsuario,
	type EditarUsuarioPayload,
	editarUsuario,
	eliminarUsuario,
	listarUsuarios,
} from '@/api/usuarios'

export const useUsuariosStore = defineStore('usuarios', () => {
	const usuarios = ref<User[]>([])
	const cargando = ref(false)
	const procesandoId = ref<number | null>(null)
	const error = ref<string | null>(null)

	const terminoBusqueda = ref('')
	const filtroEstado = ref<'TODOS' | 'PENDING' | 'APPROVED' | 'REJECTED'>('TODOS')
	const filtroRol = ref<'TODOS' | 'ADMIN' | 'USER'>('TODOS')

	const totalUsuarios = computed(() => usuarios.value.length)
	const totalAprobados = computed(() => usuarios.value.filter((u) => u.status === 'APPROVED').length)
	const totalPendientes = computed(() => usuarios.value.filter((u) => u.status === 'PENDING').length)
	const totalRechazados = computed(() => usuarios.value.filter((u) => u.status === 'REJECTED').length)
	const totalAdministradores = computed(() => usuarios.value.filter((u) => u.role === 'ADMIN').length)

	const usuariosFiltrados = computed(() => {
		const termino = terminoBusqueda.value.toLowerCase().trim()

		return usuarios.value.filter((u) => {
			const coincideBusqueda =
				!termino ||
				u.full_name.toLowerCase().includes(termino) ||
				u.email.toLowerCase().includes(termino) ||
				u.google_sub.toLowerCase().includes(termino)

			const coincideEstado = filtroEstado.value === 'TODOS' || u.status === filtroEstado.value
			const coincideRol = filtroRol.value === 'TODOS' || u.role === filtroRol.value

			return coincideBusqueda && coincideEstado && coincideRol
		})
	})

	async function cargarUsuarios() {
		cargando.value = true
		error.value = null
		try {
			usuarios.value = await listarUsuarios()
		} catch (e: any) {
			error.value = e?.error || 'Error al obtener la lista de usuarios'
			throw e
		} finally {
			cargando.value = false
		}
	}

	async function modificarUsuario(datos: EditarUsuarioPayload) {
		procesandoId.value = datos.id
		error.value = null
		try {
			await editarUsuario(datos)
			const indice = usuarios.value.findIndex((u) => u.id === datos.id)
			if (indice !== -1) {
				usuarios.value[indice] = {
					...usuarios.value[indice],
					role: datos.role,
					status: datos.status,
					updated_at: new Date().toISOString(),
				}
			}
		} catch (e: any) {
			error.value = e?.error || 'Error al actualizar usuario'
			throw e
		} finally {
			procesandoId.value = null
		}
	}

	async function cambiarRol(id: number, nuevoRol: 'ADMIN' | 'USER') {
		const usuarioActual = usuarios.value.find((u) => u.id === id)
		if (!usuarioActual) return
		await modificarUsuario({
			id,
			role: nuevoRol,
			status: usuarioActual.status,
		})
	}

	async function cambiarEstado(id: number, nuevoEstado: 'PENDING' | 'APPROVED' | 'REJECTED') {
		const usuarioActual = usuarios.value.find((u) => u.id === id)
		if (!usuarioActual) return
		await modificarUsuario({
			id,
			role: usuarioActual.role,
			status: nuevoEstado,
		})
	}

	async function removerUsuario(id: number) {
		procesandoId.value = id
		error.value = null
		try {
			await eliminarUsuario(id)
			usuarios.value = usuarios.value.filter((u) => u.id !== id)
		} catch (e: any) {
			error.value = e?.error || 'Error al eliminar usuario'
			throw e
		} finally {
			procesandoId.value = null
		}
	}

	async function registrarNuevoUsuario(datos: CrearUsuarioPayload) {
		cargando.value = true
		error.value = null
		try {
			await crearUsuario(datos)
			await cargarUsuarios()
		} catch (e: any) {
			error.value = e?.error || 'Error al crear usuario'
			throw e
		} finally {
			cargando.value = false
		}
	}

	return {
		usuarios,
		cargando,
		procesandoId,
		error,
		terminoBusqueda,
		filtroEstado,
		filtroRol,
		totalUsuarios,
		totalAprobados,
		totalPendientes,
		totalRechazados,
		totalAdministradores,
		usuariosFiltrados,
		cargarUsuarios,
		modificarUsuario,
		cambiarRol,
		cambiarEstado,
		removerUsuario,
		registrarNuevoUsuario,
	}
})
