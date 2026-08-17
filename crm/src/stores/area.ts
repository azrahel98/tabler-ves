import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type { PersonalActivoReporteItem } from '@/api/types'

export const useAreaStore = defineStore('area', () => {
	const trabajadoresArea = ref<PersonalActivoReporteItem[]>([])
	const identificadorArea = ref('')
	const totalGeneralMunicipal = ref(0)
	const estaCargando = ref(false)
	const error = ref<string | null>(null)

	const filtroRegimen = ref('todos')
	const terminoBusqueda = ref('')
	const paginaActual = ref(1)
	const elementosPorPagina = ref(10)

	const totalGeneral = computed(() => totalGeneralMunicipal.value || trabajadoresArea.value.length)
	const totalArea = computed(() => trabajadoresArea.value.length)
	const porcentajeMunicipal = computed(() => {
		if (!totalGeneral.value || !totalArea.value) return 0
		return Math.round((totalArea.value / totalGeneral.value) * 100)
	})

	const listaRegimenes = computed(() => {
		const conjunto = new Set<string>()
		for (const t of trabajadoresArea.value) {
			if (t.regimen) {
				conjunto.add(t.regimen)
			}
		}
		return Array.from(conjunto).sort()
	})

	const distribucionRegimenes = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresArea.value) {
			const reg = t.regimen || 'Sin Régimen'
			conteo[reg] = (conteo[reg] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
	})

	const distribucionCargos = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresArea.value) {
			const cargo = t.cargo || 'Sin Cargo'
			conteo[cargo] = (conteo[cargo] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
			.slice(0, 10)
	})

	const regimenPrincipal = computed(() => distribucionRegimenes.value[0] || null)

	const trabajadoresFiltrados = computed(() => {
		let resultado = trabajadoresArea.value

		if (filtroRegimen.value !== 'todos') {
			resultado = resultado.filter((t) => (t.regimen || '').toLowerCase() === filtroRegimen.value.toLowerCase())
		}

		if (terminoBusqueda.value.trim()) {
			const termino = terminoBusqueda.value.trim().toLowerCase()
			resultado = resultado.filter((t) => {
				const nombre = (t.nombre || '').toLowerCase()
				const dni = t.dni || ''
				const cargo = (t.cargo || '').toLowerCase()
				return nombre.includes(termino) || dni.includes(termino) || cargo.includes(termino)
			})
		}

		return resultado
	})

	const totalPaginas = computed(() => {
		return Math.max(1, Math.ceil(trabajadoresFiltrados.value.length / elementosPorPagina.value))
	})

	const trabajadoresPaginados = computed(() => {
		const inicio = (paginaActual.value - 1) * elementosPorPagina.value
		return trabajadoresFiltrados.value.slice(inicio, inicio + elementosPorPagina.value)
	})

	async function cargarArea(idONombre: string) {
		identificadorArea.value = idONombre
		paginaActual.value = 1
		estaCargando.value = true
		error.value = null
		try {
			const idNumerico = Number(idONombre)
			const [personal, resumen] = await Promise.allSettled([
				!Number.isNaN(idNumerico) && Number.isInteger(idNumerico)
					? dashboardApi.getPersonalActivoArea({ area_id: idNumerico })
					: dashboardApi.getPersonalActivoArea({ area: idONombre }),
				dashboardApi.getResumen(),
			])

			if (personal.status === 'fulfilled') {
				trabajadoresArea.value = personal.value || []
			}
			if (resumen.status === 'fulfilled' && resumen.value?.activos) {
				totalGeneralMunicipal.value = resumen.value.activos
			}
		} catch (err: any) {
			error.value = err?.error || 'No se pudo cargar la información del área'
		} finally {
			estaCargando.value = false
		}
	}

	function cambiarPagina(pagina: number) {
		if (pagina >= 1 && pagina <= totalPaginas.value) {
			paginaActual.value = pagina
		}
	}

	function restablecerFiltros() {
		filtroRegimen.value = 'todos'
		terminoBusqueda.value = ''
		paginaActual.value = 1
	}

	return {
		identificadorArea,
		estaCargando,
		error,
		filtroRegimen,
		terminoBusqueda,
		paginaActual,
		elementosPorPagina,
		trabajadoresArea,
		totalGeneral,
		totalArea,
		porcentajeMunicipal,
		listaRegimenes,
		distribucionRegimenes,
		distribucionCargos,
		regimenPrincipal,
		trabajadoresFiltrados,
		totalPaginas,
		trabajadoresPaginados,
		cargarArea,
		cambiarPagina,
		restablecerFiltros,
	}
})
