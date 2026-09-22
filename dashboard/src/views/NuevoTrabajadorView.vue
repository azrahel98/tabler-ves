<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  fetchAreas,
  fetchCargos,
  fetchTiposDocumentos,
  fetchPlazasVacantes,
  fetchPlazaDetalle,
  consultarDniReniec,
  registrarNuevoTrabajador,
  formatMoneda,
  requiereArea,
  type AreaOption,
  type CargoOption,
  type TipoDocumentoOption,
  type VacanteOption,
  type NuevoTrabajadorPayload,
} from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconUserPlus,
  IconArrowLeft,
  IconId,
  IconSearch,
  IconLoader2,
  IconCheck,
  IconBriefcase,
  IconFileText,
  IconBuildingSkyscraper,
  IconCalendar,
  IconPhone,
  IconMail,
  IconMapPin,
  IconCoin,
  IconFileDescription,
  IconSparkles,
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
const filtroVacante = ref<string>('')

const formData = reactive({
  personal: {
    dni: '',
    nombre: '',
    apaterno: '',
    amaterno: '',
    nacimiento: '',
    sexo: 'M' as 'M' | 'F',
    telf: '',
    direccion: '',
    email: '',
    ruc: '',
    region: 'LIMA',
    distrito: 'VILLA EL SALVADOR',
  },
  plaza: {
    codigo: '',
    areaId: null as number | null,
    cargoId: null as number | null,
    regimen: 3,
    sueldo: 0,
    regimenNombre: 'D.L. 1057 (CAS)',
    cargoEstructural: '',
    grupoOcupacional: '',
  },
  documento: {
    tipoDocumento: '1',
    areaId: null as number | null,
    numeroDocumento: new Date().getDate() * 10 + 1,
    añoDocumento: new Date().getFullYear(),
    fecha: new Date().toISOString().slice(0, 10),
    fechaValida: new Date().toISOString().slice(0, 10),
    descripcion: '',
  },
})

