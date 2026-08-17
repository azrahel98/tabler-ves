import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type ArchivoLegajo, type DocumentoDetalle, perfilApi } from '@/api/perfil'

export const useDetalleDocumentoStore = defineStore('detalleDocumento', () => {
	const estaAbierto = ref(false)
	const estaCargando = ref(false)
	const documento = ref<DocumentoDetalle | null>(null)
	const archivos = ref<ArchivoLegajo[]>([])
	const error = ref<string | null>(null)
	const idActual = ref<number | string | null>(null)

	async function abrir(id: number | string, archivosAdjuntos: ArchivoLegajo[] = []) {
		idActual.value = id
		archivos.value = archivosAdjuntos
		estaAbierto.value = true
		estaCargando.value = true
		error.value = null
		documento.value = null

		try {
			const respuesta = await perfilApi.getDocumento(id)
			documento.value = respuesta
		} catch (err: any) {
			error.value = err?.message || 'No se pudo cargar la información del documento'
		} finally {
			estaCargando.value = false
		}
	}

	function cerrar() {
		estaAbierto.value = false
	}

	return {
		estaAbierto,
		estaCargando,
		documento,
		archivos,
		error,
		idActual,
		abrir,
		cerrar,
	}
})
