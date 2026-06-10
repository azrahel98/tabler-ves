<template>
  <Card>
  
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between border-b border-stroke pt-0 px-3 pb-3 dark:border-strokedark gap-4">
      <div class="flex items-center gap-2.5">
        <div class="flex h-9 w-9 items-center justify-center rounded-lg bg-primary/10 text-primary dark:bg-white/5 dark:text-brand-300">
          <Files class="h-5 w-5" />
        </div>
        <div>
          <h3 class="text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-white/90">Legajo Virtual</h3>
          <p class="data-label normal-case tracking-normal">Gestión de documentos administrativos</p>
        </div>
      </div>
      
      <div class="flex items-center gap-2 w-full sm:w-auto">
        <button @click="abrirModalUrl" class="flex flex-1 sm:flex-none items-center justify-center gap-2 rounded-lg border border-stroke px-3 py-2 text-xs font-semibold text-gray-600 hover:bg-gray-50 dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-all">
          <Link2 class="h-3.5 w-3.5" />
          Vincular URL
        </button>
        <button @click="abrirModal" class="flex flex-1 sm:flex-none items-center justify-center gap-2 rounded-lg bg-primary px-4 py-2 text-xs font-semibold text-white hover:bg-opacity-90 transition-all shadow-sm">
          <Upload class="h-3.5 w-3.5" />
          Subir PDF
        </button>
      </div>
    </div>



    <div class="p-6 dark:bg-transparent relative">
      <div v-if="cargandoLista" class="flex flex-col items-center justify-center py-0 gap-4">
        <div class="relative flex h-min w-14 items-center justify-center rounded-full bg-primary/10">
          <Loader2 class="h-7 w-7 animate-spin text-primary" />
        </div>
        <p class="text-xs font-medium text-gray-500">Sincronizando legajo...</p>
      </div>

      <div v-else-if="archivosSubidos.length === 0" class="flex flex-col items-center justify-center py-0 gap-4 text-center">
        <div class="relative flex h-24 w-24 items-center justify-center rounded-full bg-white shadow-sm dark:bg-meta-4 mb-2 group-hover/folder:scale-105 transition-transform duration-500">
          <div class="absolute inset-0 rounded-full border-2 border-dashed border-gray-300 dark:border-strokedark animate-[spin_20s_linear_infinite]"></div>
          <FolderOpen class="h-10 w-10 text-gray-400" stroke-width="1.5" />
        </div>
        <div>
          <h4 class="text-xs font-semibold text-gray-800 dark:text-white/90">El legajo está vacío</h4>
          <p class="data-label normal-case tracking-normal mt-1 max-w-sm mx-auto">Sube tu primer documento o vincula una URL para empezar a organizar este legajo.</p>
        </div>
        <button @click="abrirModal" class="mt-2 text-xs font-semibold text-primary hover:text-primary/80 hover:underline underline-offset-4 transition-all">
          Subir archivo ahora &rarr;
        </button>
      </div>

      <div v-else class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4 sm:gap-5">
        <legajo-item
          v-for="(item, i) in archivosSubidos"
          :key="item.id || i"
          :item="item"
          @delete="eliminarDocumento(item.id)"
          @preview="abrirVistaPrevia(item)"
          @rename="abrirModalRenombrar(item)"
        />
      </div>
    </div>

    <modal :isOpen="mostrarModal" @close="cerrarModal">
      <template #header>
        <h3 class="text-title-md font-semibold leading-snug text-black dark:text-white flex items-center gap-2 m-0">
          <Upload class="h-5 w-5 text-primary" />
          Subir Nuevo Documento
        </h3>
      </template>

      <div class="space-y-4">
        <div>
          <label class="mb-1.5 block text-xs font-medium text-gray-700 dark:text-white/80">Archivo PDF (Máx 10MB) <span class="text-red-500">*</span></label>
          <input
            type="file"
            accept=".pdf"
            @change="manejarArchivo"
            class="w-full cursor-pointer rounded-lg border-[1.5px] border-stroke bg-transparent font-medium outline-none transition file:mr-4 file:border-0 file:bg-primary file:py-2.5 file:px-4 file:text-sm file:font-semibold file:text-white hover:file:bg-opacity-90 focus:border-primary active:border-primary disabled:cursor-default disabled:bg-whiter dark:border-form-strokedark dark:bg-form-input dark:file:bg-primary dark:file:text-white dark:focus:border-primary"
            ref="inputArchivo" />
        </div>

        <div>
          <label class="mb-1.5 block text-xs font-medium text-gray-700 dark:text-white/80">Nombre del documento (Opcional)</label>
          <input
            v-model="nombreArchivoOpcional"
            type="text"
            placeholder="Ej: Contrato de Trabajo 2024"
            class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary" />
          <p class="mt-1 text-2xs text-gray-500">Si se deja en blanco, se usará el nombre original del archivo.</p>
        </div>

        <div>
          <label class="mb-1.5 block text-xs font-medium text-gray-700 dark:text-white/80">Asociar a documento (Opcional)</label>
          <select
            v-model="documentoSeleccionado"
            class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary">
            <option :value="null">Sin asociar</option>
            <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">
              {{ doc.sigla }}
            </option>
          </select>
          <p class="mt-1 text-2xs text-gray-500">Selecciona un documento administrativo para vincular a este archivo.</p>
        </div>

        <p v-if="errorArchivo" class="text-xs text-red-500 font-medium">{{ errorArchivo }}</p>
        <p v-if="mensajeExito" class="text-xs text-green-500 font-medium">{{ mensajeExito }}</p>
      </div>

      <template #footer>
        <div class="flex justify-end gap-3 w-full">
          <button
            @click="cerrarModal"
            class="rounded-lg border border-gray-200 px-4 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50 dark:border-white/10 dark:text-white/80 dark:hover:bg-white/5 transition-colors">
            Cancelar
          </button>
          <button
            @click="subirArchivo"
            :disabled="cargando || !archivo"
            class="flex items-center justify-center gap-2 rounded-lg bg-primary px-6 py-1.5 text-xs font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all">
            <Loader2 v-if="cargando" class="h-4 w-4 animate-spin" />
            <span>{{ cargando ? 'Subiendo...' : 'Subir Archivo' }}</span>
          </button>
        </div>
      </template>
    </Modal>

    <PreviewModal :isOpen="mostrarVistaPrevia" @close="cerrarVistaPrevia" :url="urlPrevia" :documentoActual="documentoActual" :urlPrevia="urlPrevia" @documento-asignado="onDocumentoAsignado" />

    <modal :isOpen="mostrarModalRenombrar" @close="cerrarModalRenombrar">
      <template #header>
        <h3 class="text-title-md font-semibold leading-snug text-black dark:text-white flex items-center gap-2 m-0">
          <Pencil class="h-5 w-5 text-primary" />
          Renombrar documento
        </h3>
      </template>

      <div class="space-y-4">
        <div>
          <label class="mb-1.5 block text-xs font-medium text-gray-700 dark:text-white/80">Nuevo nombre <span class="text-red-500">*</span></label>
          <input
            v-model="nuevoNombreArchivo"
            type="text"
            placeholder="Ej: Contrato de Trabajo 2024"
            class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary"
            @keyup.enter="renombrarArchivo" />
          <p class="mt-1 text-2xs text-gray-500">La extensión .pdf se agrega automáticamente.</p>
        </div>

        <p v-if="errorRenombrar" class="text-xs text-red-500 font-medium">{{ errorRenombrar }}</p>
      </div>

      <template #footer>
        <div class="flex justify-end gap-3 w-full">
          <button
            @click="cerrarModalRenombrar"
            class="rounded-lg border border-gray-200 px-4 py-1.5 text-xs font-medium text-gray-700 hover:bg-gray-50 dark:border-white/10 dark:text-white/80 dark:hover:bg-white/5 transition-colors">
            Cancelar
          </button>
          <button
            @click="renombrarArchivo"
            :disabled="cargandoRenombrar || !nuevoNombreArchivo.trim()"
            class="flex items-center justify-center gap-2 rounded-lg bg-primary px-6 py-1.5 text-xs font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all">
            <Loader2 v-if="cargandoRenombrar" class="h-4 w-4 animate-spin" />
            <span>{{ cargandoRenombrar ? 'Guardando...' : 'Guardar' }}</span>
          </button>
        </div>
      </template>
    </modal>

    <RegistrarUrlModal
      :isOpen="mostrarModalUrl"
      :dni="perfilActual?.dni ?? null"
      :documentosDisponibles="documentosDisponibles"
      @close="mostrarModalUrl = false"
      @registrado="onUrlRegistrada" />

  </Card>
