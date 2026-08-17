import { defineStore } from 'pinia'
import { ref } from 'vue'
import {
	type ArchivoLegajo,
	type ContactoEmergencia,
	type GradoAcademico,
	type InfoBancaria,
	type PerfilPersona,
	type PersonaBusqueda,
	perfilApi,
	type VinculoLaboral,
} from '@/api/perfil'

interface CacheItem {
	perfil: PerfilPersona
	vinculos: VinculoLaboral[]
	banco: InfoBancaria | null
	contacto: ContactoEmergencia | null
	grados: GradoAcademico[]
	documentos: ArchivoLegajo[]
}

export const usePerfilStore = defineStore('perfil', () => {
	const dniActual = ref<string>('')
	const perfil = ref<PerfilPersona | null>(null)
	const vinculos = ref<VinculoLaboral[]>([])
	const banco = ref<InfoBancaria | null>(null)
	const contacto = ref<ContactoEmergencia | null>(null)
	const grados = ref<GradoAcademico[]>([])
	const documentos = ref<ArchivoLegajo[]>([])

	const resultadosBusqueda = ref<PersonaBusqueda[]>([])
	const buscandoSugerencias = ref<boolean>(false)
	const isLoading = ref<boolean>(false)
	const cargandoLegajo = ref<boolean>(false)
	const cargandoBanco = ref<boolean>(false)
	const error = ref<string | null>(null)

	const legajoCargadoDni = ref<string>('')
	const bancoCargadoDni = ref<string>('')

	const cache = ref<Record<string, CacheItem>>({})

	async function cargarPerfil(dni: string, forceRefresh = false) {
		const cleanDni = dni.trim()
		if (!cleanDni) return

		dniActual.value = cleanDni
		error.value = null

		if (cache.value[cleanDni] && !forceRefresh) {
			const item = cache.value[cleanDni]
			perfil.value = item.perfil
			vinculos.value = item.vinculos
			banco.value = item.banco
			contacto.value = item.contacto
			grados.value = item.grados
			documentos.value = item.documentos
			legajoCargadoDni.value = cleanDni
			bancoCargadoDni.value = cleanDni
			return
		}

		isLoading.value = true
		perfil.value = null
		vinculos.value = []
		banco.value = null
		contacto.value = null
		grados.value = []
		documentos.value = []
		legajoCargadoDni.value = ''
		bancoCargadoDni.value = ''

		try {
			const perfilRes = await perfilApi.getPerfil(cleanDni)
			perfil.value = perfilRes
		} catch (err: any) {
			error.value = err.error || 'No se encontró a la persona con el DNI ingresado'
			isLoading.value = false
			return
		}

		const results = await Promise.allSettled([
			perfilApi.getVinculos(cleanDni),
			perfilApi.getContacto(cleanDni),
			perfilApi.getGrados(cleanDni),
			perfilApi.getDocumentosLegajo(cleanDni),
		])

		if (results[0].status === 'fulfilled' && Array.isArray(results[0].value)) {
			vinculos.value = results[0].value
		}
		if (results[1].status === 'fulfilled') {
			contacto.value = results[1].value
		}
		if (results[2].status === 'fulfilled' && Array.isArray(results[2].value)) {
			grados.value = results[2].value
		}
		if (results[3].status === 'fulfilled' && Array.isArray(results[3].value)) {
			documentos.value = results[3].value
			legajoCargadoDni.value = cleanDni
		}

		isLoading.value = false
	}

	async function cargarLegajo(dni: string, forceRefresh = false) {
		const cleanDni = dni.trim()
		if (!cleanDni) return
		if (legajoCargadoDni.value === cleanDni && !forceRefresh && documentos.value.length > 0) {
			return
		}

		cargandoLegajo.value = true
		try {
			const docs = await perfilApi.getDocumentosLegajo(cleanDni)
			documentos.value = docs || []
			legajoCargadoDni.value = cleanDni
		} catch (err) {
			documentos.value = []
		} finally {
			cargandoLegajo.value = false
		}
	}

	async function cargarBanco(dni: string, forceRefresh = false) {
		const cleanDni = dni.trim()
		if (!cleanDni) return
		if (bancoCargadoDni.value === cleanDni && !forceRefresh && banco.value !== null) {
			return
		}

		cargandoBanco.value = true
		try {
			const infoBanco = await perfilApi.getBanco(cleanDni)
			banco.value = infoBanco
			bancoCargadoDni.value = cleanDni
		} catch (err) {
			banco.value = null
		} finally {
			cargandoBanco.value = false
		}
	}

	async function buscarSugerencias(query: string) {
		const q = query.trim()
		if (!q || q.length < 2) {
			resultadosBusqueda.value = []
			return
		}

		buscandoSugerencias.value = true
		try {
			const res = await perfilApi.buscarPersonas(q)
			resultadosBusqueda.value = res || []
		} catch {
			resultadosBusqueda.value = []
		} finally {
			buscandoSugerencias.value = false
		}
	}

	function limpiarPerfil() {
		dniActual.value = ''
		perfil.value = null
		vinculos.value = []
		banco.value = null
		contacto.value = null
		grados.value = []
		documentos.value = []
		error.value = null
		legajoCargadoDni.value = ''
		bancoCargadoDni.value = ''
	}

	return {
		dniActual,
		perfil,
		vinculos,
		banco,
		contacto,
		grados,
		documentos,
		resultadosBusqueda,
		buscandoSugerencias,
		isLoading,
		cargandoLegajo,
		cargandoBanco,
		error,
		cargarPerfil,
		cargarLegajo,
		cargarBanco,
		buscarSugerencias,
		limpiarPerfil,
	}
})
