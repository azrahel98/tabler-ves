<template>
  <Modal :isOpen="isOpen" @close="close" maxWidth="!w-[96vw] !max-w-[96vw]">
    <template #header>
      <h3 class="text-base font-semibold tracking-tight leading-snug text-gray-800 dark:text-white/90 flex items-center gap-2 m-0 truncate pr-4">
        <FileText class="h-4.5 w-4.5 text-primary shrink-0" />
        <span class="truncate">{{ documentoActual?.original_name || 'Vista Previa del Documento' }}</span>
      </h3>
    </template>

    <div class="relative w-full bg-gray-100 dark:bg-gray-900 rounded-xl overflow-hidden flex flex-col" style="height: 75vh">

      <div class="flex items-center justify-between px-4 py-2 bg-white dark:bg-gray-800 border-b border-gray-100 dark:border-white/5 shrink-0">
        <!-- Controles de Página -->
        <div class="flex items-center gap-1">
          <template v-if="!usarVisorNativo">
            <button
              @click="paginaAnterior"
              :disabled="paginaActual <= 1 || cargandoPdf"
              class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-gray-200 dark:border-white/10 text-gray-500 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
              <ChevronLeft class="h-3.5 w-3.5" />
            </button>
            <span class="text-2xs font-semibold tracking-wider text-gray-400 dark:text-gray-500 uppercase px-2 tabular-nums">
              Pág. {{ paginaActual }} / {{ totalPaginas || '—' }}
            </span>
            <button
              @click="paginaSiguiente"
              :disabled="paginaActual >= totalPaginas || cargandoPdf"
              class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-gray-200 dark:border-white/10 text-gray-500 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:border-strokedark dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
              <ChevronRight class="h-3.5 w-3.5" />
            </button>
          </template>
          <span v-else class="text-xs font-semibold text-gray-500 dark:text-gray-400 flex items-center gap-1.5 py-1">
            <span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
            Visor Nativo del Navegador
          </span>
        </div>

        <!-- Controles de Zoom / Alternador -->
        <div class="flex items-center gap-1">
          <template v-if="!usarVisorNativo">
            <button
              @click="ajustarZoom(-0.25)"
              :disabled="escala <= 0.5"
              class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-gray-200 dark:border-white/10 text-gray-500 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
              <Minus class="h-3.5 w-3.5" />
            </button>
            <span class="text-2xs font-semibold tracking-wider text-gray-400 dark:text-gray-500 w-12 text-center tabular-nums">
              {{ Math.round(escala * 100) }}%
            </span>
            <button
              @click="ajustarZoom(0.25)"
              :disabled="escala >= 3"
              class="inline-flex items-center justify-center h-7 w-7 rounded-md border border-gray-200 dark:border-white/10 text-gray-500 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed dark:text-gray-400 dark:hover:bg-white/5 transition-colors">
              <Plus class="h-3.5 w-3.5" />
            </button>
            <button
              @click="resetZoom"
              class="ml-1 inline-flex items-center justify-center h-7 px-2.5 rounded-md border border-gray-200 dark:border-white/10 text-2xs font-semibold uppercase tracking-wider text-gray-500 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
              Ajustar
            </button>
          </template>

          <button
            @click="usarVisorNativo = !usarVisorNativo"
            class="ml-1.5 inline-flex items-center justify-center h-7 px-2.5 rounded-md border border-gray-200 dark:border-white/10 text-2xs font-bold uppercase tracking-wider text-primary hover:bg-primary/5 dark:text-brand-300 dark:hover:bg-white/5 transition-colors">
            {{ usarVisorNativo ? 'Usar Visor Pro' : 'Usar Visor Navegador' }}
          </button>
        </div>
      </div>

      <div ref="contenedorRef" class="flex-1 overflow-auto flex items-start justify-center p-4">
        <canvas v-show="!usarVisorNativo" ref="canvasRef" class="shadow-lg rounded max-w-none"></canvas>
        <iframe v-if="usarVisorNativo && urlPrevia" :src="urlPrevia" class="w-full h-full border-none bg-white rounded-lg"></iframe>
      </div>

      <Transition name="fade">
        <div v-if="cargandoPdf && !usarVisorNativo" class="absolute inset-0 flex flex-col items-center justify-center bg-white/80 dark:bg-gray-900/80 z-10">
          <Loader2 class="h-8 w-8 animate-spin text-primary mb-3" />
          <p class="text-sm font-medium text-gray-500 dark:text-gray-400">Cargando PDF...</p>
        </div>
      </Transition>
    </div>

    <template #footer>
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 w-full">
        <!-- Izquierda: Documento Asociado -->
        <div class="flex-1 min-w-0 max-w-md w-full relative">
          <div class="flex items-center gap-2">
            <span class="text-2xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 whitespace-nowrap">Asociar:</span>
            <div class="relative flex-1 flex gap-1.5 items-center">
              <select
                v-model="documentoLocal"
                class="w-full text-xs rounded-lg border border-gray-200 dark:border-white/10 bg-transparent px-3 py-1.5 outline-none transition focus:border-primary active:border-primary dark:bg-white/5 dark:text-white/80">
                <option :value="null">Sin asociar</option>
                <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">{{ doc.sigla }}</option>
              </select>
              <button
                @click="guardarAsociacion"
                :disabled="guardando || documentoLocal === documentoActual?.documento_id"
                class="flex items-center justify-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-semibold text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all whitespace-nowrap">
                <Loader2 v-if="guardando" class="h-3 w-3 animate-spin" />
                <Save v-else class="h-3 w-3" />
                Asociar
              </button>
            </div>
          </div>
          <p v-if="mensajeAsociacion" class="absolute left-[54px] mt-1 text-3xs font-medium" :class="errorAsociacion ? 'text-red-500' : 'text-green-500'">
            {{ mensajeAsociacion }}
          </p>
        </div>

        <!-- Derecha: Acciones del Documento -->
        <div class="flex items-center justify-end gap-2.5 shrink-0 w-full md:w-auto">
          <a
            v-if="urlPrevia"
            :href="urlPrevia"
            target="_blank"
            rel="noopener noreferrer"
            class="rounded-lg border border-gray-200 dark:border-white/10 px-3.5 py-1.5 text-xs font-semibold text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors flex items-center gap-1.5">
            <ExternalLink class="h-3.5 w-3.5" />
            Abrir
          </a>
          <a
            v-if="urlPrevia"
            :href="urlPrevia"
            :download="documentoActual?.original_name"
            class="rounded-lg border border-gray-200 dark:border-white/10 px-3.5 py-1.5 text-xs font-semibold text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors flex items-center gap-1.5">
            <Download class="h-3.5 w-3.5" />
            Descargar
          </a>
          <button
            @click="close"
            class="rounded-lg bg-primary px-4 py-1.5 text-xs font-semibold text-white hover:bg-opacity-90 transition-all">
            Cerrar
          </button>
        </div>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import Modal from '../../ui/Modal.vue'
  import { Loader2, Download, FileText, Save, ChevronLeft, ChevronRight, Minus, Plus, ExternalLink } from 'lucide-vue-next'
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
  const escala = ref(1.0)
  const cargandoPdf = ref(false)
  const errorPdf = ref('')
  const usarVisorNativo = ref(false)

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
      const response = await api.get(url, { responseType: 'arraybuffer' })
      const data = new Uint8Array(response.data)
      const tarea = pdfjsLib.getDocument({
        data,
        cMapPacked: true,
      })
      pdfDoc = await tarea.promise
      totalPaginas.value = pdfDoc.numPages

      await renderPagina(1)
      await renderPagina(1)
    } catch (e: any) {
      errorPdf.value = e?.message || 'Error al procesar el documento en Canvas'
      usarVisorNativo.value = true
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
        usarVisorNativo.value = false
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
