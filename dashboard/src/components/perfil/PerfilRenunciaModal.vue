<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  fetchTiposDocumentos,
  fetchAreas,
  requiereArea,
  type PersonalVinculo,
  type RenunciaPayload,
  type TipoDocumentoOption,
  type AreaOption,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconFileX,
  IconX,
  IconAlertTriangle,
  IconBriefcase,
  IconBuildingSkyscraper,
  IconCalendar,
  IconId,
  IconSearch,
  IconCheck,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  vinculo: PersonalVinculo | null
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
  (e: 'save', payload: RenunciaPayload): void
}>()

const tiposDocumento = ref<TipoDocumentoOption[]>([])
const areas = ref<AreaOption[]>([])
const searchAreaQuery = ref<string>('')
const formErrors = ref<Record<string, string>>({})
const isLoadingCatalogos = ref<boolean>(false)

const getTodayDateString = (): string => {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const formData = ref<{
  tipoDocumento: number | string
  areaId: number | ''
  numeroDocumento: number | ''
  añoDocumento: number
  fecha: string
  fechaValida: string
  descripcion: string
}>({
  tipoDocumento: '',
  areaId: '',
  numeroDocumento: '',
  añoDocumento: new Date().getFullYear(),
  fecha: getTodayDateString(),
  fechaValida: '',
  descripcion: 'Renuncia voluntaria',
})

const necesitaArea = computed(() => {
  if (!formData.value.tipoDocumento) return false
  return requiereArea(formData.value.tipoDocumento)
})

const areasFiltradas = computed(() => {
  const query = searchAreaQuery.value.trim().toLowerCase()
  if (!query) return areas.value
  return areas.value.filter((a) => {
    const nombre = a.nombre ? a.nombre.toLowerCase() : ''
    const sigla = a.sigla ? a.sigla.toLowerCase() : ''
    return nombre.includes(query) || sigla.includes(query)
  })
})

const areaSeleccionada = computed(() => {
  if (!formData.value.areaId) return null
  return areas.value.find((a) => a.id === Number(formData.value.areaId)) || null
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

    if (!formData.value.tipoDocumento && tiposRes.length > 0) {
      formData.value.tipoDocumento = tiposRes[0].id
    }
  } finally {
    isLoadingCatalogos.value = false
  }
}

watch(
  () => formData.value.tipoDocumento,
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
      searchAreaQuery.value = ''
      formData.value = {
        tipoDocumento: tiposDocumento.value[0]?.id || '',
        areaId: '',
        numeroDocumento: '',
        añoDocumento: new Date().getFullYear(),
        fecha: getTodayDateString(),
        fechaValida: '',
        descripcion: 'Renuncia voluntaria',
      }
      await loadCatalogos()
    }
  },
)

