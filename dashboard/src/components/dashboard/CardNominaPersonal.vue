<script setup lang="ts">
import { ref, computed } from 'vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  resolveAvatarUrl,
  type TrabajadorNuevo,
  type TrabajadorRenuncia,
} from './types'
import { formatMoneda } from '@/services/personal'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconUsers,
  IconX,
  IconUserOff,
  IconChevronLeft,
  IconChevronRight,
  IconChevronUp,
  IconArrowsSort,
} from '@tabler/icons-vue'

interface Props {
  nuevos: TrabajadorNuevo[]
  renuncias?: TrabajadorRenuncia[]
  isLoading?: boolean
  selectedArea?: string | null
  selectedRegimen?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  renuncias: () => [],
  isLoading: false,
  selectedArea: null,
  selectedRegimen: null,
})

const emit = defineEmits<{
  (e: 'clear-filter'): void
}>()

const activeTab = ref<'ingresos' | 'salidas'>('ingresos')
const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(8)
const avatarErrors = ref<Record<string, boolean>>({})

type SortKey = 'nombre' | 'area' | 'cargo' | 'detalle' | 'sueldo' | 'fecha'
const sortKey = ref<SortKey>('fecha')
const sortDir = ref<'asc' | 'desc'>('desc')

const tabs = computed(() => [
  { id: 'ingresos' as const, label: 'Ingresos recientes', count: props.nuevos.length, dot: 'bg-emerald-500' },
  { id: 'salidas' as const, label: 'Salidas y renuncias', count: props.renuncias.length, dot: 'bg-rose-500' },
])

const changeTab = (tab: 'ingresos' | 'salidas') => {
  activeTab.value = tab
  sortKey.value = 'fecha'
  sortDir.value = 'desc'
  currentPage.value = 1
}

const setSort = (key: SortKey) => {
  if (sortKey.value === key) {
    sortDir.value = sortDir.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortKey.value = key
    sortDir.value = key === 'fecha' || key === 'sueldo' ? 'desc' : 'asc'
  }
  currentPage.value = 1
}

const ariaSort = (key: SortKey) =>
  sortKey.value === key ? (sortDir.value === 'asc' ? 'ascending' : 'descending') : 'none'

const getInitials = (nombre: string): string => {
  const parts = (nombre || '').trim().split(/\s+/).filter(Boolean)
  if (parts.length === 0) return 'SP'
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase()
  return (parts[0][0] + (parts[1]?.[0] || '')).toUpperCase()
}

const normalizar = (t: string | null | undefined) =>
  (t || '').normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase()

const isIngreso = (item: TrabajadorNuevo | TrabajadorRenuncia): item is TrabajadorNuevo => {
  return 'ingreso' in item
}

const filteredIngresos = computed(() => {
  let list = props.nuevos
  if (props.selectedArea) {
    const areaNorm = normalizar(props.selectedArea)
    list = list.filter((t) => normalizar(t.area).includes(areaNorm))
  }
  if (props.selectedRegimen) {
    const regNorm = normalizar(props.selectedRegimen)
    list = list.filter((t) => normalizar(t.regimen).includes(regNorm))
  }
  if (searchQuery.value.trim()) {
    const q = normalizar(searchQuery.value.trim())
    list = list.filter((t) =>
      normalizar([t.nombre, t.dni, t.cargo, t.area, t.plaza, t.regimen, t.documento].join(' ')).includes(q)
    )
  }
  return list
})

const filteredSalidas = computed(() => {
  let list = props.renuncias
  if (props.selectedArea) {
    const areaNorm = normalizar(props.selectedArea)
    list = list.filter((t) => normalizar(t.area).includes(areaNorm))
  }
  if (searchQuery.value.trim()) {
    const q = normalizar(searchQuery.value.trim())
    list = list.filter((t) =>
      normalizar([t.nombre, t.dni, t.cargo, t.area, t.codigo].join(' ')).includes(q)
    )
  }
  return list
})

