import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type { PersonalActivoReporteItem } from '@/api/types'

export const useSindicatoStore = defineStore('sindicato', () => {
	const trabajadoresSindicato = ref<PersonalActivoReporteItem[]>([])
	const identificadorSindicato = ref('')
	const totalGeneralMunicipal = ref(0)
	const estaCargando = ref(false)
	const error = ref<string | null>(null)

	const filtroArea = ref('todas')
	const filtroRegimen = ref('todos')
	const terminoBusqueda = ref('')
	const paginaActual = ref(1)
	const elementosPorPagina = ref(10)

	const totalGeneral = computed(() => totalGeneralMunicipal.value || trabajadoresSindicato.value.length)
	const totalSindicato = computed(() => trabajadoresSindicato.value.length)
	const porcentajeMunicipal = computed(() => {
		if (!totalGeneral.value || !totalSindicato.value) return 0
		return Math.round((totalSindicato.value / totalGeneral.value) * 100)
	})

	const listaAreas = computed(() => {
		const conjunto = new Set<string>()
		for (const t of trabajadoresSindicato.value) {
			if (t.area) {
				conjunto.add(t.area)
			}
		}
		return Array.from(conjunto).sort()
	})

	const listaRegimenes = computed(() => {
		const conjunto = new Set<string>()
		for (const t of trabajadoresSindicato.value) {
			if (t.regimen) {
				conjunto.add(t.regimen)
			}
		}
		return Array.from(conjunto).sort()
	})

	const distribucionAreas = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresSindicato.value) {
			const area = t.area || 'Sin Área'
			conteo[area] = (conteo[area] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
	})

	const distribucionRegimenes = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresSindicato.value) {
			const reg = t.regimen || 'Sin Régimen'
			conteo[reg] = (conteo[reg] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
	})

	const distribucionCargos = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresSindicato.value) {
			const cargo = t.cargo || 'Sin Cargo'
			conteo[cargo] = (conteo[cargo] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
			.slice(0, 10)
	})

	const areaPrincipal = computed(() => distribucionAreas.value[0] || null)
	const regimenPrincipal = computed(() => distribucionRegimenes.value[0] || null)

	const trabajadoresFiltrados = computed(() => {
		let resultado = trabajadoresSindicato.value

		if (filtroArea.value !== 'todas') {
			resultado = resultado.filter((t) => (t.area || '').toLowerCase() === filtroArea.value.toLowerCase())
		}

		if (filtroRegimen.value !== 'todos') {
			resultado = resultado.filter((t) => (t.regimen || '').toLowerCase() === filtroRegimen.value.toLowerCase())
		}

		if (terminoBusqueda.value.trim()) {
			const termino = terminoBusqueda.value.trim().toLowerCase()
			resultado = resultado.filter((t) => {
				const nombre = (t.nombre || '').toLowerCase()
				const dni = t.dni || ''
				const cargo = (t.cargo || '').toLowerCase()
				const area = (t.area || '').toLowerCase()
				const regimen = (t.regimen || '').toLowerCase()
				return (
					nombre.includes(termino) ||
					dni.includes(termino) ||
					cargo.includes(termino) ||
					area.includes(termino) ||
					regimen.includes(termino)
				)
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

	async function cargarSindicato(idONombre: string) {
		identificadorSindicato.value = idONombre
		paginaActual.value = 1
		estaCargando.value = true
		error.value = null
		try {
			const idNumerico = Number(idONombre)
			const [personal, resumen] = await Promise.allSettled([
				!Number.isNaN(idNumerico) && Number.isInteger(idNumerico)
					? dashboardApi.getPersonalActivoSindicato({ sindicato_id: idNumerico })
					: dashboardApi.getPersonalActivoSindicato({ sindicato: idONombre }),
				dashboardApi.getResumen(),
			])

			if (personal.status === 'fulfilled') {
				trabajadoresSindicato.value = personal.value || []
			}
			if (resumen.status === 'fulfilled' && resumen.value?.activos) {
				totalGeneralMunicipal.value = resumen.value.activos
			}
		} catch (err: any) {
			error.value = err?.error || 'No se pudo cargar la información del sindicato'
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
		filtroArea.value = 'todas'
		filtroRegimen.value = 'todos'
		terminoBusqueda.value = ''
		paginaActual.value = 1
	}

	return {
		identificadorSindicato,
		estaCargando,
		error,
		filtroArea,
		filtroRegimen,
		terminoBusqueda,
		paginaActual,
		elementosPorPagina,
		trabajadoresSindicato,
		totalGeneral,
		totalSindicato,
		porcentajeMunicipal,
		listaAreas,
		listaRegimenes,
		distribucionAreas,
		distribucionRegimenes,
		distribucionCargos,
		areaPrincipal,
		regimenPrincipal,
		trabajadoresFiltrados,
		totalPaginas,
		trabajadoresPaginados,
		cargarSindicato,
		cambiarPagina,
		restablecerFiltros,
	}
})
