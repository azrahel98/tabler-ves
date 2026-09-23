<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Button from '@/components/ui/button/Button.vue'
import NuevoTrabajadorStepper from '@/components/nuevo-trabajador/NuevoTrabajadorStepper.vue'
import StepIdentidad from '@/components/nuevo-trabajador/StepIdentidad.vue'
import StepVinculoLaboral from '@/components/nuevo-trabajador/StepVinculoLaboral.vue'
import StepDocumentoSustento from '@/components/nuevo-trabajador/StepDocumentoSustento.vue'
import FichaResumen from '@/components/nuevo-trabajador/FichaResumen.vue'
import ModalVacantes from '@/components/nuevo-trabajador/ModalVacantes.vue'
import type {
  PersonalFormState,
  PlazaFormState,
  DocumentoFormState,
  RegimenOption,
} from '@/components/nuevo-trabajador/types'
import {
  fetchAreas,
  fetchCargos,
  fetchTiposDocumentos,
  fetchPlazasVacantes,
  fetchPlazaDetalle,
  consultarDniReniec,
  registrarNuevoTrabajador,
  requiereArea,
  type AreaOption,
  type CargoOption,
  type TipoDocumentoOption,
  type VacanteOption,
  type NuevoTrabajadorPayload,
} from '@/services/personal'
import {
  IconUserPlus,
  IconArrowLeft,
  IconLoader2,
  IconCheck,
} from '@tabler/icons-vue'

const router = useRouter()

const currentStep = ref<1 | 2 | 3>(1)
const isConsultandoReniec = ref<boolean>(false)
const reniecSuccess = ref<boolean>(false)
const reniecError = ref<string | null>(null)
const isSubmitting = ref<boolean>(false)
const submitError = ref<string | null>(null)
const toastMessage = ref<{ type: 'success' | 'error'; text: string } | null>(null)

const areas = ref<AreaOption[]>([])
const cargos = ref<CargoOption[]>([])
const tiposDocumento = ref<TipoDocumentoOption[]>([])
const plazasVacantes = ref<VacanteOption[]>([])
const isLoadingCatalogos = ref<boolean>(true)

const isVacantesModalOpen = ref<boolean>(false)

const formData = reactive<{
  personal: PersonalFormState
  plaza: PlazaFormState
  documento: DocumentoFormState
}>({
  personal: {
    dni: '',
    nombre: '',
    apaterno: '',
    amaterno: '',
    nacimiento: '',
    sexo: 'M',
    telf: '',
    direccion: '',
    email: '',
    ruc: '',
    region: 'LIMA',
    distrito: 'VILLA EL SALVADOR',
  },
  plaza: {
    codigo: '',
    areaId: null,
    cargoId: null,
    regimen: 3,
    sueldo: 0,
    regimenNombre: 'D.L. 1057 (CAS)',
    cargoEstructural: '',
    grupoOcupacional: '',
  },
  documento: {
    tipoDocumento: '1',
    areaId: null,
    numeroDocumento: new Date().getDate() * 10 + 1,
    añoDocumento: new Date().getFullYear(),
    fecha: new Date().toISOString().slice(0, 10),
    fechaValida: new Date().toISOString().slice(0, 10),
    descripcion: '',
  },
})

const regímenesDisponibles: RegimenOption[] = [
  { id: 1, nombre: 'D.L. 276 - Carrera Administrativa' },
  { id: 2, nombre: 'D.L. 728 - Régimen Privado' },
  { id: 3, nombre: 'D.L. 1057 - Contrato Administrativo de Servicios (CAS)' },
  { id: 4, nombre: 'Ley 30057 - Servicio Civil' },
  { id: 5, nombre: 'Designación de Confianza / FAG' },
]

const showToast = (type: 'success' | 'error', text: string) => {
  toastMessage.value = { type, text }
  setTimeout(() => {
    toastMessage.value = null
  }, 4000)
}