const activeList = computed<Array<TrabajadorNuevo | TrabajadorRenuncia>>(() => {
  return activeTab.value === 'ingresos' ? filteredIngresos.value : filteredSalidas.value
})

const totalItems = computed(() => activeList.value.length)
const totalPages = computed(() => Math.max(1, Math.ceil(totalItems.value / pageSize.value)))

const sortValue = (t: TrabajadorNuevo | TrabajadorRenuncia, key: SortKey): number | string => {
  switch (key) {
    case 'nombre':
      return normalizar(t.nombre)
    case 'area':
      return normalizar(t.area)
    case 'cargo':
      return normalizar(t.cargo)
    case 'detalle':
      return isIngreso(t) ? normalizar(t.regimen) : normalizar(t.codigo)
    case 'sueldo':
      return isIngreso(t) ? Number(t.sueldo) || 0 : 0
    case 'fecha': {
      const rawDate = isIngreso(t) ? t.ingreso : t.fecha
      return parseDateSafe(rawDate)?.getTime() || 0
    }
  }
}

const sortedList = computed(() => {
  const key = sortKey.value
  const dir = sortDir.value === 'asc' ? 1 : -1
  return [...activeList.value].sort((a, b) => {
    const A = sortValue(a, key)
    const B = sortValue(b, key)
    return (A < B ? -1 : A > B ? 1 : 0) * dir
  })
})

const paginatedList = computed(() => {
  const p = Math.min(currentPage.value, totalPages.value)
  const start = (p - 1) * pageSize.value
  return sortedList.value.slice(start, start + pageSize.value)
})

const clearSearch = () => {
  searchQuery.value = ''
  currentPage.value = 1
}