</template>

<script setup lang="ts">
  import LegajoItem from './items.vue'
  import { ref, watch, defineAsyncComponent } from 'vue'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'
  import api from '../../../services/api'
  import { Upload, Loader2, FolderOpen, Link2, Files, Pencil } from 'lucide-vue-next'
  import { baseURL } from '../../../services/api'
  import Card from '../../ui/card.vue'

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const PreviewModal = defineAsyncComponent(() => import('./PreviewModal.vue'))
  const Modal = defineAsyncComponent(() => import('../../ui/Modal.vue'))
  const RegistrarUrlModal = defineAsyncComponent(() => import('./RegistrarUrlModal.vue'))

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

  const mostrarModalUrl = ref(false)

  const mostrarModalRenombrar = ref(false)
  const archivoARenombrar = ref<any>(null)
  const nuevoNombreArchivo = ref('')
  const errorRenombrar = ref('')
  const cargandoRenombrar = ref(false)
  


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

  const abrirModalRenombrar = (item: any) => {
    archivoARenombrar.value = item
    const nombre = item.original_name || ''
    const ultimoPunto = nombre.lastIndexOf('.')
    nuevoNombreArchivo.value = ultimoPunto > 0 ? nombre.slice(0, ultimoPunto) : nombre
    errorRenombrar.value = ''
    mostrarModalRenombrar.value = true
  }

  const cerrarModalRenombrar = () => {
    mostrarModalRenombrar.value = false
    archivoARenombrar.value = null
    nuevoNombreArchivo.value = ''
    errorRenombrar.value = ''
  }

  const renombrarArchivo = async () => {
    if (!nuevoNombreArchivo.value.trim() || !archivoARenombrar.value) return

    cargandoRenombrar.value = true
    errorRenombrar.value = ''

    try {
      const response = await api.post('/fileserver/renombrar_archivo', {
        id: archivoARenombrar.value.id,
        nuevo_nombre: nuevoNombreArchivo.value.trim(),
      })

      const archivo = archivosSubidos.value.find((a) => a.id === archivoARenombrar.value.id)
      if (archivo) {
        archivo.original_name = response.data.original_name
      }

      cerrarModalRenombrar()
    } catch (error: any) {
      errorRenombrar.value = error.response?.data?.error || 'No se pudo renombrar el documento'
    } finally {
      cargandoRenombrar.value = false
    }
  }

  const abrirModalUrl = () => {
    mostrarModalUrl.value = true
    cargarDocumentos()
  }

  const onUrlRegistrada = (archivo: any) => {
    if (archivo) {
      archivosSubidos.value.unshift(archivo)
    } else {
      buscarDocumentos()
    }
  }

  const abrirVistaPrevia = async (doc: any) => {
    if (doc.external_url) {
      window.open(doc.external_url, '_blank')
      return
    }

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

