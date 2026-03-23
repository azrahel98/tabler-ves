<template>
  <div class="rounded-2xl border border-stroke bg-white shadow-sm dark:border-strokedark dark:bg-boxdark overflow-hidden relative">
    <div class="flex items-center justify-between border-b border-stroke p-6 dark:border-strokedark">
      <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z" />
        </svg>
        VIRTUAL FOLDER (LEGAJO)
      </div>
      <button @click="abrirModal" class="flex items-center gap-2 rounded bg-primary px-3 py-1.5 text-xs font-medium text-white hover:bg-opacity-90 transition-all">
        <Upload class="h-4 w-4" />
        Subir Documento
      </button>
    </div>

    <div class="p-6">
      <div v-if="cargandoLista" class="flex justify-center p-8">
        <Loader2 class="h-8 w-8 animate-spin text-primary" />
      </div>
      <div v-else-if="archivosSubidos.length === 0" class="text-center text-gray-500 py-8">No hay documentos subidos aún.</div>
      <div v-else class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        <legajo-item v-for="(item, i) in archivosSubidos" :key="item.id || i" :item="item" @delete="eliminarDocumento(item.id)" @preview="abrirVistaPrevia(item)" />
      </div>
    </div>


     <modal :isOpen="mostrarModal" @close="cerrarModal">
      <template #header>
        <h3 class="text-lg font-bold text-black dark:text-white flex items-center gap-2 m-0">
          <Upload class="h-5 w-5 text-primary" />
          Subir Nuevo Documento
        </h3>
      </template>

      <div class="space-y-4">
        <div>
          <label class="mb-2 block text-sm font-medium text-black dark:text-white">Archivo PDF (Máx 10MB) <span class="text-red-500">*</span></label>
          <input
            type="file"
            accept=".pdf"
            @change="manejarArchivo"
            class="w-full cursor-pointer rounded-lg border-[1.5px] border-stroke bg-transparent font-medium outline-none transition file:mr-4 file:border-0 file:bg-primary file:py-2.5 file:px-4 file:text-sm file:font-semibold file:text-white hover:file:bg-opacity-90 focus:border-primary active:border-primary disabled:cursor-default disabled:bg-whiter dark:border-form-strokedark dark:bg-form-input dark:file:bg-primary dark:file:text-white dark:focus:border-primary"
            ref="inputArchivo" />
        </div>

        <div>
          <label class="mb-2 block text-sm font-medium text-black dark:text-white">Nombre del documento (Opcional)</label>
          <input
            v-model="nombreArchivoOpcional"
            type="text"
            placeholder="Ej: Contrato de Trabajo 2024"
            class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary" />
          <p class="mt-1 text-[10px] text-gray-500">Si se deja en blanco, se usará el nombre original del archivo.</p>
        </div>

        <div>
          <label class="mb-2 block text-sm font-medium text-black dark:text-white">Asociar a documento (Opcional)</label>
          <select
            v-model="documentoSeleccionado"
            class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary">
            <option :value="null">Sin asociar</option>
            <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">
              {{ doc.sigla }}
            </option>
          </select>
          <p class="mt-1 text-[10px] text-gray-500">Selecciona un documento administrativo para vincular a este archivo.</p>
        </div>

        <p v-if="errorArchivo" class="text-xs text-red-500 font-medium">{{ errorArchivo }}</p>
        <p v-if="mensajeExito" class="text-xs text-green-500 font-medium">{{ mensajeExito }}</p>
      </div>

      <template #footer>
        <div class="flex justify-end gap-3 w-full">
          <button
            @click="cerrarModal"
            class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-black hover:bg-gray-100 dark:border-strokedark dark:text-white dark:hover:bg-meta-4 transition-colors">
            Cancelar
          </button>
          <button
            @click="subirArchivo"
            :disabled="cargando || !archivo"
            class="flex items-center justify-center gap-2 rounded-lg bg-primary px-6 py-2 text-sm font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all">
            <Loader2 v-if="cargando" class="h-4 w-4 animate-spin" />
            <span>{{ cargando ? 'Subiendo...' : 'Subir Archivo' }}</span>
          </button>
        </div>
      </template>
    </Modal>

    <PreviewModal :isOpen="mostrarVistaPrevia" @close="cerrarVistaPrevia" :url="urlPrevia" :documentoActual="documentoActual" :urlPrevia="urlPrevia" @documento-asignado="onDocumentoAsignado" />

     </div>
</template>

