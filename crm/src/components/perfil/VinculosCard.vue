<script setup lang="ts">
import { ref } from 'vue'
import { useDetalleDocumentoStore } from '@/stores/detalleDocumento'
import type { VinculoLaboral, ArchivoLegajo } from '@/api/perfil'
import {
  IconBriefcase,
  IconCalendar,
  IconBuildingSkyscraper,
  IconChevronDown,
  IconChevronUp,
  IconFolder,
  IconFileText,
  IconExternalLink,
  IconDownload,
  IconChevronRight,
} from '@tabler/icons-vue'

const props = defineProps<{
  vinculos: VinculoLaboral[]
  documentos?: ArchivoLegajo[]
}>()

const emit = defineEmits<{
  (e: 'cambiarTab', tab: 'vinculos' | 'banco' | 'legajo'): void
}>()

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'
const vinculoExpandidoId = ref<number | null>(null)
const detalleDocumentoStore = useDetalleDocumentoStore()

function verDetalleDocumento(id: number | string, archivosAdjuntos: ArchivoLegajo[] = []) {
  detalleDocumentoStore.abrir(id, archivosAdjuntos)
}

function alternarVinculo(id: number) {
  vinculoExpandidoId.value = vinculoExpandidoId.value === id ? null : id
}

function formatearMoneda(monto: number | null | undefined): string {
  if (monto == null) return 'S/ 0.00'
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'PEN' }).format(monto)
}

function normalizarTexto(texto: string): string {
  return texto.toLowerCase().replace(/[^a-z0-9]/g, '')
}

function buscarDocumentosVinculo(v: VinculoLaboral): ArchivoLegajo[] {
  if (!props.documentos || props.documentos.length === 0) return []

  const numerosBuscar: string[] = []
  if (v.numero_doc_ingreso) {
    const numLimpio = normalizarTexto(v.numero_doc_ingreso)
    if (numLimpio.length >= 2) numerosBuscar.push(numLimpio)
  }
  if (v.numero_doc_salida) {
    const numLimpio = normalizarTexto(v.numero_doc_salida)
    if (numLimpio.length >= 2) numerosBuscar.push(numLimpio)
  }
  if (v.numero_doc_evento) {
    const numStr = String(v.numero_doc_evento)
    if (numStr.length >= 2) numerosBuscar.push(numStr)
  }

  const idsDocumentos: (number | string)[] = []
  if (v.doc_ingreso_id) idsDocumentos.push(v.doc_ingreso_id)
  if (v.documento_id) idsDocumentos.push(v.documento_id)
  if (v.doc_salida_id) idsDocumentos.push(v.doc_salida_id)
  if (v.doc_evento_id) idsDocumentos.push(v.doc_evento_id)
  if (v.id_evento) idsDocumentos.push(v.id_evento)

  return props.documentos.filter((doc) => {
    if (doc.documento_id && idsDocumentos.includes(doc.documento_id)) {
      return true
    }
    if (numerosBuscar.length === 0) return false
    const nombreArchivo = normalizarTexto(doc.original_name)
    return numerosBuscar.some((num) => nombreArchivo.includes(num))
  })
}

function obtenerDocIngresoId(v: VinculoLaboral): number | null {
  if (v.doc_ingreso_id) return v.doc_ingreso_id
  if (v.documento_id) return v.documento_id
  return null
}

function obtenerDocSalidaId(v: VinculoLaboral): number | null {
  if (v.doc_salida_id) return v.doc_salida_id
  return null
}

function obtenerDocEventoId(v: VinculoLaboral): number | null {
  if (v.doc_evento_id) return v.doc_evento_id
  if (v.id_evento) return v.id_evento
  return null
}

function obtenerRutaArchivo(archivo: ArchivoLegajo): string {
  if (archivo.external_url) {
    return archivo.external_url
  }
  return `${apiBaseUrl.replace(/\/$/, '')}/fileserver/${archivo.file_hash}`
}
</script>

