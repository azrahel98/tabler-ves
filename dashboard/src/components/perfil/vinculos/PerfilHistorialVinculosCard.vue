<script setup lang="ts">
import { ref, computed } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  formatMoneda,
  getVinculoStatusType,
  getTipoEventoLabel,
  getEstadoVigencia,
  getEstadoVigenciaLabel,
  type PersonalVinculo,
  type VinculoStatusType,
  type EventoVinculoDetalle,
} from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconBriefcase,
  IconCalendar,
  IconCalendarOff,
  IconFileText,
  IconFileDescription,
  IconFileCheck,
  IconFileCode,
  IconFileX,
  IconId,
  IconShieldCheck,
  IconAlertCircle,
  IconChevronDown,
  IconBuildingSkyscraper,
  IconFileCertificate,
  IconExternalLink,
  IconCalendarPlus,
  IconSearch,
  IconArrowsSort,
  IconChevronUp,
  IconChevronLeft,
  IconChevronRight,
} from '@tabler/icons-vue'

interface Props {
  vinculos: PersonalVinculo[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'registrarRenuncia', vinculo: PersonalVinculo): void
  (e: 'verDocumento', payload: { tipo: 'ingreso' | 'salida' | 'evento'; vinculo: PersonalVinculo; evento?: EventoVinculoDetalle; doc?: 'inicio' | 'salida' }): void
  (e: 'verEventos', vinculo: PersonalVinculo): void
  (e: 'crearEvento', vinculo: PersonalVinculo): void
}>()

type FiltroVinculo = 'todos' | 'vigentes' | 'concluidos'
type SortKey = 'cargo' | 'area' | 'ingreso' | 'cese' | 'sueldo'

const PAGE_SIZE = 8
const filtro = ref<FiltroVinculo>('todos')
const busqueda = ref('')
const sortKey = ref<SortKey>('ingreso')
const sortDir = ref<'desc' | 'asc'>('desc')
const page = ref(1)
const expandedIds = ref<Set<number | string>>(new Set())

