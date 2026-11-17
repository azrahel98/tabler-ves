<script setup lang="ts">
import Modal from '@/components/ui/modal/Modal.vue'
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import { type DocumentoVinculoInfo } from '@/components/perfil/types'
import {
  type PersonalArchivo,
  type TipoDocumentoOption,
  type AreaOption,
  getTipoEventoLabel,
  getFileDownloadUrl,
  fetchTiposDocumentos,
  fetchAreas,
  requiereArea,
  obtenerDocumentoPorId,
  editarDocumento,
} from '@/services/personal'
import { useToastStore } from '@/stores/toast'
import { getApiBaseUrl, fetchAuthBlob } from '@/services/api'
import { formatDate } from '@/utils/date'
import {
  IconX,
  IconFileText,
  IconFileCheck,
  IconFileCode,
  IconFileTypePdf,
  IconLink,
  IconAlertCircle,
  IconAlertTriangle,
  IconCalendar,
  IconTrash,
  IconEye,
  IconExternalLink,
  IconFileAlert,
  IconShieldCheck,
  IconCopy,
  IconCheck,
  IconUpload,
  IconNotes,
  IconFileDescription,
  IconHash,
  IconId,
  IconRefresh,
  IconPencil,
  IconBuildingSkyscraper,
  IconLoader2,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  documento: DocumentoVinculoInfo | null
  archivo?: PersonalArchivo | null
  isDeleting?: boolean
  dni?: string
}

const props = withDefaults(defineProps<Props>(), {
  archivo: null,
  isDeleting: false,
  dni: '',
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'eliminarEvento', eventoId: number): void
  (e: 'abrirVisor', archivo: PersonalArchivo, titulo?: string): void
  (e: 'subirArchivo'): void
  (
    e: 'documentoActualizado',
    payload: {
      documentoId: number
      tipoDocumentoNombre: string
      numeroDocumento: string
      fecha: string
      descripcion: string
    }
  ): void
}>()

const toast = useToastStore()

const confirmandoEliminar = ref<boolean>(false)
const copiado = ref<boolean>(false)
const previewUrl = ref<string | null>(null)
const previewIsBlob = ref<boolean>(false)
const isLoadingPreview = ref<boolean>(false)
const previewError = ref<string | null>(null)

// Estado de edición inline del documento
const editando = ref<boolean>(false)
const isLoadingEdit = ref<boolean>(false)
const isSavingEdit = ref<boolean>(false)
const tiposDocumento = ref<TipoDocumentoOption[]>([])
const areas = ref<AreaOption[]>([])
const formErrors = ref<Record<string, string>>({})
const formEdit = ref<{
  tipoDocumentoId: number | ''
  areaId: number | ''
  numeroDocumento: number | ''
  añoDocumento: number
  fecha: string
  fechaValida: string
  descripcion: string
}>({
  tipoDocumentoId: '',
  areaId: '',
  numeroDocumento: '',
  añoDocumento: new Date().getFullYear(),
  fecha: '',
  fechaValida: '',
  descripcion: '',
})

const necesitaArea = computed(() => {
  if (!formEdit.value.tipoDocumentoId) return false
  return requiereArea(formEdit.value.tipoDocumentoId)
})

watch(
  () => formEdit.value.tipoDocumentoId,
  (nuevoTipo) => {
    if (nuevoTipo && !requiereArea(nuevoTipo)) {
      formEdit.value.areaId = ''
      if (formErrors.value.areaId) {
        delete formErrors.value.areaId
      }
    }
  }
)

const esImagen = computed(() => {
  const name = props.archivo?.original_name?.toLowerCase() || ''
  const ext = props.archivo?.extension?.toLowerCase() || ''
  return /\.(png|jpe?g|webp|gif)$/.test(name) || ['png', 'jpg', 'jpeg', 'webp', 'gif'].includes(ext)
})

const previewIframeUrl = computed(() => {
  if (!previewUrl.value) return ''
  if (esEnlaceExterno.value) return previewUrl.value
  return previewUrl.value.includes('#')
    ? previewUrl.value
    : `${previewUrl.value}#toolbar=0&navpanes=0`
})

const limpiarPreviewUrl = () => {
  if (previewUrl.value && previewIsBlob.value) {
    URL.revokeObjectURL(previewUrl.value)
  }
  previewUrl.value = null
  previewIsBlob.value = false
}

