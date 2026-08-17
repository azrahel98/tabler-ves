<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { perfilApi, type DocumentoDetalle } from '@/api/perfil'
import {
  IconFileText,
  IconArrowLeft,
  IconCalendar,
  IconBuildingCommunity,
  IconHash,
  IconLoader2,
  IconAlertCircle,
  IconInfoCircle,
  IconCopy,
  IconCheck,
  IconRefresh,
  IconBriefcase,
} from '@tabler/icons-vue'

const route = useRoute()
const router = useRouter()

const documento = ref<DocumentoDetalle | null>(null)
const estaCargando = ref(true)
const mensajeError = ref<string | null>(null)
const copiadoId = ref(false)
const copiadoNumero = ref(false)

const tituloDocumento = computed(() => {
  if (!documento.value) return ''
  const tipo = documento.value.tipo || documento.value.tipoDocumento || 'Documento'
  const numero = documento.value.numero ?? documento.value.numeroDocumento
  const anio = documento.value.año ?? documento.value.añoDocumento
  if (numero && anio) return `${tipo} N° ${numero} - ${anio}`
  if (numero) return `${tipo} N° ${numero}`
  return tipo
})

const areaTexto = computed(() => {
  if (!documento.value) return 'No especificada'
  if (documento.value.sigla) return documento.value.sigla
  if (documento.value.area) return documento.value.area
  if (documento.value.areaId) return `Área #${documento.value.areaId}`
  return 'No especificada'
})

async function cargarDetalleDocumento(id: string | number) {
  estaCargando.value = true
  mensajeError.value = null
  try {
    const respuesta = await perfilApi.getDocumento(id)
    documento.value = respuesta
  } catch (error) {
    mensajeError.value = error instanceof Error ? error.message : 'No se pudo cargar el documento'
  } finally {
    estaCargando.value = false
  }
}

async function copiarAlPortapapeles(texto: string, tipo: 'id' | 'numero') {
  try {
    await navigator.clipboard.writeText(texto)
    if (tipo === 'id') {
      copiadoId.value = true
      setTimeout(() => {
        copiadoId.value = false
      }, 2000)
    } else {
      copiadoNumero.value = true
      setTimeout(() => {
        copiadoNumero.value = false
      }, 2000)
    }
  } catch (_e) {
    // fallback
  }
}

onMounted(() => {
  const id = route.params.id as string
  if (id) {
    cargarDetalleDocumento(id)
  }
})

