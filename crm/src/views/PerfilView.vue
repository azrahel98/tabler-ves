<script setup lang="ts">
import { ref, onMounted, watch, defineAsyncComponent } from 'vue'
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import PerfilHeader from '@/components/perfil/PerfilHeader.vue'
import { usePerfilStore } from '@/stores/perfil'
import {
  IconPhone,
  IconMail,
  IconMapPin,
  IconAlertCircle,
  IconLoader2,
  IconCalendar,
  IconCake,
  IconGenderMale,
  IconGenderFemale,
  IconBuildingCommunity,
  IconPhoneCall,
} from '@tabler/icons-vue'

const VinculosCard = defineAsyncComponent(() => import('@/components/perfil/VinculosCard.vue'))
const InfoBancariaCard = defineAsyncComponent(() => import('@/components/perfil/InfoBancariaCard.vue'))
const LegajoCard = defineAsyncComponent(() => import('@/components/perfil/LegajoCard.vue'))

const route = useRoute()
const tabActiva = ref<'vinculos' | 'banco' | 'legajo'>('vinculos')

const perfilStore = usePerfilStore()
const { perfil, vinculos, banco, contacto, grados, documentos, isLoading, cargandoLegajo, cargandoBanco, error } =
  storeToRefs(perfilStore)
const cargarPerfil = perfilStore.cargarPerfil
const cargarLegajo = perfilStore.cargarLegajo
const cargarBanco = perfilStore.cargarBanco

onMounted(() => {
  const dni = route.params.dni as string
  if (dni) cargarPerfil(dni)
})

watch(
  () => route.params.dni,
  (nuevoDni) => {
    if (nuevoDni && typeof nuevoDni === 'string') {
      tabActiva.value = 'vinculos'
      cargarPerfil(nuevoDni)
    }
  },
)

watch(
  [tabActiva, () => route.params.dni],
  ([nuevaTab, dni]) => {
    if (typeof dni === 'string' && dni) {
      if (nuevaTab === 'legajo') {
        cargarLegajo(dni)
      } else if (nuevaTab === 'banco') {
        cargarBanco(dni)
      }
    }
  },
  { immediate: true },
)

