<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  fetchTiposDocumentos,
  fetchAreas,
  fetchCargos,
  requiereArea,
  type PersonalVinculo,
  type EventoVinculoDetalle,
  type EventoVinculoPayload,
  type TipoDocumentoOption,
  type AreaOption,
  type CargoOption,
  type DocumentoData,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconX,
  IconFileText,
  IconCheck,
  IconFileCode,
  IconFileCheck,
} from '@tabler/icons-vue'

interface Props {
  isOpen: boolean
  vinculo: PersonalVinculo | null
  evento?: EventoVinculoDetalle | null
  isSaving?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  evento: null,
  isSaving: false,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', payload: EventoVinculoPayload): void
}>()

const tiposEvento = [
  { value: 'rotacion', label: 'Rotación' },
  { value: 'destaque', label: 'Destaque' },
  { value: 'encargo_puesto', label: 'Encargo de Puesto' },
  { value: 'encargo_funciones', label: 'Encargo de Funciones' },
  { value: 'abandono', label: 'Abandono de Cargo' },
  { value: 'suspension', label: 'Suspensión' },
  { value: 'licencia', label: 'Licencia' },
  { value: 'otro', label: 'Otro Evento' },
]

const tiposDocumento = ref<TipoDocumentoOption[]>([])
const areas = ref<AreaOption[]>([])
const cargos = ref<CargoOption[]>([])
const isLoadingCatalogos = ref<boolean>(false)
const formErrors = ref<Record<string, string>>({})

const getTodayDateString = (): string => {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const tipoEvento = ref<string>('rotacion')
const nuevaAreaId = ref<number | ''>('')
const nuevoCargoId = ref<number | ''>('')
const mismoDocumento = ref<boolean>(false)
const accionCierre = ref<'salida' | 'mismo'>('salida')

const docInicio = ref<{
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
  descripcion: '',
})

const docSalida = ref<{
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
  descripcion: '',
})

const esModoEdicion = computed(() => Boolean(props.evento && props.evento.id > 0))

const requiereMovimiento = computed(() => {
  return ['rotacion', 'encargo_puesto', 'encargo_funciones', 'destaque'].includes(tipoEvento.value)
})

const necesitaAreaInicio = computed(() => {
  if (!docInicio.value.tipoDocumento) return false
  return requiereArea(docInicio.value.tipoDocumento)
})

const necesitaAreaSalida = computed(() => {
  if (!docSalida.value.tipoDocumento) return false
  return requiereArea(docSalida.value.tipoDocumento)
})

const cargarCatalogos = async () => {
  if (tiposDocumento.value.length > 0 && areas.value.length > 0 && cargos.value.length > 0) return
  isLoadingCatalogos.value = true
  try {
    const [docsRes, areasRes, cargosRes] = await Promise.all([
      fetchTiposDocumentos(),
      fetchAreas(),
      fetchCargos(),
    ])
    tiposDocumento.value = docsRes
    areas.value = areasRes
    cargos.value = cargosRes
  } finally {
    isLoadingCatalogos.value = false
  }
}

const resetForm = () => {
  formErrors.value = {}
  mismoDocumento.value = false
  accionCierre.value = 'salida'

  if (props.evento) {
    tipoEvento.value = props.evento.tipo_evento || 'rotacion'
    nuevaAreaId.value = props.evento.nueva_area_id || ''
    nuevoCargoId.value = props.evento.nuevo_cargo_id || ''
    docSalida.value = {
      tipoDocumento: '',
      areaId: '',
      numeroDocumento: '',
      añoDocumento: new Date().getFullYear(),
      fecha: getTodayDateString(),
      fechaValida: '',
      descripcion: `Cierre de ${props.evento.tipo_evento}`,
    }
  } else {
    tipoEvento.value = 'rotacion'
    nuevaAreaId.value = ''
    nuevoCargoId.value = ''
    docInicio.value = {
      tipoDocumento: '',
      areaId: '',
      numeroDocumento: '',
      añoDocumento: new Date().getFullYear(),
      fecha: getTodayDateString(),
      fechaValida: '',
      descripcion: '',
    }
    docSalida.value = {
      tipoDocumento: '',
      areaId: '',
      numeroDocumento: '',
      añoDocumento: new Date().getFullYear(),
      fecha: getTodayDateString(),
      fechaValida: '',
      descripcion: '',
    }
  }
}