<script setup lang="ts">
  import LegajoItem from './items.vue'
  import { ref, watch,defineAsyncComponent } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'
  import api from '../../../services/api'
  import { Upload, Loader2 } from 'lucide-vue-next'
  import { baseURL } from '../../../services/api'

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const PreviewModal = defineAsyncComponent(() => import('./PreviewModal.vue'))
  const Modal = defineAsyncComponent(() => import('../../ui/Modal.vue'))

  const archivo = ref<File | null>(null)
  const errorArchivo = ref('')
  const cargando = ref(false)
  const cargandoLista = ref(false)
  const mensajeExito = ref('')
  const inputArchivo = ref<HTMLInputElement | null>(null)
  const archivosSubidos = ref<any[]>([])

  const mostrarModal = ref(false)
  const nombreArchivoOpcional = ref('')
  const documentoSeleccionado = ref<number | null>(null)
  const documentosDisponibles = ref<any[]>([])

  const mostrarVistaPrevia = ref(false)
  const urlPrevia = ref<string | null>(null)
  const documentoActual = ref<any>(null)
  
  const apiUrlBase = baseURL

  const cargarDocumentos = async () => {
    const dni = perfilActual.value?.dni
    if (!dni) return
    try {
      const response = await api.post('/fileserver/documentos_por_dni', { dni })
      documentosDisponibles.value = response.data || []
    } catch (error) {
      console.error('Error al cargar documentos:', error)
    }
  }

  const abrirModal = () => {
    mostrarModal.value = true
    errorArchivo.value = ''
    mensajeExito.value = ''
    archivo.value = null
    nombreArchivoOpcional.value = ''
    documentoSeleccionado.value = null
    if (inputArchivo.value) inputArchivo.value.value = ''
    cargarDocumentos()
  }

  const cerrarModal = () => {
    mostrarModal.value = false
  }

  const abrirVistaPrevia = async (doc: any) => {
    documentoActual.value = doc
    mostrarVistaPrevia.value = true
    urlPrevia.value = null 
    
    if (doc.file_hash) {
       urlPrevia.value = `${apiUrlBase}/fileserver/${doc.file_hash}`
    } else {
       console.error("El documento no tiene un hash válido para visualizar");
       cerrarVistaPrevia();
       alert("No se puede visualizar este documento (falta file_hash).");
    }
  }

  const cerrarVistaPrevia = () => {
    mostrarVistaPrevia.value = false
    documentoActual.value = null
    urlPrevia.value = null
  }

  const onDocumentoAsignado = ({ fileId, documentoId }: { fileId: number; documentoId: number | null }) => {
    const archivo = archivosSubidos.value.find((a) => a.id === fileId)
    if (archivo) {
      archivo.documento_id = documentoId
    }
  }



  const buscarDocumentos = async () => {
    const dni = perfilActual.value?.dni
    if (!dni) return

    cargandoLista.value = true
    try {
      const response = await api.post('/fileserver/listar_archivos_dni', { dni })
      archivosSubidos.value = response.data || []
    } catch (error) {
      console.error('Error al listar documentos:', error)
    } finally {
      cargandoLista.value = false
    }
  }

  const eliminarDocumento = async (id: number) => {
    if (!confirm('¿Estás seguro de que deseas eliminar este documento?')) return

    try {
      await api.post('/fileserver/eliminar_archivo', { id })
      archivosSubidos.value = archivosSubidos.value.filter((doc) => doc.id !== id)
    } catch (error: any) {
      console.error('Error al eliminar documento:', error)
      alert(error.response?.data?.error || 'No se pudo eliminar el documento')
    }
  }

  watch(
    perfilActual,
    () => {
      if (perfilActual.value?.dni) {
        buscarDocumentos()
      } else {
        archivosSubidos.value = []
      }
    },
    { immediate: true }
  )

  const manejarArchivo = (event: Event) => {
    errorArchivo.value = ''
    mensajeExito.value = ''
    archivo.value = null

    const target = event.target as HTMLInputElement
    if (target.files && target.files.length > 0) {
      const file = target.files[0]
      if (!file) return

      if (file.type !== 'application/pdf') {
        errorArchivo.value = 'Solo se permiten archivos en formato PDF'
        if (inputArchivo.value) inputArchivo.value.value = ''
        return
      }

      const maxSize = 10 * 1024 * 1024
      if (file.size > maxSize) {
        errorArchivo.value = 'El archivo no debe superar los 10MB'
        if (inputArchivo.value) inputArchivo.value.value = ''
        return
      }

      archivo.value = file
    }
  }

  const subirArchivo = async () => {
    if (!archivo.value) {
      errorArchivo.value = 'Por favor selecciona un archivo'
      return
    }

    const dni = perfilActual.value?.dni
    if (!dni) {
      errorArchivo.value = 'No se encontró el DNI asociado'
      return
    }

    cargando.value = true
    errorArchivo.value = ''
    mensajeExito.value = ''

    try {
      const formData = new FormData()

      let nombreAEnviar = archivo.value.name
      if (nombreArchivoOpcional.value.trim()) {
        const extension = archivo.value.name.split('.').pop()
        let nuevoNombre = nombreArchivoOpcional.value.trim()
        if (extension && !nuevoNombre.toLowerCase().endsWith(`.${extension.toLowerCase()}`)) {
          nuevoNombre += `.${extension}`
        }
        nombreAEnviar = nuevoNombre
      }

      formData.append('file', archivo.value, nombreAEnviar)
      formData.append('dni_asociado', dni)
      if (documentoSeleccionado.value) {
        formData.append('documento_id', documentoSeleccionado.value.toString())
      }

      const response = await api.post('/fileserver/upload', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      })

      if (response.data && Array.isArray(response.data) && response.data.length > 0) {
        mensajeExito.value = 'Archivo subido con éxito'
        archivosSubidos.value.unshift(response.data[0])

        archivo.value = null
        nombreArchivoOpcional.value = ''
        if (inputArchivo.value) inputArchivo.value.value = ''

        setTimeout(() => {
          mensajeExito.value = ''
          cerrarModal()
        }, 1500)
      }
    } catch (error: any) {
      console.error('Error al subir archivo:', error)
      errorArchivo.value = error.response?.data?.error || 'Ocurrió un error al subir el archivo'
    } finally {
      cargando.value = false
    }
  }
</script>
