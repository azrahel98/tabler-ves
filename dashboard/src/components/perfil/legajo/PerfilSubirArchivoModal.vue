<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import {
  type PersonalArchivo,
  type PersonalDocumento,
  type RegistrarUrlPayload,
} from '@/components/perfil/types'
import { formatDate } from '@/utils/date'
import {
  IconUpload,
  IconX,
  IconAlertTriangle,
  IconFileText,
  IconLink,
  IconWorld,
  IconPaperclip,
  IconCheck,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  dni: string
  documentos: PersonalDocumento[]
  archivos: PersonalArchivo[]
  isSaving?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isSaving: false,
  dni: '',
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'uploadFile', payload: { file: File; customName: string; documentoId: number | null }): void
  (e: 'uploadUrl', payload: RegistrarUrlPayload): void
}>()

const modo = ref<'file' | 'url'>('file')
const selectedFile = ref<File | null>(null)
const url = ref<string>('')
const fileName = ref<string>('')
const selectedDocumentoId = ref<number | ''>('')
const formErrors = ref<Record<string, string>>({})
const isDragging = ref<boolean>(false)

const documentosDisponibles = computed(() => {
  const idsVinculados = new Set(
    props.archivos
      .map((a) => a.documento_id)
      .filter((id): id is number => id !== null && id !== undefined)
  )
  return props.documentos.filter((doc) => !idsVinculados.has(doc.id))
})

const documentoSeleccionadoInfo = computed(() => {
  if (!selectedDocumentoId.value) return null
  return props.documentos.find((d) => d.id === Number(selectedDocumentoId.value)) || null
})

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      modo.value = 'file'
      selectedFile.value = null
      url.value = ''
      fileName.value = ''
      selectedDocumentoId.value = ''
      formErrors.value = {}
      isDragging.value = false
    }
  }
)

watch(
  () => selectedDocumentoId.value,
  (docId) => {
    if (docId && (!fileName.value || fileName.value === 'documento.pdf')) {
      const doc = props.documentos.find((d) => d.id === Number(docId))
      if (doc) {
        const clean = doc.sigla
          .replace(/[^\w\s-]/g, '')
          .trim()
          .replace(/\s+/g, '_')
        fileName.value = `${clean || 'documento'}.pdf`
      }
    }
  }
)

const handleFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    validateAndSetFile(target.files[0])
  }
}

const handleDrop = (e: DragEvent) => {
  isDragging.value = false
  if (e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0]) {
    validateAndSetFile(e.dataTransfer.files[0])
  }
}

const validateAndSetFile = (file: File) => {
  formErrors.value = {}
  if (!file.name.toLowerCase().endsWith('.pdf') && file.type !== 'application/pdf') {
    formErrors.value.file = 'Solo se permiten archivos en formato PDF.'
    selectedFile.value = null
    return
  }
  const maxBytes = 10 * 1024 * 1024
  if (file.size > maxBytes) {
    formErrors.value.file = 'El archivo supera el límite máximo permitido de 10 MB.'
    selectedFile.value = null
    return
  }
  selectedFile.value = file
  if (!fileName.value) {
    fileName.value = file.name
  }
}

const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