watch(
  () => props.isOpen,
  async (val) => {
    if (val) {
      resetForm()
      await cargarCatalogos()
    }
  },
  { immediate: true }
)

watch(
  () => props.evento,
  () => {
    if (props.isOpen) {
      resetForm()
    }
  }
)

const validarFormulario = (): boolean => {
  const errors: Record<string, string> = {}

  if (!tipoEvento.value) {
    errors.tipoEvento = 'Seleccione el tipo de evento.'
  }

  if (!esModoEdicion.value) {
    if (!docInicio.value.tipoDocumento) {
      errors.tipoDocumentoInicio = 'El tipo de documento es requerido.'
    }
    if (!docInicio.value.numeroDocumento || Number(docInicio.value.numeroDocumento) <= 0) {
      errors.numeroDocumentoInicio = 'Ingrese un número válido.'
    }
    if (!docInicio.value.añoDocumento || Number(docInicio.value.añoDocumento) < 1900) {
      errors.añoDocumentoInicio = 'Ingrese un año válido.'
    }
    if (!docInicio.value.fecha) {
      errors.fechaInicio = 'La fecha del documento es requerida.'
    }
    if (necesitaAreaInicio.value && !docInicio.value.areaId) {
      errors.areaInicio = 'Debe seleccionar el área emisora del documento.'
    }
    if (!docInicio.value.descripcion.trim()) {
      errors.descripcionInicio = 'Ingrese una descripción o referencia.'
    }
  } else {
    if (accionCierre.value === 'salida') {
      if (!docSalida.value.tipoDocumento) {
        errors.tipoDocumentoSalida = 'El tipo de documento de término es requerido.'
      }
      if (!docSalida.value.numeroDocumento || Number(docSalida.value.numeroDocumento) <= 0) {
        errors.numeroDocumentoSalida = 'Ingrese un número válido.'
      }
      if (!docSalida.value.añoDocumento || Number(docSalida.value.añoDocumento) < 1900) {
        errors.añoDocumentoSalida = 'Ingrese un año válido.'
      }
      if (!docSalida.value.fecha) {
        errors.fechaSalida = 'La fecha de término es requerida.'
      }
      if (necesitaAreaSalida.value && !docSalida.value.areaId) {
        errors.areaSalida = 'Debe seleccionar el área emisora.'
      }
      if (!docSalida.value.descripcion.trim()) {
        errors.descripcionSalida = 'Ingrese una descripción de salida o término.'
      }
    }
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleSubmit = () => {
  if (!props.vinculo) return
  if (!validarFormulario()) return

  if (esModoEdicion.value && props.evento) {
    const payload: EventoVinculoPayload = {
      id: props.evento.id,
      vinculo_id: props.vinculo.id,
      tipo_evento: props.evento.tipo_evento,
      mismo_documento: accionCierre.value === 'mismo',
      documento_salida:
        accionCierre.value === 'salida'
          ? {
              tipoDocumento: String(docSalida.value.tipoDocumento),
              areaId: docSalida.value.areaId ? Number(docSalida.value.areaId) : null,
              numeroDocumento: Number(docSalida.value.numeroDocumento),
              añoDocumento: Number(docSalida.value.añoDocumento),
              fecha: docSalida.value.fecha,
              fechaValida: docSalida.value.fechaValida || null,
              descripcion: docSalida.value.descripcion.trim(),
            }
          : null,
    }
    emit('save', payload)
  } else {
    const payloadDocumentoInicio: DocumentoData = {
      tipoDocumento: String(docInicio.value.tipoDocumento),
      areaId: docInicio.value.areaId ? Number(docInicio.value.areaId) : null,
      numeroDocumento: Number(docInicio.value.numeroDocumento),
      añoDocumento: Number(docInicio.value.añoDocumento),
      fecha: docInicio.value.fecha,
      fechaValida: docInicio.value.fechaValida || null,
      descripcion: docInicio.value.descripcion.trim(),
    }

    const payload: EventoVinculoPayload = {
      vinculo_id: props.vinculo.id,
      tipo_evento: tipoEvento.value,
      nueva_area_id: nuevaAreaId.value ? Number(nuevaAreaId.value) : null,
      nuevo_cargo_id: nuevoCargoId.value ? Number(nuevoCargoId.value) : null,
      documento_inicio: payloadDocumentoInicio,
      mismo_documento: mismoDocumento.value,
      estado: 'activo',
    }
    emit('save', payload)
  }
}
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 overflow-y-auto" role="dialog" aria-modal="true">
    <transition appear enter-active-class="transition-opacity duration-250 ease-out" enter-from-class="opacity-0"
      enter-to-class="opacity-100" leave-active-class="transition-opacity duration-200 ease-in"
      leave-from-class="opacity-100" leave-to-class="opacity-0">
      <div class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs" @click="!isSaving && emit('close')"></div>
    </transition>

    <div class="flex min-h-screen items-center justify-center p-3 sm:p-4 text-center">
      <transition appear enter-active-class="transform transition ease-out duration-300"
        enter-from-class="scale-95 opacity-0" enter-to-class="scale-100 opacity-100"
        leave-active-class="transform transition ease-in duration-200" leave-from-class="scale-100 opacity-100"
        leave-to-class="scale-95 opacity-0">
        <div
          class="relative w-full max-w-xl rounded-2xl bg-card border border-border shadow-2xl p-5 sm:p-6 text-left flex flex-col max-h-[92vh] overflow-hidden"
          @click.stop>
          <div class="flex items-center justify-between pb-3.5 border-b border-border shrink-0">
            <div class="flex items-center gap-2.5 min-w-0">
              <div
                class="size-8 rounded-lg bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center justify-center border border-purple-500/20 shrink-0">
                <IconFileCode class="size-4.5" />
              </div>
              <div class="min-w-0">
                <h3 class="font-bold text-base text-foreground tracking-tight truncate">
                  {{ esModoEdicion ? 'Finalizar / Cerrar Evento Laboral' : 'Registrar Nuevo Evento Laboral' }}
                </h3>
                <p class="text-xs text-muted-foreground truncate">
                  {{ vinculo ? `${vinculo.cargo} • ${vinculo.area || 'Sin área'}` : 'Vínculo laboral' }}
                </p>
              </div>
            </div>

            <button type="button"
              class="size-8 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted/60 flex items-center justify-center transition-colors cursor-pointer shrink-0"
              aria-label="Cerrar modal" :disabled="isSaving" @click="emit('close')">
              <IconX class="size-4" />
            </button>
          </div>

          <form class="flex-1 overflow-y-auto space-y-4 py-4 pr-1 text-xs" @submit.prevent="handleSubmit">
            <div v-if="esModoEdicion && evento" class="p-3.5 rounded-xl border border-border bg-muted/20 space-y-2">
              <div class="flex items-center justify-between gap-2">
                <span class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">Evento Actual</span>
                <Badge size="xs" variant="warning">
                  ID #{{ evento.id }} &bull; {{ evento.tipo_evento }}
                </Badge>
              </div>
              <div class="grid grid-cols-2 gap-2 text-xs pt-1">
                <div>
                  <span class="text-muted-foreground block text-[11px]">Doc. Inicio:</span>
                  <span class="font-medium text-foreground truncate block">
                    {{ [evento.tipo_doc_inicio, evento.numero_doc_inicio].filter(Boolean).join(' N° ') || '-' }}
                  </span>
                </div>
                <div>
                  <span class="text-muted-foreground block text-[11px]">Fecha Inicio:</span>
                  <span class="font-mono text-foreground font-medium block">
                    {{ evento.fecha_inicio ? formatDate(evento.fecha_inicio) : '-' }}
                  </span>
                </div>
                <div v-if="evento.nueva_area" class="col-span-2">
                  <span class="text-muted-foreground block text-[11px]">Área Asignada:</span>
                  <span class="font-medium text-foreground block">{{ evento.nueva_area }}</span>
                </div>
              </div>
            </div>

            <div v-if="!esModoEdicion" class="space-y-3">
              <div>
                <label class="block font-medium text-foreground mb-1 text-xs">
                  Tipo de Evento o Movimiento <span class="text-rose-500">*</span>
                </label>
                <select v-model="tipoEvento"
                  class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer"
                  :class="formErrors.tipoEvento ? 'border-rose-500 ring-rose-500' : 'border-border'">
                  <option v-for="t in tiposEvento" :key="t.value" :value="t.value">
                    {{ t.label }}
                  </option>
                </select>
                <p v-if="formErrors.tipoEvento" class="text-rose-500 text-[11px] mt-1">
                  {{ formErrors.tipoEvento }}
                </p>
              </div>

              <div v-if="requiereMovimiento" class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/80 bg-muted/10">
                <div>
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Nueva Área Asignada
                  </label>
                  <select v-model="nuevaAreaId"
                    class="w-full rounded-lg border border-border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer">
                    <option value="">Mantener área actual</option>
                    <option v-for="a in areas" :key="a.id" :value="a.id">
                      {{ a.nombre }}
                    </option>
                  </select>
                </div>

                <div>
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Nuevo Cargo
                  </label>
                  <select v-model="nuevoCargoId"
                    class="w-full rounded-lg border border-border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer">
                    <option value="">Mantener cargo actual</option>
                    <option v-for="c in cargos" :key="c.id" :value="c.id">
                      {{ c.nombre }}
                    </option>
                  </select>
                </div>
              </div>

              <div class="space-y-3 pt-2 border-t border-border/70">
                <div class="flex items-center gap-2">
                  <IconFileText class="size-4 text-primary shrink-0" />
                  <span class="font-semibold text-foreground text-xs uppercase tracking-wider">
                    Documento Sustentatorio de Inicio
                  </span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                  <div class="sm:col-span-2">
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Tipo de Documento <span class="text-rose-500">*</span>
                    </label>
                    <select v-model="docInicio.tipoDocumento"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer"
                      :class="formErrors.tipoDocumentoInicio ? 'border-rose-500 ring-rose-500' : 'border-border'">
                      <option value="" disabled>Seleccione tipo de documento</option>
                      <option v-for="td in tiposDocumento" :key="td.id" :value="td.id">
                        {{ td.nombre }} {{ td.sigla ? `(${td.sigla})` : '' }}
                      </option>
                    </select>
                    <p v-if="formErrors.tipoDocumentoInicio" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.tipoDocumentoInicio }}
                    </p>
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Año <span class="text-rose-500">*</span>
                    </label>
                    <input v-model.number="docInicio.añoDocumento" type="number" min="1950" max="2099"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.añoDocumentoInicio ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                  </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Número de Documento <span class="text-rose-500">*</span>
                    </label>
                    <input v-model.number="docInicio.numeroDocumento" type="number" min="1" placeholder="Ej. 124"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.numeroDocumentoInicio ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.numeroDocumentoInicio" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.numeroDocumentoInicio }}
                    </p>
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Fecha de Documento <span class="text-rose-500">*</span>
                    </label>
                    <input v-model="docInicio.fecha" type="date"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.fechaInicio ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.fechaInicio" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.fechaInicio }}
                    </p>
                  </div>
                </div>

                <div v-if="necesitaAreaInicio">
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Área Emisora del Documento <span class="text-rose-500">*</span>
                  </label>
                  <select v-model="docInicio.areaId"
                    class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer"
                    :class="formErrors.areaInicio ? 'border-rose-500 ring-rose-500' : 'border-border'">
                    <option value="" disabled>Seleccione área emisora</option>
                    <option v-for="a in areas" :key="a.id" :value="a.id">
                      {{ a.nombre }} {{ a.sigla ? `(${a.sigla})` : '' }}
                    </option>
                  </select>
                  <p v-if="formErrors.areaInicio" class="text-rose-500 text-[11px] mt-1">
                    {{ formErrors.areaInicio }}
                  </p>
                </div>

                <div>
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Fecha Válida / Vigencia (Opcional)
                  </label>
                  <input v-model="docInicio.fechaValida" type="date"
                    class="w-full rounded-lg border border-border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono" />
                </div>

                <div>
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Descripción / Asunto del Documento <span class="text-rose-500">*</span>
                  </label>
                  <textarea v-model="docInicio.descripcion" rows="2" placeholder="Motivo o detalle del evento..."
                    class="w-full rounded-lg border bg-background p-2.5 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden leading-relaxed"
                    :class="formErrors.descripcionInicio ? 'border-rose-500 ring-rose-500' : 'border-border'"></textarea>
                  <p v-if="formErrors.descripcionInicio" class="text-rose-500 text-[11px] mt-1">
                    {{ formErrors.descripcionInicio }}
                  </p>
                </div>

                <div class="flex items-center gap-2.5 pt-1">
                  <input id="check-mismo-doc" v-model="mismoDocumento" type="checkbox"
                    class="size-4 rounded border-border text-primary focus:ring-primary cursor-pointer" />
                  <label for="check-mismo-doc" class="text-xs text-foreground cursor-pointer select-none">
                    El documento define el inicio y término del evento simultáneamente (mismo documento)
                  </label>
                </div>
              </div>
            </div>

            <div v-else class="space-y-3">
              <div class="space-y-2">
                <span class="block font-medium text-foreground text-xs">Modalidad de Cierre de Evento</span>
                <div class="grid grid-cols-2 gap-2">
                  <label
                    class="p-2.5 rounded-xl border flex items-center gap-2 cursor-pointer transition-colors"
                    :class="accionCierre === 'salida' ? 'border-primary bg-primary/5 text-primary' : 'border-border bg-card text-foreground'">
                    <input v-model="accionCierre" type="radio" value="salida" class="sr-only" />
                    <IconFileCheck class="size-4 shrink-0" />
                    <span class="text-xs font-medium">Registrar Doc. de Término</span>
                  </label>

                  <label
                    class="p-2.5 rounded-xl border flex items-center gap-2 cursor-pointer transition-colors"
                    :class="accionCierre === 'mismo' ? 'border-primary bg-primary/5 text-primary' : 'border-border bg-card text-foreground'">
                    <input v-model="accionCierre" type="radio" value="mismo" class="sr-only" />
                    <IconCheck class="size-4 shrink-0" />
                    <span class="text-xs font-medium">Usar Mismo Doc. de Inicio</span>
                  </label>
                </div>
              </div>

              <div v-if="accionCierre === 'salida'" class="space-y-3 pt-2 border-t border-border/70">
                <div class="flex items-center gap-2">
                  <IconFileCheck class="size-4 text-emerald-600 dark:text-emerald-400 shrink-0" />
                  <span class="font-semibold text-foreground text-xs uppercase tracking-wider">
                    Documento Sustentatorio de Cierre / Término
                  </span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                  <div class="sm:col-span-2">
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Tipo de Documento <span class="text-rose-500">*</span>
                    </label>
                    <select v-model="docSalida.tipoDocumento"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer"
                      :class="formErrors.tipoDocumentoSalida ? 'border-rose-500 ring-rose-500' : 'border-border'">
                      <option value="" disabled>Seleccione tipo de documento</option>
                      <option v-for="td in tiposDocumento" :key="td.id" :value="td.id">
                        {{ td.nombre }} {{ td.sigla ? `(${td.sigla})` : '' }}
                      </option>
                    </select>
                    <p v-if="formErrors.tipoDocumentoSalida" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.tipoDocumentoSalida }}
                    </p>
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Año <span class="text-rose-500">*</span>
                    </label>
                    <input v-model.number="docSalida.añoDocumento" type="number" min="1950" max="2099"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.añoDocumentoSalida ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                  </div>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Número de Documento <span class="text-rose-500">*</span>
                    </label>
                    <input v-model.number="docSalida.numeroDocumento" type="number" min="1" placeholder="Ej. 189"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.numeroDocumentoSalida ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.numeroDocumentoSalida" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.numeroDocumentoSalida }}
                    </p>
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Fecha de Término <span class="text-rose-500">*</span>
                    </label>
                    <input v-model="docSalida.fecha" type="date"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.fechaSalida ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.fechaSalida" class="text-rose-500 text-[11px] mt-1">
                      {{ formErrors.fechaSalida }}
                    </p>
                  </div>
                </div>

                <div v-if="necesitaAreaSalida">
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Área Emisora <span class="text-rose-500">*</span>
                  </label>
                  <select v-model="docSalida.areaId"
                    class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden transition-colors cursor-pointer"
                    :class="formErrors.areaSalida ? 'border-rose-500 ring-rose-500' : 'border-border'">
                    <option value="" disabled>Seleccione área emisora</option>
                    <option v-for="a in areas" :key="a.id" :value="a.id">
                      {{ a.nombre }} {{ a.sigla ? `(${a.sigla})` : '' }}
                    </option>
                  </select>
                  <p v-if="formErrors.areaSalida" class="text-rose-500 text-[11px] mt-1">
                    {{ formErrors.areaSalida }}
                  </p>
                </div>

                <div>
                  <label class="block font-medium text-foreground mb-1 text-xs">
                    Descripción de Cierre <span class="text-rose-500">*</span>
                  </label>
                  <textarea v-model="docSalida.descripcion" rows="2" placeholder="Detalles de la conclusión del evento..."
                    class="w-full rounded-lg border bg-background p-2.5 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden leading-relaxed"
                    :class="formErrors.descripcionSalida ? 'border-rose-500 ring-rose-500' : 'border-border'"></textarea>
                  <p v-if="formErrors.descripcionSalida" class="text-rose-500 text-[11px] mt-1">
                    {{ formErrors.descripcionSalida }}
                  </p>
                </div>
              </div>

              <div v-else class="p-3.5 rounded-xl border border-border/80 bg-muted/20 flex items-start gap-2.5">
                <IconCheck class="size-4.5 text-emerald-600 dark:text-emerald-400 shrink-0 mt-0.5" />
                <div class="space-y-0.5">
                  <p class="font-medium text-foreground text-xs">Cierre con Documento de Inicio</p>
                  <p class="text-muted-foreground text-[11px] leading-relaxed">
                    Se utilizará el mismo documento de inicio registrado anteriormente para marcar la finalización de este evento y actualizar el estado laboral del trabajador.
                  </p>
                </div>
              </div>
            </div>
          </form>

          <div class="p-3.5 border-t border-border flex items-center justify-end gap-2.5 shrink-0 bg-muted/10">
            <Button size="sm" variant="outline" class="cursor-pointer" :disabled="isSaving" @click="emit('close')">
              Cancelar
            </Button>
            <Button size="sm" class="bg-primary hover:bg-primary/90 text-primary-foreground gap-1.5 cursor-pointer"
              :disabled="isSaving" @click="handleSubmit">
              <IconCheck class="size-4" />
              <span>{{ isSaving ? 'Guardando...' : (esModoEdicion ? 'Confirmar Cierre' : 'Registrar Evento') }}</span>
            </Button>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>
