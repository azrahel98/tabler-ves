<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Resumen del Registro</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Verifique todos los datos antes de confirmar el registro</p>
    </div>

    <div class="p-5 space-y-5">
      
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-2 mb-3">
          <MapPin class="h-4 w-4 text-brand-500" />
          <h5 class="text-sm font-semibold text-gray-800 dark:text-white/90">Plaza Asignada</h5>
        </div>
        <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-4">
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Código</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ plaza?.codigo }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Cargo</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ plaza?.cargo_descripcion }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Régimen</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ plaza?.regimen }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Condición</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ plaza?.condicion }}</p>
          </div>
        </div>
      </div>

      
      <div v-if="necesitaAreaCargo && (areaSeleccionada || cargoSeleccionado)" class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-2 mb-3">
          <Building class="h-4 w-4 text-brand-500" />
          <h5 class="text-sm font-semibold text-gray-800 dark:text-white/90">Área y Cargo Asignados</h5>
        </div>
        <div class="grid grid-cols-2 gap-3 text-sm">
          <div v-if="areaSeleccionada">
            <span class="text-xs text-gray-500 dark:text-gray-400">Área</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ areaSeleccionada.nombre }}</p>
          </div>
          <div v-if="cargoSeleccionado">
            <span class="text-xs text-gray-500 dark:text-gray-400">Cargo</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ cargoSeleccionado.nombre }}</p>
          </div>
        </div>
      </div>

      
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-2 mb-3">
          <User class="h-4 w-4 text-brand-500" />
          <h5 class="text-sm font-semibold text-gray-800 dark:text-white/90">Datos Personales</h5>
        </div>
        <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-3">
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">DNI</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.dni }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Apellido Paterno</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.apaterno }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Apellido Materno</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.amaterno }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Nombres</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.nombre }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Nacimiento</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.nacimiento }}</p>
          </div>
          <div v-if="persona.sexo">
            <span class="text-xs text-gray-500 dark:text-gray-400">Sexo</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.sexo === 'M' ? 'Masculino' : 'Femenino' }}</p>
          </div>
          <div v-if="persona.telf">
            <span class="text-xs text-gray-500 dark:text-gray-400">Teléfono</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ formatPhone(persona.telf) }}</p>
          </div>
          <div v-if="persona.email">
            <span class="text-xs text-gray-500 dark:text-gray-400">Email</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.email }}</p>
          </div>
          <div v-if="persona.region">
            <span class="text-xs text-gray-500 dark:text-gray-400">Región</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.region }}</p>
          </div>
          <div v-if="persona.distrito">
            <span class="text-xs text-gray-500 dark:text-gray-400">Distrito</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ persona.distrito }}</p>
          </div>
        </div>
      </div>

      
      <div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
        <div class="flex items-center gap-2 mb-3">
          <FileText class="h-4 w-4 text-brand-500" />
          <h5 class="text-sm font-semibold text-gray-800 dark:text-white/90">Documento de Ingreso</h5>
        </div>
        <div class="grid grid-cols-2 gap-3 text-sm sm:grid-cols-3">
          <div v-if="nombreDocumento">
            <span class="text-xs text-gray-500 dark:text-gray-400">Tipo</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ nombreDocumento }}</p>
          </div>
          <div v-if="documento.numeroDocumento">
            <span class="text-xs text-gray-500 dark:text-gray-400">Número</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ documento.numeroDocumento }}</p>
          </div>
          <div v-if="documento.añoDocumento">
            <span class="text-xs text-gray-500 dark:text-gray-400">Año</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ documento.añoDocumento }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Fecha de Ingreso</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ documento.fecha }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500 dark:text-gray-400">Sueldo</span>
            <p class="font-medium text-gray-800 dark:text-white/90">S/ {{ Number(documento.sueldo || 0).toFixed(2) }}</p>
          </div>
          <div v-if="documento.descripcion" class="sm:col-span-3">
            <span class="text-xs text-gray-500 dark:text-gray-400">Descripción</span>
            <p class="font-medium text-gray-800 dark:text-white/90">{{ documento.descripcion }}</p>
          </div>
        </div>
      </div>

      <div v-if="error" class="flex items-center gap-3 rounded-lg border border-red-200 bg-red-50 px-4 py-3 dark:border-red-800 dark:bg-red-500/10">
        <AlertCircle class="h-5 w-5 text-red-600 dark:text-red-400 shrink-0" />
        <p class="text-sm font-medium text-red-800 dark:text-red-300">{{ error }}</p>
      </div>

      <div class="flex items-center justify-between pt-4">
        <button
          @click="store.retroceder()"
          :disabled="guardando"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 disabled:opacity-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <ArrowLeft class="h-4 w-4" />
          Anterior
        </button>
        <button
          @click="registrar"
          :disabled="guardando"
          class="inline-flex items-center gap-2 rounded-lg bg-success-500 px-6 py-2.5 text-sm font-medium text-white transition hover:bg-success-600 disabled:opacity-50 disabled:cursor-not-allowed">
          <Loading v-if="guardando" size="xs" />
          <Check v-else class="h-4 w-4" />
          Registrar Trabajador
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { MapPin, User, FileText, AlertCircle, ArrowLeft, Check, Building } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'
  import { formatPhone } from '../../utils/formatters'

  const emit = defineEmits<{
    (e: 'exito', mensaje: string): void
  }>()

  const store = useVinculoStore()
  const {
    plazaSeleccionada: plaza,
    formularioPersonal: persona,
    formularioDocumento: documento,
    documentos,
    guardando,
    necesitaAreaCargo,
    areaSeleccionada,
    cargoSeleccionado,
  } = storeToRefs(store)

  const error = ref('')

  const nombreDocumento = computed(() => {
    if (!documento.value.tipoDocumento) return ''
    const doc = documentos.value.find((d: any) => d.id == documento.value.tipoDocumento)
    return doc ? `${doc.sigla} — ${doc.nombre}` : ''
  })

  const registrar = async () => {
    error.value = ''
    try {
      await store.registrarTrabajador()
      emit('exito', 'Trabajador registrado correctamente')
    } catch (err: any) {
      error.value = err?.response?.data?.error || err.message || 'Error al registrar el trabajador'
    }
  }
</script>