<template>
  <div
    class="bg-white dark:bg-navy-800 rounded-xl p-3.5 sm:p-4 border border-slate-100 dark:border-navy-700/80 shadow-sm space-y-4">
    <div class="flex items-center justify-between pb-3 border-b border-slate-100 dark:border-navy-700/80">
      <div class="flex items-center space-x-2.5">
        <div class="p-1.5 rounded-lg bg-blue-50 dark:bg-blue-500/10 text-blue-600 dark:text-blue-400">
          <IconBriefcase class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold text-slate-800 dark:text-white">Vínculos y Trayectoria Laboral</h3>
          <p class="text-2xs text-slate-400">Historial contractual en la institución</p>
        </div>
      </div>

      <div class="flex items-center gap-1.5">
        <button
          v-if="documentos && documentos.length > 0"
          type="button"
          @click="emit('cambiarTab', 'legajo')"
          class="px-2 py-0.5 bg-amber-50 hover:bg-amber-100 dark:bg-amber-500/10 dark:hover:bg-amber-500/20 text-amber-700 dark:text-amber-400 text-2xs font-semibold rounded-md flex items-center gap-1 transition-colors cursor-pointer border border-amber-200/60 dark:border-amber-700/50">
          <IconFolder class="h-3 w-3" />
          <span>Legajo ({{ documentos.length }})</span>
        </button>

        <span
          class="px-2.5 py-1 bg-slate-100 dark:bg-navy-700 text-slate-600 dark:text-navy-200 text-2xs font-semibold rounded-md">
          Total: {{ vinculos.length }}
        </span>
      </div>
    </div>

    <div v-if="vinculos.length > 0" class="space-y-2.5">
      <div
        v-for="v in vinculos"
        :key="v.id"
        class="border border-slate-100 dark:border-navy-700/80 rounded-lg overflow-hidden bg-slate-50/50 dark:bg-navy-900/30 transition-all">
        <div
          class="p-3 flex items-center justify-between cursor-pointer hover:bg-slate-50 dark:hover:bg-navy-900/60 transition-colors select-none"
          @click="alternarVinculo(v.id)">
          <div class="flex items-center space-x-3 min-w-0">
            <div
              :class="[
                'w-2 h-2 rounded-full shrink-0',
                v.estado?.toLowerCase() === 'activo'
                  ? 'bg-emerald-500 shadow-xs shadow-emerald-500/50'
                  : 'bg-slate-400 dark:bg-navy-500',
              ]" />

            <div class="min-w-0 space-y-0.5">
              <div class="flex items-center space-x-2">
                <span class="text-2xs font-bold text-slate-800 dark:text-white">
                  {{ v.cargo }}
                </span>
                <span
                  :class="[
                    'px-1.5 py-0.2 rounded text-3xs font-bold uppercase tracking-wider',
                    v.estado?.toLowerCase() === 'activo'
                      ? 'bg-emerald-50 text-emerald-600 dark:bg-emerald-500/10 dark:text-emerald-400 border border-emerald-200/50 dark:border-emerald-800/40'
                      : 'bg-slate-100 text-slate-500 dark:bg-navy-700 dark:text-navy-300',
                  ]">
                  {{ v.estado }}
                </span>
              </div>

              <div class="flex flex-wrap items-center gap-x-3 gap-y-0.5 text-2xs text-slate-500 dark:text-slate-400">
                <span v-if="v.area" class="flex items-center space-x-1">
                  <IconBuildingSkyscraper class="h-3 w-3 text-slate-400" />
                  <span>{{ v.area }}</span>
                </span>
                <span v-if="v.regimen" class="font-medium text-slate-600 dark:text-slate-300">
                  {{ v.regimen }}
                </span>
                <span v-if="v.codigo" class="font-mono text-sm text-slate-400 dark:text-slate-500">
                  #{{ v.codigo }}
                </span>
              </div>
            </div>
          </div>

          <div class="flex items-center space-x-3 shrink-0 ml-2">
            <div class="text-right hidden sm:block">
              <div class="text-2xs font-semibold text-slate-700 dark:text-slate-200 flex items-center gap-1">
                <IconCalendar class="h-3 w-3 text-slate-400" />
                <span>{{ $formatearFecha(v.fecha_ingreso) }}</span>
                <span v-if="v.fecha_salida" class="text-slate-400"> - {{ $formatearFecha(v.fecha_salida) }}</span>
              </div>
              <div v-if="v.sueldo" class="text-2xs font-bold text-blue-600 dark:text-blue-400">
                {{ formatearMoneda(v.sueldo) }}
              </div>
            </div>

            <span class="p-1 rounded-md text-slate-400 hover:text-slate-600 dark:hover:text-slate-200">
              <component :is="vinculoExpandidoId === v.id ? IconChevronUp : IconChevronDown" class="h-4 w-4" />
            </span>
          </div>
        </div>

        <div
          v-if="vinculoExpandidoId === v.id"
          class="p-3.5 bg-white dark:bg-navy-800 border-t border-slate-100 dark:border-navy-700/60 space-y-3">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-2xs">
            <div class="space-y-1.5">
              <span class="text-slate-400 block uppercase font-medium text-xs"> Documento de Ingreso </span>
              <div class="flex flex-wrap items-center gap-2">
                <span class="font-semibold text-slate-800 dark:text-white text-[10px]">
                  {{ [v.doc_ingreso, v.numero_doc_ingreso].filter(Boolean).join(' N° ') || 'Sin documento de ingreso' }}
                </span>

                <button
                  v-if="obtenerDocIngresoId(v)"
                  type="button"
                  @click.stop="verDetalleDocumento(obtenerDocIngresoId(v)!, buscarDocumentosVinculo(v))"
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md bg-blue-50 hover:bg-blue-100 dark:bg-blue-500/10 dark:hover:bg-blue-500/20 text-blue-600 dark:text-blue-400 text-2xs font-semibold border border-blue-200/60 dark:border-blue-700/50 transition-colors cursor-pointer">
                  <IconFileText class="h-3 w-3" />
                  <span>Ver detalles</span>
                  <IconChevronRight class="h-3 w-3" />
                </button>
              </div>
              <p v-if="v.descrip_ingreso" class="text-slate-500 dark:text-navy-300 leading-normal pt-0.5">
                {{ v.descrip_ingreso }}
              </p>
            </div>

            <div
              class="grid grid-cols-2 gap-2 border-t sm:border-t-0 sm:border-l border-slate-100 dark:border-navy-700/60 sm:pl-3 pt-2 sm:pt-0">
              <div>
                <span class="text-slate-400 block uppercase font-medium text-3xs">Cargo Estructural</span>
                <p class="font-semibold text-slate-800 dark:text-white">
                  {{ v.cargo_estructural || 'No especificado' }}
                </p>
              </div>
              <div>
                <span class="text-slate-400 block uppercase font-medium text-3xs">Grupo Ocupacional</span>
                <p class="font-semibold text-slate-800 dark:text-white">
                  {{ v.grupo_ocupacional || 'No especificado' }}
                </p>
              </div>
            </div>
          </div>

          <div
            v-if="v.fecha_salida || v.doc_salida || v.numero_doc_salida || v.descrip_salida || obtenerDocSalidaId(v)"
            class="pt-2.5 border-t border-slate-100 dark:border-navy-700/60 text-2xs space-y-1.5">
            <div class="flex items-center justify-between">
              <span class="text-slate-400 uppercase font-medium text-3xs">Documento de Cese / Salida</span>
              <span v-if="v.fecha_salida" class="text-slate-400 font-mono"> Fecha: {{ v.fecha_salida }} </span>
            </div>
            <div class="flex flex-wrap items-center gap-2">
              <span class="font-semibold text-slate-800 dark:text-white text-xs">
                {{ [v.doc_salida, v.numero_doc_salida].filter(Boolean).join(' N° ') || 'Documento de Salida' }}
              </span>

              <button
                v-if="obtenerDocSalidaId(v)"
                type="button"
                @click.stop="verDetalleDocumento(obtenerDocSalidaId(v)!, buscarDocumentosVinculo(v))"
                class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md bg-rose-50 hover:bg-rose-100 dark:bg-rose-500/10 dark:hover:bg-rose-500/20 text-rose-600 dark:text-rose-400 text-2xs font-semibold border border-rose-200/60 dark:border-rose-700/50 transition-colors cursor-pointer">
                <IconFileText class="h-3 w-3" />
                <span>Ver detalles</span>
                <IconChevronRight class="h-3 w-3" />
              </button>
            </div>
            <p v-if="v.descrip_salida" class="text-slate-500 dark:text-navy-300 leading-normal">
              {{ v.descrip_salida }}
            </p>
          </div>

          <div
            v-if="v.tipo_evento || v.id_evento || v.doc_evento_tipo || v.numero_doc_evento || obtenerDocEventoId(v)"
            class="pt-2.5 border-t border-slate-100 dark:border-navy-700/60 text-2xs flex flex-wrap items-center justify-between gap-2">
            <div>
              <span class="text-slate-400 block uppercase font-medium text-3xs">Movimiento Laboral</span>
              <p class="font-semibold text-slate-800 dark:text-white">
                {{ v.tipo_evento || 'Asignación Regular' }}
              </p>
            </div>

            <div v-if="v.doc_evento_tipo || v.numero_doc_evento || obtenerDocEventoId(v)" class="text-right">
              <span class="text-slate-400 block uppercase font-medium text-3xs">Documento</span>
              <div class="flex items-center gap-2">
                <span class="font-medium text-slate-700 dark:text-slate-200">
                  {{ [v.doc_evento_tipo, v.numero_doc_evento].filter(Boolean).join(' N° ') }}
                </span>

                <button
                  v-if="obtenerDocEventoId(v)"
                  type="button"
                  @click.stop="verDetalleDocumento(obtenerDocEventoId(v)!, buscarDocumentosVinculo(v))"
                  class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md bg-indigo-50 hover:bg-indigo-100 dark:bg-indigo-500/10 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 text-2xs font-semibold border border-indigo-200/60 dark:border-indigo-700/50 transition-colors cursor-pointer">
                  <span>Ver detalles</span>
                  <IconChevronRight class="h-3 w-3" />
                </button>
              </div>
            </div>
          </div>

          <div
            v-if="buscarDocumentosVinculo(v).length > 0"
            class="pt-2.5 border-t border-slate-100 dark:border-navy-700/60 space-y-1.5">
            <span class="text-2xs font-semibold text-slate-500 dark:text-navy-300 block">
              Archivos del Legajo Asociados ({{ buscarDocumentosVinculo(v).length }})
            </span>
            <div class="flex flex-wrap gap-1.5">
              <div
                v-for="docLegajo in buscarDocumentosVinculo(v)"
                :key="docLegajo.id"
                class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md bg-slate-50 dark:bg-navy-900 border border-slate-200/80 dark:border-navy-700 text-2xs">
                <span class="font-medium text-slate-700 dark:text-slate-200">
                  {{ docLegajo.original_name }}
                </span>

                <button
                  v-if="docLegajo.documento_id"
                  type="button"
                  @click.stop="verDetalleDocumento(docLegajo.documento_id, [docLegajo])"
                  class="text-blue-600 dark:text-blue-400 hover:underline font-semibold text-2xs cursor-pointer"
                  title="Ver detalle del documento">
                  Detalles
                </button>

                <a
                  :href="obtenerRutaArchivo(docLegajo)"
                  target="_blank"
                  rel="noopener noreferrer"
                  @click.stop
                  class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 p-0.5 transition-colors"
                  title="Abrir archivo original">
                  <component :is="docLegajo.external_url ? IconExternalLink : IconDownload" class="h-4 w-4" />
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div
      v-else
      class="p-4 text-center text-slate-400 text-xs font-medium bg-slate-50/50 dark:bg-navy-900/30 rounded-lg">
      No se registran vínculos laborales para esta persona.
    </div>
  </div>
</template>
