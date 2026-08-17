import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { distritoApi } from '@/api/distrito'
import type { DetalleDistritoResponse, PersonaDistritoItem } from '@/api/types'

export const useDistritoStore = defineStore('distrito', () => {
	const detalle = ref<DetalleDistritoResponse | null>(null)
	const estaCargando = ref(false)
	const error = ref<string | null>(null)

	const terminoBusqueda = ref('')
	const filtroArea = ref('todos')
	const filtroRegimen = ref('todos')
	const paginaActual = ref(1)
	const elementosPorPagina = ref(10)

	const totalTrabajadores = computed(() => detalle.value?.total ?? 0)
	const areas = computed(() => detalle.value?.areas ?? [])
	const rangosEdad = computed(() => detalle.value?.rangos_edad ?? [])
	const personas = computed(() => detalle.value?.personas ?? [])

	const listaRegimenes = computed(() => {
		const conjunto = new Set<string>()
		for (const persona of personas.value) {
			if (persona.regimen?.nombre) {
				conjunto.add(persona.regimen.nombre)
			}
		}
		return Array.from(conjunto).sort()
	})

	const listaAreas = computed(() => {
		return areas.value.map((a) => a.nombre).sort()
	})

	const personasFiltradas = computed<PersonaDistritoItem[]>(() => {
		let resultado = personas.value

		if (terminoBusqueda.value.trim()) {
			const termino = terminoBusqueda.value.trim().toLowerCase()
			resultado = resultado.filter((p) => {
				const nombre = p.nombre?.toLowerCase() || ''
				const dni = p.dni || ''
				const cargo = p.cargo?.nombre?.toLowerCase() || ''
				const direccion = p.direccion?.toLowerCase() || ''
				return (
					nombre.includes(termino) || dni.includes(termino) || cargo.includes(termino) || direccion.includes(termino)
				)
			})
		}

		if (filtroArea.value !== 'todos') {
			resultado = resultado.filter((p) => p.area?.nombre === filtroArea.value)
		}

		if (filtroRegimen.value !== 'todos') {
			resultado = resultado.filter((p) => p.regimen?.nombre === filtroRegimen.value)
		}

		return resultado
	})

	const totalPaginas = computed(() => {
		return Math.max(1, Math.ceil(personasFiltradas.value.length / elementosPorPagina.value))
	})

	const personasPaginadas = computed<PersonaDistritoItem[]>(() => {
		const inicio = (paginaActual.value - 1) * elementosPorPagina.value
		return personasFiltradas.value.slice(inicio, inicio + elementosPorPagina.value)
	})

	async function cargarDistrito(nombreDistrito: string) {
		estaCargando.value = true
		error.value = null
		paginaActual.value = 1
		terminoBusqueda.value = ''
		filtroArea.value = 'todos'
		filtroRegimen.value = 'todos'

		try {
			const respuesta = await distritoApi.obtenerActivosPorDistrito(nombreDistrito)
			detalle.value = respuesta
		} catch (err: any) {
			error.value = err?.error || 'No se pudieron cargar los datos del distrito'
			detalle.value = null
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
		terminoBusqueda.value = ''
		filtroArea.value = 'todos'
		filtroRegimen.value = 'todos'
		paginaActual.value = 1
	}

	return {
		detalle,
		estaCargando,
		error,
		terminoBusqueda,
		filtroArea,
		filtroRegimen,
		paginaActual,
		elementosPorPagina,
		totalTrabajadores,
		areas,
		rangosEdad,
		personas,
		listaRegimenes,
		listaAreas,
		personasFiltradas,
		totalPaginas,
		personasPaginadas,
		cargarDistrito,
		cambiarPagina,
		restablecerFiltros,
	}
})