const cargarPreview = async () => {
  limpiarPreviewUrl()
  previewError.value = null

  if (!props.isOpen || !props.archivo) return

  if (esEnlaceExterno.value && props.archivo.external_url) {
    previewUrl.value = props.archivo.external_url
    previewIsBlob.value = false
    isLoadingPreview.value = false
    return
  }

  const targetArchivo = props.archivo
  const fileUrl = targetArchivo.external_url || getFileDownloadUrl(targetArchivo.file_hash)

  isLoadingPreview.value = true
  try {
    const rawBlob = await fetchAuthBlob(fileUrl)
    if (!props.isOpen || props.archivo !== targetArchivo) return

    const isPdf =
      targetArchivo.original_name.toLowerCase().endsWith('.pdf') ||
      !rawBlob.type ||
      rawBlob.type === 'application/octet-stream'

    const finalBlob =
      isPdf && rawBlob.type !== 'application/pdf' && !esImagen.value
        ? new Blob([rawBlob], { type: 'application/pdf' })
        : rawBlob

    previewUrl.value = URL.createObjectURL(finalBlob)
    previewIsBlob.value = true
  } catch (err) {
    if (props.isOpen && props.archivo === targetArchivo) {
      previewError.value =
        err instanceof Error ? err.message : 'No se pudo cargar la vista previa.'
    }
  } finally {
    if (props.archivo === targetArchivo) {
      isLoadingPreview.value = false
    }
  }
}

watch(
  () => [props.isOpen, props.archivo] as const,
  ([abierto]) => {
    if (!abierto) {
      confirmandoEliminar.value = false
      copiado.value = false
      editando.value = false
      isLoadingEdit.value = false
      isSavingEdit.value = false
      formErrors.value = {}
      limpiarPreviewUrl()
      previewError.value = null
      isLoadingPreview.value = false
    } else {
      cargarPreview()
    }
  },
  { immediate: true }
)

watch(
  () => props.documento?.documentoId,
  () => {
    editando.value = false
    formErrors.value = {}
  }
)

const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape' && props.isOpen && !props.isDeleting && !isSavingEdit.value) {
    emit('close')
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  limpiarPreviewUrl()
  window.removeEventListener('keydown', handleKeyDown)
})

const tituloPanel = computed(() => {
  if (!props.documento) return 'Detalle de Documento'
  if (props.documento.titulo) return props.documento.titulo
  switch (props.documento.tipo) {
    case 'ingreso':
      return 'Documento de Inicio / Ingreso'
    case 'salida':
      return 'Documento de Salida'
    case 'evento':
      return 'Documento de Evento'
  }
})

const etiquetaTipo = computed(() => {
  if (!props.documento) return ''
  switch (props.documento.tipo) {
    case 'ingreso':
      return 'Ingreso'
    case 'salida':
      return 'Cese / Salida'
    case 'evento':
      return 'Evento'
  }
})

const etiquetaActo = computed(() => {
  if (!props.documento) return '-'
  switch (props.documento.tipo) {
    case 'ingreso':
      return 'Inicio de vínculo laboral'
    case 'salida':
      return 'Término de vínculo laboral'
    case 'evento':
      return props.documento.tipoEvento
        ? getTipoEventoLabel(props.documento.tipoEvento)
        : 'Acción administrativa'
  }
})

const numeroDocumentoFormateado = computed(() => {
  if (!props.documento) return '-'
  const parts = [props.documento.tipoDocumentoNombre, props.documento.numeroDocumento].filter(Boolean)
  return parts.length > 0 ? parts.join(' N° ') : 'Sin numeración registrada'
})

const tieneNumeracion = computed(() => {
  if (!props.documento) return false
  return Boolean(props.documento.tipoDocumentoNombre || props.documento.numeroDocumento)
})

const esEvento = computed(() => {
  return props.documento?.tipo === 'evento'
})

const tieneEventoId = computed(() => {
  return Boolean(props.documento?.eventoId && props.documento.eventoId > 0)
})

const esEnlaceExterno = computed(() => {
  if (!props.archivo?.external_url) return false
  const baseUrl = getApiBaseUrl()
  return !props.archivo.external_url.startsWith(baseUrl) && !props.archivo.external_url.startsWith('/')
})

