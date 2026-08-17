import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type { Alerta70AnosItem } from '@/api/types'

export const useJubilacionStore = defineStore('jubilacion', () => {
	const servidores = ref<Alerta70AnosItem[]>([])
	const estaCargando = ref<boolean>(false)
	const estaCargado = ref<boolean>(false)
	const error = ref<string | null>(null)

	const busqueda = ref<string>('')
	const filtroEstado = ref<string>('TODOS')
	const filtroRegimen = ref<string>('TODOS')
	const edadMinima = ref<number>(69)

	const totalServidores = computed(() => servidores.value.length)

	const totalCumplenEsteMes = computed(() => {
		return servidores.value.filter((s) => s.estado_alerta === 'CUMPLE_ESTE_MES').length
	})

	const totalProximos = computed(() => {
		return servidores.value.filter((s) => s.estado_alerta === 'PROXIMO_A_CUMPLIR').length
	})

	const totalEnExtension = computed(() => {
		return servidores.value.filter((s) => s.estado_alerta === 'EN_PERIODO_EXTENSION').length
	})

	const totalLimiteSuperado = computed(() => {
		return servidores.value.filter((s) => s.estado_alerta === 'LIMITE_SUPERADO').length
	})

	const regimenesDisponibles = computed(() => {
		const conjunto = new Set<string>()
		for (const servidor of servidores.value) {
			if (servidor.regimen && servidor.regimen.trim()) {
				conjunto.add(servidor.regimen.trim())
			}
		}
		return Array.from(conjunto).sort()
	})

	const conteoPorRegimen = computed(() => {
		const mapa: Record<string, number> = {}
		for (const servidor of servidores.value) {
			const reg = servidor.regimen?.trim() || 'Sin Régimen'
			mapa[reg] = (mapa[reg] || 0) + 1
		}
		return Object.entries(mapa).map(([nombre, cantidad]) => ({
			nombre,
			cantidad,
		})).sort((a, b) => b.cantidad - a.cantidad)
	})

	const servidoresFiltrados = computed(() => {
		let resultado = [...servidores.value]

		if (filtroEstado.value !== 'TODOS') {
			resultado = resultado.filter((s) => s.estado_alerta === filtroEstado.value)
		}

		if (filtroRegimen.value !== 'TODOS') {
			resultado = resultado.filter((s) => (s.regimen?.trim() || 'Sin Régimen') === filtroRegimen.value)
		}

		if (busqueda.value.trim()) {
			const termino = busqueda.value.trim().toLowerCase()
			resultado = resultado.filter((s) => {
				const dni = s.dni.toLowerCase()
				const nombre = (s.nombre || '').toLowerCase()
				const cargo = (s.cargo || '').toLowerCase()
				const area = (s.area || '').toLowerCase()
				const plaza = (s.plaza || '').toLowerCase()
				return (
					dni.includes(termino) ||
					nombre.includes(termino) ||
					cargo.includes(termino) ||
					area.includes(termino) ||
					plaza.includes(termino)
				)
			})
		}

		return resultado
	})

	async function cargarServidores(edad?: number, forzar = false) {
		const edadConsultar = edad !== undefined ? edad : edadMinima.value
		edadMinima.value = edadConsultar

		if (estaCargado.value && !forzar) {
			return
		}

		estaCargando.value = true
		error.value = null

		try {
			const datos = await dashboardApi.getAlerta70Anos(edadConsultar)
			servidores.value = datos
			estaCargado.value = true
		} catch (e: any) {
			error.value = e?.message || e?.error || 'Error al cargar la lista de alerta de 70 años'
			servidores.value = []
		} finally {
			estaCargando.value = false
		}
	}

	async function cambiarEdadMinima(nuevaEdad: number) {
		if (edadMinima.value === nuevaEdad && estaCargado.value) return
		edadMinima.value = nuevaEdad
		await cargarServidores(nuevaEdad, true)
	}

	function limpiarFiltros() {
		busqueda.value = ''
		filtroEstado.value = 'TODOS'
		filtroRegimen.value = 'TODOS'
	}

	return {
		servidores,
		estaCargando,
		estaCargado,
		error,
		busqueda,
		filtroEstado,
		filtroRegimen,
		edadMinima,
		totalServidores,
		totalCumplenEsteMes,
		totalProximos,
		totalEnExtension,
		totalLimiteSuperado,
		regimenesDisponibles,
		conteoPorRegimen,
		servidoresFiltrados,
		cargarServidores,
		cambiarEdadMinima,
		limpiarFiltros,
	}
})
