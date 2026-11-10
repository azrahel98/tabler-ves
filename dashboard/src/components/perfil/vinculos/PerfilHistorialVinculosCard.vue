<script setup lang="ts">
import { ref, computed } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import PerfilVinculoPopover from './PerfilVinculoPopover.vue'
import {
  formatMoneda,
  getVinculoStatusType,
  type PersonalVinculo,
  type VinculoStatusType,
  type EventoVinculoDetalle,
} from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconBriefcase,
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
        class="relative inline-flex items-center gap-2 px-3 py-2.5 text-micro whitespace-nowrap cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary rounded-md"
        :class="filtro === t.id ? 'text-foreground font-semibold after:absolute after:inset-x-2 after:-bottom-px after:h-0.5 after:bg-foreground' : 'text-muted-foreground hover:text-foreground font-medium'"
        @click="setFiltro(t.id)">
        {{ t.label }}
        <span class="text-micro text-muted-foreground">{{ t.count }}</span>
      </button>
    </div>

    <div class="p-2.5 flex flex-wrap items-center justify-between gap-3">
      <label class="relative flex-1 min-w-48 max-w-xl">
        <IconSearch class="size-3.5 absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <input v-model="busqueda" type="search" placeholder="Buscar por cargo, área, régimen o plaza"
          class="w-full ps-8 pe-3 h-control-sm py-0 rounded-md bg-muted border-transparent text-micro text-foreground placeholder:text-muted-foreground focus:bg-card focus:border-primary focus:ring-primary"
          @input="page = 1" />
      </label>
      <div class="hidden md:flex items-center gap-3 text-micro text-muted-foreground">
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-emerald-500"></span> Vigente</span>
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-amber-500"></span> Sin doc. salida</span>
        <span class="inline-flex items-center gap-1.5"><span class="size-2 rounded-full bg-muted-foreground/40"></span> Concluido</span>
      </div>
    </div>

    <div v-if="vinculos.length > 0" class="border-t border-border">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-meta" aria-label="Tabla de historial de vínculos laborales">
          <thead class="border-b border-border text-micro text-muted-foreground select-none">
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
              <th scope="col" class="px-2 py-3 text-center w-28"><span class="sr-only">Detalle</span></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border">
            <template v-for="v in sortedVinculos" :key="v.id">
              <tr class="transition-colors hover:bg-muted/40">
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
                        <span class="font-medium text-foreground text-micro block wrap-break-word"
                          :title="v.cargo || (v.origen === 'SUNAT' ? 'Vínculo declarado en SUNAT' : '-')">
                          {{ v.cargo || (v.origen === 'SUNAT' ? 'Vínculo SUNAT (T-Registro)' : '-') }}
                        </span>
                        <Badge v-if="v.origen === 'SUNAT'" variant="outline" size="xs"
                          class="text-nano px-1 py-0 border-blue-500/40 text-blue-600 dark:text-blue-400 font-semibold uppercase">
                          SUNAT
                        </Badge>
                                              </div>
                    </div>
                  </div>
                </td>

                <td class="px-3 py-2.5 text-micro text-foreground max-w-56">
                  <span class="line-clamp-2" :title="v.area || undefined">{{ v.area || '-' }}</span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <span class="font-mono text-muted-foreground text-micro">
                    {{ formatDate(v.fecha_ingreso) }}
                  </span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <span
                    :class="v.fecha_salida ? 'font-mono text-muted-foreground text-micro' : 'text-micro font-medium text-emerald-600 dark:text-emerald-400'">
                    {{ v.fecha_salida ? formatDate(v.fecha_salida) : 'Vigente' }}
                  </span>
                </td>

                <td class="px-3 py-2.5 whitespace-nowrap">
                  <Badge :variant="getBadgeVariant(getVinculoStatusType(v))" size="xs">{{ estadoLabel(v) }}</Badge>
                </td>

                <td
                  class="px-3 py-2.5 text-right font-mono font-medium text-foreground tabular-nums text-meta whitespace-nowrap">
                  {{ v.origen === 'SUNAT' ? '-' : formatMoneda(v.sueldo) }}
                </td>

                <td class="px-2 py-2.5 text-center">
                  <div class="inline-flex items-center gap-0.5">
                    <PerfilVinculoPopover v-if="v.origen !== 'SUNAT'" modo="info" :vinculo="v"
                      @registrar-renuncia="emit('registrarRenuncia', $event)"
                      @ver-documento="emit('verDocumento', $event)" @ver-eventos="emit('verEventos', $event)"
                      @crear-evento="emit('crearEvento', $event)" />
                    <PerfilVinculoPopover v-if="v.origen !== 'SUNAT' && v.eventos && v.eventos.length > 0" modo="eventos" :vinculo="v"
                      @registrar-renuncia="emit('registrarRenuncia', $event)"
                      @ver-documento="emit('verDocumento', $event)" @ver-eventos="emit('verEventos', $event)"
                      @crear-evento="emit('crearEvento', $event)" />
                    <PerfilVinculoPopover v-if="v.origen !== 'SUNAT'" modo="acciones" :vinculo="v"
                      @registrar-renuncia="emit('registrarRenuncia', $event)"
                      @ver-documento="emit('verDocumento', $event)" @ver-eventos="emit('verEventos', $event)"
                      @crear-evento="emit('crearEvento', $event)" />
                  </div>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>

      <div v-if="sortedVinculos.length === 0" class="py-10 text-center text-meta text-muted-foreground">
        Ningún vínculo coincide con el filtro o la búsqueda.
      </div>

      <div class="px-4 py-3 border-t border-border flex flex-wrap items-center justify-between gap-3 text-meta">
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

    <div v-else class="text-meta text-muted-foreground py-8 text-center space-y-2">
      <IconBriefcase class="size-8 mx-auto text-muted-foreground/40" />
      <p class="font-semibold text-foreground">Sin historial de vínculos laborales</p>
      <p>No se registran resoluciones de designación o contratos en el sistema.</p>
    </div>
  </div>
</template>
