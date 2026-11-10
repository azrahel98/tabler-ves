<script setup lang="ts">
import { ref, computed } from 'vue'
import { format } from 'date-fns'
import { es } from 'date-fns/locale'
import Badge from '@/components/ui/badge/Badge.vue'
import FilterDropdown from '@/components/ui/form/FilterDropdown.vue'
import { type PersonalArchivo, type PersonalDocumento } from '@/services/personal'
import { getApiBaseUrl } from '@/services/api'
import { parseDateSafe } from '@/utils/date'
import {
  IconLayoutGrid,
  IconMenu2,
  IconEye,
  IconExternalLink,
  IconSearch,
  IconFileText,
  IconFileTypePdf,
  IconLink,
  IconUpload,
  IconTrash,
} from '@tabler/icons-vue'

interface Props {
  archivos: PersonalArchivo[]
  documentos: PersonalDocumento[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'nuevoDocumento'): void
  (e: 'vincularUrl', doc: PersonalDocumento): void
  (e: 'subirArchivo'): void
  (e: 'eliminarArchivo', archivo: PersonalArchivo): void
  (e: 'abrirVisor', archivo: PersonalArchivo): void
}>()

const viewMode = ref<'list' | 'grid'>('list')
const searchQuery = ref('')
const filtroTipo = ref('todos')
const filtroFecha = ref('todas')
const filtroVinculo = ref('todos')
const selectedSort = ref('newest')

const tipoOptions = [
  { value: 'todos', label: 'Tipo' },
  { value: 'pdf', label: 'Archivo PDF' },
  { value: 'enlace', label: 'Enlace externo' },
]
const fechaOptions = [
  { value: 'todas', label: 'Fecha' },
  { value: '30d', label: 'Últimos 30 días' },
  { value: 'anio', label: 'Este año' },
  { value: 'antes', label: 'Años anteriores' },
]
const vinculoOptions = [
  { value: 'todos', label: 'Todos los archivos' },
  { value: 'con_doc', label: 'Con documento formal' },
  { value: 'sin_doc', label: 'Sin documento formal' },
]
const sortOptions = [
  { value: 'newest', label: 'Más recientes' },
  { value: 'oldest', label: 'Más antiguos' },
  { value: 'name_asc', label: 'Nombre (A-Z)' },
  { value: 'name_desc', label: 'Nombre (Z-A)' },
  { value: 'size', label: 'Tamaño' },
]

function parseSizeToKb(sizeStr?: string): number {
  if (!sizeStr) return 0
  const clean = sizeStr.toLowerCase().trim()
  const num = parseFloat(clean)
  if (isNaN(num)) return 0
  if (clean.includes('mb')) return num * 1024
  if (clean.includes('gb')) return num * 1024 * 1024
  return num
}

function formatFileDate(val: string | null | undefined): string {
  const d = parseDateSafe(val)
  if (!d) return ''
  return format(d, "d 'de' MMM yyyy", { locale: es })
}

function subtitulo(a: PersonalArchivo): string {
  return [a.usuario_subida, formatFileDate(a.fecha_subida), a.external_url ? null : a.size]
    .filter(Boolean)
    .join(' · ')
}

function handleOpenFile(archivo: PersonalArchivo) {
  emit('abrirVisor', archivo)
}

function handleAbrirArchivo(archivo: PersonalArchivo) {
  if (archivo.external_url) {
    const baseUrl = getApiBaseUrl()
    if (!archivo.external_url.startsWith(baseUrl) && !archivo.external_url.startsWith('/')) {
      window.open(archivo.external_url, '_blank', 'noopener,noreferrer')
      return
    }
  }
  handleOpenFile(archivo)
}

const filteredAndSortedArchivos = computed(() => {
  const now = Date.now()
  const currentYear = new Date().getFullYear()
  const q = searchQuery.value.toLowerCase().trim()

  const result = props.archivos.filter((a) => {
    if (q && !a.original_name.toLowerCase().includes(q)) return false
    if (filtroTipo.value === 'pdf' && a.external_url) return false
    if (filtroTipo.value === 'enlace' && !a.external_url) return false
    if (filtroVinculo.value === 'con_doc' && !a.documento_id) return false
    if (filtroVinculo.value === 'sin_doc' && a.documento_id) return false
    if (filtroFecha.value !== 'todas') {
      const d = parseDateSafe(a.fecha_subida)
      if (!d) return false
      if (filtroFecha.value === '30d' && now - d.getTime() > 30 * 86400000) return false
      if (filtroFecha.value === 'anio' && d.getFullYear() !== currentYear) return false
      if (filtroFecha.value === 'antes' && d.getFullYear() >= currentYear) return false
    }
    return true
  })

  const time = (a: PersonalArchivo) => parseDateSafe(a.fecha_subida)?.getTime() || 0
  result.sort((a, b) => {
    switch (selectedSort.value) {
      case 'oldest': return time(a) - time(b)
      case 'name_asc': return a.original_name.localeCompare(b.original_name)
      case 'name_desc': return b.original_name.localeCompare(a.original_name)
      case 'size': return parseSizeToKb(b.size) - parseSizeToKb(a.size)
      default: return time(b) - time(a)
    }
  })
  return result
})