function calcularEdad(fechaNacimiento: string | null): number | null {
  if (!fechaNacimiento) return null
  const fecha = new Date(fechaNacimiento)
  if (isNaN(fecha.getTime())) return null
  const hoy = new Date()
  let edad = hoy.getFullYear() - fecha.getFullYear()
  const m = hoy.getMonth() - fecha.getMonth()
  if (m < 0 || (m === 0 && hoy.getDate() < fecha.getDate())) {
    edad--
  }
  return edad >= 0 ? edad : null
}
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-5 max-w-[1600px] mx-auto">
    <div
      v-if="isLoading"
      class="flex flex-col items-center justify-center gap-3 rounded-xl border border-gray-200 bg-white p-12 shadow-sm dark:border-gray-700 dark:bg-gray-800">
      <IconLoader2 class="h-6 w-6 animate-spin text-blue-600" />
      <p class="text-sm text-gray-500 dark:text-gray-400">Cargando perfil del trabajador...</p>
    </div>

    <!-- Error State -->
    <div
      v-else-if="error"
      class="flex items-center gap-3 rounded-xl border border-red-200 bg-red-50 p-4 text-red-700 dark:border-red-900/50 dark:bg-red-950/20 dark:text-red-300">
      <IconAlertCircle class="h-5 w-5 flex-shrink-0 text-red-500" />
      <div>
        <p class="text-sm font-semibold">No se pudieron cargar los datos</p>
        <p class="text-xs text-red-600 dark:text-red-400 mt-0.5">{{ error }}</p>
      </div>
    </div>

    <!-- Content Top Header Banner Card -->
    <template v-else-if="perfil">
      <PerfilHeader
        :perfil="perfil"
        :vinculos="vinculos"
        :tab-activa="tabActiva"
        @cambiar-tab="(t) => (tabActiva = t)" />

      <!-- 2 Column Layout Matching Screenshot -->
      <div class="flex flex-col lg:flex-row gap-5 items-start">
        <!-- Left Column: About / Overview Sidebar Card -->
        <div class="w-full lg:w-80 flex-shrink-0 space-y-4">
          <div
            class="rounded-xl border border-gray-200 bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800 space-y-4">
            <div class="pb-2 border-b border-gray-100 dark:border-gray-700">
              <h3 class="text-2xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500">
                Información de Contacto
              </h3>
            </div>

            <!-- Datos de Contacto Directo -->
            <div class="space-y-2 text-2xs">
              <div v-if="perfil.telf" class="flex items-center gap-2.5 text-gray-700 dark:text-gray-200">
                <div class="p-1 rounded bg-slate-100 dark:bg-navy-900 text-slate-500 shrink-0">
                  <IconPhone class="h-3.5 w-3.5" />
                </div>
                <span class="font-medium">{{ perfil.telf }}</span>
              </div>

              <div v-if="perfil.email" class="flex items-center gap-2.5 text-gray-700 dark:text-gray-200 min-w-0">
                <div class="p-1 rounded bg-slate-100 dark:bg-navy-900 text-slate-500 shrink-0">
                  <IconMail class="h-3.5 w-3.5" />
                </div>
                <span class="truncate font-medium">{{ perfil.email }}</span>
              </div>
            </div>

            <!-- Datos Personales y Ubicación -->
            <div class="pt-3 border-t border-gray-100 dark:border-gray-700/80 space-y-2.5 text-2xs">
              <div v-if="perfil.nacimiento" class="flex items-center justify-between">
                <span class="flex items-center gap-2 text-gray-600 dark:text-gray-300">
                  <IconCalendar class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 shrink-0" />
                  <span>F. Nacimiento</span>
                </span>
                <span class="font-semibold text-gray-800 dark:text-white">
                  {{ $formatearFecha(perfil.nacimiento) }}
                </span>
              </div>

              <div v-if="calcularEdad(perfil.nacimiento) !== null" class="flex items-center justify-between">
                <span class="flex items-center gap-2 text-gray-600 dark:text-gray-300">
                  <IconCake class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 shrink-0" />
                  <span>Edad</span>
                </span>
                <span class="font-semibold text-gray-800 dark:text-white">
                  {{ calcularEdad(perfil.nacimiento) }} años
                </span>
              </div>

              <div v-if="perfil.sexo" class="flex items-center justify-between">
                <span class="flex items-center gap-2 text-gray-600 dark:text-gray-300">
                  <component
                    :is="perfil.sexo === 'F' ? IconGenderFemale : IconGenderMale"
                    class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 shrink-0" />
                  <span>Sexo</span>
                </span>
                <span class="font-semibold text-gray-800 dark:text-white">
                  {{ perfil.sexo === 'M' ? 'Masculino' : perfil.sexo === 'F' ? 'Femenino' : perfil.sexo }}
                </span>
              </div>

              <div v-if="perfil.region || perfil.distrito" class="space-y-1">
                <span class="flex items-center gap-2 text-gray-600 dark:text-gray-300">
                  <IconBuildingCommunity class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 shrink-0" />
                  <span>Región / Distrito</span>
                </span>
                <p class="pl-5 text-sm font-semibold text-gray-800 dark:text-white">
                  {{ perfil.region || '-' }}
                  <template v-if="perfil.distrito">
                    /
                    <RouterLink
                      :to="`/distrito/${encodeURIComponent(perfil.distrito)}`"
                      class="text-blue-600 dark:text-blue-400 hover:underline">
                      {{ perfil.distrito }}
                    </RouterLink>
                  </template>
                </p>
              </div>

              <div v-if="perfil.direccion" class="space-y-1">
                <span class="flex items-center gap-2 text-gray-600 dark:text-gray-300">
                  <IconMapPin class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 shrink-0" />
                  <span>Dirección</span>
                </span>
                <p class="pl-5 text-sm font-medium text-gray-700 dark:text-gray-300 leading-normal">
                  {{ perfil.direccion }}
                </p>
              </div>
            </div>

            <div v-if="contacto" class="pt-3 border-t border-gray-100 dark:border-gray-700/80">
              <div
                class="p-3 rounded-xl bg-slate-50/70 dark:bg-gray-800/40 border border-slate-200/70 dark:border-gray-700/70 space-y-2">
                <div class="flex items-center justify-between gap-2">
                  <span
                    class="text-3xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500 flex items-center gap-1">
                    <IconPhoneCall class="h-3 w-3 text-slate-400 dark:text-slate-500" />
                    <span>Contacto de Emergencia</span>
                  </span>
                  <span
                    class="px-2 py-0.5 rounded-md text-3xs font-bold uppercase tracking-wider bg-rose-50 text-rose-700 dark:bg-rose-950/40 dark:text-rose-300 border border-rose-200/60 dark:border-rose-800/40">
                    {{ contacto.relacion }}
                  </span>
                </div>

                <p class="text-xs font-bold text-gray-900 dark:text-white leading-snug">
                  {{ contacto.nombre }}
                </p>

                <a
                  v-if="contacto.telefono"
                  :href="`tel:${contacto.telefono}`"
                  class="inline-flex items-center gap-1.5 text-xs font-semibold font-mono text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors">
                  <IconPhone class="h-3.5 w-3.5 shrink-0" />
                  <span>{{ contacto.telefono }}</span>
                </a>
              </div>
            </div>
          </div>
        </div>

        <div class="flex-1 w-full min-w-0">
          <div
            id="tab-panel-vinculos"
            role="tabpanel"
            aria-labelledby="tab-btn-vinculos"
            :hidden="tabActiva !== 'vinculos'">
            <VinculosCard
              v-if="tabActiva === 'vinculos'"
              :vinculos="vinculos"
              :documentos="documentos"
              @cambiar-tab="(t) => (tabActiva = t)" />
          </div>

          <div id="tab-panel-banco" role="tabpanel" aria-labelledby="tab-btn-banco" :hidden="tabActiva !== 'banco'">
            <InfoBancariaCard v-if="tabActiva === 'banco'" :banco="banco" :grados="grados" :cargando="cargandoBanco" />
          </div>

          <div id="tab-panel-legajo" role="tabpanel" aria-labelledby="tab-btn-legajo" :hidden="tabActiva !== 'legajo'">
            <LegajoCard v-if="tabActiva === 'legajo'" :documentos="documentos" :cargando="cargandoLegajo" />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