const regímenesDisponibles = [
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

const plazasVacantesFiltradas = computed(() => {
  if (!filtroVacante.value.trim()) return plazasVacantes.value
  const query = filtroVacante.value.toLowerCase().trim()
  return plazasVacantes.value.filter(
    (v) =>
      v.codigo.toLowerCase().includes(query) ||
      (v.area && v.area.toLowerCase().includes(query)) ||
      (v.cargo && v.cargo.toLowerCase().includes(query))
  )
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
    <div v-if="toastMessage" class="fixed bottom-5 right-5 z-50 transition-all duration-300 transform translate-y-0">
      <div class="flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg border text-xs font-medium backdrop-blur-md"
        :class="toastMessage.type === 'success' ? 'bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400' : 'bg-rose-500/10 border-rose-500/20 text-rose-600 dark:text-rose-400'">
        <span class="size-2 rounded-full shrink-0"
          :class="toastMessage.type === 'success' ? 'bg-emerald-500' : 'bg-rose-500'"></span>
        <span>{{ toastMessage.text }}</span>
      </div>
    </div>

    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 border-b border-border pb-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2">
          <button type="button"
            class="p-1.5 rounded-lg border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted transition-colors cursor-pointer"
            title="Volver" @click="router.back()">
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
        <Button variant="primary" size="sm" class="gap-1.5 text-xs cursor-pointer"
          :disabled="isSubmitting || !isStep1Valid || !isStep2Valid || !isStep3Valid" @click="onSubmitRegistro">
          <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" />
          <IconCheck v-else class="size-3.5" />
          <span>{{ isSubmitting ? 'Guardando...' : 'Finalizar Registro' }}</span>
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-3 gap-2 sm:gap-4 select-none">
      <button type="button"
        class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
        :class="currentStep === 1 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep1Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
        @click="currentStep = 1">
        <div class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
          :class="currentStep === 1 ? 'bg-primary text-primary-foreground' : isStep1Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'">
          <IconCheck v-if="isStep1Valid && currentStep !== 1" class="size-4" />
          <span v-else>1</span>
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold truncate">1. Identidad y Contacto</p>
          <p class="text-[10px] text-muted-foreground truncate hidden sm:block">DNI, nombres y datos personales</p>
        </div>
      </button>

      <button type="button"
        class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
        :class="currentStep === 2 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep2Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
        @click="currentStep = 2">
        <div class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
          :class="currentStep === 2 ? 'bg-primary text-primary-foreground' : isStep2Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'">
          <IconCheck v-if="isStep2Valid && currentStep !== 2" class="size-4" />
          <span v-else>2</span>
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold truncate">2. Vínculo Laboral</p>
          <p class="text-[10px] text-muted-foreground truncate hidden sm:block">Régimen, cargo, área y remuneración</p>
        </div>
      </button>

      <button type="button"
        class="flex items-center gap-2 sm:gap-3 p-2.5 sm:p-3 rounded-xl border text-left transition-all cursor-pointer"
        :class="currentStep === 3 ? 'border-primary bg-primary/5 text-primary shadow-xs' : isStep3Valid ? 'border-border bg-card text-foreground' : 'border-border/60 bg-muted/20 text-muted-foreground'"
        @click="currentStep = 3">
        <div class="size-7 rounded-lg flex items-center justify-center font-bold text-xs shrink-0"
          :class="currentStep === 3 ? 'bg-primary text-primary-foreground' : isStep3Valid ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground'">
          <IconCheck v-if="isStep3Valid && currentStep !== 3" class="size-4" />
          <span v-else>3</span>
        </div>
        <div class="min-w-0 flex-1">
          <p class="text-xs font-semibold truncate">3. Documento de Sustento</p>
          <p class="text-[10px] text-muted-foreground truncate hidden sm:block">Resolución o contrato de inicio</p>
        </div>
      </button>
    </div>

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-6 items-start">
      <div class="xl:col-span-8 space-y-6">
        <Card v-show="currentStep === 1" class="space-y-4">
          <div class="flex items-center justify-between border-b border-border pb-3">
            <span class="text-sm font-bold text-foreground flex items-center gap-2">
              <IconId class="size-4 text-primary shrink-0" />
              Datos de Identidad y Contacto
            </span>
            <span class="text-[11px] text-muted-foreground">Campos con (*) son obligatorios</span>
          </div>

          <div class="p-3.5 rounded-xl bg-primary/5 border border-primary/20 space-y-3">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
              <div class="space-y-0.5">
                <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                  <IconSparkles class="size-3.5 text-primary" />
                  Búsqueda asistida en RENIEC / Base Local
                </span>
                <p class="text-[11px] text-muted-foreground">
                  Ingresa el DNI y consulta para prellenar nombres, apellidos y fecha de nacimiento.
                </p>
              </div>

              <div class="flex items-center gap-2">
                <div class="relative w-36">
                  <input v-model="formData.personal.dni" type="text" maxlength="8" placeholder="DNI (8 dígitos)"
                    class="w-full px-3 py-1.5 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
                    @keyup.enter="onConsultarReniec" />
                </div>
                <Button size="xs" variant="primary" class="gap-1.5 text-xs cursor-pointer shrink-0"
                  :disabled="isConsultandoReniec || formData.personal.dni.length !== 8" @click="onConsultarReniec">
                  <IconLoader2 v-if="isConsultandoReniec" class="size-3.5 animate-spin" />
                  <IconSearch v-else class="size-3.5" />
                  <span>Consultar</span>
                </Button>
              </div>
            </div>

            <p v-if="reniecError" class="text-xs text-rose-500 font-medium">
              {{ reniecError }}
            </p>
            <p v-if="reniecSuccess"
              class="text-xs text-emerald-600 dark:text-emerald-400 font-medium flex items-center gap-1">
              <IconCheck class="size-3.5" /> Datos autocompletados desde RENIEC / Base local.
            </p>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5 pt-1">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Nombres *</label>
              <input v-model="formData.personal.nombre" type="text" placeholder="Ej. Juan Carlos"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Apellido Paterno *</label>
              <input v-model="formData.personal.apaterno" type="text" placeholder="Ej. Pérez"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Apellido Materno *</label>
              <input v-model="formData.personal.amaterno" type="text" placeholder="Ej. Gómez"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Fecha de Nacimiento *</label>
              <input v-model="formData.personal.nacimiento" type="date"
                class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Sexo</label>
              <select v-model="formData.personal.sexo"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary">
                <option value="M">Masculino</option>
                <option value="F">Femenino</option>
              </select>
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">RUC (opcional)</label>
              <input v-model="formData.personal.ruc" type="text" maxlength="11" placeholder="11 dígitos"
                class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
          </div>

          <div class="border-t border-border/70 pt-3 space-y-3">
            <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground block">
              Datos de Contacto y Residencia
            </span>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                  <IconPhone class="size-3.5 text-muted-foreground" /> Teléfono / Celular
                </label>
                <input v-model="formData.personal.telf" type="text" placeholder="Ej. 987654321"
                  class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
              </div>

              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                  <IconMail class="size-3.5 text-muted-foreground" /> Correo Electrónico
                </label>
                <input v-model="formData.personal.email" type="email" placeholder="ejemplo@munives.gob.pe"
                  class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
              </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground">Región / Dpto.</label>
                <input v-model="formData.personal.region" type="text" placeholder="LIMA"
                  class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
              </div>

              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground">Distrito</label>
                <input v-model="formData.personal.distrito" type="text" placeholder="VILLA EL SALVADOR"
                  class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
              </div>

              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                  <IconMapPin class="size-3.5 text-muted-foreground" /> Dirección
                </label>
                <input v-model="formData.personal.direccion" type="text" placeholder="Av. / Jr. / Calle / Mz."
                  class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
              </div>
            </div>
          </div>

          <div class="flex items-center justify-end pt-3 border-t border-border">
            <Button variant="primary" size="sm" class="gap-1.5 text-xs cursor-pointer" :disabled="!isStep1Valid"
              @click="avanzarPaso">
              <span>Continuar a Vínculo Laboral</span>
              <IconArrowLeft class="size-3.5 rotate-180" />
            </Button>
          </div>
        </Card>

        <Card v-show="currentStep === 2" class="space-y-4">
          <div class="flex items-center justify-between border-b border-border pb-3">
            <span class="text-sm font-bold text-foreground flex items-center gap-2">
              <IconBriefcase class="size-4 text-primary shrink-0" />
              Asignación y Vínculo Laboral
            </span>
            <span class="text-[11px] text-muted-foreground">Paso 2 de 3</span>
          </div>

          <div class="p-3.5 rounded-xl bg-card border border-border space-y-3">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
              <div class="space-y-0.5">
                <span class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                  <IconBriefcase class="size-3.5 text-primary" />
                  Asistente de Plaza Vacante (Opcional)
                </span>
                <p class="text-[11px] text-muted-foreground">
                  Puedes seleccionar una vacante disponible o ingresar el código para autocompletar datos del vínculo.
                </p>
              </div>

              <div class="flex items-center gap-2">
                <Button size="xs" variant="outline"
                  class="gap-1.5 text-xs border-primary/30 text-primary cursor-pointer"
                  @click="isVacantesModalOpen = true">
                  <IconSearch class="size-3.5" />
                  <span>Ver Vacantes Disponibles ({{ plazasVacantes.length }})</span>
                </Button>
              </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5 pt-1">
              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground flex items-center justify-between">
                  <span>Código de Plaza AIRHSP</span>
                  <span class="text-[10px] font-normal text-muted-foreground">Opcional</span>
                </label>
                <div class="relative">
                  <input v-model="formData.plaza.codigo" type="text" placeholder="Ej. 000124 (opcional)"
                    class="w-full px-3 py-2 text-xs font-mono font-bold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
                    @blur="onCambioCodigoPlazaManual" />
                </div>
              </div>

              <div class="space-y-1">
                <label class="text-xs font-semibold text-foreground">Régimen Laboral *</label>
                <select v-model="formData.plaza.regimen"
                  class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary"
                  @change="onRegimenChange">
                  <option v-for="r in regímenesDisponibles" :key="r.id" :value="r.id">
                    {{ r.nombre }}
                  </option>
                </select>
              </div>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                <IconBuildingSkyscraper class="size-3.5 text-muted-foreground" /> Área de Adscripción *
              </label>
              <select v-model="formData.plaza.areaId"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary">
                <option v-for="a in areas" :key="a.id" :value="a.id">
                  {{ a.nombre }} ({{ a.sigla || 'N/A' }})
                </option>
              </select>
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                <IconBriefcase class="size-3.5 text-muted-foreground" /> Cargo Institucional *
              </label>
              <select v-model="formData.plaza.cargoId"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary">
                <option v-for="c in cargos" :key="c.id" :value="c.id">
                  {{ c.nombre }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                <IconCoin class="size-3.5 text-muted-foreground" /> Remuneración Mensual (S/.) *
              </label>
              <input v-model.number="formData.plaza.sueldo" type="number" step="0.01" min="0" placeholder="0.00"
                class="w-full px-3 py-2 text-xs font-mono font-semibold rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Cargo Estructural / Clasificador</label>
              <input v-model="formData.plaza.cargoEstructural" type="text" placeholder="Ej. Especialista Administrativo"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
          </div>

          <div class="flex items-center justify-between pt-3 border-t border-border">
            <Button variant="outline" size="sm" class="gap-1.5 text-xs cursor-pointer" @click="retrocederPaso">
              <IconArrowLeft class="size-3.5" />
              <span>Anterior</span>
            </Button>
            <Button variant="primary" size="sm" class="gap-1.5 text-xs cursor-pointer" :disabled="!isStep2Valid"
              @click="avanzarPaso">
              <span>Continuar a Documento</span>
              <IconArrowLeft class="size-3.5 rotate-180" />
            </Button>
          </div>
        </Card>

        <Card v-show="currentStep === 3" class="space-y-4">
          <div class="flex items-center justify-between border-b border-border pb-3">
            <span class="text-sm font-bold text-foreground flex items-center gap-2">
              <IconFileText class="size-4 text-primary shrink-0" />
              Documento de Sustento del Ingreso
            </span>
            <span class="text-[11px] text-muted-foreground">Paso 3 de 3</span>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3.5">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Tipo de Documento *</label>
              <select v-model="formData.documento.tipoDocumento"
                class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary">
                <option v-for="t in tiposDocumento" :key="t.id" :value="String(t.id)">
                  {{ t.nombre }}
                </option>
              </select>
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Número de Documento *</label>
              <input v-model.number="formData.documento.numeroDocumento" type="number" min="1" placeholder="105"
                class="w-full px-3 py-2 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground">Año *</label>
              <input v-model.number="formData.documento.añoDocumento" type="number" min="1990" max="2050"
                class="w-full px-3 py-2 text-xs font-mono font-medium rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3.5">
            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                <IconCalendar class="size-3.5 text-muted-foreground" /> Fecha de Emisión *
              </label>
              <input v-model="formData.documento.fecha" type="date"
                class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>

            <div class="space-y-1">
              <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
                <IconCalendar class="size-3.5 text-muted-foreground" /> Fecha de Vigencia / Inicio
              </label>
              <input v-model="formData.documento.fechaValida" type="date"
                class="w-full px-3 py-2 text-xs font-mono rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
            </div>
          </div>

          <div class="space-y-1">
            <label class="text-xs font-semibold text-foreground flex items-center gap-1.5">
              <IconFileDescription class="size-3.5 text-muted-foreground" /> Descripción / Asunto del Documento *
            </label>
            <textarea v-model="formData.documento.descripcion" rows="3"
              placeholder="Ej. Designación en el cargo de Especialista mediante Resolución de Alcaldía N° 105-2026-MDS..."
              class="w-full px-3 py-2 text-xs rounded-lg border border-border bg-card text-foreground focus:outline-hidden focus:border-primary resize-none"></textarea>
          </div>

          <div class="flex items-center justify-between pt-3 border-t border-border">
            <Button variant="outline" size="sm" class="gap-1.5 text-xs cursor-pointer" @click="retrocederPaso">
              <IconArrowLeft class="size-3.5" />
              <span>Anterior</span>
            </Button>
            <Button variant="primary" size="sm" class="gap-1.5 text-xs cursor-pointer"
              :disabled="isSubmitting || !isStep1Valid || !isStep2Valid || !isStep3Valid" @click="onSubmitRegistro">
              <IconLoader2 v-if="isSubmitting" class="size-3.5 animate-spin" />
              <IconCheck v-else class="size-3.5" />
              <span>{{ isSubmitting ? 'Registrando...' : 'Confirmar y Guardar Registro' }}</span>
            </Button>
          </div>
        </Card>
      </div>

      <div class="xl:col-span-4 space-y-4 sticky top-6">
        <Card class="space-y-3.5 border-primary/20 bg-primary/2">
          <div class="flex items-center justify-between border-b border-border pb-2.5">
            <span class="text-xs font-bold uppercase tracking-wider text-foreground flex items-center gap-1.5">
              <IconId class="size-3.5 text-primary" />
              Ficha Previa del Servidor
            </span>
            <Badge variant="outline" size="xs" class="font-mono text-[10px] uppercase">
              {{ formData.personal.dni || 'Sin DNI' }}
            </Badge>
          </div>

          <div class="space-y-2 text-xs">
            <div>
              <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">Servidor</span>
              <p class="font-semibold text-foreground text-sm truncate" :title="nombreCompleto">
                {{ nombreCompleto }}
              </p>
            </div>

            <div class="grid grid-cols-2 gap-2 border-y border-border/60 py-2">
              <div>
                <span class="text-[10px] text-muted-foreground uppercase font-bold block">Nacimiento</span>
                <span class="font-mono text-foreground">{{ formData.personal.nacimiento ?
                  formatDate(formData.personal.nacimiento) : '-' }}</span>
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground uppercase font-bold block">Sexo</span>
                <span class="text-foreground">{{ formData.personal.sexo === 'M' ? 'Masculino' : 'Femenino' }}</span>
              </div>
            </div>

            <div>
              <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">Cargo
                Asignado</span>
              <p class="font-semibold text-foreground truncate">{{ cargoSeleccionadoNombre }}</p>
              <p class="text-[11px] text-primary truncate">{{ areaSeleccionadaNombre }}</p>
            </div>

            <div class="grid grid-cols-2 gap-2 border-y border-border/60 py-2">
              <div>
                <span class="text-[10px] text-muted-foreground uppercase font-bold block">Régimen</span>
                <span class="font-medium text-foreground truncate block">{{ formData.plaza.regimenNombre || '-'
                }}</span>
              </div>
              <div>
                <span class="text-[10px] text-muted-foreground uppercase font-bold block">Sueldo</span>
                <span class="font-mono font-bold text-foreground">{{ formatMoneda(formData.plaza.sueldo) }}</span>
              </div>
            </div>

            <div>
              <span class="text-[10px] uppercase font-bold text-muted-foreground tracking-wider block">Documento de
                Sustento</span>
              <p class="font-medium text-foreground truncate">
                {{ tipoDocSeleccionadoNombre }} N° {{ formData.documento.numeroDocumento }}-{{
                  formData.documento.añoDocumento }}
              </p>
              <p class="text-[11px] text-muted-foreground font-mono">
                Fecha: {{ formData.documento.fecha ? formatDate(formData.documento.fecha) : '-' }}
              </p>
            </div>
          </div>

          <div class="pt-2 border-t border-border">
            <div class="flex items-center gap-2">
              <span class="size-2 rounded-full shrink-0"
                :class="isStep1Valid && isStep2Valid && isStep3Valid ? 'bg-emerald-500 animate-pulse' : 'bg-amber-500'"></span>
              <span class="text-[11px] font-medium"
                :class="isStep1Valid && isStep2Valid && isStep3Valid ? 'text-emerald-600 dark:text-emerald-400' : 'text-amber-600 dark:text-amber-400'">
                {{ isStep1Valid && isStep2Valid && isStep3Valid ? 'Formulario completo y listo para registrar' :
                  'Completa todos los campos obligatorios' }}
              </span>
            </div>
          </div>
        </Card>
      </div>
    </div>

    <div v-if="isVacantesModalOpen" class="fixed inset-0 z-50 overflow-y-auto" role="dialog" aria-modal="true">
      <div class="fixed inset-0 bg-neutral-900/60 backdrop-blur-xs" @click="isVacantesModalOpen = false"></div>
      <div class="flex min-h-full items-center justify-center p-4">
        <div class="relative w-full max-w-2xl rounded-2xl bg-card border border-border shadow-2xl p-5 space-y-4">
          <div class="flex items-center justify-between border-b border-border pb-3">
            <div class="space-y-0.5">
              <h3 class="font-bold text-foreground text-sm tracking-tight flex items-center gap-2">
                <IconBriefcase class="size-4 text-primary shrink-0" />
                Plazas Vacantes Disponibles
              </h3>
              <p class="text-xs text-muted-foreground">
                Selecciona una plaza vacante para autocompletar el vínculo laboral.
              </p>
            </div>
            <button type="button"
              class="text-muted-foreground hover:text-foreground text-xs p-1 rounded-md cursor-pointer"
              @click="isVacantesModalOpen = false">
              Cerrar
            </button>
          </div>

          <div class="relative">
            <IconSearch
              class="size-4 absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <input v-model="filtroVacante" type="text" placeholder="Buscar por código de plaza, área o cargo..."
              class="w-full pl-9 pr-3 py-2 text-xs rounded-xl border border-border bg-card text-foreground focus:outline-hidden focus:border-primary" />
          </div>

          <div class="max-h-80 overflow-y-auto space-y-2 pr-1">
            <template v-if="plazasVacantesFiltradas.length > 0">
              <div v-for="v in plazasVacantesFiltradas" :key="v.codigo"
                class="flex items-center justify-between p-3 rounded-xl border border-border bg-muted/20 hover:bg-muted/50 transition-colors cursor-pointer"
                @click="seleccionarPlazaVacante(v)">
                <div class="min-w-0 flex-1 space-y-0.5">
                  <div class="flex items-center gap-2 flex-wrap">
                    <span class="font-mono font-bold text-xs text-foreground">{{ v.codigo }}</span>
                    <Badge variant="success" size="xs">Vacante</Badge>
                  </div>
                  <p class="text-xs font-semibold text-foreground truncate">{{ v.cargo || 'Sin cargo asignado' }}</p>
                  <p class="text-[11px] text-muted-foreground truncate">{{ v.area || 'Sin área asignada' }}</p>
                </div>

                <div class="text-right shrink-0 pl-3">
                  <span class="font-mono font-bold text-foreground text-xs block">
                    {{ formatMoneda(v.sueldo) }}
                  </span>
                  <Button size="xs" variant="primary" class="mt-1 text-[10px]">
                    Seleccionar
                  </Button>
                </div>
              </div>
            </template>
            <div v-else class="text-center py-8 text-xs text-muted-foreground">
              No se encontraron plazas vacantes que coincidan con la búsqueda.
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