const copiarDocumento = async () => {
  if (!tieneNumeracion.value) return
  try {
    await navigator.clipboard.writeText(numeroDocumentoFormateado.value)
    copiado.value = true
    setTimeout(() => {
      copiado.value = false
    }, 2000)
  } catch {}
}

const handleOpenFile = () => {
  if (!props.archivo) return
  if (esEnlaceExterno.value && props.archivo.external_url) {
    window.open(props.archivo.external_url, '_blank', 'noopener,noreferrer')
    return
  }
  const titulo = props.documento
    ? [props.documento.tipoDocumentoNombre, props.documento.numeroDocumento].filter(Boolean).join(' N° ')
    : props.archivo.original_name
  emit('abrirVisor', props.archivo, titulo)
}

const handleConfirmarEliminar = () => {
  if (props.documento?.eventoId) {
    emit('eliminarEvento', props.documento.eventoId)
  }
}

const abrirEdicion = async () => {
  const docId = props.documento?.documentoId
  if (!docId) return

  editando.value = true
  isLoadingEdit.value = true
  formErrors.value = {}

  try {
    const [tiposRes, areasRes, detalle] = await Promise.all([
      tiposDocumento.value.length > 0 ? Promise.resolve(tiposDocumento.value) : fetchTiposDocumentos(),
      areas.value.length > 0 ? Promise.resolve(areas.value) : fetchAreas(),
      obtenerDocumentoPorId(docId),
    ])

    tiposDocumento.value = tiposRes
    areas.value = areasRes

    let tipoId: number | '' = ''
    if (detalle.tipoDocumento) {
      const parsed = Number(detalle.tipoDocumento)
      if (!isNaN(parsed) && parsed > 0) {
        tipoId = parsed
      } else {
        const match = tiposRes.find(
          (t) => t.nombre.toLowerCase() === detalle.tipoDocumento?.toLowerCase()
        )
        if (match) tipoId = match.id
      }
    }

    formEdit.value = {
      tipoDocumentoId: tipoId,
      areaId: detalle.areaId ?? '',
      numeroDocumento: detalle.numeroDocumento ?? '',
      añoDocumento: detalle.añoDocumento ?? new Date().getFullYear(),
      fecha: detalle.fecha ? detalle.fecha.slice(0, 10) : '',
      fechaValida: detalle.fechaDocumento ? detalle.fechaDocumento.slice(0, 10) : '',
      descripcion: detalle.descripcion || '',
    }
  } catch (err) {
    formErrors.value.general =
      err instanceof Error ? err.message : 'No se pudo cargar la información del documento.'
  } finally {
    isLoadingEdit.value = false
  }
}

const cancelarEdicion = () => {
  if (isSavingEdit.value) return
  editando.value = false
  formErrors.value = {}
}

