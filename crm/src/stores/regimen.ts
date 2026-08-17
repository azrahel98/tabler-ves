import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type { PersonalActivoReporteItem } from '@/api/types'

export const useRegimenStore = defineStore('regimen', () => {
	const trabajadoresRegimen = ref<PersonalActivoReporteItem[]>([])
	const identificadorRegimen = ref('')
	const totalGeneralMunicipal = ref(0)
	const estaCargando = ref(false)
	const error = ref<string | null>(null)

	const filtroArea = ref('todas')
	const terminoBusqueda = ref('')
	const paginaActual = ref(1)
	const elementosPorPagina = ref(10)

	const totalGeneral = computed(() => totalGeneralMunicipal.value || trabajadoresRegimen.value.length)
	const totalRegimen = computed(() => trabajadoresRegimen.value.length)
	const porcentajeMunicipal = computed(() => {
		if (!totalGeneral.value || !totalRegimen.value) return 0
		return Math.round((totalRegimen.value / totalGeneral.value) * 100)
	})

	const listaAreas = computed(() => {
		const conjunto = new Set<string>()
		for (const t of trabajadoresRegimen.value) {
			if (t.area) {
				conjunto.add(t.area)
			}
		}
		return Array.from(conjunto).sort()
	})

	const distribucionAreas = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresRegimen.value) {
			const area = t.area || 'Sin Área'
			conteo[area] = (conteo[area] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
	})

	const distribucionCargos = computed(() => {
		const conteo: Record<string, number> = {}
		for (const t of trabajadoresRegimen.value) {
			const cargo = t.cargo || 'Sin Cargo'
			conteo[cargo] = (conteo[cargo] || 0) + 1
		}
		return Object.entries(conteo)
			.map(([nombre, cantidad]) => ({ nombre, cantidad }))
			.sort((a, b) => b.cantidad - a.cantidad)
			.slice(0, 10)
	})

	const areaPrincipal = computed(() => distribucionAreas.value[0] || null)

	const trabajadoresFiltrados = computed(() => {
		let resultado = trabajadoresRegimen.value

		if (filtroArea.value !== 'todas') {
			resultado = resultado.filter((t) => (t.area || '').toLowerCase() === filtroArea.value.toLowerCase())
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

	async function cargarRegimen(idONombre: string) {
		identificadorRegimen.value = idONombre
		paginaActual.value = 1
		estaCargando.value = true
		error.value = null
		try {
			const idNumerico = Number(idONombre)
			const [personal, resumen] = await Promise.allSettled([
				!Number.isNaN(idNumerico) && Number.isInteger(idNumerico)
					? dashboardApi.getPersonalActivoRegimen({ regimen_id: idNumerico })
					: dashboardApi.getPersonalActivoRegimen({ regimen: idONombre }),
				dashboardApi.getResumen(),
			])

			if (personal.status === 'fulfilled') {
				trabajadoresRegimen.value = personal.value || []
			}
			if (resumen.status === 'fulfilled' && resumen.value?.activos) {
				totalGeneralMunicipal.value = resumen.value.activos
			}
		} catch (err: any) {
			error.value = err?.error || 'No se pudo cargar la información del régimen'
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
		terminoBusqueda.value = ''
		paginaActual.value = 1
	}

	return {
		identificadorRegimen,
		estaCargando,
		error,
		filtroArea,
		terminoBusqueda,
		paginaActual,
		elementosPorPagina,
		trabajadoresRegimen,
		totalGeneral,
		totalRegimen,
		porcentajeMunicipal,
		listaAreas,
		distribucionAreas,
		distribucionCargos,
		areaPrincipal,
		trabajadoresFiltrados,
		totalPaginas,
		trabajadoresPaginados,
		cargarRegimen,
		cambiarPagina,
		restablecerFiltros,
	}
})
