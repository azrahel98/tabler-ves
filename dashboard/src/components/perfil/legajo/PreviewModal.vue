<template>
  <Modal :isOpen="isOpen" @close="close" maxWidth="w-full sm:max-w-3xl md:max-w-5xl lg:max-w-6xl xl:max-w-[90vw]">
    <div class="w-full h-[80vh] flex flex-col items-center justify-center bg-gray-100/50 dark:bg-gray-900/50 rounded-xl border border-gray-200 dark:border-gray-800 overflow-hidden relative">
      <iframe v-if="urlPrevia" :src="urlPrevia" class="w-full h-full border-0 absolute inset-0 z-20" title="Vista previa del documento PDF"> </iframe>

      <div v-else class="flex flex-col items-center justify-center text-gray-500 z-10 w-full h-full absolute inset-0 bg-white dark:bg-gray-900">
        <Loader2 class="h-8 w-8 animate-spin mb-4 text-brand-500" />
        <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Cargando vista previa...</p>
      </div>
    </div>

    <div class="mt-4 px-1">
      <label class="mb-2 block text-sm font-medium text-black dark:text-white">
        <FileText class="inline h-4 w-4 mr-1" />
        Documento asociado
      </label>
      <div class="flex gap-2 items-center">
        <select
          v-model="documentoLocal"
          class="flex-1 rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2 text-sm outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary">
          <option :value="null">Sin asociar</option>
          <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">
            {{ doc.sigla }}
          </option>
        </select>
        <button
          @click="guardarAsociacion"
          :disabled="guardando || documentoLocal === documentoActual?.documento_id"
          class="flex items-center gap-1.5 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all whitespace-nowrap">
          <Loader2 v-if="guardando" class="h-3.5 w-3.5 animate-spin" />
          <Save v-else class="h-3.5 w-3.5" />
          Guardar
        </button>
      </div>
      <p v-if="mensajeAsociacion" class="mt-1 text-xs font-medium" :class="errorAsociacion ? 'text-red-500' : 'text-green-500'">{{ mensajeAsociacion }}</p>
    </div>

    <template #footer>
      <div class="flex flex-col-reverse sm:flex-row justify-between w-full gap-3 sm:gap-0">
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          :download="documentoActual?.original_name"
          class="inline-flex w-full sm:w-auto justify-center items-center gap-2 rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700">
          <Download class="h-4 w-4" />
          Descargar Archivo
        </a>
        <span v-else></span>

        <button
          @click="close"
          class="inline-flex w-full sm:w-auto justify-center items-center rounded-lg bg-brand-600 px-8 py-2 text-sm font-medium text-white shadow-sm hover:bg-brand-700 transition">
          Cerrar Vista
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import Modal from '../../ui/Modal.vue'
  import { Loader2, Download, FileText, Save } from 'lucide-vue-next'
  import { ref, watch } from 'vue'
  import api from '../../../services/api'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const props = defineProps<{
    isOpen: boolean
    url: string | null
    documentoActual: any
    urlPrevia: string | null
  }>()

  const emit = defineEmits(['close', 'documento-asignado'])

  const documentoLocal = ref<number | null>(null)
  const documentosDisponibles = ref<any[]>([])
  const guardando = ref(false)
  const mensajeAsociacion = ref('')
  const errorAsociacion = ref(false)

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

  const guardarAsociacion = async () => {
    if (!props.documentoActual?.id) return

    guardando.value = true
    mensajeAsociacion.value = ''
    errorAsociacion.value = false

    try {
      await api.post('/fileserver/asignar_documento', {
        id: props.documentoActual.id,
        documento_id: documentoLocal.value,
      })
      mensajeAsociacion.value = 'Documento asociado correctamente'
      emit('documento-asignado', {
        fileId: props.documentoActual.id,
        documentoId: documentoLocal.value,
      })
      setTimeout(() => {
        mensajeAsociacion.value = ''
      }, 2000)
    } catch (error: any) {
      errorAsociacion.value = true
      mensajeAsociacion.value = error.response?.data?.error || 'Error al asignar documento'
    } finally {
      guardando.value = false
    }
  }

  watch(
    () => props.isOpen,
    (open) => {
      if (open) {
        documentoLocal.value = props.documentoActual?.documento_id ?? null
        mensajeAsociacion.value = ''
        errorAsociacion.value = false
        cargarDocumentos()
      }
    }
  )

  const close = () => {
    emit('close')
  }
</script>
