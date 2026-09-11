<script setup lang="ts">
import { ref, watch } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import {
  type PersonalDocumento,
  type RegistrarUrlPayload,
} from './types'
import { formatDate } from '@/utils/date'
import {
  IconLink,
  IconX,
  IconAlertTriangle,
  IconFileText,
  IconWorld,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  documento: PersonalDocumento | null
  dni: string
  isSaving?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isSaving: false,
  dni: '',
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', payload: RegistrarUrlPayload): void
}>()

const url = ref<string>('')
const fileName = ref<string>('')
const formErrors = ref<Record<string, string>>({})

function generateDefaultFileName(doc: PersonalDocumento | null): string {
  if (!doc) return 'documento.pdf'
  const clean = doc.sigla
    .replace(/[^\w\s-]/g, '')
    .trim()
    .replace(/\s+/g, '_')
  const base = clean || 'documento'
  return base.toLowerCase().endsWith('.pdf') ? base : `${base}.pdf`
}

watch(
  () => props.isOpen,
  (open) => {
    if (open) {
      formErrors.value = {}
      url.value = ''
      fileName.value = generateDefaultFileName(props.documento)
    }
  }
)

const validateForm = (): boolean => {
  const errors: Record<string, string> = {}
  const trimmedUrl = url.value.trim()
  const trimmedName = fileName.value.trim()

  if (!trimmedUrl) {
    errors.url = 'La URL del documento es obligatoria.'
  } else if (!trimmedUrl.startsWith('https://')) {
    errors.url = 'La URL debe ser segura y comenzar con https://'
  }

  if (!trimmedName) {
    errors.fileName = 'El nombre del archivo es obligatorio.'
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleClose = () => {
  if (props.isSaving) return
  emit('close')
}

const handleSubmit = () => {
  if (!validateForm() || !props.documento) return

  let finalName = fileName.value.trim()
  if (!finalName.toLowerCase().endsWith('.pdf')) {
    finalName = `${finalName}.pdf`
  }

  const payload: RegistrarUrlPayload = {
    dni_asociado: props.dni,
    original_name: finalName,
    external_url: url.value.trim(),
    documento_id: props.documento.id,
  }

  emit('save', payload)
}
</script>

<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/65 backdrop-blur-xs"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-vincular-title"
    @click.self="handleClose"
  >
    <div
      class="w-full max-w-lg bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-xs max-h-[92vh] flex flex-col"
    >
      <div class="p-4 border-b border-border flex items-center justify-between bg-muted/20 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0"
          >
            <IconLink class="size-4.5" aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <h3 id="modal-vincular-title" class="font-bold text-foreground text-sm tracking-tight truncate">
              Vincular PDF al Documento
            </h3>
            <p class="text-[11px] text-muted-foreground truncate">
              Asociar una URL externa al acto administrativo
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
        <div v-if="documento" class="p-3.5 rounded-xl border border-border bg-muted/20 space-y-2">
          <div class="flex items-center justify-between gap-2">
            <div class="flex items-center gap-2 min-w-0">
              <IconFileText class="size-4 text-primary shrink-0" />
              <span class="font-semibold text-foreground text-xs truncate">{{ documento.sigla }}</span>
            </div>
            <span class="font-mono text-[11px] text-muted-foreground shrink-0">{{ formatDate(documento.fecha) }}</span>
          </div>
          <p class="text-[11px] text-muted-foreground leading-relaxed">{{ documento.descripcion }}</p>
        </div>

        <div class="space-y-1.5">
          <label for="url-externa" class="block font-medium text-foreground text-xs">
            URL del documento PDF <span class="text-destructive">*</span>
          </label>
          <div class="relative">
            <IconWorld class="size-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              id="url-externa"
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
            Debe ser un enlace HTTPS accesible (portal de transparencia, repositorio institucional, etc.).
          </p>
        </div>

        <div class="space-y-1.5">
          <label for="nombre-archivo" class="block font-medium text-foreground text-xs">
            Nombre del archivo <span class="text-destructive">*</span>
          </label>
          <div class="relative">
            <IconFileText class="size-4 text-muted-foreground absolute left-3 top-1/2 -translate-y-1/2" />
            <input
              id="nombre-archivo"
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
          <p v-else class="text-[10px] text-muted-foreground">
            Identificador con el que se mostrará en el legajo digital (debe terminar en .pdf).
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
          <IconLink class="size-3.5" />
          <span>{{ isSaving ? 'Vinculando...' : 'Vincular PDF' }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
