<script setup lang="ts">
import Modal from '@/components/ui/modal/Modal.vue'
import { ref, watch, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  fetchTiposDocumentos,
  fetchAreas,
  requiereArea,
  type AreaOption,
  type TipoDocumentoOption,
  type DocumentoData,
} from '@/services/personal'
import {
  IconAlertTriangle,
  IconBuildingSkyscraper,
  IconCalendar,
  IconFileDescription,
  IconHash,
  IconCheck,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  isSaving?: boolean
  servidorNombre?: string
  servidorDni?: string
}

const props = withDefaults(defineProps<Props>(), {
  isSaving: false,
  servidorNombre: '',
  servidorDni: '',
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', data: DocumentoData): void
}>()

const tiposDocumento = ref<TipoDocumentoOption[]>([])
const areas = ref<AreaOption[]>([])
const isLoadingCatalogos = ref<boolean>(false)
const formErrors = ref<Record<string, string>>({})

const getTodayDateString = (): string => {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const formData = ref<{
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
  fecha: getTodayDateString(),
  fechaValida: '',
  descripcion: '',
})

const necesitaArea = computed(() => {
  if (!formData.value.tipoDocumentoId) return false
  return requiereArea(formData.value.tipoDocumentoId)
})

const loadCatalogos = async () => {
  isLoadingCatalogos.value = true
  try {
    const [tiposRes, areasRes] = await Promise.all([
      fetchTiposDocumentos(),
      fetchAreas(),
    ])
    tiposDocumento.value = tiposRes
    areas.value = areasRes

    if (!formData.value.tipoDocumentoId && tiposRes.length > 0) {
      formData.value.tipoDocumentoId = tiposRes[0].id
    }
  } finally {
    isLoadingCatalogos.value = false
  }
}

watch(
  () => formData.value.tipoDocumentoId,
  (nuevoTipo) => {
    if (nuevoTipo && !requiereArea(nuevoTipo)) {
      formData.value.areaId = ''
      if (formErrors.value.areaId) {
        delete formErrors.value.areaId
      }
    }
  }
)

watch(
  () => props.isOpen,
  async (open) => {
    if (open) {
      formErrors.value = {}
      formData.value = {
        tipoDocumentoId: tiposDocumento.value[0]?.id || '',
        areaId: '',
        numeroDocumento: '',
        añoDocumento: new Date().getFullYear(),
        fecha: getTodayDateString(),
        fechaValida: '',
        descripcion: '',
      }
      await loadCatalogos()
    }
  }
)

const validateForm = (): boolean => {
  const errors: Record<string, string> = {}

  if (!formData.value.tipoDocumentoId) {
    errors.tipoDocumentoId = 'Debe seleccionar un tipo de documento.'
  }

  if (necesitaArea.value && (!formData.value.areaId || Number(formData.value.areaId) <= 0)) {
    errors.areaId = 'El área es requerida para este tipo de documento.'
  }

  const num = Number(formData.value.numeroDocumento)
  if (!formData.value.numeroDocumento || isNaN(num) || num <= 0) {
    errors.numeroDocumento = 'Ingrese un número de documento válido y mayor a cero.'
  }

  const anio = Number(formData.value.añoDocumento)
  if (!formData.value.añoDocumento || isNaN(anio) || anio < 1980 || anio > 2050) {
    errors.añoDocumento = 'Ingrese un año válido.'
  }

  if (!formData.value.fecha || !formData.value.fecha.trim()) {
    errors.fecha = 'La fecha del documento es obligatoria.'
  }

  if (!formData.value.descripcion || !formData.value.descripcion.trim()) {
    errors.descripcion = 'La descripción o asunto es obligatoria.'
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

  const payload: DocumentoData = {
    tipoDocumento: String(formData.value.tipoDocumentoId),
    areaId: necesitaArea.value ? Number(formData.value.areaId) : null,
    numeroDocumento: Number(formData.value.numeroDocumento),
    añoDocumento: Number(formData.value.añoDocumento),
    fecha: formData.value.fecha.trim(),
    fechaValida: formData.value.fechaValida.trim() ? formData.value.fechaValida.trim() : null,
    descripcion: formData.value.descripcion.trim(),
  }

  emit('save', payload)
}
</script>

<template>
  <Modal :is-open="isOpen" @close="handleClose" title="Registrar Nuevo Documento" subtitle="Alta formal de acto administrativo en legajo" size="xl">

      <div class="p-4 overflow-y-auto space-y-4 flex-1">
        <div
          v-if="formErrors.general"
          class="p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-meta flex items-center gap-2"
        >
          <IconAlertTriangle class="size-4 shrink-0" />
          <span>{{ formErrors.general }}</span>
        </div>

        <div
          v-if="servidorNombre || servidorDni"
          class="p-3 rounded-xl bg-muted/40 border border-border/80 flex items-center justify-between gap-2"
        >
          <div class="flex items-center gap-2 min-w-0">
            <span class="font-bold text-foreground text-meta truncate">{{ servidorNombre || 'Servidor' }}</span>
            <span v-if="servidorDni" class="font-mono text-muted-foreground text-micro">
              (DNI {{ servidorDni }})
            </span>
          </div>
          <Badge variant="outline" size="xs">Legajo Personal</Badge>
        </div>

        <div class="space-y-3.5">
          <div class="space-y-1.5">
            <label for="doc-tipo" class="hs-label flex items-center gap-1.5">
              <IconFileDescription class="size-3.5 text-muted-foreground" />
              Tipo de Documento
              <span class="text-destructive">*</span>
            </label>

            <select
              id="doc-tipo"
              v-model="formData.tipoDocumentoId"
              :disabled="isLoadingCatalogos || isSaving"
              class="hs-input px-3"
              :class="formErrors.tipoDocumentoId ? 'border-destructive' : ''"
            >
              <option value="" disabled>Seleccione el tipo de documento...</option>
              <option
                v-for="tipo in tiposDocumento"
                :key="tipo.id"
                :value="tipo.id"
              >
                {{ tipo.nombre }}
              </option>
            </select>
            <p v-if="formErrors.tipoDocumentoId" class="text-micro text-destructive">
              {{ formErrors.tipoDocumentoId }}
            </p>
          </div>

          <div v-if="necesitaArea" class="space-y-1.5 transition-all">
            <div class="flex items-center justify-between">
              <label for="doc-area" class="hs-label flex items-center gap-1.5">
                <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" />
                Área de Procedencia / Emisión
                <span class="text-destructive">*</span>
              </label>
              <Badge variant="secondary" size="xs">Requerido</Badge>
            </div>

            <select
              id="doc-area"
              v-model="formData.areaId"
              :disabled="isLoadingCatalogos || isSaving"
              class="hs-input px-3"
              :class="formErrors.areaId ? 'border-destructive' : ''"
            >
              <option value="" disabled>Seleccione el área correspondiente...</option>
              <option
                v-for="area in areas"
                :key="area.id"
                :value="area.id"
              >
                {{ area.nombre }} {{ area.sigla ? `(${area.sigla})` : '' }}
              </option>
            </select>
            <p v-if="formErrors.areaId" class="text-micro text-destructive">
              {{ formErrors.areaId }}
            </p>
          </div>

          <div
            v-else-if="formData.tipoDocumentoId"
            class="p-2.5 rounded-lg bg-muted/50 border border-border/60 text-muted-foreground text-micro flex items-center gap-2"
          >
            <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
            <span>
              Este tipo de documento (ID {{ formData.tipoDocumentoId }}) está exento de asignación de área.
            </span>
          </div>

          <div class="grid grid-cols-[repeat(auto-fit,minmax(14rem,1fr))] gap-3">
            <div class="space-y-1.5">
              <label for="doc-numero" class="hs-label flex items-center gap-1.5">
                <IconHash class="size-3.5 text-muted-foreground" />
                Número
                <span class="text-destructive">*</span>
              </label>
              <input
                id="doc-numero"
                v-model.number="formData.numeroDocumento"
                type="number"
                min="1"
                placeholder="Ej. 123"
                :disabled="isSaving"
                class="hs-input px-3 font-mono"
                :class="formErrors.numeroDocumento ? 'border-destructive' : ''"
              />
              <p v-if="formErrors.numeroDocumento" class="text-micro text-destructive">
                {{ formErrors.numeroDocumento }}
              </p>
            </div>

            <div class="space-y-1.5">
              <label for="doc-año" class="hs-label flex items-center gap-1.5">
                <IconCalendar class="size-3.5 text-muted-foreground" />
                Año
                <span class="text-destructive">*</span>
              </label>
              <input
                id="doc-año"
                v-model.number="formData.añoDocumento"
                type="number"
                min="1980"
                max="2050"
                :disabled="isSaving"
                class="hs-input px-3 font-mono"
                :class="formErrors.añoDocumento ? 'border-destructive' : ''"
              />
              <p v-if="formErrors.añoDocumento" class="text-micro text-destructive">
                {{ formErrors.añoDocumento }}
              </p>
            </div>
          </div>

          <div class="grid grid-cols-[repeat(auto-fit,minmax(14rem,1fr))] gap-3">
            <div class="space-y-1.5">
              <label for="doc-fecha" class="hs-label flex items-center gap-1.5">
                <IconCalendar class="size-3.5 text-muted-foreground" />
                Fecha de Emisión
                <span class="text-destructive">*</span>
              </label>
              <input
                id="doc-fecha"
                v-model="formData.fecha"
                type="date"
                :disabled="isSaving"
                class="hs-input px-3"
                :class="formErrors.fecha ? 'border-destructive' : ''"
              />
              <p v-if="formErrors.fecha" class="text-micro text-destructive">
                {{ formErrors.fecha }}
              </p>
            </div>

            <div class="space-y-1.5">
              <label for="doc-fecha-valida" class="hs-label flex items-center gap-1.5">
                <IconCalendar class="size-3.5 text-muted-foreground" />
                Fecha de Vigencia
                <span class="text-muted-foreground font-normal">(Opcional)</span>
              </label>
              <input
                id="doc-fecha-valida"
                v-model="formData.fechaValida"
                type="date"
                :disabled="isSaving"
                class="hs-input px-3"
              />
            </div>
          </div>

          <div class="space-y-1.5">
            <label for="doc-descripcion" class="hs-label flex items-center gap-1.5">
              <IconFileDescription class="size-3.5 text-muted-foreground" />
              Descripción / Asunto
              <span class="text-destructive">*</span>
            </label>
            <textarea
              id="doc-descripcion"
              v-model="formData.descripcion"
              rows="3"
              placeholder="Ingrese el asunto, motivo o resolución del documento..."
              :disabled="isSaving"
              class="hs-input resize-none"
              :class="formErrors.descripcion ? 'border-destructive' : ''"
            ></textarea>
            <p v-if="formErrors.descripcion" class="text-micro text-destructive">
              {{ formErrors.descripcion }}
            </p>
          </div>
        </div>
      </div>
    <template #footer>
        <Button
          variant="outline"
          size="sm"
          :disabled="isSaving"
          @click="handleClose"
        >
          Cancelar
        </Button>
        <Button
          variant="primary"
          size="sm"
          :loading="isSaving"
          @click="handleSubmit"
        >
          Registrar Documento
        </Button>
    </template>
  </Modal>
</template>