const regimenBadgeVariant = (regimen: string): 'primary' | 'success' | 'warning' | 'secondary' | 'neutral' => {
  if ((regimen || '').includes('276')) return 'primary'
  if ((regimen || '').includes('1057')) return 'success'
  if ((regimen || '').includes('728')) return 'warning'
  return 'secondary'
}
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="px-4 py-2.5 border-b border-border flex flex-wrap items-center justify-between gap-2">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconUsers class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Movimientos de Personal</span>
        </h3>
        <p class="text-micro text-muted-foreground">
          Altas y renuncias registradas en los últimos 120 días
        </p>
      </div>

      <div v-if="selectedArea || (activeTab === 'ingresos' && selectedRegimen)" class="flex items-center gap-1.5">
        <button
          type="button"
          class="inline-flex items-center gap-1 px-2 h-control-xs rounded-md bg-primary/10 text-primary border border-primary/20 text-nano font-semibold hover:bg-primary/15 transition cursor-pointer"
          title="Quitar filtro activo"
          @click="emit('clear-filter')"
        >
          <span>Filtro: {{ selectedArea || selectedRegimen }}</span>
          <IconX class="size-3" aria-hidden="true" />
        </button>
      </div>
    </div>

    <div
      class="flex items-center gap-1 px-3 border-b border-border overflow-x-auto"
      role="tablist"
      aria-label="Filtrar movimientos de personal"
    >
      <button
        v-for="t in tabs"
        :key="t.id"
        type="button"
        role="tab"
        :aria-selected="activeTab === t.id"
        class="relative inline-flex items-center gap-2 px-3 py-2.5 text-micro whitespace-nowrap cursor-pointer transition focus:outline-hidden focus-visible:ring-2 focus-visible:ring-primary rounded-md"
        :class="
          activeTab === t.id
            ? 'text-foreground font-semibold after:absolute after:inset-x-2 after:-bottom-px after:h-0.5 after:bg-foreground'
            : 'text-muted-foreground hover:text-foreground font-medium'
        "
        @click="changeTab(t.id)"
      >
        <span class="size-2 rounded-full shrink-0" :class="t.dot"></span>
        <span>{{ t.label }}</span>
        <span class="text-micro text-muted-foreground font-mono">{{ t.count }}</span>
      </button>
    </div>

    <div v-if="isLoading" class="p-3.5 space-y-2.5 border-t border-border animate-pulse" aria-busy="true">
      <div v-for="i in 5" :key="i" class="h-9 rounded-md bg-muted/60"></div>
    </div>

    <div
      v-else-if="totalItems === 0"
      class="border-t border-border text-meta text-muted-foreground py-10 text-center space-y-2 px-4"
    >
      <IconUserOff class="size-7 mx-auto text-muted-foreground/40" aria-hidden="true" />
      <p class="font-semibold text-foreground text-xs">
        {{ activeTab === 'ingresos' ? 'Sin ingresos registrados' : 'Sin salidas o renuncias registradas' }}
      </p>
      <p class="text-micro max-w-sm mx-auto">
        {{
          selectedArea || selectedRegimen || searchQuery
            ? 'Ningún registro coincide con el filtro o la búsqueda actual.'
            : activeTab === 'ingresos'
              ? 'No constan altas de personal en los últimos 120 días.'
              : 'No constan ceses o renuncias en los últimos 120 días.'
        }}
      </p>
      <div v-if="selectedArea || selectedRegimen || searchQuery" class="pt-1">
        <Button
          variant="outline"
          size="xs"
          class="gap-1.5 text-micro cursor-pointer"
          @click="
            clearSearch();
            emit('clear-filter')
          "
        >
          <IconX class="size-3" aria-hidden="true" />
          Restablecer filtros
        </Button>
      </div>
    </div>

    <div v-else class="border-t border-border">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-meta" aria-label="Tabla de movimientos de personal">
          <thead class="border-b border-border text-micro text-muted-foreground select-none">
            <tr>
              <th scope="col" :aria-sort="ariaSort('nombre')" class="px-3 py-2.5 font-medium">
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('nombre')"
                >
                  Servidor
                  <IconChevronUp
                    v-if="sortKey === 'nombre'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('cargo')" class="px-3 py-2.5 font-medium">
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('cargo')"
                >
                  Cargo
                  <IconChevronUp
                    v-if="sortKey === 'cargo'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('area')" class="px-3 py-2.5 font-medium">
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('area')"
                >
                  Área
                  <IconChevronUp
                    v-if="sortKey === 'area'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('detalle')" class="px-3 py-2.5 font-medium">
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('detalle')"
                >
                  {{ activeTab === 'ingresos' ? 'Régimen' : 'Plaza AIRHSP' }}
                  <IconChevronUp
                    v-if="sortKey === 'detalle'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th
                v-if="activeTab === 'ingresos'"
                scope="col"
                :aria-sort="ariaSort('sueldo')"
                class="px-3 py-2.5 font-medium text-right"
              >
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('sueldo')"
                >
                  Sueldo
                  <IconChevronUp
                    v-if="sortKey === 'sueldo'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
              <th scope="col" :aria-sort="ariaSort('fecha')" class="px-3 py-2.5 font-medium text-right">
                <button
                  type="button"
                  class="inline-flex items-center gap-1.5 cursor-pointer hover:text-foreground transition"
                  @click="setSort('fecha')"
                >
                  {{ activeTab === 'ingresos' ? 'Ingreso' : 'Cese' }}
                  <IconChevronUp
                    v-if="sortKey === 'fecha'"
                    class="size-3 transition-transform"
                    :class="sortDir === 'desc' ? 'rotate-180' : ''"
                  />
                  <IconArrowsSort v-else class="size-3 opacity-60" />
                </button>
              </th>
            </tr>
          </thead>

          <tbody class="divide-y divide-border">
            <tr
              v-for="t in paginatedList"
              :key="`${activeTab}-${t.id}`"
              class="transition-colors hover:bg-muted/40"
            >
              <td class="px-3 py-2.5 min-w-0">
                <router-link
                  :to="{ name: 'perfil', params: { dni: t.dni } }"
                  class="flex items-center gap-2.5 group cursor-pointer"
                >
                  <img
                    v-if="t.avatar && !avatarErrors[t.dni]"
                    v-auth-src="resolveAvatarUrl(t.dni)!"
                    :alt="t.nombre"
                    class="size-7 rounded-full border border-border object-cover shrink-0 group-hover:border-primary transition-colors"
                    @error="avatarErrors[t.dni] = true"
                  />
                  <div
                    v-else
                    class="flex size-7 shrink-0 items-center justify-center rounded-full font-semibold text-nano border transition-colors select-none"
                    :class="
                      activeTab === 'ingresos'
                        ? 'bg-primary/10 text-primary border-primary/20'
                        : 'bg-rose-500/10 text-rose-600 dark:text-rose-400 border-rose-500/20'
                    "
                  >
                    {{ getInitials(t.nombre) }}
                  </div>

                  <div class="min-w-0">
                    <span
                      class="text-micro font-medium text-foreground group-hover:text-primary transition-colors block wrap-break-word"
                    >
                      {{ t.nombre }}
                    </span>
                    <span class="font-mono text-nano text-muted-foreground">
                      DNI {{ t.dni }}<template v-if="isIngreso(t) && t.plaza"> · Plaza {{ t.plaza }}</template>
                    </span>
                  </div>
                </router-link>
              </td>

              <td class="px-3 py-2.5 text-micro text-foreground max-w-52">
                <span class="line-clamp-2" :title="t.cargo">{{ t.cargo || '-' }}</span>
              </td>

              <td class="px-3 py-2.5 text-micro max-w-56">
                <router-link
                  v-if="t.area"
                  :to="{ path: '/personal/area', query: { area: t.area } }"
                  class="text-foreground hover:text-primary hover:underline line-clamp-2 transition-colors"
                  :title="t.area"
                >
                  {{ t.area }}
                </router-link>
                <span v-else class="text-muted-foreground">-</span>
              </td>

              <template v-if="isIngreso(t)">
                <td class="px-3 py-2.5 whitespace-nowrap">
                  <Badge :variant="regimenBadgeVariant(t.regimen)" size="xs">
                    {{ t.regimen }}
                  </Badge>
                </td>
                <td class="px-3 py-2.5 text-right font-mono font-medium text-foreground tabular-nums text-micro whitespace-nowrap">
                  {{ t.sueldo ? formatMoneda(t.sueldo) : '-' }}
                </td>
                <td class="px-3 py-2.5 text-right font-mono text-micro text-muted-foreground tabular-nums whitespace-nowrap">
                  {{ formatDate(t.ingreso) }}
                </td>
              </template>

              <template v-else>
                <td class="px-3 py-2.5 whitespace-nowrap">
                  <span class="font-mono text-micro text-muted-foreground">
                    {{ t.codigo || '-' }}
                  </span>
                </td>
                <td class="px-3 py-2.5 text-right font-mono text-micro font-medium text-rose-600 dark:text-rose-400 tabular-nums whitespace-nowrap">
                  {{ formatDate(t.fecha) }}
                </td>
              </template>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="px-4 py-2.5 border-t border-border flex flex-wrap items-center justify-between gap-3 text-micro">
        <span class="text-muted-foreground">
          <span class="font-semibold text-foreground font-mono">{{ totalItems }}</span>
          {{ totalItems === 1 ? 'resultado' : 'resultados' }}
        </span>

        <nav v-if="totalPages > 1" class="flex items-center gap-1.5 text-muted-foreground" aria-label="Paginación">
          <button
            type="button"
            class="size-7 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
            :disabled="currentPage <= 1"
            aria-label="Página anterior"
            @click="currentPage--"
          >
            <IconChevronLeft class="size-3.5" />
          </button>
          <span class="min-w-7 h-7 px-2 inline-flex items-center justify-center rounded-lg bg-muted text-foreground font-mono font-medium">
            {{ Math.min(currentPage, totalPages) }}
          </span>
          <span>de <span class="font-mono">{{ totalPages }}</span></span>
          <button
            type="button"
            class="size-7 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
            :disabled="currentPage >= totalPages"
            aria-label="Página siguiente"
            @click="currentPage++"
          >
            <IconChevronRight class="size-3.5" />
          </button>
        </nav>
      </div>
    </div>
  </div>
</template>