const validateForm = (): boolean => {
  const errors: Record<string, string> = {}

  if (modo.value === 'file') {
    if (!selectedFile.value) {
      errors.file = 'Debe seleccionar un archivo PDF para subir.'
    }
    if (selectedFile.value && !fileName.value.trim()) {
      errors.fileName = 'El nombre del archivo es obligatorio.'
    }
  } else {
    const trimmedUrl = url.value.trim()
    const trimmedName = fileName.value.trim()

    if (!trimmedUrl) {
      errors.url = 'La URL del documento es obligatoria.'
    } else if (!trimmedUrl.startsWith('https://')) {
      errors.url = 'La URL debe comenzar con https://'
    }

    if (!trimmedName) {
      errors.fileName = 'El nombre del archivo es obligatorio.'
    }
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleClose = () => {
  if (props.isSaving) return
  emit('close')
}

const handleSubmit = () => {
  if (!validateForm()) return

  const docId = selectedDocumentoId.value ? Number(selectedDocumentoId.value) : null
  let finalName = fileName.value.trim()
  if (!finalName.toLowerCase().endsWith('.pdf')) {
    finalName = `${finalName}.pdf`
  }

  if (modo.value === 'file' && selectedFile.value) {
    emit('uploadFile', {
      file: selectedFile.value,
      customName: finalName,
      documentoId: docId,
    })
  } else if (modo.value === 'url') {
    emit('uploadUrl', {
      dni_asociado: props.dni,
      original_name: finalName,
      external_url: url.value.trim(),
      documento_id: docId,
    })
  }
}
</script>

<template>
  <div
    v-if="isOpen"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-subir-archivo-title"
    tabindex="-1"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/65 backdrop-blur-xs focus:outline-hidden"
    @click.self="handleClose"
    @keydown.esc="handleClose"
  >
    <div
      class="w-full max-w-xl bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-xs max-h-[92vh] flex flex-col"
    >
      <div class="p-4 border-b border-border flex items-center justify-between bg-muted/20 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0"
          >
            <IconUpload class="size-4.5" aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <h3 id="modal-subir-archivo-title" class="font-bold text-foreground text-sm tracking-tight truncate">
              Subir Archivo al Legajo
            </h3>
            <p class="text-[11px] text-muted-foreground truncate">
              Incorporar nuevo documento PDF al expediente digital
            </p>
          </div>
        </div>

        <button
          type="button"
          class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer shrink-0"
          aria-label="Cerrar modal"
          :disabled="isSaving"
          @click="handleClose"
        >
          <IconX class="size-4" />
        </button>
      </div>

      <div class="p-5 space-y-4 overflow-y-auto">
        <div class="flex items-center p-1 bg-muted/30 border border-border rounded-xl text-xs">
          <button
            type="button"
            class="flex-1 py-1.5 px-3 rounded-lg font-medium transition cursor-pointer flex items-center justify-center gap-2"
            :class="modo === 'file' ? 'bg-card text-foreground shadow-xs border border-border/50' : 'text-muted-foreground hover:text-foreground'"
            @click="modo = 'file'"
          >
            <IconPaperclip class="size-3.5" />
            <span>Subir Archivo PDF</span>
          </button>
          <button
            type="button"
            class="flex-1 py-1.5 px-3 rounded-lg font-medium transition cursor-pointer flex items-center justify-center gap-2"
            :class="modo === 'url' ? 'bg-card text-foreground shadow-xs border border-border/50' : 'text-muted-foreground hover:text-foreground'"
            @click="modo = 'url'"
          >
            <IconWorld class="size-3.5" />
            <span>Enlace URL Externo</span>
          </button>
        </div>

        <div v-if="modo === 'file'" class="space-y-2">
          <label class="block font-medium text-foreground text-xs">
            Seleccionar archivo PDF <span class="text-destructive">*</span>
          </label>
          <div
            class="border-2 border-dashed rounded-xl p-5 text-center transition cursor-pointer flex flex-col items-center justify-center gap-2"
            :class="[
              isDragging ? 'border-primary bg-primary/5' : 'border-border hover:border-primary/40 hover:bg-muted/10',
              selectedFile ? 'border-emerald-500/40 bg-emerald-500/5' : ''
            ]"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
            @click="($refs.fileInput as HTMLInputElement)?.click()"
          >
            <input
              ref="fileInput"
              type="file"
              accept=".pdf,application/pdf"
              class="hidden"
              @change="handleFileChange"
            />
            <div
              class="size-10 rounded-xl flex items-center justify-center shrink-0 transition"
              :class="selectedFile ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' : 'bg-muted text-muted-foreground'"
            >
              <IconCheck v-if="selectedFile" class="size-5" />
              <IconFileText v-else class="size-5" />
            </div>
            <div v-if="selectedFile" class="space-y-0.5">
              <p class="font-medium text-foreground text-xs truncate max-w-sm">
                {{ selectedFile.name }}
              </p>
              <p class="text-[10px] text-muted-foreground font-mono">
                {{ formatFileSize(selectedFile.size) }}
              </p>
              <span class="text-[10px] text-primary hover:underline block pt-1">
                Hacer clic para cambiar archivo
              </span>
            </div>
            <div v-else class="space-y-1">
              <p class="font-medium text-foreground text-xs">
                Arrastra tu PDF aquí o <span class="text-primary underline">haz clic para examinar</span>
              </p>
              <p class="text-[10px] text-muted-foreground">
                Documentos PDF de hasta 10 MB
              </p>
            </div>
          </div>
          <p v-if="formErrors.file" class="text-[11px] text-destructive flex items-center gap-1 mt-1">
            <IconAlertTriangle class="size-3 shrink-0" />
            <span>{{ formErrors.file }}</span>
          </p>

          <div v-if="selectedFile" class="space-y-1.5 pt-1">
            <label for="subir-nombre-archivo-local" class="block font-medium text-foreground text-xs">
              Nombre para el archivo en legajo <span class="text-destructive">*</span>
            </label>
            <div class="relative">
              <IconFileText class="size-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                id="subir-nombre-archivo-local"
                v-model="fileName"
                type="text"
                placeholder="documento.pdf"
                class="w-full h-9 pl-9 pr-3 rounded-lg border text-xs bg-background focus:outline-hidden focus:ring-1 transition"
                :class="formErrors.fileName ? 'border-destructive focus:ring-destructive' : 'border-border focus:ring-primary'"
              />
            </div>
            <p v-if="formErrors.fileName" class="text-[11px] text-destructive flex items-center gap-1 mt-1">
              <IconAlertTriangle class="size-3 shrink-0" />
              <span>{{ formErrors.fileName }}</span>
            </p>
            <p v-else class="text-[10px] text-muted-foreground">
              Puedes personalizar el nombre con el que se registrará el PDF (debe terminar en .pdf).
            </p>
          </div>
        </div>

        <div v-else class="space-y-3">
          <div class="space-y-1.5">
            <label for="subir-url-externa" class="block font-medium text-foreground text-xs">
              URL del PDF externo <span class="text-destructive">*</span>
            </label>
            <div class="relative">
              <IconWorld class="size-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                id="subir-url-externa"
                v-model="url"
                type="url"
                placeholder="https://transparencia.munives.gob.pe/documentos/archivo.pdf"
                class="w-full h-9 pl-9 pr-3 rounded-lg border text-xs bg-background focus:outline-hidden focus:ring-1 transition"
                :class="formErrors.url ? 'border-destructive focus:ring-destructive' : 'border-border focus:ring-primary'"
              />
            </div>
            <p v-if="formErrors.url" class="text-[11px] text-destructive flex items-center gap-1 mt-1">
              <IconAlertTriangle class="size-3 shrink-0" />
              <span>{{ formErrors.url }}</span>
            </p>
            <p v-else class="text-[10px] text-muted-foreground">
              Debe comenzar con https:// (enlace público o institucional).
            </p>
          </div>

          <div class="space-y-1.5">
            <label for="subir-nombre-archivo" class="block font-medium text-foreground text-xs">
              Nombre para el archivo <span class="text-destructive">*</span>
            </label>
            <div class="relative">
              <IconFileText class="size-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                id="subir-nombre-archivo"
                v-model="fileName"
                type="text"
                placeholder="resolucion_123_2024.pdf"
                class="w-full h-9 pl-9 pr-3 rounded-lg border text-xs bg-background focus:outline-hidden focus:ring-1 transition"
                :class="formErrors.fileName ? 'border-destructive focus:ring-destructive' : 'border-border focus:ring-primary'"
              />
            </div>
            <p v-if="formErrors.fileName" class="text-[11px] text-destructive flex items-center gap-1 mt-1">
              <IconAlertTriangle class="size-3 shrink-0" />
              <span>{{ formErrors.fileName }}</span>
            </p>
          </div>
        </div>

        <div class="pt-2 border-t border-border/60 space-y-2">
          <div class="flex items-center justify-between">
            <label for="select-doc-vincular" class="block font-medium text-foreground text-xs">
              Vincular a Documento Formal (Opcional)
            </label>
            <span class="text-[10px] text-muted-foreground">
              {{ documentosDisponibles.length }} sin vincular
            </span>
          </div>

          <div class="relative">
            <select
              id="select-doc-vincular"
              v-model="selectedDocumentoId"
              class="w-full h-9 px-3 rounded-lg border border-border bg-background text-xs text-foreground focus:outline-hidden focus:ring-1 focus:ring-primary transition cursor-pointer"
            >
              <option value="">-- Ninguno (Archivo independiente del legajo) --</option>
              <option
                v-for="doc in documentosDisponibles"
                :key="doc.id"
                :value="doc.id"
              >
                {{ doc.sigla }} — {{ doc.descripcion }}
              </option>
            </select>
          </div>

          <div
            v-if="documentoSeleccionadoInfo"
            class="p-2.5 rounded-lg border border-primary/20 bg-primary/5 text-[11px] flex items-center justify-between gap-2"
          >
            <div class="flex items-center gap-2 min-w-0">
              <IconLink class="size-3.5 text-primary shrink-0" />
              <span class="font-medium text-foreground truncate">
                {{ documentoSeleccionadoInfo.sigla }}
              </span>
            </div>
            <span class="font-mono text-muted-foreground shrink-0">
              {{ formatDate(documentoSeleccionadoInfo.fecha) }}
            </span>
          </div>

          <p v-else-if="documentosDisponibles.length === 0" class="text-[10px] text-muted-foreground italic">
            No hay documentos formales pendientes de vinculación. El archivo se guardará como documento digital general de legajo.
          </p>
          <p v-else class="text-[10px] text-muted-foreground">
            Si seleccionas un documento, el archivo quedará directamente enlazado a esa resolución o acto administrativo.
          </p>
        </div>
      </div>

      <div class="p-4 border-t border-border bg-muted/20 flex items-center justify-end gap-2 shrink-0">
        <Button
          type="button"
          variant="outline"
          size="sm"
          class="cursor-pointer"
          :disabled="isSaving"
          @click="handleClose"
        >
          Cancelar
        </Button>
        <Button
          type="button"
          variant="primary"
          size="sm"
          class="cursor-pointer gap-1.5"
          :disabled="isSaving"
          @click="handleSubmit"
        >
          <IconUpload class="size-3.5" />
          <span>{{ isSaving ? 'Guardando...' : 'Subir Archivo' }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
