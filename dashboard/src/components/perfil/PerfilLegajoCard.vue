<script setup lang="ts">
import { ref, computed } from 'vue'
import { format } from 'date-fns'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { getFileDownloadUrl, type PersonalArchivo, type PersonalDocumento } from '@/services/personal'
import { parseDateSafe, formatDate } from '@/utils/date'
import {
  IconChevronDown,
  IconLayoutGrid,
  IconList,
  IconDownload,
  IconExternalLink,
  IconSearch,
  IconUser,
  IconFileCheck,
  IconFileText,
} from '@tabler/icons-vue'

interface Props {
  archivos: PersonalArchivo[]
  documentos: PersonalDocumento[]
}

const props = defineProps<Props>()

const viewMode = ref<'list' | 'grid'>('list')
const selectedSort = ref<string>('newest')
const searchQuery = ref<string>('')

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
  if (!d) return '-'
  return format(d, 'MMM d, yyyy')
}

function getAccessDetails(archivo: PersonalArchivo) {
  const access = archivo.access || 'Todos'
  const isOnlyYou = access.toLowerCase().includes('solo tú') || access.toLowerCase().includes('only you')

  if (isOnlyYou) {
    return {
      isOnlyYou: true,
      label: access,
      initial: 'U',
    }
  }

  const match = access.match(/\d+/)
  const membersCount = archivo.access_members || (match ? Number(match[0]) : null)
  let initial = 'M'
  if (membersCount === 2) initial = 'R'
  else if (membersCount === 4) initial = 'A'
  else if (membersCount === 9) initial = 'D'
  else if (archivo.usuario_subida) initial = archivo.usuario_subida.charAt(0).toUpperCase()

  return {
    isOnlyYou: false,
    label: access,
    initial,
  }
}

function getFileUrl(archivo: PersonalArchivo): string {
  return archivo.external_url || getFileDownloadUrl(archivo.file_hash)
}

const filteredAndSortedArchivos = computed(() => {
  let result = [...props.archivos]

  if (searchQuery.value.trim()) {
    const q = searchQuery.value.toLowerCase().trim()
    result = result.filter(a => a.original_name.toLowerCase().includes(q))
  }

  result.sort((a, b) => {
    if (selectedSort.value === 'newest') {
      const dateA = parseDateSafe(a.fecha_subida)?.getTime() || 0
      const dateB = parseDateSafe(b.fecha_subida)?.getTime() || 0
      return dateB - dateA
    }
    if (selectedSort.value === 'oldest') {
      const dateA = parseDateSafe(a.fecha_subida)?.getTime() || 0
      const dateB = parseDateSafe(b.fecha_subida)?.getTime() || 0
      return dateA - dateB
    }
    if (selectedSort.value === 'name_asc') {
      return a.original_name.localeCompare(b.original_name)
    }
    if (selectedSort.value === 'name_desc') {
      return b.original_name.localeCompare(a.original_name)
    }
    if (selectedSort.value === 'size') {
      return parseSizeToKb(b.size) - parseSizeToKb(a.size)
    }
    return 0
  })

  return result
})
</script>

