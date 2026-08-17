<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useDetalleDocumentoStore } from '@/stores/detalleDocumento'
import type { ArchivoLegajo } from '@/api/perfil'
import {
  IconX,
  IconFileText,
  IconFileTypePdf,
  IconCalendar,
  IconBuildingCommunity,
  IconHash,
  IconDownload,
  IconExternalLink,
  IconLoader2,
  IconAlertCircle,
  IconFolder,
  IconInfoCircle,
} from '@tabler/icons-vue'

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'

const detalleStore = useDetalleDocumentoStore()
const { estaAbierto, estaCargando, documento, archivos, error, idActual } = storeToRefs(detalleStore)

function cerrarPanel() {
  detalleStore.cerrar()
}

function manejarTeclaEscape(evento: KeyboardEvent) {
  if (evento.key === 'Escape' && estaAbierto.value) {
    cerrarPanel()
  }
}

onMounted(() => {
  window.addEventListener('keydown', manejarTeclaEscape)
})

onUnmounted(() => {
  window.removeEventListener('keydown', manejarTeclaEscape)
})

watch(estaAbierto, (abierto) => {
  if (abierto) {
    document.body.style.overflow = 'hidden'
  } else {
    document.body.style.overflow = ''
  }
})

function obtenerRutaArchivo(archivo: ArchivoLegajo): string {
  if (archivo.external_url) {
    return archivo.external_url
  }
  return `${apiBaseUrl.replace(/\/$/, '')}/fileserver/${archivo.file_hash}`
}
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity ease-out duration-300"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-opacity ease-in duration-200"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0">
      <div
        v-if="estaAbierto"
        class="fixed inset-0 bg-slate-900/40 dark:bg-black/60 backdrop-blur-xs z-50 transition-opacity cursor-pointer"
        @click="cerrarPanel" />
    </Transition>

    <Transition
      enter-active-class="transform transition ease-in-out duration-300 sm:duration-300"
      enter-from-class="translate-x-full"
      enter-to-class="translate-x-0"
      leave-active-class="transform transition ease-in-out duration-250 sm:duration-250"
      leave-from-class="translate-x-0"
      leave-to-class="translate-x-full">
      <div
        v-if="estaAbierto"
        class="fixed inset-y-0 right-0 z-50 w-full max-w-lg bg-white dark:bg-navy-800 shadow-2xl border-l border-slate-200/80 dark:border-navy-700/80 flex flex-col focus:outline-none">
        <div
          class="px-5 py-4 border-b border-slate-100 dark:border-navy-700/80 flex items-center justify-between bg-slate-50/50 dark:bg-navy-900/40">
          <div class="flex items-center gap-2.5">
            <div class="p-2 rounded-lg bg-blue-50 dark:bg-blue-500/10 text-blue-600 dark:text-blue-400">
              <IconFileText class="h-4 w-4" />
            </div>
            <div>
              <h2 class="text-xs font-bold uppercase tracking-wider text-slate-800 dark:text-white">
                Detalle del Documento
              </h2>
              <p class="text-3xs text-slate-400">Información del registro administrativo</p>
            </div>
          </div>

          <div class="flex items-center gap-1.5">
            <RouterLink
              v-if="idActual"
              :to="`/documento/${idActual}`"
              target="_blank"
              class="p-1.5 rounded-lg text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-navy-700 transition-colors"
              title="Abrir en página completa">
              <IconExternalLink class="h-4 w-4" />
            </RouterLink>

            <button
              type="button"
              @click="cerrarPanel"
              class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-navy-700 transition-colors"
              title="Cerrar">
              <IconX class="h-4 w-4" />
            </button>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto p-5 space-y-5">
          <div v-if="estaCargando" class="flex flex-col items-center justify-center py-16 text-center space-y-3">
            <IconLoader2 class="h-8 w-8 animate-spin text-blue-600 dark:text-blue-400" />
            <p class="text-xs font-medium text-slate-500 dark:text-slate-400">Cargando información del documento...</p>
          </div>

          <div
            v-else-if="error"
            class="p-4 rounded-xl bg-red-50 dark:bg-red-950/20 border border-red-200/70 dark:border-red-900/50 text-red-700 dark:text-red-300 flex items-start gap-3 text-xs">
            <IconAlertCircle class="h-5 w-5 shrink-0 text-red-500 mt-0.5" />
            <div class="space-y-1">
              <p class="font-bold">Error al cargar datos</p>
              <p class="text-xs text-red-600 dark:text-red-400">{{ error }}</p>
            </div>
          </div>

          <template v-else-if="documento">
            <div class="space-y-2 pb-3 border-b border-slate-100 dark:border-navy-700/80">
              <div class="flex items-center justify-between gap-2">
                <span
                  class="inline-flex items-center px-2 py-0.5 rounded-md text-2xs font-medium uppercase tracking-wider bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300 border border-blue-200/50 dark:border-blue-700/40">
                  {{ documento.tipo || documento.tipoDocumento || 'Documento' }}
                </span>

                <span
                  class="px-2 py-0.5 rounded text-xs font-mono font-semibold bg-slate-100 dark:bg-navy-700 text-slate-600 dark:text-slate-300">
                  ID: #{{ documento.id || idActual }}
                </span>
              </div>

              <h3 class="text-sm font-bold text-slate-900 dark:text-white leading-snug">
                {{ documento.tipo || documento.tipoDocumento || 'Documento' }} N°
                {{ documento.numero ?? documento.numeroDocumento ?? '-' }} -
                {{ documento.año ?? documento.añoDocumento ?? '' }}
              </h3>
            </div>

            <div
              class="bg-slate-50/70 dark:bg-navy-900/40 rounded-xl p-3.5 border border-slate-100 dark:border-navy-700/60 space-y-3">
              <div class="grid grid-cols-2 gap-3 text-xs">
                <div class="space-y-1" v-if="documento.areaId">
                  <div class="flex items-center gap-1 text-slate-400 font-semibold uppercase text-3xs">
                    <IconBuildingCommunity class="h-3 w-3 text-indigo-500" />
                    <span>Área Emisora</span>
                  </div>
                  <p class="font-semibold text-slate-800 dark:text-slate-200">
                    {{
                      documento.sigla ||
                      documento.area ||
                      (documento.areaId ? `Área #${documento.areaId}` : 'No especificada')
                    }}
                  </p>
                </div>

                <div class="space-y-1" v-if="documento.conv">
                  <div class="flex items-center gap-1 text-slate-400 font-semibold uppercase text-3xs">
                    <IconHash class="h-3 w-3 text-amber-500" />
                    <span>Convocatoria</span>
                  </div>
                  <p class="font-semibold text-slate-800 dark:text-slate-200">
                    {{ documento.conv ? `Proceso #${documento.conv}` : 'Sin convocatoria' }}
                  </p>
                </div>

                <div class="space-y-1 pt-2 border-t border-slate-200/60 dark:border-navy-700/60">
                  <div class="flex items-center gap-1 text-slate-400 font-semibold uppercase text-2xs">
                    <IconCalendar class="h-3 w-3 text-blue-500" />
                    <span>Fecha Emisión</span>
                  </div>
                  <p class="font-semibold text-slate-800 dark:text-slate-200">
                    {{ documento.fecha || 'No registrada' }}
                  </p>
                </div>

                <div
                  v-if="documento.fechaValida"
                  class="space-y-1 pt-2 border-t border-slate-200/60 dark:border-navy-700/60">
                  <div class="flex items-center gap-1 text-slate-400 font-semibold uppercase text-2xs">
                    <IconCalendar class="h-3 w-3 text-emerald-500" />
                    <span>Fecha Validez</span>
                  </div>
                  <p class="font-semibold text-slate-800 dark:text-slate-200">
                    {{ documento.fechaValida || 'Indefinida' }}
                  </p>
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <div
                class="flex items-center gap-1.5 text-slate-500 dark:text-slate-400 text-3xs font-bold uppercase tracking-wider">
                <IconInfoCircle class="h-3.5 w-3.5 text-blue-500" />
                <span>Descripción / Asunto</span>
              </div>
              <div
                class="p-3.5 bg-slate-50/70 dark:bg-navy-900/40 rounded-xl border border-slate-100 dark:border-navy-700/60">
                <p class="text-xs text-slate-700 dark:text-slate-200 leading-relaxed whitespace-pre-wrap">
                  {{ documento.descripcion || 'Sin descripción registrada para este documento.' }}
                </p>
              </div>
            </div>

            <div class="space-y-2.5 pt-2">
              <div class="flex items-center justify-between">
                <div
                  class="flex items-center gap-1.5 text-slate-500 dark:text-slate-400 text-3xs font-bold uppercase tracking-wider">
                  <IconFolder class="h-3.5 w-3.5 text-amber-500" />
                  <span>Archivos Digitales Asociados ({{ archivos.length }})</span>
                </div>
              </div>

              <div
                v-if="archivos.length === 0"
                class="p-4 text-center rounded-xl bg-slate-50/40 dark:bg-navy-900/20 border border-dashed border-slate-200 dark:border-navy-700 text-xs text-slate-400">
                No hay archivos adjuntos en el legajo asociados a este documento.
              </div>

              <div v-else class="space-y-2">
                <div
                  v-for="archivo in archivos"
                  :key="archivo.id"
                  class="group flex items-center justify-between gap-3 p-2.5 rounded-lg bg-slate-50/80 dark:bg-navy-900/60 border border-slate-200/70 dark:border-navy-700 hover:border-blue-400 dark:hover:border-blue-500 transition-all">
                  <div class="flex items-center gap-2.5 min-w-0">
                    <div class="p-1.5 rounded-md bg-red-50 text-red-600 dark:bg-red-500/10 dark:text-red-400 shrink-0">
                      <IconFileTypePdf class="h-4 w-4" />
                    </div>
                    <div class="min-w-0">
                      <p class="text-xs font-semibold text-slate-800 dark:text-white truncate">
                        {{ archivo.original_name }}
                      </p>
                      <div class="flex items-center gap-2 text-[10px] text-slate-400 font-mono">
                        <span>{{ archivo.fecha_subida || 'Sin fecha' }}</span>
                        <span v-if="archivo.usuario_subida">• {{ archivo.usuario_subida }}</span>
                      </div>
                    </div>
                  </div>

                  <a
                    :href="obtenerRutaArchivo(archivo)"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-1 px-2 py-1 rounded bg-white hover:bg-blue-600 hover:text-white dark:bg-navy-800 dark:hover:bg-blue-600 text-slate-700 dark:text-slate-200 border border-slate-200 dark:border-navy-700 text-3xs font-semibold shadow-xs transition-colors shrink-0">
                    <component :is="archivo.external_url ? IconExternalLink : IconDownload" class="h-3 w-3" />
                    <span>{{ archivo.external_url ? 'Abrir' : 'Ver PDF' }}</span>
                  </a>
                </div>
              </div>
            </div>
          </template>
        </div>

        <div
          class="p-3 px-5 border-t border-slate-100 dark:border-navy-700/80 bg-slate-50/50 dark:bg-navy-900/40 flex items-center justify-end">
          <button
            type="button"
            @click="cerrarPanel"
            class="px-3.5 py-1.5 rounded-lg bg-slate-200 hover:bg-slate-300 dark:bg-navy-700 dark:hover:bg-navy-600 text-slate-700 dark:text-slate-200 text-xs font-semibold transition-colors cursor-pointer">
            Cerrar
          </button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