watch(
  () => route.params.id,
  (nuevoId) => {
    if (nuevoId) {
      cargarDetalleDocumento(nuevoId as string)
    }
  },
)
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-6 max-w-[1100px] mx-auto">
    <div class="flex flex-wrap items-center justify-between gap-4 pb-3 border-b border-gray-100 dark:border-gray-800">
      <div class="flex items-center gap-3">
        <button
          type="button"
          @click="router.back()"
          class="flex h-9 w-9 items-center justify-center rounded-xl border border-gray-200 bg-white text-gray-600 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors shadow-xs cursor-pointer"
          title="Regresar">
          <IconArrowLeft class="h-4 w-4" />
        </button>

        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-xl bg-blue-50 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400 border border-blue-100 dark:border-blue-800/40">
            <IconFileText class="h-5 w-5" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <h1 class="text-base sm:text-lg font-bold text-gray-900 dark:text-white">
                Detalle del Documento
              </h1>
              <span
                v-if="documento"
                class="px-2 py-0.5 rounded-full text-3xs font-mono font-bold bg-blue-50 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300 border border-blue-200/60 dark:border-blue-700/50">
                #{{ documento.id || route.params.id }}
              </span>
            </div>
            <p class="text-xs text-gray-500 dark:text-gray-400">
              Registro del acto administrativo y metadatos documentales
            </p>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          @click="cargarDetalleDocumento(route.params.id as string)"
          class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl border border-gray-200 bg-white text-xs font-semibold text-gray-700 hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700 transition-colors cursor-pointer shadow-xs">
          <IconRefresh class="h-3.5 w-3.5" :class="{ 'animate-spin': estaCargando }" />
          <span>Actualizar</span>
        </button>
      </div>
    </div>

    <div
      v-if="estaCargando"
      class="flex flex-col items-center justify-center gap-3 rounded-2xl border border-gray-200 bg-white p-16 shadow-xs dark:border-gray-800 dark:bg-gray-900">
      <IconLoader2 class="h-8 w-8 animate-spin text-blue-600 dark:text-blue-400" />
      <p class="text-sm font-medium text-gray-500 dark:text-gray-400">Cargando datos del documento...</p>
    </div>

    <div
      v-else-if="mensajeError"
      class="flex items-start gap-3.5 rounded-2xl border border-red-200 bg-red-50/80 p-5 text-red-700 dark:border-red-900/50 dark:bg-red-950/20 dark:text-red-300 shadow-xs">
      <IconAlertCircle class="h-5 w-5 flex-shrink-0 text-red-500 mt-0.5" />
      <div class="space-y-2 flex-1">
        <div>
          <p class="text-sm font-bold">Error al cargar el documento</p>
          <p class="text-xs text-red-600 dark:text-red-400 mt-0.5">{{ mensajeError }}</p>
        </div>
        <button
          type="button"
          @click="cargarDetalleDocumento(route.params.id as string)"
          class="inline-flex items-center gap-1 px-3 py-1 rounded-lg bg-red-600 text-white text-xs font-semibold hover:bg-red-700 transition-colors cursor-pointer">
          <IconRefresh class="h-3.5 w-3.5" />
          <span>Reintentar</span>
        </button>
      </div>
    </div>

    <div
      v-else-if="documento"
      class="space-y-6">
      <div
        class="bg-white dark:bg-gray-900 rounded-2xl p-6 border border-gray-200/80 dark:border-gray-800 shadow-xs space-y-6">
        <div
          class="flex flex-wrap items-start justify-between gap-4 pb-5 border-b border-gray-100 dark:border-gray-800">
          <div class="space-y-1.5">
            <div class="flex items-center gap-2">
              <span
                class="inline-flex items-center px-2.5 py-1 rounded-lg text-2xs font-bold uppercase tracking-wider bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300 border border-blue-200/60 dark:border-blue-700/50">
                {{ documento.tipo || documento.tipoDocumento || 'Documento Oficial' }}
              </span>

              <span
                v-if="documento.sigla"
                class="inline-flex items-center px-2.5 py-1 rounded-lg text-2xs font-bold uppercase tracking-wider bg-indigo-50 text-indigo-700 dark:bg-indigo-900/30 dark:text-indigo-300 border border-indigo-200/60 dark:border-indigo-700/50">
                {{ documento.sigla }}
              </span>
            </div>

            <h2 class="text-xl sm:text-2xl font-black text-gray-900 dark:text-white tracking-tight">
              {{ tituloDocumento }}
            </h2>
          </div>

          <div class="flex items-center gap-2">
            <button
              type="button"
              @click="copiarAlPortapapeles(String(documento.id || route.params.id), 'id')"
              class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl border border-gray-200 bg-gray-50/80 hover:bg-gray-100 text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700 dark:text-gray-300 text-xs font-mono font-semibold transition-colors cursor-pointer shadow-xs"
              title="Copiar ID del documento">
              <component :is="copiadoId ? IconCheck : IconCopy" class="h-3.5 w-3.5 text-blue-600 dark:text-blue-400" />
              <span>ID: #{{ documento.id || route.params.id }}</span>
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <div
            class="p-4 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-1.5">
            <div class="flex items-center gap-1.5 text-gray-400 dark:text-gray-500 font-semibold text-3xs uppercase tracking-wider">
              <IconCalendar class="h-3.5 w-3.5 text-blue-500" />
              <span>Fecha Emisión</span>
            </div>
            <p class="text-sm font-bold text-gray-900 dark:text-white">
              {{ documento.fecha || 'No registrada' }}
            </p>
          </div>

          <div
            class="p-4 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-1.5">
            <div class="flex items-center gap-1.5 text-gray-400 dark:text-gray-500 font-semibold text-3xs uppercase tracking-wider">
              <IconCalendar class="h-3.5 w-3.5 text-emerald-500" />
              <span>Fecha de Validez</span>
            </div>
            <p class="text-sm font-bold text-gray-900 dark:text-white">
              {{ documento.fechaValida || 'Indefinida' }}
            </p>
          </div>

          <div
            class="p-4 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-1.5">
            <div class="flex items-center gap-1.5 text-gray-400 dark:text-gray-500 font-semibold text-3xs uppercase tracking-wider">
              <IconBuildingCommunity class="h-3.5 w-3.5 text-indigo-500" />
              <span>Área Emisora</span>
            </div>
            <p class="text-sm font-bold text-gray-900 dark:text-white truncate">
              {{ areaTexto }}
            </p>
          </div>

          <div
            class="p-4 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-1.5">
            <div class="flex items-center gap-1.5 text-gray-400 dark:text-gray-500 font-semibold text-3xs uppercase tracking-wider">
              <IconHash class="h-3.5 w-3.5 text-amber-500" />
              <span>Convocatoria / Proceso</span>
            </div>
            <p class="text-sm font-bold text-gray-900 dark:text-white">
              {{ documento.conv ? `Proceso #${documento.conv}` : 'Sin convocatoria' }}
            </p>
          </div>
        </div>

        <div
          v-if="documento.funcion"
          class="p-4 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-1.5">
          <div class="flex items-center gap-1.5 text-gray-400 dark:text-gray-500 font-semibold text-3xs uppercase tracking-wider">
            <IconBriefcase class="h-3.5 w-3.5 text-purple-500" />
            <span>Función Asociada</span>
          </div>
          <p class="text-sm font-bold text-gray-900 dark:text-white">
            Función #{{ documento.funcion }}
          </p>
        </div>

        <div
          class="p-5 rounded-xl bg-gray-50/70 dark:bg-gray-800/40 border border-gray-100 dark:border-gray-800 space-y-2.5">
          <div
            class="flex items-center justify-between gap-2">
            <div class="flex items-center gap-2 text-gray-600 dark:text-gray-300 font-bold text-xs uppercase tracking-wider">
              <IconInfoCircle class="h-4 w-4 text-blue-500" />
              <span>Descripción / Asunto del Documento</span>
            </div>

            <button
              v-if="documento.descripcion"
              type="button"
              @click="copiarAlPortapapeles(documento.descripcion, 'numero')"
              class="inline-flex items-center gap-1 text-3xs font-semibold text-blue-600 dark:text-blue-400 hover:underline cursor-pointer">
              <component :is="copiadoNumero ? IconCheck : IconCopy" class="h-3 w-3" />
              <span>{{ copiadoNumero ? 'Copiado' : 'Copiar texto' }}</span>
            </button>
          </div>

          <p class="text-sm text-gray-800 dark:text-gray-200 leading-relaxed whitespace-pre-wrap font-normal">
            {{ documento.descripcion || 'Sin descripción registrada' }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