const validateEditForm = (): boolean => {
  const errors: Record<string, string> = {}

  if (!formEdit.value.tipoDocumentoId) {
    errors.tipoDocumentoId = 'Debe seleccionar un tipo de documento.'
  }

  if (necesitaArea.value && (!formEdit.value.areaId || Number(formEdit.value.areaId) <= 0)) {
    errors.areaId = 'El área es requerida para este tipo de documento.'
  }

  const num = Number(formEdit.value.numeroDocumento)
  if (!formEdit.value.numeroDocumento || isNaN(num) || num <= 0) {
    errors.numeroDocumento = 'Ingrese un número válido mayor a cero.'
  }

  const anio = Number(formEdit.value.añoDocumento)
  if (!formEdit.value.añoDocumento || isNaN(anio) || anio < 1980 || anio > 2050) {
    errors.añoDocumento = 'Ingrese un año válido.'
  }

  if (!formEdit.value.fecha || !formEdit.value.fecha.trim()) {
    errors.fecha = 'La fecha del documento es obligatoria.'
  }

  if (!formEdit.value.descripcion || !formEdit.value.descripcion.trim()) {
    errors.descripcion = 'La referencia o descripción es obligatoria.'
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleGuardarEdicion = async () => {
  const docId = props.documento?.documentoId
  if (!docId || !validateEditForm()) return

  const dniObjetivo = props.dni || props.documento?.vinculo?.dni || ''
  if (!dniObjetivo) {
    formErrors.value.general = 'No se pudo identificar el DNI del trabajador.'
    return
  }

  isSavingEdit.value = true
  formErrors.value = {}
  try {
    const tipoId = String(formEdit.value.tipoDocumentoId)
    const areaId = necesitaArea.value ? Number(formEdit.value.areaId) : null
    const numero = Number(formEdit.value.numeroDocumento)
    const anio = Number(formEdit.value.añoDocumento)
    const fecha = formEdit.value.fecha.trim()
    const fechaValida = formEdit.value.fechaValida.trim() || null
    const descripcion = formEdit.value.descripcion.trim()

    await editarDocumento({
      dni: dniObjetivo,
      documento: {
        id: docId,
        tipoDocumento: tipoId,
        areaId,
        numeroDocumento: numero,
        añoDocumento: anio,
        fecha,
        fechaValida,
        descripcion,
      },
    })

    const tipoObj = tiposDocumento.value.find((t) => t.id === Number(tipoId))
    const areaObj = areaId ? areas.value.find((a) => a.id === areaId) : null
    const numeroFormateado = areaObj?.sigla
      ? `${numero}-${anio}-${areaObj.sigla}`
      : `${numero}-${anio}`

    toast.success('Documento actualizado correctamente.')
    editando.value = false

    emit('documentoActualizado', {
      documentoId: docId,
      tipoDocumentoNombre: tipoObj?.nombre || props.documento?.tipoDocumentoNombre || '',
      numeroDocumento: numeroFormateado,
      fecha,
      descripcion,
    })
  } catch (err) {
    const msg = err instanceof Error ? err.message : 'Error al actualizar el documento.'
    formErrors.value.general = msg
    toast.error(msg)
  } finally {
    isSavingEdit.value = false
  }
}
</script>

<template>
  <Modal :is-open="isOpen" placement="right" :persistent="isDeleting || isSavingEdit" @close="emit('close')">
    <aside
      v-if="isOpen && documento"
      class="perfil-scope w-screen max-w-md h-full bg-card border-l border-border shadow-2xl flex flex-col z-10 overflow-hidden text-meta"
    >
      <!-- Cabecera -->
      <header class="px-4 py-3 border-b border-border flex items-center justify-between gap-3 shrink-0 bg-muted/15">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg flex items-center justify-center shrink-0 border"
            :class="{
              'bg-primary/10 text-primary border-primary/20': documento.tipo === 'ingreso',
              'bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20': documento.tipo === 'salida',
              'bg-sky-500/10 text-sky-600 dark:text-sky-400 border-sky-500/20': documento.tipo === 'evento',
            }"
          >
            <IconFileText v-if="documento.tipo === 'ingreso'" class="size-4" />
            <IconFileCheck v-else-if="documento.tipo === 'salida'" class="size-4" />
            <IconFileCode v-else class="size-4" />
          </div>

          <div class="min-w-0">
            <div class="flex items-center gap-2 flex-wrap">
              <h3 class="font-semibold text-foreground text-xs tracking-tight truncate">
                {{ tituloPanel }}
              </h3>
              <Badge
                size="xs"
                :variant="documento.tipo === 'ingreso' ? 'default' : documento.tipo === 'evento' ? 'warning' : 'secondary'"
              >
                {{ etiquetaTipo }}
              </Badge>
            </div>
            <p class="text-micro text-muted-foreground truncate mt-0.5">
              Sustento documental del vínculo laboral
            </p>
          </div>
        </div>

        <div class="flex items-center gap-1.5 shrink-0">
          <span
            v-if="documento.documentoId"
            class="font-mono text-micro text-muted-foreground px-2 py-0.5 rounded-md bg-muted border border-border/60"
            title="Identificador interno del documento"
          >
            ID #{{ documento.documentoId }}
          </span>
          <button
            type="button"
            class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 flex items-center justify-center transition-colors cursor-pointer"
            aria-label="Cerrar panel de documento"
            :disabled="isDeleting"
            @click="emit('close')"
          >
            <IconX class="size-4" />
          </button>
        </div>
      </header>

      <!-- Contenido principal -->
      <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-3.5">
        <!-- Archivo Digital en Legajo (principal actor cuando existe archivo) -->
        <section
          class="rounded-xl border border-border bg-card overflow-hidden flex flex-col"
          :class="archivo ? 'order-1 flex-1 min-h-[26rem]' : 'order-2 p-3.5 space-y-2.5'"
        >
          <div
            class="flex items-center justify-between gap-2 border-b border-border"
            :class="archivo ? 'px-3.5 py-2.5 shrink-0' : 'pb-2'"
          >
            <div class="flex items-center gap-2 min-w-0">
              <IconLink v-if="archivo?.external_url" class="size-3.5 text-primary shrink-0" />
              <IconFileTypePdf v-else class="size-3.5 text-primary shrink-0" />
              <div class="min-w-0">
                <h4 class="font-semibold text-foreground tracking-tight text-xs truncate" :title="archivo?.original_name || 'Archivo Digital en Legajo'">
                  {{ archivo ? archivo.original_name : 'Archivo Digital en Legajo' }}
                </h4>
                <p v-if="archivo" class="text-micro text-muted-foreground font-mono truncate">
                  {{ (archivo.extension || 'PDF').toUpperCase() }}
                  <template v-if="archivo.size"> &bull; {{ archivo.size }}</template>
                  <template v-if="archivo.fecha_subida"> &bull; {{ formatDate(archivo.fecha_subida) }}</template>
                </p>
              </div>
            </div>

            <div class="flex items-center gap-1.5 shrink-0">
              <Button
                v-if="archivo"
                size="xs"
                variant="outline"
                class="gap-1 cursor-pointer"
                @click="handleOpenFile"
              >
                <IconExternalLink v-if="esEnlaceExterno" class="size-3.5" />
                <IconEye v-else class="size-3.5" />
                <span>{{ esEnlaceExterno ? 'Abrir' : 'Ampliar' }}</span>
              </Button>
              <Badge v-else size="xs" variant="secondary">Sin adjunto</Badge>
            </div>
          </div>

          <!-- Vista previa integrada (cuando existe archivo) -->
          <div v-if="archivo" class="relative flex-1 bg-muted/15 overflow-hidden flex flex-col">
            <div
              v-if="isLoadingPreview"
              class="flex-1 flex flex-col items-center justify-center gap-2 text-center p-4"
            >
              <div class="size-6 rounded-full border-2 border-primary/20 border-t-primary animate-spin"></div>
              <p class="text-micro text-muted-foreground">Cargando vista previa...</p>
            </div>

            <div
              v-else-if="previewError"
              class="flex-1 flex flex-col items-center justify-center gap-2 text-center p-4"
            >
              <IconAlertCircle class="size-5 text-destructive/80" />
              <p class="text-micro text-muted-foreground max-w-xs">{{ previewError }}</p>
              <Button size="xs" variant="outline" class="gap-1 cursor-pointer" @click="cargarPreview">
                <IconRefresh class="size-3" />
                <span>Reintentar</span>
              </Button>
            </div>

            <img
              v-else-if="previewUrl && esImagen"
              :src="previewUrl"
              :alt="archivo.original_name"
              class="w-full h-full object-contain bg-muted/20"
            />

            <iframe
              v-else-if="previewUrl"
              :src="previewIframeUrl"
              class="w-full h-full flex-1 border-0"
              title="Vista previa del documento"
            ></iframe>
          </div>

          <!-- Estado sin archivo -->
          <div v-else class="flex items-center justify-between gap-3 rounded-lg border border-dashed border-border bg-muted/15 p-3">
            <div class="flex items-start gap-2.5 min-w-0">
              <div class="size-8 rounded-lg bg-muted border border-border/70 flex items-center justify-center shrink-0 text-muted-foreground">
                <IconFileAlert class="size-4" />
              </div>
              <div class="space-y-0.5 min-w-0">
                <p class="text-meta font-semibold text-foreground">Sin archivo PDF vinculado</p>
                <p class="text-micro text-muted-foreground leading-relaxed">
                  El acto está registrado pero aún no cuenta con sustento digital en el legajo.
                </p>
              </div>
            </div>

            <Button
              size="xs"
              variant="outline"
              class="gap-1.5 cursor-pointer shrink-0 bg-card"
              @click="emit('subirArchivo')"
            >
              <IconUpload class="size-3.5" />
              <span>Subir</span>
            </Button>
          </div>
        </section>

        <!-- Identificación Formal del Documento -->
        <section
          class="rounded-xl border border-border bg-card p-3.5 space-y-2.5 shrink-0"
          :class="archivo ? 'order-2' : 'order-1'"
        >
          <div class="flex items-center justify-between gap-2 border-b border-border pb-2">
            <span class="text-xs font-bold text-foreground tracking-wider flex items-center gap-2">
              <IconFileText class="size-3.5 text-primary shrink-0" />
              <h4 class="font-semibold text-foreground tracking-tight text-xs">
                {{ editando ? 'Editar Documento' : 'Identificación Formal' }}
              </h4>
            </span>

            <div class="flex items-center gap-1.5 shrink-0">
              <button
                v-if="tieneNumeracion && !editando"
                type="button"
                class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md border border-border bg-muted/40 hover:bg-muted text-micro font-medium text-muted-foreground hover:text-foreground transition cursor-pointer shrink-0"
                title="Copiar numeración del documento"
                @click="copiarDocumento"
              >
                <IconCheck v-if="copiado" class="size-3 text-emerald-600 dark:text-emerald-400" />
                <IconCopy v-else class="size-3" />
                <span>{{ copiado ? 'Copiado' : 'Copiar' }}</span>
              </button>

              <button
                v-if="documento.documentoId && !editando"
                type="button"
                class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md border border-primary/30 bg-primary/10 hover:bg-primary/15 text-micro font-medium text-primary transition cursor-pointer shrink-0"
                title="Editar datos del documento"
                @click="abrirEdicion"
              >
                <IconPencil class="size-3" />
                <span>Editar</span>
              </button>
            </div>
          </div>

          <!-- Modo Edición -->
          <div v-if="editando">
            <div v-if="isLoadingEdit" class="py-6 flex flex-col items-center justify-center gap-2 text-center">
              <IconLoader2 class="size-5 text-primary animate-spin" />
              <p class="text-micro text-muted-foreground">Cargando datos del documento...</p>
            </div>

            <form v-else class="space-y-3" @submit.prevent="handleGuardarEdicion">
              <div
                v-if="formErrors.general"
                class="p-2.5 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-micro flex items-center gap-2"
              >
                <IconAlertTriangle class="size-3.5 shrink-0" />
                <span>{{ formErrors.general }}</span>
              </div>

              <div class="space-y-1">
                <label for="edit-doc-tipo" class="hs-label flex items-center gap-1.5">
                  <IconFileDescription class="size-3.5 text-muted-foreground" />
                  Tipo de Documento
                  <span class="text-destructive">*</span>
                </label>
                <select
                  id="edit-doc-tipo"
                  v-model="formEdit.tipoDocumentoId"
                  :disabled="isSavingEdit"
                  class="hs-input px-2.5"
                  :class="formErrors.tipoDocumentoId ? 'border-destructive' : ''"
                >
                  <option value="" disabled>Seleccione el tipo...</option>
                  <option v-for="tipo in tiposDocumento" :key="tipo.id" :value="tipo.id">
                    {{ tipo.nombre }}
                  </option>
                </select>
                <p v-if="formErrors.tipoDocumentoId" class="text-micro text-destructive">
                  {{ formErrors.tipoDocumentoId }}
                </p>
              </div>

              <div v-if="necesitaArea" class="space-y-1">
                <label for="edit-doc-area" class="hs-label flex items-center gap-1.5">
                  <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" />
                  Área de Emisión
                  <span class="text-destructive">*</span>
                </label>
                <select
                  id="edit-doc-area"
                  v-model="formEdit.areaId"
                  :disabled="isSavingEdit"
                  class="hs-input px-2.5"
                  :class="formErrors.areaId ? 'border-destructive' : ''"
                >
                  <option value="" disabled>Seleccione el área...</option>
                  <option v-for="area in areas" :key="area.id" :value="area.id">
                    {{ area.nombre }} {{ area.sigla ? `(${area.sigla})` : '' }}
                  </option>
                </select>
                <p v-if="formErrors.areaId" class="text-micro text-destructive">
                  {{ formErrors.areaId }}
                </p>
              </div>

              <div
                v-else-if="formEdit.tipoDocumentoId"
                class="p-2 rounded-lg bg-muted/50 border border-border/60 text-muted-foreground text-micro flex items-center gap-2"
              >
                <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
                <span>Este tipo de documento está exento de asignación de área.</span>
              </div>

              <div class="grid grid-cols-2 gap-2.5">
                <div class="space-y-1">
                  <label for="edit-doc-numero" class="hs-label flex items-center gap-1.5">
                    <IconHash class="size-3.5 text-muted-foreground" />
                    Número
                    <span class="text-destructive">*</span>
                  </label>
                  <input
                    id="edit-doc-numero"
                    v-model.number="formEdit.numeroDocumento"
                    type="number"
                    min="1"
                    :disabled="isSavingEdit"
                    class="hs-input px-2.5 font-mono"
                    :class="formErrors.numeroDocumento ? 'border-destructive' : ''"
                  />
                  <p v-if="formErrors.numeroDocumento" class="text-micro text-destructive">
                    {{ formErrors.numeroDocumento }}
                  </p>
                </div>

                <div class="space-y-1">
                  <label for="edit-doc-anio" class="hs-label flex items-center gap-1.5">
                    <IconCalendar class="size-3.5 text-muted-foreground" />
                    Año
                    <span class="text-destructive">*</span>
                  </label>
                  <input
                    id="edit-doc-anio"
                    v-model.number="formEdit.añoDocumento"
                    type="number"
                    min="1980"
                    max="2050"
                    :disabled="isSavingEdit"
                    class="hs-input px-2.5 font-mono"
                    :class="formErrors.añoDocumento ? 'border-destructive' : ''"
                  />
                  <p v-if="formErrors.añoDocumento" class="text-micro text-destructive">
                    {{ formErrors.añoDocumento }}
                  </p>
                </div>
              </div>

              <div class="grid grid-cols-2 gap-2.5">
                <div class="space-y-1">
                  <label for="edit-doc-fecha" class="hs-label flex items-center gap-1.5">
                    <IconCalendar class="size-3.5 text-muted-foreground" />
                    Inicio del vínculo
                    <span class="text-destructive">*</span>
                  </label>
                  <input
                    id="edit-doc-fecha"
                    v-model="formEdit.fecha"
                    type="date"
                    :disabled="isSavingEdit"
                    class="hs-input px-2.5"
                    :class="formErrors.fecha ? 'border-destructive' : ''"
                  />
                  <p v-if="formErrors.fecha" class="text-micro text-destructive">
                    {{ formErrors.fecha }}
                  </p>
                </div>

                <div class="space-y-1">
                  <label for="edit-doc-fecha-valida" class="hs-label flex items-center gap-1.5">
                    <IconCalendar class="size-3.5 text-muted-foreground" />
                    Fecha del documento
                  </label>
                  <input
                    id="edit-doc-fecha-valida"
                    v-model="formEdit.fechaValida"
                    type="date"
                    :disabled="isSavingEdit"
                    class="hs-input px-2.5"
                  />
                </div>
              </div>

              <div class="space-y-1">
                <label for="edit-doc-descripcion" class="hs-label flex items-center gap-1.5">
                  <IconNotes class="size-3.5 text-muted-foreground" />
                  Referencia / Descripción
                  <span class="text-destructive">*</span>
                </label>
                <textarea
                  id="edit-doc-descripcion"
                  v-model="formEdit.descripcion"
                  rows="3"
                  :disabled="isSavingEdit"
                  class="hs-input resize-none"
                  :class="formErrors.descripcion ? 'border-destructive' : ''"
                ></textarea>
                <p v-if="formErrors.descripcion" class="text-micro text-destructive">
                  {{ formErrors.descripcion }}
                </p>
              </div>

              <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/60">
                <Button
                  type="button"
                  size="xs"
                  variant="outline"
                  class="cursor-pointer"
                  :disabled="isSavingEdit"
                  @click="cancelarEdicion"
                >
                  Cancelar
                </Button>
                <Button
                  type="submit"
                  size="xs"
                  variant="primary"
                  class="cursor-pointer"
                  :loading="isSavingEdit"
                >
                  Guardar cambios
                </Button>
              </div>
            </form>
          </div>

          <!-- Modo Lectura -->
          <template v-else>
            <div class="space-y-1">
              <h5 class="text-xs font-semibold text-foreground tracking-tight wrap-break-word leading-snug">
                {{ numeroDocumentoFormateado }}
              </h5>
              <p class="inline-flex items-center gap-1.5 text-micro text-primary font-medium">
                <IconShieldCheck class="size-3 shrink-0" />
                <span>{{ etiquetaActo }}</span>
              </p>
            </div>

            <div class="space-y-1.5 pt-2 border-t border-border/60 text-meta">
              <div class="grid grid-cols-[7.75rem_1fr] items-start gap-2.5">
                <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
                  <IconFileDescription class="size-3.5 text-muted-foreground shrink-0" /> Tipo de doc.:
                </span>
                <span class="font-medium text-foreground wrap-break-word">
                  {{ documento.tipoDocumentoNombre || '-' }}
                </span>
              </div>

              <div class="grid grid-cols-[7.75rem_1fr] items-start gap-2.5">
                <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
                  <IconHash class="size-3.5 text-muted-foreground shrink-0" /> N° Documento:
                </span>
                <span class="font-mono font-medium text-foreground wrap-break-word">
                  {{ documento.numeroDocumento || '-' }}
                </span>
              </div>

              <div class="grid grid-cols-[7.75rem_1fr] items-start gap-2.5">
                <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
                  <IconCalendar class="size-3.5 text-muted-foreground shrink-0" /> Fecha registro:
                </span>
                <span class="font-mono font-medium text-foreground tabular-nums">
                  {{ documento.fecha ? formatDate(documento.fecha) : 'No registrada' }}
                </span>
              </div>

              <div class="grid grid-cols-[7.75rem_1fr] items-start gap-2.5">
                <span class="text-muted-foreground flex items-center gap-1.5 shrink-0 mt-0.5">
                  <IconNotes class="size-3.5 text-muted-foreground shrink-0" /> Referencia:
                </span>
                <span class="text-foreground/90 whitespace-pre-line wrap-break-word">
                  {{ documento.descripcion || 'Sin descripción registrada' }}
                </span>
              </div>

              <div v-if="documento.documentoId" class="grid grid-cols-[7.75rem_1fr] items-center gap-2.5">
                <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                  <IconId class="size-3.5 text-muted-foreground shrink-0" /> ID Registro:
                </span>
                <span class="font-mono text-muted-foreground">#{{ documento.documentoId }}</span>
              </div>
            </div>
          </template>
        </section>

        <!-- 3. Confirmación de eliminación de evento (si aplica) -->
        <section v-if="esEvento && confirmandoEliminar" class="order-3 shrink-0">
          <div class="p-3.5 rounded-xl border border-destructive/40 bg-destructive/10 space-y-2.5">
            <div class="space-y-1">
              <h5 class="font-semibold text-destructive text-meta flex items-center gap-1.5">
                <IconAlertCircle class="size-4 shrink-0" />
                ¿Confirmar eliminación del evento?
              </h5>
              <p class="text-micro text-foreground/90 leading-relaxed">
                Esta acción eliminará de forma irreversible el registro del evento laboral #{{ documento.eventoId }} y desvinculará sus documentos asociados.
              </p>
            </div>

            <div class="flex items-center justify-end gap-2 pt-1">
              <Button
                size="xs"
                variant="outline"
                class="cursor-pointer"
                :disabled="isDeleting"
                @click="confirmandoEliminar = false"
              >
                Cancelar
              </Button>
              <Button
                size="xs"
                class="bg-destructive hover:bg-destructive/90 text-destructive-foreground gap-1.5 cursor-pointer"
                :disabled="isDeleting"
                @click="handleConfirmarEliminar"
              >
                <IconTrash class="size-3.5" />
                <span>{{ isDeleting ? 'Eliminando...' : 'Sí, eliminar evento' }}</span>
              </Button>
            </div>
          </div>
        </section>
      </div>

      <!-- Pie del panel -->
      <footer class="px-4 py-3 border-t border-border flex items-center justify-between gap-2 bg-muted/10 shrink-0">
        <div>
          <Button
            v-if="esEvento && tieneEventoId && !confirmandoEliminar"
            size="xs"
            variant="outline"
            class="text-destructive border-destructive/30 hover:bg-destructive/10 gap-1.5 cursor-pointer"
            :disabled="isDeleting"
            @click="confirmandoEliminar = true"
          >
            <IconTrash class="size-3.5" />
            <span>Eliminar evento</span>
          </Button>
        </div>

        <div class="flex items-center gap-2">
          <Button
            v-if="archivo"
            size="xs"
            variant="primary"
            class="gap-1.5 cursor-pointer"
            @click="handleOpenFile"
          >
            <IconEye class="size-3.5" />
            <span>Visualizar PDF</span>
          </Button>
          <Button
            size="xs"
            variant="outline"
            class="cursor-pointer"
            :disabled="isDeleting"
            @click="emit('close')"
          >
            Cerrar
          </Button>
        </div>
      </footer>
    </aside>
  </Modal>
</template>