const total = computed(() => filteredAndSortedArchivos.value.length)
</script>

<template>
  <div class="space-y-6">
    <div class="rounded-xl border border-border bg-card p-3 sm:p-4">
      <div class="flex flex-wrap items-center justify-between gap-2">
        <h3 class="text-meta font-semibold text-foreground tracking-tight">
          {{ total }} {{ total === 1 ? 'resultado' : 'resultados' }}
        </h3>

        <div class="flex items-center gap-2">
          <button type="button"
            class="inline-flex items-center gap-1.5 px-3 h-control-sm rounded-md bg-primary text-primary-foreground text-micro font-semibold hover:bg-primary-hover transition cursor-pointer shrink-0"
            @click="emit('subirArchivo')">
            <IconUpload class="size-3.5" />
            Subir archivo
          </button>

          <div class="inline-flex items-center rounded-lg border border-border p-0.5 bg-card" role="group"
            aria-label="Tipo de vista">
            <button type="button" class="p-1.5 rounded-md transition cursor-pointer"
              :class="viewMode === 'grid' ? 'bg-muted text-foreground' : 'text-muted-foreground hover:text-foreground'"
              title="Vista en cuadrícula" aria-label="Vista en cuadrícula" :aria-pressed="viewMode === 'grid'"
              @click="viewMode = 'grid'">
              <IconLayoutGrid class="size-4" />
            </button>
            <button type="button" class="p-1.5 rounded-md transition cursor-pointer"
              :class="viewMode === 'list' ? 'bg-muted text-foreground' : 'text-muted-foreground hover:text-foreground'"
              title="Vista en lista" aria-label="Vista en lista" :aria-pressed="viewMode === 'list'"
              @click="viewMode = 'list'">
              <IconMenu2 class="size-4" />
            </button>
          </div>
        </div>
      </div>

      <div class="mt-3 pt-3 border-t border-border flex flex-wrap items-center justify-between gap-3">
        <label class="relative flex-1 min-w-48 max-w-lg">
          <IconSearch class="size-3.5 absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <input v-model="searchQuery" type="search" placeholder="Buscar por nombre de archivo"
            class="w-full ps-8 pe-3 h-control-sm py-0 rounded-md bg-muted border-transparent text-micro text-foreground placeholder:text-muted-foreground focus:bg-card focus:border-primary focus:ring-primary" />
        </label>

        <div class="flex flex-wrap items-center gap-1">
          <FilterDropdown v-model="filtroTipo" :options="tipoOptions" aria-label="Filtrar por tipo" />
          <FilterDropdown v-model="filtroFecha" :options="fechaOptions" aria-label="Filtrar por fecha" />
          <FilterDropdown v-model="filtroVinculo" :options="vinculoOptions" aria-label="Filtrar por vínculo" />
          <FilterDropdown v-model="selectedSort" :options="sortOptions" prefix="Orden:" aria-label="Ordenar" />
        </div>
      </div>

      <div class="mt-3 pt-3 border-t border-border">
        <template v-if="total > 0">
          <ul v-if="viewMode === 'list'" class="space-y-2">
            <li v-for="archivo in filteredAndSortedArchivos" :key="archivo.id"
              class="group flex items-center gap-3 rounded-lg border border-border bg-card px-2.5 py-2 hover:border-primary/40 transition">
              <div class="size-8 rounded-md border border-border bg-card flex items-center justify-center shrink-0"
                :class="archivo.external_url ? 'text-primary' : 'text-rose-500'">
                <IconLink v-if="archivo.external_url" class="size-5" />
                <IconFileTypePdf v-else class="size-5" />
              </div>

              <div class="min-w-0 flex-1">
                <button type="button"
                  class="block max-w-full text-left text-micro font-semibold text-foreground hover:text-primary truncate transition cursor-pointer"
                  :title="archivo.original_name" @click="handleAbrirArchivo(archivo)">
                  {{ archivo.original_name }}
                </button>
                <p class="text-micro text-muted-foreground truncate">{{ subtitulo(archivo) }}</p>
              </div>

              <Badge v-if="archivo.documento_id" variant="outline" size="xs" class="hidden sm:inline-flex shrink-0">
                Documento formal
              </Badge>

              <div class="flex items-center gap-1 shrink-0 opacity-100 sm:opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition">
                <button type="button"
                  class="size-8 inline-flex items-center justify-center rounded-lg border border-border text-muted-foreground hover:text-primary hover:bg-muted transition cursor-pointer"
                  :title="archivo.external_url ? 'Abrir enlace externo' : 'Visualizar documento'"
                  :aria-label="archivo.external_url ? 'Abrir enlace externo' : 'Visualizar documento'"
                  @click="handleAbrirArchivo(archivo)">
                  <IconExternalLink v-if="archivo.external_url" class="size-4" />
                  <IconEye v-else class="size-4" />
                </button>
                <button type="button"
                  class="size-8 inline-flex items-center justify-center rounded-lg border border-border text-rose-500 hover:bg-rose-500/10 transition cursor-pointer"
                  title="Eliminar archivo del legajo" aria-label="Eliminar archivo del legajo"
                  @click="emit('eliminarArchivo', archivo)">
                  <IconTrash class="size-4" />
                </button>
              </div>
            </li>
          </ul>

          <ul v-else class="grid grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-3">
            <li v-for="archivo in filteredAndSortedArchivos" :key="archivo.id"
              class="group relative rounded-xl border border-border bg-card overflow-hidden hover:border-primary/40 transition">
              <button type="button"
                class="w-full h-24 sm:h-28 flex items-center justify-center bg-muted/40 cursor-pointer"
                :aria-label="`Abrir ${archivo.original_name}`" @click="handleAbrirArchivo(archivo)">
                <IconLink v-if="archivo.external_url" class="size-12 text-primary/50" :stroke-width="1.25" />
                <IconFileTypePdf v-else class="size-12 text-rose-400/70" :stroke-width="1.25" />
              </button>

              <div class="absolute top-2.5 right-2.5 flex items-center rounded-xl bg-card border border-border shadow-md p-1 opacity-100 sm:opacity-0 group-hover:opacity-100 focus-within:opacity-100 transition">
                <button type="button"
                  class="size-8 inline-flex items-center justify-center rounded-lg text-muted-foreground hover:text-primary hover:bg-muted transition cursor-pointer"
                  :title="archivo.external_url ? 'Abrir enlace externo' : 'Visualizar documento'"
                  :aria-label="archivo.external_url ? 'Abrir enlace externo' : 'Visualizar documento'"
                  @click="handleAbrirArchivo(archivo)">
                  <IconExternalLink v-if="archivo.external_url" class="size-4" />
                  <IconEye v-else class="size-4" />
                </button>
                <span class="w-px h-5 bg-border mx-0.5"></span>
                <button type="button"
                  class="size-8 inline-flex items-center justify-center rounded-lg text-rose-500 hover:bg-rose-500/10 transition cursor-pointer"
                  title="Eliminar archivo del legajo" aria-label="Eliminar archivo del legajo"
                  @click="emit('eliminarArchivo', archivo)">
                  <IconTrash class="size-4" />
                </button>
              </div>

              <div class="flex items-center gap-3 p-3 border-t border-border">
                <div class="size-8 rounded-md border border-border bg-card flex items-center justify-center shrink-0"
                  :class="archivo.external_url ? 'text-primary' : 'text-rose-500'">
                  <IconLink v-if="archivo.external_url" class="size-5" />
                  <IconFileTypePdf v-else class="size-5" />
                </div>
                <div class="min-w-0">
                  <p class="text-micro font-semibold text-foreground truncate" :title="archivo.original_name">
                    {{ archivo.original_name }}
                  </p>
                  <p class="text-micro text-muted-foreground truncate">{{ subtitulo(archivo) }}</p>
                </div>
              </div>
            </li>
          </ul>
        </template>

        <div v-else class="text-meta text-muted-foreground py-10 text-center space-y-2">
          <IconFileText class="size-8 mx-auto text-muted-foreground/40" />
          <p class="font-semibold text-foreground">No se encontraron archivos</p>
          <p>
            {{ archivos.length === 0
              ? 'Este legajo aún no tiene archivos digitales. Usa "Subir archivo" para agregar el primero.'
              : 'No existen archivos que coincidan con los filtros o la búsqueda seleccionados.' }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
