<script setup lang="ts">
import { ref, computed } from 'vue'
import { useDetalleDocumentoStore } from '@/stores/detalleDocumento'
import type { ArchivoLegajo } from '@/api/perfil'
import {
  IconFolder,
  IconFileText,
  IconFileTypePdf,
  IconDownload,
  IconExternalLink,
  IconSearch,
  IconLayoutGrid,
  IconList,
  IconLoader2,
  IconInfoCircle,
} from '@tabler/icons-vue'

const props = withDefaults(
  defineProps<{
    documentos?: ArchivoLegajo[]
    cargando?: boolean
  }>(),
  {
    documentos: () => [],
    cargando: false,
  },
)

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'

const vistaActiva = ref<'iconos' | 'detalles'>('iconos')
const busqueda = ref('')
const docSeleccionado = ref<ArchivoLegajo | null>(null)
const detalleDocumentoStore = useDetalleDocumentoStore()

const documentosFiltrados = computed(() => {
  const lista = props.documentos || []
  if (!busqueda.value.trim()) return lista
  const query = busqueda.value.toLowerCase().trim()
  return lista.filter(
    (doc) =>
      doc.original_name.toLowerCase().includes(query) ||
      (doc.usuario_subida && doc.usuario_subida.toLowerCase().includes(query)),
  )
})

function obtenerRutaArchivo(archivo: ArchivoLegajo): string {
  if (archivo.external_url) {
    return archivo.external_url
  }
  return `${apiBaseUrl.replace(/\/$/, '')}/fileserver/${archivo.file_hash}`
}

function seleccionarArchivo(archivo: ArchivoLegajo) {
  docSeleccionado.value = archivo
}

function abrirArchivo(archivo: ArchivoLegajo) {
  const url = obtenerRutaArchivo(archivo)
  window.open(url, '_blank', 'noopener,noreferrer')
}

function verDetalleDocumento(archivo: ArchivoLegajo) {
  if (!archivo.documento_id) return
  detalleDocumentoStore.abrir(archivo.documento_id, [archivo])
}
</script>