onMounted(async () => {
  isLoadingCatalogos.value = true
  try {
    const [ar, cr, docs, vac] = await Promise.all([
      fetchAreas(),
      fetchCargos(),
      fetchTiposDocumentos(),
      fetchPlazasVacantes(),
    ])
    areas.value = ar
    cargos.value = cr
    tiposDocumento.value = docs
    plazasVacantes.value = vac

    if (ar.length > 0 && !formData.plaza.areaId) {
      formData.plaza.areaId = ar[0].id
    }
    if (cr.length > 0 && !formData.plaza.cargoId) {
      formData.plaza.cargoId = cr[0].id
    }
    if (docs.length > 0) {
      formData.documento.tipoDocumento = String(docs[0].id)
    }
  } finally {
    isLoadingCatalogos.value = false
  }
})

const areaSeleccionadaNombre = computed(() => {
  const a = areas.value.find((item) => item.id === formData.plaza.areaId)
  return a ? a.nombre : '-'
})

const cargoSeleccionadoNombre = computed(() => {
  const c = cargos.value.find((item) => item.id === formData.plaza.cargoId)
  return c ? c.nombre : '-'
})

const tipoDocSeleccionadoNombre = computed(() => {
  const t = tiposDocumento.value.find(
    (item) => String(item.id) === String(formData.documento.tipoDocumento)
  )
  return t ? t.nombre : 'Documento'
})

const nombreCompleto = computed(() => {
  const parts = [
    formData.personal.apaterno,
    formData.personal.amaterno,
    formData.personal.nombre,
  ].filter(Boolean)
  return parts.length > 0 ? parts.join(' ') : 'Nombre y Apellidos del Servidor'
})

const onConsultarReniec = async () => {
  const dni = formData.personal.dni.trim()
  if (!/^\d{8}$/.test(dni)) {
    reniecError.value = 'El DNI debe contener exactamente 8 dígitos numéricos.'
    return
  }
  isConsultandoReniec.value = true
  reniecError.value = null
  reniecSuccess.value = false
  try {
    const res = await consultarDniReniec(dni)
    if (res) {
      formData.personal.nombre = res.nombre || formData.personal.nombre
      formData.personal.apaterno = res.apaterno || formData.personal.apaterno
      formData.personal.amaterno = res.amaterno || formData.personal.amaterno
      if (res.nacimiento) {
        formData.personal.nacimiento = res.nacimiento.slice(0, 10)
      }
      if (res.sexo === 'M' || res.sexo === 'F') {
        formData.personal.sexo = res.sexo
      }
      if (res.telf) formData.personal.telf = res.telf
      if (res.direccion) formData.personal.direccion = res.direccion
      if (res.email) formData.personal.email = res.email
      if (res.ruc) formData.personal.ruc = res.ruc
      reniecSuccess.value = true
      showToast('success', 'Datos obtenidos exitosamente de RENIEC / Base local.')
    } else {
      reniecError.value = 'No se encontraron datos automáticos. Puedes completar los campos manualmente.'
    }
  } catch {
    reniecError.value = 'No se pudo conectar con el servicio de consulta. Completa los datos manualmente.'
  } finally {
    isConsultandoReniec.value = false
  }
}

const seleccionarPlazaVacante = async (vacante: VacanteOption) => {
  formData.plaza.codigo = vacante.codigo
  if (vacante.area_id) formData.plaza.areaId = vacante.area_id
  if (vacante.cargo_id) formData.plaza.cargoId = vacante.cargo_id
  if (vacante.sueldo !== null && vacante.sueldo !== undefined) {
    formData.plaza.sueldo = Number(vacante.sueldo)
  }

  try {
    const detalle = await fetchPlazaDetalle(vacante.codigo)
    if (detalle) {
      formData.plaza.regimen = detalle.regimen_id || formData.plaza.regimen
      formData.plaza.regimenNombre = detalle.regimen || formData.plaza.regimenNombre
      formData.plaza.cargoEstructural = detalle.cargo_descripcion || ''
      formData.plaza.grupoOcupacional = detalle.grupo_descripcion || ''
    }
  } catch { }

  isVacantesModalOpen.value = false
  showToast('success', `Plaza ${vacante.codigo} seleccionada correctamente.`)
}

