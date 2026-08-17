import { defineStore } from 'pinia'
import { ref } from 'vue'
import { dashboardApi } from '@/api/dashboard'
import type {
	ActivosDistritoItem,
	AreaReportItem,
	CumpleanosItem,
	RangoAntiguedadItem,
	RangoEdadItem,
	RenunciasAnoItem,
	ResumenData,
	TrabajadorNuevoItem,
} from '@/api/types'

export const useDashboardStore = defineStore('dashboard', () => {
	const resumen = ref<ResumenData | null>(null)
	const cumpleanos = ref<CumpleanosItem[]>([])
	const areaReport = ref<AreaReportItem[]>([])
	const renunciasAno = ref<RenunciasAnoItem[]>([])
	const trabajadoresNuevos = ref<TrabajadorNuevoItem[]>([])
	const rangosEdad = ref<RangoEdadItem[]>([])
	const rangosAntiguedad = ref<RangoAntiguedadItem[]>([])
	const activosDistrito = ref<ActivosDistritoItem[]>([])

	const isLoading = ref<boolean>(false)
	const isLoaded = ref<boolean>(false)
	const error = ref<string | null>(null)

	async function fetchDashboard(forceRefresh = false) {
		if (isLoaded.value && !forceRefresh) {
			return
		}

		isLoading.value = true
		error.value = null

		const results = await Promise.allSettled([
			dashboardApi.getResumen(),
			dashboardApi.getCumpleanos(),
			dashboardApi.getAreaReport(),
			dashboardApi.getRenunciasAno(),
			dashboardApi.getTrabajadoresNuevos(),
			dashboardApi.getRangosEdad(),
			dashboardApi.getRangosAntiguedad(),
			dashboardApi.getActivosDistrito(),
		])

		if (results[0].status === 'fulfilled') resumen.value = results[0].value
		if (results[1].status === 'fulfilled') cumpleanos.value = results[1].value
		if (results[2].status === 'fulfilled') areaReport.value = results[2].value
		if (results[3].status === 'fulfilled') renunciasAno.value = results[3].value
		if (results[4].status === 'fulfilled') trabajadoresNuevos.value = results[4].value
		if (results[5].status === 'fulfilled') rangosEdad.value = results[5].value
		if (results[6].status === 'fulfilled') rangosAntiguedad.value = results[6].value
		if (results[7].status === 'fulfilled') activosDistrito.value = results[7].value

		const rejected = results.filter((r) => r.status === 'rejected')
		if (rejected.length === results.length) {
			error.value = 'No se pudieron cargar los datos del dashboard'
		} else {
			isLoaded.value = true
		}

		isLoading.value = false
	}

	function reset() {
		resumen.value = null
		cumpleanos.value = []
		areaReport.value = []
		renunciasAno.value = []
		trabajadoresNuevos.value = []
		rangosEdad.value = []
		rangosAntiguedad.value = []
		activosDistrito.value = []
		isLoaded.value = false
		isLoading.value = false
		error.value = null
	}

	return {
		resumen,
		cumpleanos,
		areaReport,
		renunciasAno,
		trabajadoresNuevos,
		rangosEdad,
		rangosAntiguedad,
		activosDistrito,
		isLoading,
		isLoaded,
		error,
		fetchDashboard,
		reset,
	}
})
