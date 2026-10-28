<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import { useToastStore } from '@/stores/toast'
import {
  buscarTrabajadores,
  fetchTiposDocumentos,
  fetchAreas,
  fetchCargos,
  requiereArea,
  getTipoEventoLabel,
  type PersonalVinculo,
  type EventoVinculoDetalle,
  type EventoVinculoPayload,
  type TipoDocumentoOption,
  type AreaOption,
  type CargoOption,
  type DocumentoData,
  type PersonalSearchResult,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconX,
  IconFileText,
  IconCheck,
  IconFileCode,
  IconFileCheck,
  IconEdit,
  IconUser,
  IconSearch,
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
  { value: 'vacaciones', label: 'Vacaciones' },
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

const toast = useToastStore()

const tabModoEvento = ref<'cierre' | 'editar'>('cierre')
const tipoEvento = ref<string>('rotacion')
const nuevaAreaId = ref<number | ''>('')
const nuevoCargoId = ref<number | ''>('')
const mismoDocumento = ref<boolean>(false)
const accionCierre = ref<'salida' | 'mismo'>('salida')

// Encargaturas / vigencia
const fechaDesde = ref<string>('')
const fechaHasta = ref<string>('')

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

const esEncargo = computed(() => {
  return ['encargatura', 'encargo_puesto', 'encargo_funciones'].includes(tipoEvento.value)
})

const esVacaciones = computed(() => tipoEvento.value === 'vacaciones')
const requiereVigencia = computed(() => esEncargo.value || esVacaciones.value)

// Buscador de encargado temporal para vacaciones
const encargadoSearchQuery = ref<string>('')
const encargadoSearchResults = ref<PersonalSearchResult[]>([])
const isSearchingEncargado = ref<boolean>(false)
const encargadoSeleccionado = ref<{ dni: string; nombre: string } | null>(null)
const encargadoError = ref<string>('')

let currentEncargadoAbortController: AbortController | null = null
let debounceEncargadoTimer: ReturnType<typeof setTimeout> | null = null

const handleSearchEncargado = async () => {
  const query = encargadoSearchQuery.value.trim()
  if (query.length < 3) {
    if (currentEncargadoAbortController) {
      currentEncargadoAbortController.abort()
      currentEncargadoAbortController = null
    }
    encargadoSearchResults.value = []
    isSearchingEncargado.value = false
    return
  }

  if (currentEncargadoAbortController) {
    currentEncargadoAbortController.abort()
  }
  const controller = new AbortController()
  currentEncargadoAbortController = controller

  isSearchingEncargado.value = true
  try {
    const data = await buscarTrabajadores(query, controller.signal)
    if (controller.signal.aborted) return
    encargadoSearchResults.value = data
  } catch {
    if (controller.signal.aborted) return
    encargadoSearchResults.value = []
  } finally {
    if (currentEncargadoAbortController === controller) {
      isSearchingEncargado.value = false
      currentEncargadoAbortController = null
    }
  }
}

const onSearchEncargadoInput = () => {
  encargadoError.value = ''
  if (debounceEncargadoTimer) {
    clearTimeout(debounceEncargadoTimer)
  }
  debounceEncargadoTimer = setTimeout(() => {
    handleSearchEncargado()
  }, 350)
}

const seleccionarEncargado = (trabajador: PersonalSearchResult) => {
  if (props.vinculo && trabajador.dni === props.vinculo.dni) {
    encargadoError.value = 'El encargado no puede ser el propio titular'
    return
  }
  encargadoError.value = ''
  encargadoSeleccionado.value = {
    dni: trabajador.dni,
    nombre: trabajador.nombre,
  }
  encargadoSearchQuery.value = ''
  encargadoSearchResults.value = []
}

const limpiarEncargado = () => {
  encargadoSeleccionado.value = null
  encargadoError.value = ''
}

const esModoExistente = computed(() => Boolean(props.evento && props.evento.id > 0))
const esEventoFinalizado = computed(() => {
  if (!props.evento) return false
  const est = props.evento.estado?.toLowerCase()
  return est === 'desactivado' || est === 'finalizado' || Boolean(props.evento.doc_salida_id || props.evento.fecha_salida)
})

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

const areaOptions = computed<SelectOption<number>[]>(() => {
  return areas.value.map((a) => ({
    value: a.id,
    label: a.nombre,
    sublabel: a.sigla ? `Sigla: ${a.sigla}` : undefined,
    badge: a.sigla || undefined,
  }))
})

const cargoOptions = computed<SelectOption<number>[]>(() => {
  return cargos.value.map((c) => ({
    value: c.id,
    label: c.nombre,
  }))
})

const textoBotonGuardar = computed(() => {
  if (props.isSaving) return 'Guardando...'
  if (!esModoExistente.value) return 'Guardar Evento'
  if (tabModoEvento.value === 'cierre' && !esEventoFinalizado.value) return 'Guardar y Cerrar Evento'
  return 'Guardar Cambios'
})

const debeMostrarBotonGuardar = computed(() => {
  if (!esModoExistente.value) return true
  if (tabModoEvento.value === 'editar') return true
  return !esEventoFinalizado.value
})

watch(tipoEvento, (nuevo) => {
  if (!['rotacion', 'encargo_puesto', 'encargo_funciones', 'destaque'].includes(nuevo)) {
    nuevaAreaId.value = ''
    nuevoCargoId.value = ''
  }
})

watch(
  () => docInicio.value.tipoDocumento,
  (nuevo) => {
    if (nuevo && !requiereArea(nuevo)) {
      docInicio.value.areaId = ''
    }
  }
)

watch(
  () => docSalida.value.tipoDocumento,
  (nuevo) => {
    if (nuevo && !requiereArea(nuevo)) {
      docSalida.value.areaId = ''
    }
  }
)

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
  fechaDesde.value = ''
  fechaHasta.value = ''

  if (debounceEncargadoTimer) {
    clearTimeout(debounceEncargadoTimer)
    debounceEncargadoTimer = null
  }
  if (currentEncargadoAbortController) {
    currentEncargadoAbortController.abort()
    currentEncargadoAbortController = null
  }
  encargadoSearchQuery.value = ''
  encargadoSearchResults.value = []
  isSearchingEncargado.value = false
  encargadoSeleccionado.value = null
  encargadoError.value = ''

  if (props.evento) {
    tipoEvento.value = props.evento.tipo_evento || 'rotacion'
    nuevaAreaId.value = props.evento.nueva_area_id || ''
    nuevoCargoId.value = props.evento.nuevo_cargo_id || ''

    if (props.evento.fecha_desde) {
      fechaDesde.value = props.evento.fecha_desde.substring(0, 10)
    }
    if (props.evento.fecha_hasta) {
      fechaHasta.value = props.evento.fecha_hasta.substring(0, 10)
    }

    const estado = props.evento.estado?.toLowerCase()
    const estaFinalizado = estado === 'desactivado' || estado === 'finalizado' || Boolean(props.evento.fecha_salida)
    if (!estaFinalizado && estado === 'activo') {
      tabModoEvento.value = 'cierre'
    } else {
      tabModoEvento.value = 'editar'
    }

    docSalida.value = {
      tipoDocumento: '',
      areaId: '',
      numeroDocumento: '',
      añoDocumento: new Date().getFullYear(),
      fecha: getTodayDateString(),
      fechaValida: '',
      descripcion: `Cierre de ${getTipoEventoLabel(props.evento.tipo_evento)}`,
    }
  } else {
    tabModoEvento.value = 'cierre'
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
    } else {
      if (debounceEncargadoTimer) {
        clearTimeout(debounceEncargadoTimer)
        debounceEncargadoTimer = null
      }
      if (currentEncargadoAbortController) {
        currentEncargadoAbortController.abort()
        currentEncargadoAbortController = null
      }
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

  if (requiereVigencia.value) {
    if (!fechaDesde.value) {
      errors.fechaDesde = esVacaciones.value
        ? 'La fecha de inicio de vacaciones es obligatoria.'
        : 'La fecha de vigencia desde es obligatoria.'
    }
    if (esVacaciones.value && !fechaHasta.value) {
      errors.fechaHasta = 'La fecha de fin de vacaciones es obligatoria.'
    }
  }

  if (fechaDesde.value && fechaHasta.value) {
    if (fechaHasta.value < fechaDesde.value) {
      toast.error('La fecha de vigencia hasta no puede ser anterior a la fecha desde.')
      errors.fechaHasta = 'La fecha hasta no puede ser anterior a la fecha desde.'
    }
  }

  if (!esModoExistente.value) {
    if (!tipoEvento.value) {
      errors.tipoEvento = 'Seleccione el tipo de evento.'
    }
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
    if (tabModoEvento.value === 'cierre' && !esEventoFinalizado.value) {
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
    } else {
      if (!tipoEvento.value) {
        errors.tipoEvento = 'Seleccione el tipo de evento.'
      }
    }
  }

  formErrors.value = errors
  return Object.keys(errors).length === 0
}

const handleSubmit = () => {
  if (!props.vinculo) return
  if (!validarFormulario()) return

  if (esModoExistente.value && props.evento) {
    if (tabModoEvento.value === 'cierre' && !esEventoFinalizado.value) {
      const payload: EventoVinculoPayload = {
        id: props.evento.id,
        vinculo_id: props.vinculo.id,
        tipo_evento: props.evento.tipo_evento,
        nueva_area_id: props.evento.nueva_area_id || null,
        nuevo_cargo_id: props.evento.nuevo_cargo_id || null,
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
        estado: 'desactivado',
        fecha_desde: fechaDesde.value ? fechaDesde.value : null,
        fecha_hasta: fechaHasta.value ? fechaHasta.value : null,
      }
      emit('save', payload)
    } else {
      const payload: EventoVinculoPayload = {
        id: props.evento.id,
        vinculo_id: props.vinculo.id,
        tipo_evento: tipoEvento.value,
        nueva_area_id: nuevaAreaId.value ? Number(nuevaAreaId.value) : null,
        nuevo_cargo_id: nuevoCargoId.value ? Number(nuevoCargoId.value) : null,
        estado: props.evento.estado || 'activo',
        fecha_desde: fechaDesde.value ? fechaDesde.value : null,
        fecha_hasta: fechaHasta.value ? fechaHasta.value : null,
        solo_datos: true,
      }
      emit('save', payload)
    }
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
      estado: mismoDocumento.value ? 'desactivado' : 'activo',
      fecha_desde: fechaDesde.value ? fechaDesde.value : null,
      fecha_hasta: fechaHasta.value ? fechaHasta.value : null,
      encargado_dni: esVacaciones.value && encargadoSeleccionado.value ? encargadoSeleccionado.value.dni : null,
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
          class="relative w-full max-w-xl rounded-2xl bg-card border border-border shadow-2xl text-left flex flex-col max-h-[90vh] overflow-hidden"
          @click.stop>
          <div class="px-5 py-4 border-b border-border flex items-center justify-between shrink-0 bg-card">
            <div class="flex items-center gap-2.5 min-w-0">
              <div
                class="size-8 rounded-lg bg-purple-500/10 text-purple-600 dark:text-purple-400 flex items-center justify-center border border-purple-500/20 shrink-0">
                <IconFileCode class="size-4.5" />
              </div>
              <div class="min-w-0">
                <h3 class="font-bold text-base text-foreground tracking-tight truncate">
                  {{ esModoExistente ? (esEventoFinalizado ? 'Detalles del Evento Laboral' : 'Gestionar Evento Laboral')
                    : 'Registrar Nuevo Evento Laboral' }}
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

          <form id="evento-form" class="p-5 sm:p-6 overflow-y-auto min-h-0 flex-1 space-y-4 text-xs"
            @submit.prevent="handleSubmit">
            <div v-if="esModoExistente && evento" class="space-y-3">
              <div class="p-3.5 rounded-xl border border-border bg-muted/20 space-y-2">
                <div class="flex items-center justify-between gap-2">
                  <div class="flex items-center gap-2">
                    <span class="text-meta font-semibold uppercase tracking-wider text-muted-foreground">Evento
                      Actual</span>
                    <Badge size="xs" :variant="evento.estado?.toLowerCase() === 'activo' ? 'success' : 'secondary'">
                      {{ evento.estado?.toLowerCase() === 'activo' ? 'Activo' : 'Finalizado' }}
                    </Badge>
                  </div>
                  <span class="font-mono text-micro text-muted-foreground font-semibold">
                    #{{ evento.id }}
                  </span>
                </div>

                <div class="grid grid-cols-2 gap-2 text-xs pt-1">
                  <div>
                    <span class="text-muted-foreground block text-meta">Tipo de Evento:</span>
                    <span class="font-medium text-foreground capitalize block">
                      {{ getTipoEventoLabel(evento.tipo_evento) }}
                    </span>
                  </div>
                  <div>
                    <span class="text-muted-foreground block text-meta">Fecha de Inicio:</span>
                    <span class="font-mono text-foreground font-medium block">
                      {{ evento.fecha_inicio ? formatDate(evento.fecha_inicio) : '-' }}
                    </span>
                  </div>
                  <div class="col-span-2">
                    <span class="text-muted-foreground block text-meta">Documento de Sustento:</span>
                    <span class="font-medium text-foreground truncate block"
                      :title="[evento.tipo_doc_inicio, evento.numero_doc_inicio].filter(Boolean).join(' N° ') || '-'">
                      {{ [evento.tipo_doc_inicio, evento.numero_doc_inicio].filter(Boolean).join(' N° ') || '-' }}
                    </span>
                  </div>
                  <div v-if="evento.nueva_area" class="col-span-2">
                    <span class="text-muted-foreground block text-meta">Área Asignada:</span>
                    <span class="font-medium text-foreground block">{{ evento.nueva_area }}</span>
                  </div>
                  <div v-if="evento.nuevo_cargo" class="col-span-2">
                    <span class="text-muted-foreground block text-meta">Cargo Asignado:</span>
                    <span class="font-medium text-foreground block">{{ evento.nuevo_cargo }}</span>
                  </div>
                  <div v-if="evento.fecha_desde" class="col-span-2">
                    <span class="text-muted-foreground block text-meta">Vigencia de Encargo:</span>
                    <span class="font-mono text-foreground font-medium block">
                      Del {{ formatDate(evento.fecha_desde) }}{{ evento.fecha_hasta ? ` al ${formatDate(evento.fecha_hasta)}` : ' en adelante' }}
                    </span>
                  </div>
                </div>

                <div v-if="esEventoFinalizado" class="border-t border-border/60 pt-2 space-y-1">
                  <div class="flex items-center justify-between text-meta">
                    <span class="text-muted-foreground font-medium">Término de Evento:</span>
                    <span class="font-mono text-foreground font-medium">
                      {{ evento.fecha_salida ? formatDate(evento.fecha_salida) : '-' }}
                    </span>
                  </div>
                  <p class="font-medium text-foreground text-xs truncate">
                    {{ [evento.tipo_doc_salida, evento.numero_doc_salida].filter(Boolean).join(' N° ') ||
                      'Mismo documento de inicio' }}
                  </p>
                  <p v-if="evento.descrip_salida" class="text-muted-foreground text-meta leading-relaxed">
                    {{ evento.descrip_salida }}
                  </p>
                </div>
              </div>

              <div class="flex rounded-xl p-1 bg-muted/40 border border-border">
                <button type="button"
                  class="flex-1 py-1.5 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer flex items-center justify-center gap-1.5"
                  :class="tabModoEvento === 'cierre' ? 'bg-card text-foreground shadow-xs border border-border' : 'text-muted-foreground hover:text-foreground'"
                  @click="tabModoEvento = 'cierre'">
                  <IconFileCheck class="size-3.5" />
                  <span>{{ esEventoFinalizado ? 'Detalle de Cierre' : 'Finalizar / Cerrar' }}</span>
                </button>
                <button type="button"
                  class="flex-1 py-1.5 px-3 rounded-lg text-xs font-semibold transition-all cursor-pointer flex items-center justify-center gap-1.5"
                  :class="tabModoEvento === 'editar' ? 'bg-card text-foreground shadow-xs border border-border' : 'text-muted-foreground hover:text-foreground'"
                  @click="tabModoEvento = 'editar'">
                  <IconEdit class="size-3.5" />
                  <span>Modificar Datos</span>
                </button>
              </div>

              <div v-if="tabModoEvento === 'cierre' && !esEventoFinalizado" class="space-y-3 pt-1">
                <div class="space-y-2">
                  <span class="block font-medium text-foreground text-xs">Modalidad de Cierre de Evento</span>
                  <div class="grid grid-cols-2 gap-2">
                    <label class="p-2.5 rounded-xl border flex items-center gap-2 cursor-pointer transition-colors"
                      :class="accionCierre === 'salida' ? 'border-primary bg-primary/5 text-primary' : 'border-border bg-card text-foreground'">
                      <input v-model="accionCierre" type="radio" value="salida" class="sr-only" />
                      <IconFileCheck class="size-4 shrink-0" />
                      <span class="text-xs font-medium">Registrar Doc. Término</span>
                    </label>

                    <label class="p-2.5 rounded-xl border flex items-center gap-2 cursor-pointer transition-colors"
                      :class="accionCierre === 'mismo' ? 'border-primary bg-primary/5 text-primary' : 'border-border bg-card text-foreground'">
                      <input v-model="accionCierre" type="radio" value="mismo" class="sr-only" />
                      <IconCheck class="size-4 shrink-0" />
                      <span class="text-xs font-medium">Mismo Doc. Inicio</span>
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
                      <p v-if="formErrors.tipoDocumentoSalida" class="text-rose-500 text-meta mt-1">
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
                      <p v-if="formErrors.numeroDocumentoSalida" class="text-rose-500 text-meta mt-1">
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
                      <p v-if="formErrors.fechaSalida" class="text-rose-500 text-meta mt-1">
                        {{ formErrors.fechaSalida }}
                      </p>
                    </div>
                  </div>

                  <div v-if="necesitaAreaSalida">
                    <SearchSelect v-model="docSalida.areaId" label="Área Emisora"
                      placeholder="Seleccione área emisora..."
                      search-placeholder="Buscar área por nombre o sigla..." :options="areaOptions" :required="true"
                      :disabled="isLoadingCatalogos || isSaving" :error-message="formErrors.areaSalida" />
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      Descripción de Cierre <span class="text-rose-500">*</span>
                    </label>
                    <textarea v-model="docSalida.descripcion" rows="2"
                      placeholder="Detalles de la conclusión del evento..."
                      class="w-full rounded-lg border bg-background p-2.5 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden leading-relaxed"
                      :class="formErrors.descripcionSalida ? 'border-rose-500 ring-rose-500' : 'border-border'"></textarea>
                    <p v-if="formErrors.descripcionSalida" class="text-rose-500 text-meta mt-1">
                      {{ formErrors.descripcionSalida }}
                    </p>
                  </div>
                </div>

                <div v-else class="p-3.5 rounded-xl border border-border/80 bg-muted/20 flex items-start gap-2.5">
                  <IconCheck class="size-4.5 text-emerald-600 dark:text-emerald-400 shrink-0 mt-0.5" />
                  <div class="space-y-0.5">
                    <p class="font-medium text-foreground text-xs">Cierre con Documento de Inicio</p>
                    <p class="text-muted-foreground text-meta leading-relaxed">
                      Se utilizará el mismo documento de inicio registrado anteriormente para marcar la finalización de
                      este evento y actualizar el estado laboral del trabajador.
                    </p>
                  </div>
                </div>
              </div>

              <div v-if="tabModoEvento === 'cierre' && esEventoFinalizado"
                class="p-3.5 rounded-xl border border-border/80 bg-muted/10 space-y-2">
                <div class="flex items-center gap-2 text-emerald-600 dark:text-emerald-400 font-semibold text-xs">
                  <IconCheck class="size-4" />
                  <span>Este evento ya se encuentra finalizado</span>
                </div>
                <p class="text-muted-foreground text-meta leading-relaxed">
                  Si deseas rectificar el tipo de evento, el área o el cargo asignados, selecciona la pestaña <strong
                    class="text-foreground">Modificar Datos</strong> en la parte superior.
                </p>
              </div>

              <div v-if="tabModoEvento === 'editar'" class="space-y-3 pt-1">
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
                  <p v-if="formErrors.tipoEvento" class="text-rose-500 text-meta mt-1">
                    {{ formErrors.tipoEvento }}
                  </p>
                </div>

                <div v-if="requiereMovimiento"
                  class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/80 bg-muted/10">
                  <div>
                    <SearchSelect v-model="nuevaAreaId" label="Nueva Área Asignada"
                      placeholder="Mantener área actual"
                      search-placeholder="Buscar por nombre o sigla (ej. Gerencia, OGA, etc.)..." :options="areaOptions"
                      :disabled="isLoadingCatalogos || isSaving" />
                  </div>

                  <div>
                    <SearchSelect v-model="nuevoCargoId" label="Nuevo Cargo" placeholder="Mantener cargo actual"
                      search-placeholder="Buscar cargo por nombre..." :options="cargoOptions"
                      :disabled="isLoadingCatalogos || isSaving" />
                  </div>
                </div>
              </div>
            </div>

            <div v-if="!esModoExistente" class="space-y-3">
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
                <p v-if="formErrors.tipoEvento" class="text-rose-500 text-meta mt-1">
                  {{ formErrors.tipoEvento }}
                </p>
              </div>

              <div v-if="requiereMovimiento"
                class="grid grid-cols-1 sm:grid-cols-2 gap-3 p-3 rounded-xl border border-border/80 bg-muted/10">
                <div>
                  <SearchSelect v-model="nuevaAreaId" label="Nueva Área Asignada"
                    placeholder="Mantener área actual"
                    search-placeholder="Buscar por nombre o sigla (ej. Gerencia, OGA, etc.)..." :options="areaOptions"
                    :disabled="isLoadingCatalogos || isSaving" />
                </div>

                <div>
                  <SearchSelect v-model="nuevoCargoId" label="Nuevo Cargo" placeholder="Mantener cargo actual"
                    search-placeholder="Buscar cargo por nombre..." :options="cargoOptions"
                    :disabled="isLoadingCatalogos || isSaving" />
                </div>
              </div>

              <!-- Campos específicos para vigencia (encargaturas y vacaciones) -->
              <div class="p-3.5 rounded-xl border border-amber-500/30 bg-amber-500/5 space-y-3">
                <div class="flex items-center gap-2 text-amber-600 dark:text-amber-400 font-semibold text-xs">
                  <IconUser class="size-4" />
                  <span>{{ esVacaciones ? 'Periodo de Vacaciones' : esEncargo ? 'Datos de la Encargatura' : 'Vigencia del Evento' }}</span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      {{ esVacaciones ? 'Fecha de inicio' : 'Vigencia desde' }} <span v-if="requiereVigencia" class="text-rose-500">*</span>
                    </label>
                    <input v-model="fechaDesde" type="date"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.fechaDesde ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.fechaDesde" class="text-rose-500 text-meta mt-1">
                      {{ formErrors.fechaDesde }}
                    </p>
                  </div>

                  <div>
                    <label class="block font-medium text-foreground mb-1 text-xs">
                      {{ esVacaciones ? 'Fecha de fin' : 'Vigencia hasta' }} <span v-if="esVacaciones" class="text-rose-500">*</span>
                    </label>
                    <input v-model="fechaHasta" type="date"
                      class="w-full rounded-lg border bg-background px-3 py-2 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden font-mono"
                      :class="formErrors.fechaHasta ? 'border-rose-500 ring-rose-500' : 'border-border'" />
                    <p v-if="formErrors.fechaHasta" class="text-rose-500 text-meta mt-1">
                      {{ formErrors.fechaHasta }}
                    </p>
                  </div>
                </div>

                <p v-if="!requiereVigencia" class="text-meta text-muted-foreground">
                  Opcional. Si la resolución fija el inicio y el término del evento, completa ambas fechas y marca la casilla de documento único más abajo.
                </p>
              </div>

              <!-- Encargo temporal del puesto (opcional) -->
              <div v-if="esVacaciones && !esModoExistente"
                class="p-3.5 rounded-xl border border-sky-500/30 bg-sky-500/5 space-y-3">
                <div class="flex items-center gap-2 text-sky-600 dark:text-sky-400 font-semibold text-xs">
                  <IconUser class="size-4" />
                  <span>Encargo temporal del puesto (opcional)</span>
                </div>

                <!-- Si ya hay encargado seleccionado: mostrar como chip -->
                <div v-if="encargadoSeleccionado"
                  class="flex items-center justify-between p-2.5 rounded-lg bg-card border border-border">
                  <div class="flex items-center gap-2">
                    <div class="size-7 rounded-md bg-primary/10 text-primary flex items-center justify-center font-bold text-xs">
                      <IconUser class="size-4" />
                    </div>
                    <div>
                      <p class="font-semibold text-foreground text-xs">{{ encargadoSeleccionado.nombre }}</p>
                      <p class="text-meta font-mono text-muted-foreground">DNI: {{ encargadoSeleccionado.dni }}</p>
                    </div>
                  </div>
                  <button type="button"
                    class="p-1 text-muted-foreground hover:text-rose-500 rounded-md transition-colors cursor-pointer"
                    title="Quitar encargado" @click="limpiarEncargado">
                    <IconX class="size-4" />
                  </button>
                </div>

                <!-- Si no hay encargado seleccionado: buscador -->
                <div v-else class="space-y-2">
                  <div class="relative">
                    <input v-model="encargadoSearchQuery" type="text"
                      placeholder="Buscar trabajador por nombres o apellidos..."
                      class="w-full rounded-lg border border-border bg-background px-3 py-2 pl-8 text-xs focus:ring-2 focus:ring-primary focus:outline-hidden"
                      @input="onSearchEncargadoInput" />
                    <IconSearch class="size-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground" />
                    <span v-if="isSearchingEncargado" class="absolute right-2.5 top-1/2 -translate-y-1/2 text-micro text-muted-foreground">
                      Buscando...
                    </span>
                  </div>

                  <!-- Error inline si intenta elegir al propio titular u otro error -->
                  <p v-if="encargadoError" class="text-rose-500 text-meta">
                    {{ encargadoError }}
                  </p>

                  <!-- Lista de resultados -->
                  <div v-if="encargadoSearchResults.length > 0"
                    class="max-h-40 overflow-y-auto divide-y divide-border border border-border rounded-lg bg-card shadow-xs">
                    <button v-for="res in encargadoSearchResults" :key="res.dni" type="button"
                      class="w-full p-2 text-left hover:bg-muted/50 flex items-center justify-between cursor-pointer transition text-xs"
                      @click="seleccionarEncargado(res)">
                      <div>
                        <p class="font-medium text-foreground">{{ res.nombre }}</p>
                        <p class="text-meta font-mono text-muted-foreground">DNI: {{ res.dni }}</p>
                      </div>
                      <Badge :variant="res.estado === 'activo' ? 'success' : 'secondary'" size="xs">
                        {{ res.estado }}
                      </Badge>
                    </button>
                  </div>
                  <div v-else-if="encargadoSearchQuery.trim().length >= 3 && !isSearchingEncargado"
                    class="py-2 text-center text-meta text-muted-foreground">
                    No se encontraron trabajadores.
                  </div>
                </div>

                <p class="text-meta text-muted-foreground">
                  Se registrará un encargo de puesto con el mismo documento y el mismo periodo, heredando el área y el cargo del titular.
                </p>
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
                    <p v-if="formErrors.tipoDocumentoInicio" class="text-rose-500 text-meta mt-1">
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
                    <p v-if="formErrors.numeroDocumentoInicio" class="text-rose-500 text-meta mt-1">
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
                    <p v-if="formErrors.fechaInicio" class="text-rose-500 text-meta mt-1">
                      {{ formErrors.fechaInicio }}
                    </p>
                  </div>
                </div>

                <div v-if="necesitaAreaInicio">
                  <SearchSelect v-model="docInicio.areaId" label="Área Emisora del Documento"
                    placeholder="Seleccione área emisora..."
                    search-placeholder="Buscar área por nombre o sigla..." :options="areaOptions" :required="true"
                    :disabled="isLoadingCatalogos || isSaving" :error-message="formErrors.areaInicio" />
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
                  <p v-if="formErrors.descripcionInicio" class="text-rose-500 text-meta mt-1">
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
          </form>

          <div
            class="px-5 py-3.5 sm:px-6 border-t border-border flex items-center justify-end gap-2.5 shrink-0 bg-muted/20">
            <Button size="sm" variant="outline" class="cursor-pointer" :disabled="isSaving" @click="emit('close')">
              {{ !debeMostrarBotonGuardar ? 'Cerrar' : 'Cancelar' }}
            </Button>
            <Button v-if="debeMostrarBotonGuardar" size="sm" type="submit" form="evento-form"
              class="bg-primary hover:bg-primary/90 text-primary-foreground gap-1.5 cursor-pointer font-semibold shadow-xs"
              :disabled="isSaving">
              <IconCheck class="size-4" />
              <span>{{ textoBotonGuardar }}</span>
            </Button>
          </div>
        </div>
      </transition>
    </div>
  </div>
</template>