const onCambioCodigoPlazaManual = async () => {
  const cod = formData.plaza.codigo.trim()
  if (!cod) return
  try {
    const detalle = await fetchPlazaDetalle(cod)
    if (detalle) {
      formData.plaza.regimen = detalle.regimen_id || formData.plaza.regimen
      formData.plaza.regimenNombre = detalle.regimen || formData.plaza.regimenNombre
      formData.plaza.cargoEstructural = detalle.cargo_descripcion || ''
      formData.plaza.grupoOcupacional = detalle.grupo_descripcion || ''
    }
  } catch { }
}

const onRegimenChange = () => {
  const r = regímenesDisponibles.find((item) => item.id === Number(formData.plaza.regimen))
  if (r) {
    formData.plaza.regimenNombre = r.nombre
  }
}

const isStep1Valid = computed(() => {
  return (
    /^\d{8}$/.test(formData.personal.dni.trim()) &&
    formData.personal.nombre.trim().length > 0 &&
    formData.personal.apaterno.trim().length > 0 &&
    formData.personal.amaterno.trim().length > 0 &&
    Boolean(formData.personal.nacimiento)
  )
})

const isStep2Valid = computed(() => {
  return (
    formData.plaza.areaId !== null &&
    formData.plaza.cargoId !== null &&
    formData.plaza.regimen > 0 &&
    formData.plaza.sueldo >= 0
  )
})

const isStep3Valid = computed(() => {
  return (
    Boolean(formData.documento.tipoDocumento) &&
    formData.documento.numeroDocumento > 0 &&
    formData.documento.añoDocumento >= 1990 &&
    Boolean(formData.documento.fecha) &&
    formData.documento.descripcion.trim().length > 0
  )
})

const isFormValid = computed(() => {
  return isStep1Valid.value && isStep2Valid.value && isStep3Valid.value
})

const avanzarPaso = () => {
  if (currentStep.value === 1 && isStep1Valid.value) {
    currentStep.value = 2
  } else if (currentStep.value === 2 && isStep2Valid.value) {
    currentStep.value = 3
  }
}

const retrocederPaso = () => {
  if (currentStep.value > 1) {
    currentStep.value = (currentStep.value - 1) as 1 | 2 | 3
  }
}

