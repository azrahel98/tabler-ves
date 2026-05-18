<template>
  <Modal :isOpen="isOpen" @close="close" maxWidth="!w-[96vw] !max-w-[96vw]">
    <template #header>
      <h3 class="text-title-md font-semibold leading-snug text-black dark:text-white flex items-center gap-2 m-0 truncate pr-4">
        <FileText class="h-5 w-5 text-primary shrink-0" />
        <span class="truncate">{{ documentoActual?.original_name || 'Vista Previa del Documento' }}</span>
      </h3>
    </template>

    <div class="mb-4">
      <label class="mb-2 block text-sm font-medium text-black dark:text-white">Documento asociado</label>
      <div class="flex flex-col sm:flex-row gap-2 sm:items-center">
        <select
          v-model="documentoLocal"
          class="flex-1 rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary">
          <option :value="null">Sin asociar</option>
          <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">{{ doc.sigla }}</option>
        </select>
        <button
          @click="guardarAsociacion"
          :disabled="guardando || documentoLocal === documentoActual?.documento_id"
          class="flex items-center justify-center gap-2 rounded-lg bg-primary px-4 py-2 font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all">
          <Loader2 v-if="guardando" class="h-4 w-4 animate-spin" />
          <Save v-else class="h-4 w-4" />
          Guardar Asociación
        </button>
      </div>
      <p v-if="mensajeAsociacion" class="mt-1 text-xs font-medium" :class="errorAsociacion ? 'text-red-500' : 'text-green-500'">
        {{ mensajeAsociacion }}
      </p>
    </div>

    <div class="relative w-full bg-gray-100 dark:bg-gray-900 rounded-xl overflow-hidden flex flex-col" style="height: 75vh">

      <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-800 border-b border-stroke dark:border-strokedark shrink-0">
        <div class="flex items-center gap-1">
          <button
            @click="paginaAnterior"
            :disabled="paginaActual <= 1 || cargandoPdf"
            class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-stroke text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
            <ChevronLeft class="h-3.5 w-3.5" />
          </button>
          <span class="text-xs font-medium text-gray-600 dark:text-gray-400 px-2 tabular-nums">
            {{ paginaActual }} / {{ totalPaginas || '—' }}
          </span>
          <button
            @click="paginaSiguiente"
            :disabled="paginaActual >= totalPaginas || cargandoPdf"
            class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-stroke text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
            <ChevronRight class="h-3.5 w-3.5" />
          </button>
        </div>

        <div class="flex items-center gap-1">
          <button
            @click="ajustarZoom(-0.25)"
            :disabled="escala <= 0.5"
            class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-stroke text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
            <Minus class="h-3.5 w-3.5" />
          </button>
          <span class="text-xs font-medium text-gray-600 dark:text-gray-400 w-12 text-center tabular-nums">
            {{ Math.round(escala * 100) }}%
          </span>
          <button
            @click="ajustarZoom(0.25)"
            :disabled="escala >= 3"
            class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-stroke text-gray-500 hover:bg-gray-100 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
            <Plus class="h-3.5 w-3.5" />
          </button>
          <button
            @click="resetZoom"
            class="ml-1 inline-flex items-center justify-center h-7 px-2 rounded-md border border-stroke text-xs text-gray-500 hover:bg-gray-100 dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
            Ajustar
          </button>
        </div>
      </div>

      <div ref="contenedorRef" class="flex-1 overflow-auto flex items-start justify-center p-4">
        <canvas ref="canvasRef" class="shadow-lg rounded max-w-none"></canvas>
      </div>

      <Transition name="fade">
        <div v-if="cargandoPdf" class="absolute inset-0 flex flex-col items-center justify-center bg-white/80 dark:bg-gray-900/80 z-10">
          <Loader2 class="h-8 w-8 animate-spin text-primary mb-3" />
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">Cargando PDF...</p>
        </div>
      </Transition>

      <div v-if="errorPdf && !cargandoPdf" class="absolute inset-0 flex flex-col items-center justify-center z-10 p-6 text-center">
        <AlertCircle class="h-10 w-10 text-red-400 mb-3" />
        <p class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">No se pudo cargar el PDF</p>
        <p class="text-xs text-gray-500 dark:text-gray-400 mb-4">{{ errorPdf }}</p>
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          target="_blank"
          class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-white hover:bg-opacity-90 transition-all">
          <ExternalLink class="h-4 w-4" />
          Abrir en pestaña nueva
        </a>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3 w-full">
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          target="_blank"
          rel="noopener noreferrer"
          class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-black hover:bg-gray-100 dark:border-strokedark dark:text-white dark:hover:bg-meta-4 transition-colors flex items-center gap-2">
          <ExternalLink class="h-4 w-4" />
          Abrir en pestaña
        </a>
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          :download="documentoActual?.original_name"
          class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-black hover:bg-gray-100 dark:border-strokedark dark:text-white dark:hover:bg-meta-4 transition-colors flex items-center gap-2">
          <Download class="h-4 w-4" />
          Descargar
        </a>
        <button
          @click="close"
          class="rounded-lg bg-primary px-6 py-2 text-sm font-medium text-white hover:bg-opacity-90 transition-all">
          Cerrar
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import Modal from '../../ui/Modal.vue'
  import { Loader2, Download, FileText, Save, ChevronLeft, ChevronRight, Minus, Plus, AlertCircle, ExternalLink } from 'lucide-vue-next'
  import { ref, watch, onUnmounted } from 'vue'
  import * as pdfjsLib from 'pdfjs-dist'
  import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.min.mjs?url'
  import api from '../../../services/api'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const props = defineProps<{
    isOpen: boolean
    url: string | null
    documentoActual: any
    urlPrevia: string | null
  }>()

  const emit = defineEmits(['close', 'documento-asignado'])

  const canvasRef = ref<HTMLCanvasElement | null>(null)
  const contenedorRef = ref<HTMLElement | null>(null)

  const paginaActual = ref(1)
  const totalPaginas = ref(0)
  const escala = ref(1.2)
  const cargandoPdf = ref(false)
  const errorPdf = ref('')

  const documentoLocal = ref<number | null>(null)
  const documentosDisponibles = ref<any[]>([])
  const guardando = ref(false)
  const mensajeAsociacion = ref('')
  const errorAsociacion = ref(false)

  let pdfDoc: any = null
  let tareaRender: any = null

  const renderPagina = async (num: number) => {
    if (!pdfDoc || !canvasRef.value) return

    if (tareaRender) {
      tareaRender.cancel()
      tareaRender = null
    }

    const pagina = await pdfDoc.getPage(num)
    const viewport = pagina.getViewport({ scale: escala.value })

    const canvas = canvasRef.value
    const ctx = canvas.getContext('2d')!
    canvas.width = viewport.width
    canvas.height = viewport.height

    tareaRender = pagina.render({ canvasContext: ctx, viewport })
    await tareaRender.promise
    tareaRender = null
  }

  const cargarPdf = async (url: string) => {
    cargandoPdf.value = true
    errorPdf.value = ''
    paginaActual.value = 1
    totalPaginas.value = 0

    if (pdfDoc) {
      pdfDoc.destroy()
      pdfDoc = null
    }

    try {
      const tarea = pdfjsLib.getDocument({
        url,
        cMapPacked: true,
        withCredentials: false,
      })
      pdfDoc = await tarea.promise
      totalPaginas.value = pdfDoc.numPages

      await renderPagina(1)

      escala.value = calcularEscalaAjustada()
      await renderPagina(1)
    } catch (e: any) {
      errorPdf.value = e?.message || 'Error desconocido'
    } finally {
      cargandoPdf.value = false
    }
  }

  const calcularEscalaAjustada = (): number => {
    if (!contenedorRef.value || !canvasRef.value) return 1.2
    const anchoContenedor = contenedorRef.value.clientWidth - 32
    const anchoCanvas = canvasRef.value.width / escala.value
    const nueva = anchoContenedor / anchoCanvas
    return Math.min(Math.max(nueva, 0.5), 3)
  }

  const paginaAnterior = async () => {
    if (paginaActual.value <= 1) return
    paginaActual.value--
    await renderPagina(paginaActual.value)
  }

  const paginaSiguiente = async () => {
    if (paginaActual.value >= totalPaginas.value) return
    paginaActual.value++
    await renderPagina(paginaActual.value)
  }

  const ajustarZoom = async (delta: number) => {
    const nueva = Math.min(Math.max(escala.value + delta, 0.5), 3)
    escala.value = nueva
    await renderPagina(paginaActual.value)
  }

  const resetZoom = async () => {
    escala.value = calcularEscalaAjustada()
    await renderPagina(paginaActual.value)
  }

  const cargarDocumentos = async () => {
    const dni = perfilActual.value?.dni
    if (!dni) return
    try {
      const response = await api.post('/fileserver/documentos_por_dni', { dni })
      documentosDisponibles.value = response.data || []
    } catch {}
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
      emit('documento-asignado', { fileId: props.documentoActual.id, documentoId: documentoLocal.value })
      setTimeout(() => { mensajeAsociacion.value = '' }, 2000)
    } catch (error: any) {
      errorAsociacion.value = true
      mensajeAsociacion.value = error.response?.data?.error || 'Error al asignar documento'
    } finally {
      guardando.value = false
    }
  }

  watch(
    () => props.isOpen,
    async (abierto) => {
      if (abierto) {
        documentoLocal.value = props.documentoActual?.documento_id ?? null
        mensajeAsociacion.value = ''
        errorAsociacion.value = false
        cargarDocumentos()

        if (props.urlPrevia) {
          await cargarPdf(props.urlPrevia)
        }
      } else {
        if (pdfDoc) {
          pdfDoc.destroy()
          pdfDoc = null
        }
      }
    }
  )

  onUnmounted(() => {
    if (pdfDoc) {
      pdfDoc.destroy()
      pdfDoc = null
    }
  })

  const close = () => emit('close')
</script>

<style scoped>
  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.2s ease;
  }
  .fade-enter-from,
  .fade-leave-to {
    opacity: 0;
  }
</style>