<template>
  <div
    class="bg-white dark:bg-navy-800 rounded-xl p-3.5 sm:p-4 border border-slate-100 dark:border-navy-700/80 shadow-sm space-y-4">
    <div class="flex items-center justify-between pb-3 border-b border-slate-100 dark:border-navy-700/80">
      <div class="flex items-center space-x-2.5">
        <div class="p-1.5 rounded-lg bg-amber-50 dark:bg-amber-500/10 text-amber-600 dark:text-amber-400">
          <IconFolder class="h-4 w-4" />
        </div>
        <div>
          <h3 class="text-xs font-bold text-slate-800 dark:text-white">Legajo y Documentos Digitales</h3>
          <p class="text-2xs text-slate-400">Resoluciones, contratos y archivos escaneados</p>
        </div>
      </div>

      <span
        class="px-2.5 py-1 bg-slate-100 dark:bg-navy-700 text-slate-600 dark:text-navy-200 text-2xs font-semibold rounded-md">
        Archivos: {{ (documentos || []).length }}
      </span>
    </div>

    <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2.5">
      <div class="relative flex-1">
        <input
          v-model="busqueda"
          type="text"
          placeholder="Buscar documento por nombre o usuario..."
          class="w-full pl-8 pr-3 py-1.5 bg-slate-50 dark:bg-navy-900 border border-slate-200 dark:border-navy-700 rounded-lg text-xs text-slate-800 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all" />
        <IconSearch class="h-4 w-4 text-slate-400 absolute left-2.5 top-1/2 -translate-y-1/2" />
      </div>

      <div class="flex items-center justify-end gap-1 bg-slate-100 dark:bg-navy-900 p-1 rounded-lg shrink-0">
        <button
          type="button"
          @click="vistaActiva = 'iconos'"
          :class="[
            'flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium transition-all cursor-pointer select-none',
            vistaActiva === 'iconos'
              ? 'bg-white dark:bg-navy-800 text-blue-600 dark:text-blue-400 shadow-xs font-semibold'
              : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200',
          ]">
          <IconLayoutGrid class="h-3.5 w-3.5" />
          <span>Íconos</span>
        </button>

        <button
          type="button"
          @click="vistaActiva = 'detalles'"
          :class="[
            'flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium transition-all cursor-pointer select-none',
            vistaActiva === 'detalles'
              ? 'bg-white dark:bg-navy-800 text-blue-600 dark:text-blue-400 shadow-xs font-semibold'
              : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200',
          ]">
          <IconList class="h-3.5 w-3.5" />
          <span>Detalles</span>
        </button>
      </div>
    </div>

    <div v-if="cargando" class="flex items-center justify-center gap-2 py-12 text-xs text-slate-500 dark:text-navy-300">
      <IconLoader2 class="h-4 w-4 animate-spin text-amber-500" />
      <span>Cargando documentos del legajo digital...</span>
    </div>

    <div
      v-else-if="documentosFiltrados.length === 0"
      class="p-8 text-center text-slate-400 text-xs font-medium bg-slate-50/50 dark:bg-navy-900/30 rounded-lg border border-dashed border-slate-200 dark:border-navy-700">
      No se encontraron documentos en el legajo.
    </div>

    <div
      v-else-if="vistaActiva === 'iconos'"
      class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-2">
      <div
        v-for="doc in documentosFiltrados"
        :key="doc.id"
        @click="seleccionarArchivo(doc)"
        @dblclick="abrirArchivo(doc)"
        :class="[
          'group relative flex flex-col items-center p-2 rounded-lg border transition-all cursor-pointer select-none',
          docSeleccionado?.id === doc.id
            ? 'bg-blue-50/80 border-blue-500 dark:bg-blue-950/40 dark:border-blue-500 shadow-2xs'
            : 'bg-slate-50/50 border-slate-100 hover:bg-slate-50 hover:border-slate-300 dark:bg-navy-900/40 dark:border-navy-700/60 dark:hover:bg-navy-900 dark:hover:border-navy-600',
        ]">
        <div class="relative my-1">
          <div
            class="w-9 h-11 bg-white dark:bg-navy-800 rounded border border-slate-200 dark:border-navy-700 shadow-2xs flex flex-col items-center justify-between p-1 group-hover:scale-105 transition-transform">
            <div class="w-full flex justify-between items-center border-b border-slate-100 dark:border-navy-700 pb-0.5">
              <span class="text-[8px] font-black uppercase text-red-600 dark:text-red-400 tracking-tighter">PDF</span>
              <IconFileTypePdf class="h-2.5 w-2.5 text-red-500" />
            </div>
            <IconFileText class="h-3.5 w-3.5 text-slate-400 dark:text-slate-500 my-auto" />
          </div>
          <span
            v-if="doc.external_url"
            class="absolute -top-1 -right-1 flex h-3.5 w-3.5 items-center justify-center rounded-full bg-blue-600 text-white text-[8px] shadow-2xs"
            title="Enlace Externo">
            <IconExternalLink class="h-2 w-2" />
          </span>
        </div>

        <p
          class="text-2xs font-medium text-slate-800 dark:text-slate-200 text-center line-clamp-2 break-all w-full leading-tight my-0.5 group-hover:text-blue-600 dark:group-hover:text-blue-400">
          {{ doc.original_name }}
        </p>

        <span class="text-[9px] text-slate-400 dark:text-slate-500 font-mono">
          {{ doc.fecha_subida || 'Sin fecha' }}
        </span>

        <div class="mt-1.5 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            v-if="doc.documento_id"
            type="button"
            @click.stop="verDetalleDocumento(doc)"
            class="px-1.5 py-0.5 bg-slate-200 hover:bg-slate-300 dark:bg-navy-700 dark:hover:bg-navy-600 text-slate-700 dark:text-slate-200 rounded text-[9px] font-semibold flex items-center gap-0.5 shadow-2xs cursor-pointer"
            title="Ver detalle del documento">
            <IconInfoCircle class="h-2.5 w-2.5" />
            <span>Detalle</span>
          </button>

          <a
            :href="obtenerRutaArchivo(doc)"
            target="_blank"
            rel="noopener noreferrer"
            @click.stop
            class="px-2 py-0.5 bg-blue-600 hover:bg-blue-700 text-white rounded text-[9px] font-semibold flex items-center gap-0.5 shadow-2xs">
            <component :is="doc.external_url ? IconExternalLink : IconDownload" class="h-2.5 w-2.5" />
            <span>{{ doc.external_url ? 'Abrir' : 'Ver PDF' }}</span>
          </a>
        </div>
      </div>
    </div>

    <div v-else class="overflow-x-auto">
      <table class="w-full text-left border-collapse text-xs">
        <thead>
          <tr
            class="border-b border-slate-100 dark:border-navy-700 text-2xs uppercase tracking-wider text-slate-400 font-semibold select-none">
            <th class="py-2 px-3">Nombre</th>
            <th class="py-2 px-3">Tipo</th>
            <th class="py-2 px-3">Fecha de Subida</th>
            <th class="py-2 px-3">Subido por</th>
            <th class="py-2 px-3 text-right">Acciones</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100 dark:divide-navy-700/50">
          <tr
            v-for="doc in documentosFiltrados"
            :key="doc.id"
            @click="seleccionarArchivo(doc)"
            @dblclick="abrirArchivo(doc)"
            :class="[
              'group transition-colors cursor-pointer select-none',
              docSeleccionado?.id === doc.id
                ? 'bg-blue-50/80 dark:bg-blue-950/40'
                : 'hover:bg-slate-50/70 dark:hover:bg-navy-900/40',
            ]">
            <td class="py-2.5 px-3">
              <div class="flex items-center gap-2.5 min-w-0">
                <div class="p-1 rounded bg-red-50 text-red-600 dark:bg-red-500/10 dark:text-red-400 shrink-0">
                  <IconFileTypePdf class="h-4 w-4" />
                </div>
                <span
                  class="font-medium text-slate-800 dark:text-white truncate group-hover:text-blue-600 dark:group-hover:text-blue-400">
                  {{ doc.original_name }}
                </span>
              </div>
            </td>
            <td class="py-2.5 px-3 text-slate-500 dark:text-slate-400 font-mono text-2xs whitespace-nowrap">
              {{ doc.external_url ? 'Enlace Externo' : 'Documento PDF' }}
            </td>
            <td class="py-2.5 px-3 text-slate-500 dark:text-slate-400 font-mono text-2xs whitespace-nowrap">
              {{ doc.fecha_subida || '-' }}
            </td>
            <td class="py-2.5 px-3 text-slate-500 dark:text-slate-400 text-2xs whitespace-nowrap">
              {{ doc.usuario_subida || 'Sistema' }}
            </td>
            <td class="py-2.5 px-3 text-right whitespace-nowrap">
              <div class="inline-flex items-center justify-end gap-1.5">
                <button
                  v-if="doc.documento_id"
                  type="button"
                  @click.stop="verDetalleDocumento(doc)"
                  class="inline-flex items-center gap-1 px-2 py-1 bg-slate-100 hover:bg-slate-200 dark:bg-navy-700 dark:hover:bg-navy-600 text-slate-700 dark:text-slate-200 rounded-md text-2xs font-semibold transition-colors cursor-pointer">
                  <IconInfoCircle class="h-3 w-3 text-blue-500" />
                  <span>Detalles</span>
                </button>

                <a
                  :href="obtenerRutaArchivo(doc)"
                  target="_blank"
                  rel="noopener noreferrer"
                  @click.stop
                  class="inline-flex items-center gap-1 px-2.5 py-1 bg-slate-100 hover:bg-blue-600 hover:text-white dark:bg-navy-700 dark:hover:bg-blue-600 text-slate-700 dark:text-white rounded-md text-2xs font-semibold transition-colors shadow-2xs">
                  <component :is="doc.external_url ? IconExternalLink : IconDownload" class="h-3 w-3" />
                  <span>{{ doc.external_url ? 'Abrir' : 'Ver PDF' }}</span>
                </a>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div
      v-if="docSeleccionado"
      class="p-2.5 bg-blue-50/60 dark:bg-blue-950/30 rounded-lg border border-blue-100 dark:border-blue-900/40 flex items-center justify-between text-2xs text-blue-700 dark:text-blue-300">
      <span class="font-medium truncate">Seleccionado: {{ docSeleccionado.original_name }}</span>
      <div class="flex items-center gap-2.5 shrink-0 ml-2">
        <button
          v-if="docSeleccionado.documento_id"
          type="button"
          @click.stop="verDetalleDocumento(docSeleccionado)"
          class="font-bold underline hover:text-blue-900 dark:hover:text-white cursor-pointer">
          Ver detalles del documento
        </button>
        <a
          :href="obtenerRutaArchivo(docSeleccionado)"
          target="_blank"
          rel="noopener noreferrer"
          class="font-bold underline hover:text-blue-900 dark:hover:text-white">
          Abrir archivo
        </a>
      </div>
    </div>
  </div>
</template>