const onSubmitRegistro = async () => {
  if (!isStep1Valid.value) {
    currentStep.value = 1
    showToast('error', 'Complete los datos obligatorios del servidor (DNI, nombres, apellidos y nacimiento).')
    return
  }
  if (!isStep2Valid.value) {
    currentStep.value = 2
    showToast('error', 'Complete los datos de la plaza y vínculo laboral.')
    return
  }
  if (!isStep3Valid.value) {
    currentStep.value = 3
    showToast('error', 'Complete los datos del documento de sustento.')
    return
  }

  isSubmitting.value = true
  submitError.value = null

  const payload: NuevoTrabajadorPayload = {
    personal: {
      dni: formData.personal.dni.trim(),
      nombre: formData.personal.nombre.trim(),
      apaterno: formData.personal.apaterno.trim(),
      amaterno: formData.personal.amaterno.trim(),
      nacimiento: formData.personal.nacimiento,
      sexo: formData.personal.sexo || null,
      telf: formData.personal.telf.trim() || null,
      direccion: formData.personal.direccion.trim() || null,
      email: formData.personal.email.trim() || null,
      ruc: formData.personal.ruc.trim() || null,
      region: formData.personal.region?.trim() || 'LIMA',
      distrito: formData.personal.distrito?.trim() || 'VILLA EL SALVADOR',
    },
    documento: {
      tipoDocumento: formData.documento.tipoDocumento,
      areaId: requiereArea(formData.documento.tipoDocumento)
        ? formData.documento.areaId || formData.plaza.areaId
        : null,
      numeroDocumento: Number(formData.documento.numeroDocumento),
      añoDocumento: Number(formData.documento.añoDocumento),
      fecha: formData.documento.fecha,
      fechaValida: formData.documento.fechaValida || formData.documento.fecha,
      descripcion: formData.documento.descripcion.trim(),
    },
    regimen: Number(formData.plaza.regimen),
    cargo: Number(formData.plaza.cargoId),
    area: Number(formData.plaza.areaId),
    sueldo: Number(formData.plaza.sueldo),
  }

  try {
    await registrarNuevoTrabajador(payload)
    showToast('success', 'Servidor público registrado exitosamente.')
    setTimeout(() => {
      router.push({ name: 'perfil', params: { dni: payload.personal.dni } })
    }, 1200)
  } catch (err: any) {
    submitError.value = err?.message || 'Error al procesar el registro del trabajador.'
    showToast('error', submitError.value || 'Error al guardar.')
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <div
      v-if="toastMessage"
      class="fixed bottom-5 right-5 z-50 transition-all duration-300 transform translate-y-0"
    >
      <div
        class="flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg border text-xs font-medium backdrop-blur-md"
        :class="toastMessage.type === 'success' ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-rose-500/10 border-rose-500/20 text-rose-600 dark:text-rose-400'"
      >
        <span
          class="size-2 rounded-full shrink-0"
          :class="toastMessage.type === 'success' ? 'bg-emerald-500' : 'bg-rose-500'"
        ></span>
        <span>{{ toastMessage.text }}</span>
      </div>
    </div>

    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 border-b border-border pb-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="p-1.5 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
            title="Volver"
            @click="router.back()"
          >
            <IconArrowLeft class="size-4" />
          </button>
          <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
            <IconUserPlus class="size-5 text-primary shrink-0" />
            Alta de Nuevo Servidor Público
          </h1>
        </div>
        <p class="text-xs text-muted-foreground pl-8">
          Registro integral de identidad, vínculo laboral y documento de ingreso.
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" class="gap-1.5 text-xs cursor-pointer" @click="router.push('/panel')">
          Cancelar
        </Button>
        <Button
          variant="primary"
          size="sm"
          class="gap-1.5 text-xs cursor-pointer"
          :disabled="isSubmitting || !isFormValid"
          @click="onSubmitRegistro"
        >
          <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" />
          <IconCheck v-else class="size-3.5" />
          <span>{{ isSubmitting ? 'Guardando...' : 'Finalizar Registro' }}</span>
        </Button>
      </div>
    </div>

    <NuevoTrabajadorStepper
      :current-step="currentStep"
      :is-step1-valid="isStep1Valid"
      :is-step2-valid="isStep2Valid"
      :is-step3-valid="isStep3Valid"
      @select-step="(s) => (currentStep = s)"
    />

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-6 items-start">
      <div class="xl:col-span-8 space-y-6">
        <StepIdentidad
          v-show="currentStep === 1"
          :personal="formData.personal"
          :is-consultando-reniec="isConsultandoReniec"
          :reniec-success="reniecSuccess"
          :reniec-error="reniecError"
          :is-valid="isStep1Valid"
          @consultar-reniec="onConsultarReniec"
          @siguiente="avanzarPaso"
        />

        <StepVinculoLaboral
          v-show="currentStep === 2"
          :plaza="formData.plaza"
          :areas="areas"
          :cargos="cargos"
          :regimenes="regímenesDisponibles"
          :total-vacantes="plazasVacantes.length"
          :is-valid="isStep2Valid"
          @abrir-vacantes="isVacantesModalOpen = true"
          @cambio-codigo-plaza="onCambioCodigoPlazaManual"
          @regimen-change="onRegimenChange"
          @anterior="retrocederPaso"
          @siguiente="avanzarPaso"
        />

        <StepDocumentoSustento
          v-show="currentStep === 3"
          :documento="formData.documento"
          :tipos-documento="tiposDocumento"
          :is-submitting="isSubmitting"
          :can-submit="isFormValid"
          @anterior="retrocederPaso"
          @submit="onSubmitRegistro"
        />
      </div>

      <div class="xl:col-span-4 space-y-4 sticky top-6">
        <FichaResumen
          :personal="formData.personal"
          :plaza="formData.plaza"
          :documento="formData.documento"
          :cargo-nombre="cargoSeleccionadoNombre"
          :area-nombre="areaSeleccionadaNombre"
          :tipo-doc-nombre="tipoDocSeleccionadoNombre"
          :nombre-completo="nombreCompleto"
          :is-form-valid="isFormValid"
        />
      </div>
    </div>

    <ModalVacantes
      :is-open="isVacantesModalOpen"
      :vacantes="plazasVacantes"
      @update:is-open="(val) => (isVacantesModalOpen = val)"
      @seleccionar="seleccionarPlazaVacante"
    />
  </div>
</template>