const validateForm = (): boolean => {
  const errors: Record<string, string> = {}

  if (!props.vinculo?.id) {
    errors.general = 'No se ha seleccionado un vínculo laboral válido para registrar la renuncia.'
  }

  if (!formData.value.tipoDocumento) {
    errors.tipoDocumento = 'Seleccione el tipo de documento.'
  }

  if (necesitaArea.value && (!formData.value.areaId || Number(formData.value.areaId) <= 0)) {
    errors.areaId = 'El área es requerida para este tipo de documento.'
  }

  const num = Number(formData.value.numeroDocumento)
  if (!formData.value.numeroDocumento || isNaN(num) || num <= 0) {
    errors.numeroDocumento = 'Ingrese un número de documento válido y mayor a cero.'
  }

  const anio = Number(formData.value.añoDocumento)
  if (!formData.value.añoDocumento || isNaN(anio) || anio < 1990 || anio > 2050) {
    errors.añoDocumento = 'Ingrese un año válido (entre 1990 y 2050).'
  }

  if (!formData.value.fecha || !formData.value.fecha.trim()) {
    errors.fecha = 'La fecha de renuncia o cese es obligatoria.'
  }

  if (!formData.value.descripcion || !formData.value.descripcion.trim()) {
    errors.descripcion = 'La descripción o motivo es obligatoria.'
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleClose = () => {
  if (props.isSaving) return
  emit('close')
}

const handleSubmit = () => {
  if (!validateForm() || !props.vinculo?.id) return

  const payload: RenunciaPayload = {
    id: props.vinculo.id,
    tipoDocumento: String(formData.value.tipoDocumento),
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
  <div
    v-if="isOpen"
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/65 backdrop-blur-xs"
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-renuncia-title"
    @click.self="handleClose"
  >
    <div
      class="w-full max-w-xl bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-xs max-h-[92vh] flex flex-col"
    >
      <div class="p-4 border-b border-border flex items-center justify-between bg-muted/20 shrink-0">
        <div class="flex items-center gap-2.5 min-w-0">
          <div
            class="size-8 rounded-lg bg-rose-500/10 text-rose-600 dark:text-rose-400 border border-rose-500/20 flex items-center justify-center shrink-0"
          >
            <IconFileX class="size-4.5" aria-hidden="true" />
          </div>
          <div class="min-w-0">
            <h3 id="modal-renuncia-title" class="font-bold text-foreground text-sm tracking-tight truncate">
              Registrar Renuncia / Cese de Vínculo
            </h3>
            <p class="text-[11px] text-muted-foreground truncate">
              Formalización de la conclusión laboral mediante acto administrativo
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

      <div class="p-4 sm:p-5 overflow-y-auto space-y-4 flex-1">
        <div
          v-if="formErrors.general"
          class="p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs flex items-center gap-2"
        >
          <IconAlertTriangle class="size-4 shrink-0" />
          <span>{{ formErrors.general }}</span>
        </div>

        <div v-if="vinculo" class="p-3.5 rounded-xl bg-muted/40 border border-border/80 space-y-2.5">
          <div class="flex flex-wrap items-center justify-between gap-2">
            <div class="flex items-center gap-2 min-w-0">
              <span class="font-bold text-foreground text-xs truncate">{{ servidorNombre || 'Servidor' }}</span>
              <span v-if="servidorDni" class="font-mono text-muted-foreground text-[11px]">
                (DNI {{ servidorDni }})
              </span>
            </div>
            <Badge variant="warning" size="xs">Vínculo Actual</Badge>
          </div>

          <div
            class="grid grid-cols-1 sm:grid-cols-2 gap-2 text-[11px] text-muted-foreground pt-1 border-t border-border/60"
          >
            <div class="flex items-center gap-1.5 truncate">
              <IconBriefcase class="size-3.5 text-muted-foreground shrink-0" />
              <span class="font-medium text-foreground truncate">{{ vinculo.cargo }}</span>
            </div>
            <div class="flex items-center gap-1.5 truncate">
              <IconBuildingSkyscraper class="size-3.5 text-muted-foreground shrink-0" />
              <span class="truncate">{{ vinculo.area }}</span>
            </div>
            <div class="flex items-center gap-1.5 truncate">
              <IconId class="size-3.5 text-muted-foreground shrink-0" />
              <span>{{ vinculo.regimen }} &bull; {{ vinculo.codigo ? `Plaza ${vinculo.codigo}` : 'Sin plaza' }}</span>
            </div>
            <div class="flex items-center gap-1.5 truncate">
              <IconCalendar class="size-3.5 text-muted-foreground shrink-0" />
              <span>Ingreso: {{ formatDate(vinculo.fecha_ingreso) }}</span>
            </div>
          </div>
        </div>

        <form id="renuncia-form" class="space-y-4" @submit.prevent="handleSubmit">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <div class="space-y-1 sm:col-span-1">
              <label for="renuncia-tipo-doc" class="block font-medium text-foreground text-[11.5px]">
                Tipo de Documento <span class="text-rose-500">*</span>
              </label>
              <select
                id="renuncia-tipo-doc"
                v-model="formData.tipoDocumento"
                :disabled="isLoadingCatalogos || isSaving"
                class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition"
                :class="formErrors.tipoDocumento ? 'border-destructive' : 'border-input'"
              >
                <option v-if="isLoadingCatalogos" disabled value="">
                  Cargando tipos de documentos...
                </option>
                <option v-for="t in tiposDocumento" :key="t.id" :value="t.id">
                  {{ t.nombre }}
                </option>
              </select>
              <p v-if="formErrors.tipoDocumento" class="text-[10px] text-destructive font-medium">
                {{ formErrors.tipoDocumento }}
              </p>
            </div>

            <div class="space-y-1 sm:col-span-1">
              <label for="renuncia-num-doc" class="block font-medium text-foreground text-[11.5px]">
                Número de Documento <span class="text-rose-500">*</span>
              </label>
              <input
                id="renuncia-num-doc"
                v-model.number="formData.numeroDocumento"
                type="number"
                min="1"
                step="1"
                placeholder="Ej. 456"
                :disabled="isSaving"
                class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition font-mono"
                :class="formErrors.numeroDocumento ? 'border-destructive' : 'border-input'"
              />
              <p v-if="formErrors.numeroDocumento" class="text-[10px] text-destructive font-medium">
                {{ formErrors.numeroDocumento }}
              </p>
            </div>

            <div class="space-y-1 sm:col-span-1">
              <label for="renuncia-anio-doc" class="block font-medium text-foreground text-[11.5px]">
                Año <span class="text-rose-500">*</span>
              </label>
              <input
                id="renuncia-anio-doc"
                v-model.number="formData.añoDocumento"
                type="number"
                min="1990"
                max="2050"
                step="1"
                :disabled="isSaving"
                class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition font-mono"
                :class="formErrors.añoDocumento ? 'border-destructive' : 'border-input'"
              />
              <p v-if="formErrors.añoDocumento" class="text-[10px] text-destructive font-medium">
                {{ formErrors.añoDocumento }}
              </p>
            </div>
          </div>

          <div v-if="necesitaArea" class="space-y-2 p-3 rounded-xl bg-muted/20 border border-border/70">
            <div class="flex items-center justify-between">
              <label for="renuncia-buscar-area" class="font-semibold text-foreground text-xs flex items-center gap-1.5">
                <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" />
                Área de Emisión del Documento
                <span class="text-rose-500">*</span>
              </label>
              <Badge variant="secondary" size="xs">Requerido</Badge>
            </div>

            <div class="relative">
              <IconSearch class="size-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
              <input
                id="renuncia-buscar-area"
                v-model="searchAreaQuery"
                type="text"
                placeholder="Buscar área por nombre o sigla (ej. Gerencia, Recursos)..."
                :disabled="isLoadingCatalogos || isSaving"
                class="w-full h-8 pl-8 pr-3 rounded-lg border border-input bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition"
              />
            </div>

            <select
              id="renuncia-area-select"
              v-model="formData.areaId"
              :disabled="isLoadingCatalogos || isSaving"
              class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition"
              :class="formErrors.areaId ? 'border-destructive' : 'border-input'"
            >
              <option value="" disabled>Seleccione el área del documento...</option>
              <option
                v-for="area in areasFiltradas"
                :key="area.id"
                :value="area.id"
              >
                {{ area.nombre }} {{ area.sigla ? `(${area.sigla})` : '' }}
              </option>
            </select>

            <div v-if="areaSeleccionada" class="text-[11px] text-muted-foreground flex items-center gap-1.5">
              <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
              <span>
                Área seleccionada:
                <strong class="text-foreground">{{ areaSeleccionada.nombre }}</strong>
                <span v-if="areaSeleccionada.sigla"> ({{ areaSeleccionada.sigla }})</span>
              </span>
            </div>

            <p v-if="formErrors.areaId" class="text-[10px] text-destructive font-medium">
              {{ formErrors.areaId }}
            </p>
          </div>

          <div
            v-else-if="formData.tipoDocumento"
            class="p-2.5 rounded-lg bg-muted/40 border border-border/60 text-muted-foreground text-[11px] flex items-center gap-2"
          >
            <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
            <span>
              Este tipo de documento (ID {{ formData.tipoDocumento }}) está exento de asignación de área.
            </span>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="space-y-1">
              <label for="renuncia-fecha" class="block font-medium text-foreground text-[11.5px]">
                Fecha de Renuncia / Cese <span class="text-rose-500">*</span>
              </label>
              <input
                id="renuncia-fecha"
                v-model="formData.fecha"
                type="date"
                :disabled="isSaving"
                class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition font-mono"
                :class="formErrors.fecha ? 'border-destructive' : 'border-input'"
              />
              <p v-if="formErrors.fecha" class="text-[10px] text-destructive font-medium">
                {{ formErrors.fecha }}
              </p>
            </div>

            <div class="space-y-1">
              <label for="renuncia-fecha-valida" class="block font-medium text-foreground text-[11.5px]">
                Fecha Válida <span class="text-muted-foreground font-normal">(Opcional)</span>
              </label>
              <input
                id="renuncia-fecha-valida"
                v-model="formData.fechaValida"
                type="date"
                :disabled="isSaving"
                class="w-full h-9 px-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition font-mono border-input"
              />
            </div>
          </div>

          <div class="space-y-1.5">
            <div class="flex items-center justify-between">
              <label for="renuncia-descripcion" class="block font-medium text-foreground text-[11.5px]">
                Descripción / Motivo <span class="text-rose-500">*</span>
              </label>
            </div>

            <textarea
              id="renuncia-descripcion"
              v-model="formData.descripcion"
              rows="2"
              placeholder="Indique los detalles o justificación de la renuncia o término de funciones..."
              :disabled="isSaving"
              class="w-full p-2.5 rounded-lg border bg-background text-foreground text-xs focus:outline-hidden focus:ring-2 focus:ring-primary focus:border-primary transition resize-none"
              :class="formErrors.descripcion ? 'border-destructive' : 'border-input'"
            ></textarea>
            <p v-if="formErrors.descripcion" class="text-[10px] text-destructive font-medium">
              {{ formErrors.descripcion }}
            </p>
          </div>
        </form>
      </div>

      <div class="p-4 border-t border-border flex items-center justify-end gap-2.5 bg-muted/10 shrink-0">
        <Button
          variant="outline"
          size="sm"
          type="button"
          :disabled="isSaving"
          @click="handleClose"
        >
          Cancelar
        </Button>
        <Button
          variant="danger"
          size="sm"
          type="submit"
          form="renuncia-form"
          class="gap-1.5 shadow-xs cursor-pointer"
          :disabled="isSaving"
        >
          <IconFileX class="size-3.5" />
          <span>{{ isSaving ? 'Registrando...' : 'Confirmar Renuncia' }}</span>
        </Button>
      </div>
    </div>
  </div>
</template>