<template>
  <div class="space-y-6">
    <Card class="space-y-3">
      <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-3 border-b border-border pb-3">
        <div class="flex items-center gap-2">
          <IconFileText class="size-3.5 text-primary shrink-0" />
          <h3 class="text-sm font-semibold tracking-tight text-foreground">Archivos Digitales del Legajo</h3>
          <span
            class="px-2 py-0.5 rounded bg-rose-500/10 text-rose-600 dark:text-rose-400 text-[11px] font-mono font-medium shrink-0">
            {{ filteredAndSortedArchivos.length }} PDF{{ filteredAndSortedArchivos.length === 1 ? '' : 's' }}
          </span>
        </div>

        <div class="flex flex-wrap items-center gap-2.5">
          <div class="relative">
            <IconSearch class="size-3.5 text-muted-foreground absolute left-2.5 top-1/2 -translate-y-1/2" />
            <input v-model="searchQuery" type="text" placeholder="Buscar archivo PDF..."
              class="h-7.5 pl-7 pr-2.5 text-[11px] rounded-lg border border-border bg-background/50 focus:bg-background focus:outline-hidden focus:ring-1 focus:ring-primary w-36 sm:w-44 transition" />
          </div>

          <div class="relative inline-flex items-center border border-border rounded-lg px-2 h-7.5 bg-card text-[11px]">
            <span class="text-muted-foreground mr-1 text-[11px]">Ordenar:</span>
            <div class="relative inline-flex items-center">
              <select v-model="selectedSort"
                class="appearance-none bg-transparent pr-3.5 font-medium text-foreground cursor-pointer focus:outline-hidden text-[11px]">
                <option v-for="opt in sortOptions" :key="opt.value" :value="opt.value" class="bg-card text-foreground">
                  {{ opt.label }}
                </option>
              </select>
              <IconChevronDown class="size-3 text-muted-foreground absolute right-0 pointer-events-none" />
            </div>
          </div>

          <div class="inline-flex items-center rounded-lg border border-border p-0.5 bg-muted/20">
            <button type="button" class="p-1 rounded-md transition cursor-pointer"
              :class="viewMode === 'list' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'"
              title="Vista en lista" @click="viewMode = 'list'">
              <IconList class="size-3.5" />
            </button>
            <button type="button" class="p-1 rounded-md transition cursor-pointer"
              :class="viewMode === 'grid' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'"
              title="Vista en cuadrícula" @click="viewMode = 'grid'">
              <IconLayoutGrid class="size-3.5" />
            </button>
          </div>
        </div>
      </div>

      <div v-if="filteredAndSortedArchivos.length > 0">
        <div v-if="viewMode === 'list'" class="overflow-hidden rounded-xl border border-border bg-card">
          <div class="overflow-x-auto">
            <table class="w-full text-left text-xs" aria-label="Tabla de archivos digitales">
              <thead
                class="border-b border-border bg-muted/30 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground select-none">
                <tr>
                  <th scope="col" class="px-3 sm:px-4 py-2 font-semibold w-[48%]">Nombre</th>
                  <th scope="col" class="px-3 sm:px-4 py-2 font-semibold w-[14%]">Tamaño</th>
                  <th scope="col" class="px-3 sm:px-4 py-2 font-semibold w-[22%]">Acceso</th>
                  <th scope="col" class="px-3 sm:px-4 py-2 font-semibold w-[16%] text-right sm:text-left">Modificado
                  </th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border text-xs">
                <tr v-for="archivo in filteredAndSortedArchivos" :key="archivo.id"
                  class="group hover:bg-muted/40 transition-colors">
                  <td class="px-3 sm:px-4 py-2 min-w-0">
                    <div class="flex items-center gap-2.5">
                      <div class="shrink-0 flex items-center justify-center">
                        <svg class="size-6 shrink-0 drop-shadow-2xs" viewBox="0 0 24 24" fill="none"
                          xmlns="http://www.w3.org/2000/svg">
                          <path
                            d="M6 2C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2H6Z"
                            class="fill-rose-50 dark:fill-rose-950/60 stroke-rose-400 dark:stroke-rose-500"
                            stroke-width="1.5" stroke-linejoin="round" />
                          <path d="M14 2V8H20" class="stroke-rose-400 dark:stroke-rose-500" stroke-width="1.5"
                            stroke-linejoin="round" />
                          <rect x="6.5" y="12" width="11" height="6.5" rx="1.5"
                            class="fill-rose-600 dark:fill-rose-500" />
                          <text x="12" y="16.7" fill="white" font-size="4.2" font-weight="bold"
                            font-family="system-ui, -apple-system, sans-serif" text-anchor="middle"
                            letter-spacing="0.3">PDF</text>
                        </svg>
                      </div>
                      <a :href="getFileUrl(archivo)" target="_blank" rel="noopener noreferrer"
                        class="font-medium text-foreground hover:text-primary transition-colors truncate max-w-sm sm:max-w-md block text-[11px]"
                        :title="archivo.original_name">
                        {{ archivo.original_name }}
                      </a>
                    </div>
                  </td>

                  <td class="px-3 sm:px-4 py-2 text-muted-foreground font-mono text-[11px] whitespace-nowrap">
                    {{ archivo.size || '256 KB' }}
                  </td>

                  <td class="px-3 sm:px-4 py-2 whitespace-nowrap">
                    <div class="flex items-center gap-1.5">
                      <div v-if="getAccessDetails(archivo).isOnlyYou"
                        class="size-4 rounded-full bg-slate-200 dark:bg-slate-700 text-slate-600 dark:text-slate-300 flex items-center justify-center shrink-0">
                        <IconUser class="size-2.5" />
                      </div>
                      <div v-else
                        class="size-4 rounded-full bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-200 text-[9px] font-bold flex items-center justify-center shrink-0">
                        {{ getAccessDetails(archivo).initial }}
                      </div>
                      <span class="text-foreground text-[11px] font-normal">
                        {{ getAccessDetails(archivo).label }}
                      </span>
                    </div>
                  </td>

                  <td class="px-3 sm:px-4 py-2 text-muted-foreground text-right sm:text-left whitespace-nowrap">
                    <div class="flex items-center justify-between gap-2">
                      <span class="font-mono text-[11px]">{{ formatFileDate(archivo.fecha_subida) }}</span>
                      <a :href="getFileUrl(archivo)" target="_blank" rel="noopener noreferrer"
                        class="opacity-0 group-hover:opacity-100 transition p-0.5 text-muted-foreground hover:text-primary rounded hover:bg-muted"
                        :title="archivo.external_url ? 'Abrir enlace externo' : 'Descargar archivo'">
                        <IconExternalLink v-if="archivo.external_url" class="size-3" />
                        <IconDownload v-else class="size-3" />
                      </a>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
          <div v-for="archivo in filteredAndSortedArchivos" :key="archivo.id"
            class="group rounded-xl border border-border bg-card p-3 hover:shadow-xs hover:border-primary/40 transition-all flex flex-col justify-between space-y-2.5">
            <div class="flex items-start justify-between gap-2">
              <div class="shrink-0 flex items-center justify-center">
                <svg class="size-8.5 shrink-0 drop-shadow-2xs" viewBox="0 0 24 24" fill="none"
                  xmlns="http://www.w3.org/2000/svg">
                  <path
                    d="M6 2C4.89543 2 4 2.89543 4 4V20C4 21.1046 4.89543 22 6 22H18C19.1046 22 20 21.1046 20 20V8L14 2H6Z"
                    class="fill-rose-50 dark:fill-rose-950/60 stroke-rose-400 dark:stroke-rose-500" stroke-width="1.5"
                    stroke-linejoin="round" />
                  <path d="M14 2V8H20" class="stroke-rose-400 dark:stroke-rose-500" stroke-width="1.5"
                    stroke-linejoin="round" />
                  <rect x="6.5" y="12" width="11" height="6.5" rx="1.5" class="fill-rose-600 dark:fill-rose-500" />
                  <text x="12" y="16.7" fill="white" font-size="4.2" font-weight="bold"
                    font-family="system-ui, -apple-system, sans-serif" text-anchor="middle"
                    letter-spacing="0.3">PDF</text>
                </svg>
              </div>
              <a :href="getFileUrl(archivo)" target="_blank" rel="noopener noreferrer"
                class="p-1 rounded-md border border-border text-muted-foreground hover:text-primary hover:bg-muted transition"
                :title="archivo.external_url ? 'Abrir enlace' : 'Descargar archivo'">
                <IconExternalLink v-if="archivo.external_url" class="size-3" />
                <IconDownload v-else class="size-3" />
              </a>
            </div>

            <div class="space-y-0.5">
              <a :href="getFileUrl(archivo)" target="_blank" rel="noopener noreferrer"
                class="text-[11px] font-semibold text-foreground hover:text-primary transition line-clamp-2 block"
                :title="archivo.original_name">
                {{ archivo.original_name }}
              </a>
              <p class="text-[10px] text-muted-foreground font-mono">
                {{ archivo.size || '256 KB' }}
              </p>
            </div>

            <div
              class="pt-1.5 border-t border-border/50 flex items-center justify-between text-[10px] text-muted-foreground">
              <div class="flex items-center gap-1">
                <div v-if="getAccessDetails(archivo).isOnlyYou"
                  class="size-3.5 rounded-full bg-slate-200 dark:bg-slate-700 text-slate-600 dark:text-slate-300 flex items-center justify-center shrink-0">
                  <IconUser class="size-2" />
                </div>
                <div v-else
                  class="size-3.5 rounded-full bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-200 text-[8px] font-bold flex items-center justify-center shrink-0">
                  {{ getAccessDetails(archivo).initial }}
                </div>
                <span class="truncate max-w-[80px]">{{ getAccessDetails(archivo).label }}</span>
              </div>
              <span class="font-mono text-[10px]">{{ formatFileDate(archivo.fecha_subida) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
        <IconFileText class="size-8 mx-auto text-muted-foreground/40" />
        <p class="font-semibold text-foreground">No se encontraron archivos</p>
        <p>No existen documentos que coincidan con el filtro o término de búsqueda seleccionado.</p>
      </div>
    </Card>

    <Card class="space-y-3">
      <div class="flex items-center justify-between border-b border-border pb-3">
        <div class="flex items-center gap-2">
          <IconFileCheck class="size-3.5 text-primary shrink-0" />
          <div>
            <h3 class="font-semibold text-foreground tracking-tight text-sm">Documentos Formales de Legajo</h3>
            <p class="text-xs text-muted-foreground">Resoluciones de alcaldía y disposiciones emitidas</p>
          </div>
        </div>
        <Badge variant="outline" size="xs">{{ documentos.length }} Registrados</Badge>
      </div>

      <div v-if="documentos.length > 0" class="space-y-2.5">
        <div v-for="doc in documentos" :key="doc.id"
          class="p-3 rounded-xl border border-border bg-card space-y-1 text-xs hover:border-primary/30 transition">
          <div class="flex items-center justify-between">
            <span class="font-semibold text-foreground text-[11px]">{{ doc.sigla }}</span>
            <span class="font-mono text-muted-foreground text-[11px]">{{ formatDate(doc.fecha) }}</span>
          </div>
          <p class="text-muted-foreground text-[11px]">{{ doc.descripcion }}</p>
        </div>
      </div>

      <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
        <IconFileCheck class="size-8 mx-auto text-muted-foreground/40" />
        <p class="font-semibold text-foreground">Sin resoluciones formalmente registradas</p>
        <p>No constan actos administrativos ni resoluciones asociadas a este legajo.</p>
      </div>
    </Card>
  </div>
</template>
