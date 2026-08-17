import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type { CumpleanosItem } from '@/api/types'

export interface CumpleaneroCalendario {
	id: string
	dni: string
	nombre: string
	nacimiento: string
	dia: number
	mes: number
	ano?: number
	edad: number
	avatar: string | null
	regimen?: string | null
	esCasF: boolean
}

export interface DiaCalendario {
	fecha: Date
	numeroDia: number
	esMesActual: boolean
	esHoy: boolean
	esFinDeSemana: boolean
	cumpleaneros: CumpleaneroCalendario[]
}

function verificarEsCasF(regimen?: string | null): boolean {
	if (!regimen) return false
	const texto = regimen.trim().toUpperCase()
	return (
		texto.includes('1057-F') ||
		texto.includes('1057 - F') ||
		texto.includes('CAS-F') ||
		texto.includes('CAS F') ||
		/1057.*F/i.test(texto)
	)
}

export const useCalendarioStore = defineStore('calendario', () => {
	const listaCumpleanos = ref<CumpleanosItem[]>([])
	const estaCargando = ref(false)
	const error = ref<string | null>(null)

	const fechaActual = new Date()
	const mesSeleccionado = ref(fechaActual.getMonth())
	const anoSeleccionado = ref(fechaActual.getFullYear())
	const diaSeleccionado = ref<number | null>(fechaActual.getDate())
	const terminoBusqueda = ref('')
	const soloCasF = ref(false)

	const mesesNombres = [
		'Enero',
		'Febrero',
		'Marzo',
		'Abril',
		'Mayo',
		'Junio',
		'Julio',
		'Agosto',
		'Septiembre',
		'Octubre',
		'Noviembre',
		'Diciembre',
	]

	const nombreMesActual = computed(() => mesesNombres[mesSeleccionado.value])

	const todosCumpleaneros = computed<CumpleaneroCalendario[]>(() => {
		const resultado: CumpleaneroCalendario[] = []

		for (const c of listaCumpleanos.value) {
			if (c.nacimiento) {
				const partes = c.nacimiento.split('-')
				if (partes.length === 3) {
					const mes = parseInt(partes[1], 10) - 1
					const dia = parseInt(partes[2], 10)
					const ano = parseInt(partes[0], 10)
					resultado.push({
						id: `cumple-${c.dni}-${dia}-${mes}`,
						dni: c.dni,
						nombre: c.nombre,
						nacimiento: c.nacimiento,
						dia,
						mes,
						ano,
						edad: c.edad,
						avatar: c.avatar,
						regimen: c.regimen,
						esCasF: verificarEsCasF(c.regimen),
					})
				}
			}
		}

		return resultado
	})

	const cumpleanerosMesActual = computed(() => {
		let lista = todosCumpleaneros.value.filter((c) => c.mes === mesSeleccionado.value)

		if (soloCasF.value) {
			lista = lista.filter((c) => c.esCasF)
		}

		if (terminoBusqueda.value.trim()) {
			const busqueda = terminoBusqueda.value.trim().toLowerCase()
			lista = lista.filter(
				(c) =>
					c.nombre.toLowerCase().includes(busqueda) ||
					c.dni.includes(busqueda) ||
					(c.regimen || '').toLowerCase().includes(busqueda),
			)
		}

		return lista.sort((a, b) => a.dia - b.dia)
	})

	const totalCumpleanosMes = computed(() => cumpleanerosMesActual.value.length)

	const totalCasFMes = computed(() => {
		return todosCumpleaneros.value.filter((c) => c.mes === mesSeleccionado.value && c.esCasF).length
	})

	const edadPromedio = computed(() => {
		if (cumpleanerosMesActual.value.length === 0) return 0
		const suma = cumpleanerosMesActual.value.reduce((acc, curr) => acc + (curr.edad || 0), 0)
		return Math.round(suma / cumpleanerosMesActual.value.length)
	})

	const diaMayorActividad = computed(() => {
		const conteo: Record<number, number> = {}
		for (const c of cumpleanerosMesActual.value) {
			conteo[c.dia] = (conteo[c.dia] || 0) + 1
		}
		const ordenados = Object.entries(conteo).sort((a, b) => b[1] - a[1])
		if (ordenados.length === 0) return null
		return { dia: parseInt(ordenados[0][0], 10), cantidad: ordenados[0][1] }
	})

	const diasMatriz = computed<DiaCalendario[]>(() => {
		const primerDiaMes = new Date(anoSeleccionado.value, mesSeleccionado.value, 1)
		const ultimoDiaMes = new Date(anoSeleccionado.value, mesSeleccionado.value + 1, 0)
		const hoy = new Date()

		let diaSemanaInicio = primerDiaMes.getDay() - 1
		if (diaSemanaInicio === -1) diaSemanaInicio = 6

		const diasTotalesMes = ultimoDiaMes.getDate()
		const ultimoDiaMesAnterior = new Date(anoSeleccionado.value, mesSeleccionado.value, 0).getDate()

		const resultado: DiaCalendario[] = []

		for (let i = diaSemanaInicio - 1; i >= 0; i--) {
			const numDia = ultimoDiaMesAnterior - i
			const fecha = new Date(anoSeleccionado.value, mesSeleccionado.value - 1, numDia)
			const esFin = fecha.getDay() === 0 || fecha.getDay() === 6
			resultado.push({
				fecha,
				numeroDia: numDia,
				esMesActual: false,
				esHoy: false,
				esFinDeSemana: esFin,
				cumpleaneros: [],
			})
		}

		for (let dia = 1; dia <= diasTotalesMes; dia++) {
			const fecha = new Date(anoSeleccionado.value, mesSeleccionado.value, dia)
			const esHoy =
				fecha.getDate() === hoy.getDate() &&
				fecha.getMonth() === hoy.getMonth() &&
				fecha.getFullYear() === hoy.getFullYear()
			const esFin = fecha.getDay() === 0 || fecha.getDay() === 6

			const festejados = cumpleanerosMesActual.value.filter((c) => c.dia === dia)

			resultado.push({
				fecha,
				numeroDia: dia,
				esMesActual: true,
				esHoy,
				esFinDeSemana: esFin,
				cumpleaneros: festejados,
			})
		}

		const diasFaltantes = 42 - resultado.length
		for (let dia = 1; dia <= diasFaltantes; dia++) {
			const fecha = new Date(anoSeleccionado.value, mesSeleccionado.value + 1, dia)
			const esFin = fecha.getDay() === 0 || fecha.getDay() === 6
			resultado.push({
				fecha,
				numeroDia: dia,
				esMesActual: false,
				esHoy: false,
				esFinDeSemana: esFin,
				cumpleaneros: [],
			})
		}

		return resultado
	})

	const cumpleanerosDiaSeleccionado = computed(() => {
		if (diaSeleccionado.value === null) return []
		return cumpleanerosMesActual.value.filter((c) => c.dia === diaSeleccionado.value)
	})

	async function cargarCumpleanos() {
		estaCargando.value = true
		error.value = null
		try {
			const respuesta = await dashboardApi.getCumpleanos()
			listaCumpleanos.value = respuesta || []
		} catch (err: any) {
			error.value = err?.error || 'No se pudieron cargar los cumpleaños'
		} finally {
			estaCargando.value = false
		}
	}

	function mesSiguiente() {
		if (mesSeleccionado.value === 11) {
			mesSeleccionado.value = 0
			anoSeleccionado.value++
		} else {
			mesSeleccionado.value++
		}
		diaSeleccionado.value = 1
	}

	function mesAnterior() {
		if (mesSeleccionado.value === 0) {
			mesSeleccionado.value = 11
			anoSeleccionado.value--
		} else {
			mesSeleccionado.value--
		}
		diaSeleccionado.value = 1
	}

	function irAHoy() {
		const hoy = new Date()
		mesSeleccionado.value = hoy.getMonth()
		anoSeleccionado.value = hoy.getFullYear()
		diaSeleccionado.value = hoy.getDate()
	}

	function seleccionarDia(dia: number) {
		diaSeleccionado.value = dia
	}

	return {
		listaCumpleanos,
		estaCargando,
		error,
		mesSeleccionado,
		anoSeleccionado,
		diaSeleccionado,
		terminoBusqueda,
		soloCasF,
		mesesNombres,
		nombreMesActual,
		cumpleanerosMesActual,
		totalCumpleanosMes,
		totalCasFMes,
		edadPromedio,
		diaMayorActividad,
		diasMatriz,
		cumpleanerosDiaSeleccionado,
		cargarCumpleanos,
		mesSiguiente,
		mesAnterior,
		irAHoy,
		seleccionarDia,
	}
})