const toggleExpand = (id: number | string) => {
  const vinculo = props.vinculos.find((v) => v.id === id)
  if (vinculo?.origen === 'SUNAT') return
  const next = new Set(expandedIds.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  expandedIds.value = next
}

const isExpanded = (id: number | string): boolean => {
  return expandedIds.value.has(id)
}

const getBadgeVariant = (type: VinculoStatusType): 'success' | 'warning' | 'secondary' => {
  if (type === 'success') return 'success'
  if (type === 'warning') return 'warning'
  return 'secondary'
}

const esVigente = (v: PersonalVinculo) => getVinculoStatusType(v) === 'success'

const conteos = computed(() => ({
  todos: props.vinculos.length,
  vigentes: props.vinculos.filter(esVigente).length,
  concluidos: props.vinculos.filter((v) => !esVigente(v)).length,
}))

const tabsFiltro = computed(() => [
  { id: 'todos' as const, label: 'Todos', count: conteos.value.todos },
  { id: 'vigentes' as const, label: 'Vigentes', count: conteos.value.vigentes },
  { id: 'concluidos' as const, label: 'Concluidos', count: conteos.value.concluidos },
])

const normalizar = (t: string | null | undefined) =>
  (t || '').normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase()

const sortValue = (v: PersonalVinculo, key: SortKey): number | string => {
  switch (key) {
    case 'cargo': return normalizar(v.cargo)
    case 'area': return normalizar(v.area)
    case 'ingreso': return parseDateSafe(v.fecha_ingreso)?.getTime() || 0
    case 'cese': return v.fecha_salida ? parseDateSafe(v.fecha_salida)?.getTime() || 0 : Number.MAX_SAFE_INTEGER
    case 'sueldo': return Number(v.sueldo) || 0
  }
}

const filtrados = computed(() => {
  const q = normalizar(busqueda.value.trim())
  const list = props.vinculos.filter((v) => {
    if (filtro.value === 'vigentes' && !esVigente(v)) return false
    if (filtro.value === 'concluidos' && esVigente(v)) return false
    if (!q) return true
    return normalizar([v.cargo, v.area, v.regimen, v.codigo].join(' ')).includes(q)
  })
  const dir = sortDir.value === 'desc' ? -1 : 1
  return list.sort((a, b) => {
    const A = sortValue(a, sortKey.value)
    const B = sortValue(b, sortKey.value)
    return (A < B ? -1 : A > B ? 1 : 0) * dir
  })
})

const totalPages = computed(() => Math.max(1, Math.ceil(filtrados.value.length / PAGE_SIZE)))
const sortedVinculos = computed(() => {
  const p = Math.min(page.value, totalPages.value)
  return filtrados.value.slice((p - 1) * PAGE_SIZE, p * PAGE_SIZE)
})

const setSort = (key: SortKey) => {
  if (sortKey.value === key) sortDir.value = sortDir.value === 'desc' ? 'asc' : 'desc'
  else {
    sortKey.value = key
    sortDir.value = key === 'cargo' || key === 'area' ? 'asc' : 'desc'
  }
  page.value = 1
}

const setFiltro = (f: FiltroVinculo) => {
  filtro.value = f
  page.value = 1
}

const ariaSort = (key: SortKey) =>
  sortKey.value === key ? (sortDir.value === 'asc' ? 'ascending' : 'descending') : 'none'

const estadoLabel = (v: PersonalVinculo) => {
  const t = getVinculoStatusType(v)
  return t === 'success' ? 'Vigente' : t === 'warning' ? 'Sin doc. salida' : 'Concluido'
}
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="flex items-center gap-1 px-3 border-b border-border overflow-x-auto" role="tablist" aria-label="Filtrar vínculos">
      <button v-for="t in tabsFiltro" :key="t.id" type="button" role="tab" :aria-selected="filtro === t.id"
        class="relative inline-flex items-center gap-2 px-3 py-3 text-xs whitespace-nowrap cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary rounded-md"
        :class="filtro === t.id ? 'text-foreground font-semibold after:absolute after:inset-x-2 after:-bottom-px after:h-0.5 after:bg-foreground' : 'text-muted-foreground hover:text-foreground font-medium'"
        @click="setFiltro(t.id)">
        {{ t.label }}
        <span class="text-micro text-muted-foreground">{{ t.count }}</span>
      </button>
    </div>

    <div class="p-3 flex flex-wrap items-center justify-between gap-3">
      <label class="relative flex-1 min-w-48 max-w-xl">
        <IconSearch class="size-4 absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <input v-model="busqueda" type="search" placeholder="Buscar por cargo, área, régimen o plaza"
          class="w-full ps-9 pe-3 py-2 rounded-lg bg-muted border-transparent text-xs text-foreground placeholder:text-muted-foreground focus:bg-card focus:border-primary focus:ring-primary"
          @input="page = 1" />
      </label>
      <div class="hidden md:flex items-center gap-3 text-meta text-muted-foreground">
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-emerald-500"></span> Vigente</span>
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-amber-500"></span> Sin doc. salida</span>
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-muted-foreground/40"></span> Concluido</span>
      </div>
    </div>

    <div v-if="vinculos.length > 0" class="border-t border-border">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs" aria-label="Tabla de historial de vínculos laborales">
          <thead class="border-b border-border text-xs text-muted-foreground select-none">
            <tr>
              <th scope="col" :aria-sort="ariaSort('cargo')" class="px-3 py-3 font-medium ">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition" @click="setSort('cargo')">
                  Cargo
                  <IconChevronUp v-if="sortKey === 'cargo'" class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('area')" class="px-3 py-3 font-medium ">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition" @click="setSort('area')">
                  Área
                  <IconChevronUp v-if="sortKey === 'area'" class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('ingreso')" class="px-3 py-3 font-medium ">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition" @click="setSort('ingreso')">
                  Ingreso
                  <IconChevronUp v-if="sortKey === 'ingreso'" class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('cese')" class="px-3 py-3 font-medium ">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition" @click="setSort('cese')">
                  Cese
                  <IconChevronUp v-if="sortKey === 'cese'" class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" class="px-3 py-3 font-medium">Estado</th>
              <th scope="col" :aria-sort="ariaSort('sueldo')" class="px-3 py-3 font-medium text-right">
                <button type="button" class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition" @click="setSort('sueldo')">
                  Sueldo
                  <IconChevronUp v-if="sortKey === 'sueldo'" class="size-3 transition-transform" :class="sortDir === 'desc' ? 'rotate-180' : ''" />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" class="px-2 py-3 text-center w-10"><span class="sr-only">Detalle</span></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <template v-for="v in sortedVinculos" :key="v.id">
              <tr :tabindex="v.origen === 'SUNAT' ? undefined : 0"
                class="transition-colors focus-visible:bg-muted/50 focus-visible:outline-hidden" :class="[
                  v.origen !== 'SUNAT' ? 'hover:bg-muted/40 cursor-pointer' : '',
                  v.origen !== 'SUNAT' && isExpanded(v.id) ? 'bg-muted/25' : ''
                ]" :aria-expanded="v.origen !== 'SUNAT' ? isExpanded(v.id) : undefined"
                @click="v.origen !== 'SUNAT' && toggleExpand(v.id)"
                @keydown.enter.prevent="v.origen !== 'SUNAT' && toggleExpand(v.id)"
                @keydown.space.prevent="v.origen !== 'SUNAT' && toggleExpand(v.id)">
                <td class="px-3 py-2.5 min-w-0">
                  <div class="flex items-start gap-2.5">
                    <span v-if="getVinculoStatusType(v) === 'success'" class="relative flex size-2 shrink-0 mt-1"
                      title="Vínculo Activo / Vigente">
                      <span
                        class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-75"></span>
                      <span class="relative inline-flex size-2 rounded-full bg-emerald-500"></span>
                    </span>
                    <span v-else-if="getVinculoStatusType(v) === 'warning'" class="relative flex size-2 shrink-0 mt-1"
                      title="Inactivo sin documento de salida">
                      <span
                        class="absolute inline-flex size-full animate-ping rounded-full bg-amber-400 opacity-75"></span>
                      <span class="relative inline-flex size-2 rounded-full bg-amber-500"></span>
                    </span>
                    <span v-else class="inline-flex size-2 rounded-full bg-muted-foreground/35 shrink-0 mt-1"
                      title="Vínculo Concluido / Cesado"></span>

                    <div class="min-w-0 flex-1">
                      <div class="flex items-center gap-1.5 flex-wrap">
                        <span class="font-medium text-foreground text-meta block wrap-break-word"
                          :title="v.cargo || (v.origen === 'SUNAT' ? 'Vínculo declarado en SUNAT' : '-')">
                          {{ v.cargo || (v.origen === 'SUNAT' ? 'Vínculo SUNAT (T-Registro)' : '-') }}
                        </span>
                        <Badge v-if="v.origen === 'SUNAT'" variant="outline" size="xs"
                          class="text-nano px-1 py-0 border-blue-500/40 text-blue-600 dark:text-blue-400 font-semibold uppercase">
                          SUNAT
                        </Badge>
                        <span v-if="v.eventos && v.eventos.length > 0"
                          class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md text-nano font-semibold bg-purple-500/10 text-purple-700 dark:text-purple-300 border border-purple-500/20"
                          :title="`${v.eventos.length} eventos laborales registrados`">
                          <IconFileCode class="size-2.5 shrink-0" />
                          <span>{{ v.eventos.length }} {{ v.eventos.length === 1 ? 'evento' : 'eventos' }}</span>
                        </span>
                      </div>
                    </div>
                  </div>
                </td>

                <td class="px-3 py-2.5 text-meta text-foreground max-w-56">
                  <span class="line-clamp-2" :title="v.area || undefined">{{ v.area || '-' }}</span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <span class="font-mono text-muted-foreground text-meta">
                    {{ formatDate(v.fecha_ingreso) }}
                  </span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <span
                    :class="v.fecha_salida ? 'font-mono text-muted-foreground text-meta' : 'text-meta font-medium text-emerald-600 dark:text-emerald-400'">
                    {{ v.fecha_salida ? formatDate(v.fecha_salida) : 'Vigente' }}
                  </span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <Badge :variant="getBadgeVariant(getVinculoStatusType(v))" size="xs">{{ estadoLabel(v) }}</Badge>
                </td>

                <td
                  class="px-3 py-2.5 text-right font-mono font-medium text-foreground tabular-nums text-xs whitespace-nowrap">
                  {{ v.origen === 'SUNAT' ? '-' : formatMoneda(v.sueldo) }}
                </td>

                <td class="px-2 py-2.5 text-center">
                  <button v-if="v.origen !== 'SUNAT'" type="button"
                    class="size-8 rounded-md text-muted-foreground hover:text-foreground transition-transform duration-200 cursor-pointer inline-flex items-center justify-center"
                    :class="isExpanded(v.id) ? 'rotate-180 text-foreground' : ''"
                    :aria-label="isExpanded(v.id) ? 'Contraer detalles' : 'Expandir detalles'"
                    @click.stop="toggleExpand(v.id)">
                    <IconChevronDown class="size-4" />
                  </button>
                </td>
              </tr>

              <tr v-if="v.origen !== 'SUNAT' && isExpanded(v.id)" class="bg-muted/15 border-b border-border/80">
                <td colspan="7" class="p-2.5">
                  <div class="p-3 rounded-xl bg-card border border-border/70 space-y-3 text-xs">
                    <div
                      class="flex flex-wrap items-center justify-between gap-2 pb-2.5 border-b border-border/60">
                      <div class="flex items-center gap-2 flex-wrap">
                        <Badge :variant="getBadgeVariant(getVinculoStatusType(v))" size="xs" class="gap-1.5">
                          <span v-if="getVinculoStatusType(v) === 'success'"
                            class="relative inline-flex size-1.5 rounded-full bg-emerald-500 shrink-0"></span>
                          <span v-else-if="getVinculoStatusType(v) === 'warning'"
                            class="relative inline-flex size-1.5 rounded-full bg-amber-500 shrink-0"></span>
                          <span v-else class="inline-flex size-1.5 rounded-full bg-muted-foreground/50 shrink-0"></span>
                          {{ getVinculoStatusType(v) === 'warning' ? `${v.estado} (sin doc. salida)` : v.estado }}
                        </Badge>
                        <span class="font-mono text-muted-foreground text-meta">
                          Plaza AIRHSP: {{ v.codigo || '-' }}
                        </span>
                      </div>

                      <div class="flex items-center gap-2 flex-wrap justify-end">
                        <Button v-if="v.origen !== 'SUNAT'" size="xs" variant="outline"
                          class="gap-1.5 cursor-pointer"
                          @click.stop="emit('crearEvento', v)">
                          <IconCalendarPlus class="size-3.5" />
                          <span>Agregar Evento</span>
                        </Button>
                        <Button v-if="v.estado.toLowerCase() != 'inactivo' && v.origen !== 'SUNAT'" size="xs"
                          variant="outline"
                          class="gap-1.5 cursor-pointer hover:text-rose-600 dark:hover:text-rose-400 hover:border-rose-500/30 hover:bg-rose-500/5"
                          @click.stop="emit('registrarRenuncia', v)">
                          <IconFileX class="size-3.5" />
                          <span>Registrar Renuncia</span>
                        </Button>
                      </div>
                    </div>

                    <div class="grid grid-cols-[repeat(auto-fit,minmax(14rem,1fr))] gap-x-8 gap-y-2.5 pt-0.5">
                      <div class="space-y-2.5">
                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconBuildingSkyscraper class="size-3.5 text-muted-foreground/70 shrink-0" /> Área:
                          </span>
                          <span class="font-medium text-foreground wrap-break-word min-w-0" :title="v.area">
                            {{ v.area || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendar class="size-3.5 text-muted-foreground/70 shrink-0" /> Fecha Ingreso:
                          </span>
                          <span class="font-mono font-medium text-foreground min-w-0">{{ formatDate(v.fecha_ingreso)
                          }}</span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileText class="size-3.5 text-muted-foreground/70 shrink-0" /> Doc. Ingreso:
                          </span>
                          <button v-if="v.doc_ingreso || v.numero_doc_ingreso" type="button"
                            class="text-left font-medium text-primary hover:underline truncate min-w-0 inline-flex items-center gap-1 cursor-pointer transition-colors"
                            :title="[v.doc_ingreso, v.numero_doc_ingreso].filter(Boolean).join(' N° ') || '-'"
                            @click.stop="emit('verDocumento', { tipo: 'ingreso', vinculo: v })">
                            <span class="truncate">{{ [v.doc_ingreso, v.numero_doc_ingreso].filter(Boolean).join(' N° ')
                              }}</span>
                            <IconExternalLink class="size-3 shrink-0 opacity-70" />
                          </button>
                          <span v-else class="text-muted-foreground">-</span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileDescription class="size-3.5 text-muted-foreground/70 shrink-0" /> Ref. Ingreso:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0" :title="v.descrip_ingreso || ''">
                            {{ v.descrip_ingreso || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconBriefcase class="size-3.5 text-muted-foreground/70 shrink-0" /> G. Ocupacional:
                          </span>
                          <span class="font-medium text-foreground capitalize truncate min-w-0">{{ v.grupo_ocupacional
                            || '-' }}</span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconId class="size-3.5 text-muted-foreground/70 shrink-0" /> C. Estructural:
                          </span>
                          <span class="font-medium text-foreground wrap-break-word min-w-0"
                            :title="v.cargo_estructural || ''">
                            {{ v.cargo_estructural || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconShieldCheck class="size-3.5 text-muted-foreground/70 shrink-0" /> Afiliación:
                          </span>
                          <div class="flex items-center gap-2 min-w-0">
                            <span
                              class="size-5 rounded-full bg-muted text-muted-foreground flex items-center justify-center text-micro font-bold shrink-0">
                              {{ (v.sindicato || 'N')[0] }}
                            </span>
                            <span class="font-medium text-foreground truncate min-w-0">{{ v.sindicato || 'No Afiliado'
                            }}</span>
                          </div>
                        </div>
                      </div>

                      <div class="space-y-2.5">
                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileCertificate class="size-3.5 text-muted-foreground/70 shrink-0" /> Régimen:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0" :title="v.regimen">
                            {{ v.regimen || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendarOff class="size-3.5 text-muted-foreground/70 shrink-0" /> Fecha Cese:
                          </span>
                          <span class="min-w-0"
                            :class="v.fecha_salida ? 'font-mono text-foreground font-medium' : 'text-emerald-600 dark:text-emerald-400 font-medium'">
                            {{ v.fecha_salida ? formatDate(v.fecha_salida) : 'Vigente' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileCheck class="size-3.5 text-muted-foreground/70 shrink-0" /> Doc. Salida:
                          </span>
                          <button v-if="v.doc_salida || v.numero_doc_salida" type="button"
                            class="text-left font-medium text-amber-600 dark:text-amber-400 hover:underline truncate min-w-0 inline-flex items-center gap-1 cursor-pointer transition-colors"
                            :title="[v.doc_salida, v.numero_doc_salida].filter(Boolean).join(' N° ') || '-'"
                            @click.stop="emit('verDocumento', { tipo: 'salida', vinculo: v })">
                            <span class="truncate">{{ [v.doc_salida, v.numero_doc_salida].filter(Boolean).join(' N° ')
                              }}</span>
                            <IconExternalLink class="size-3 shrink-0 opacity-70" />
                          </button>
                          <span v-else class="text-muted-foreground">-</span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileDescription class="size-3.5 text-muted-foreground/70 shrink-0" /> Ref. Salida:
                          </span>
                          <span class="font-medium text-foreground truncate min-w-0" :title="v.descrip_salida || ''">
                            {{ v.descrip_salida || '-' }}
                          </span>
                        </div>

                        <div
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconAlertCircle class="size-3.5 text-muted-foreground/70 shrink-0" /> Eventos:
                          </span>
                          <div class="flex items-center gap-2 flex-wrap min-w-0">
                            <button type="button"
                              class="text-left font-medium text-primary hover:underline truncate min-w-0 inline-flex items-center gap-1 cursor-pointer transition-colors"
                              @click.stop="emit('verEventos', v)">
                              <span class="truncate">
                                {{ v.eventos && v.eventos.length > 0 ? `${v.eventos.length} ${v.eventos.length === 1 ?
                                  'evento registrado' : 'eventos registrados'}` : 'Sin eventos' }}
                              </span>
                              <IconExternalLink class="size-3 shrink-0 opacity-70" />
                            </button>
                          </div>
                        </div>

                        <div v-if="v.doc_evento_tipo || v.numero_doc_evento"
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconFileCode class="size-3.5 text-muted-foreground/70 shrink-0" /> Doc. Evento:
                          </span>
                          <button type="button"
                            class="text-left font-medium text-primary hover:underline truncate min-w-0 inline-flex items-center gap-1 cursor-pointer transition-colors"
                            :title="[v.doc_evento_tipo, v.numero_doc_evento].filter(Boolean).join(' N° ') || '-'"
                            @click.stop="emit('verDocumento', { tipo: 'evento', vinculo: v })">
                            <span class="truncate">{{
                              [v.doc_evento_tipo, v.numero_doc_evento].filter(Boolean).join(' N°') }}</span>
                            <IconExternalLink class="size-3 shrink-0 opacity-70" />
                          </button>
                        </div>

                        <div v-if="v.fecha_evento"
                          class="grid grid-cols-[8rem_1fr] items-center gap-2">
                          <span class="text-muted-foreground flex items-center gap-1.5 shrink-0">
                            <IconCalendar class="size-3.5 text-muted-foreground/70 shrink-0" /> Fecha Evento:
                          </span>
                          <span class="font-medium font-mono text-foreground min-w-0">
                            {{ formatDate(v.fecha_evento) }}
                          </span>
                        </div>
                      </div>
                    </div>

                    <div v-if="v.origen !== 'SUNAT' || (v.eventos && v.eventos.length >= 1)"
                      class="pt-2.5 border-t border-border/60 space-y-2">
                      <div class="flex items-center justify-between">
                        <span class="text-meta font-semibold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
                          <IconFileCode class="size-3.5 text-muted-foreground shrink-0" />
                          Eventos Laborales Registrados ({{ v.eventos?.length || 0 }})
                        </span>
                        <div class="flex items-center gap-2">
                          <button v-if="v.eventos && v.eventos.length > 0" type="button"
                            class="text-meta text-primary hover:underline font-medium inline-flex items-center gap-1 cursor-pointer"
                            @click.stop="emit('verEventos', v)">
                            <span>Gestionar eventos</span>
                            <IconExternalLink class="size-3 shrink-0 opacity-70" />
                          </button>
                        </div>
                      </div>
                      <div class="space-y-1.5">
                        <div v-for="ev in v.eventos" :key="ev.id"
                          class="flex flex-wrap items-center justify-between gap-1.5 p-2 rounded-lg bg-muted/40 border border-border/50 text-meta">
                          <div class="flex items-center gap-2 flex-wrap min-w-0">
                            <Badge variant="outline" size="xs"
                              class="font-semibold uppercase text-micro">
                              {{ getTipoEventoLabel(ev.tipo_evento) }}
                            </Badge>
                            <span v-if="getEstadoVigencia(ev) === 'cerrado'"
                              class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground bg-muted px-1.5 py-0.5 rounded-md">
                              {{ getEstadoVigenciaLabel('cerrado') }}
                            </span>
                            <span v-else-if="getEstadoVigencia(ev) === 'vencido'"
                              class="inline-flex items-center gap-1 text-micro font-semibold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-1.5 py-0.5 rounded-md"
                              title="El evento sigue abierto en el registro porque no tiene documento de salida">
                              <span class="size-1 rounded-full bg-amber-500"></span>
                              {{ getEstadoVigenciaLabel('vencido') }}
                            </span>
                            <span v-else-if="getEstadoVigencia(ev) === 'futuro'"
                              class="inline-flex items-center gap-1 text-micro font-medium text-muted-foreground border border-border px-1.5 py-0.5 rounded-md">
                              {{ getEstadoVigenciaLabel('futuro') }}
                            </span>
                            <span v-else-if="getEstadoVigencia(ev) === 'sin_fechas'"
                              class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded-md">
                              <span class="size-1 rounded-full bg-emerald-500"></span>
                              {{ getEstadoVigenciaLabel('sin_fechas') }}
                            </span>
                            <span v-else
                              class="inline-flex items-center gap-1 text-micro font-semibold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded-md">
                              <span class="size-1 rounded-full bg-emerald-500"></span>
                              {{ getEstadoVigenciaLabel('vigente') }}
                            </span>
                            <span v-if="ev.fecha_desde" class="text-muted-foreground text-micro font-medium">
                              {{ ev.fecha_hasta ? `Periodo: ${formatDate(ev.fecha_desde)} - ${formatDate(ev.fecha_hasta)}` : `desde ${formatDate(ev.fecha_desde)} (sin término)` }}
                            </span>
                            <span v-if="ev.nueva_area" class="text-foreground text-meta truncate"
                              :title="ev.nueva_area">
                              Área: {{ ev.nueva_area }}
                            </span>
                            <span v-if="ev.nuevo_cargo" class="text-foreground text-meta truncate"
                              :title="ev.nuevo_cargo">
                              Cargo: {{ ev.nuevo_cargo }}
                            </span>
                          </div>
                          <div class="flex items-center gap-2 shrink-0">
                            <span v-if="ev.fecha_inicio" class="text-micro font-mono text-muted-foreground">
                              {{ formatDate(ev.fecha_inicio) }}
                            </span>
                            <button v-if="ev.tipo_doc_inicio || ev.numero_doc_inicio" type="button"
                              class="text-primary hover:underline text-meta font-medium inline-flex items-center gap-1 cursor-pointer"
                              @click.stop="emit('verDocumento', { tipo: 'evento', vinculo: v, evento: ev })">
                              <span>{{ [ev.tipo_doc_inicio, ev.numero_doc_inicio].filter(Boolean).join(' N° ') }}</span>
                              <IconExternalLink class="size-3 shrink-0 opacity-70" />
                            </button>
                            <button v-if="ev.doc_salida_id || ev.numero_doc_salida" type="button"
                              class="text-primary hover:underline text-meta font-medium inline-flex items-center gap-1 cursor-pointer"
                              :title="[ev.tipo_doc_salida, ev.numero_doc_salida].filter(Boolean).join(' N° ') || undefined"
                              @click.stop="emit('verDocumento', { tipo: 'evento', vinculo: v, evento: ev, doc: 'salida' })">
                              <span>Doc. cierre</span>
                              <IconExternalLink class="size-3 shrink-0 opacity-70" />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>

      <div v-if="sortedVinculos.length === 0" class="py-10 text-center text-xs text-muted-foreground">
        Ningún vínculo coincide con el filtro o la búsqueda.
      </div>

      <div class="px-4 py-3 border-t border-border flex flex-wrap items-center justify-between gap-3 text-xs">
        <span class="text-muted-foreground"><span class="font-semibold text-foreground">{{ filtrados.length }}</span> {{ filtrados.length === 1 ? 'resultado' : 'resultados' }}</span>
        <nav v-if="totalPages > 1" class="flex items-center gap-2 text-muted-foreground" aria-label="Paginación">
          <button type="button" class="size-8 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
            :disabled="page <= 1" aria-label="Página anterior" @click="page--">
            <IconChevronLeft class="size-4" />
          </button>
          <span class="min-w-8 h-8 px-2 inline-flex items-center justify-center rounded-lg bg-muted text-foreground font-medium">{{ Math.min(page, totalPages) }}</span>
          <span>de {{ totalPages }}</span>
          <button type="button" class="size-8 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
            :disabled="page >= totalPages" aria-label="Página siguiente" @click="page++">
            <IconChevronRight class="size-4" />
          </button>
        </nav>
      </div>
    </div>

    <div v-else class="text-xs text-muted-foreground py-8 text-center space-y-2">
      <IconBriefcase class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin historial de vínculos laborales</p>
      <p>No se registran resoluciones de designación o contratos en el sistema.</p>
    </div>
  </div>
</template>
