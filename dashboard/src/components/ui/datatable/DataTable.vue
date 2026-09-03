<script setup lang="ts" generic="T extends Record<string, any>">
import { ref, computed, watch } from 'vue'
import Table from '../table/Table.vue'
import TableHeader from '../table/TableHeader.vue'
import TableBody from '../table/TableBody.vue'
import TableRow from '../table/TableRow.vue'
import TableHead from '../table/TableHead.vue'
import TableCell from '../table/TableCell.vue'
import TableEmpty from '../table/TableEmpty.vue'
import Button from '../button/Button.vue'
import DataTableExport from './DataTableExport.vue'
import {
  IconSearch,
  IconX,
  IconChevronUp,
  IconChevronDown,
  IconSelector,
  IconColumns,
  IconLoader2,
  IconChevronLeft,
  IconChevronRight,
} from '@tabler/icons-vue'

export interface DataTableColumn {
  key: string
  label: string
  sortable?: boolean
  align?: 'left' | 'center' | 'right'
  width?: string
  visible?: boolean
  filterable?: boolean
  filterPlaceholder?: string
}

interface Props {
  columns: DataTableColumn[]
  data: T[]
  rowKey?: string | ((row: T) => string | number)
  searchable?: boolean
  searchPlaceholder?: string
  searchKeys?: string[]
  paginated?: boolean
  pageSize?: number
  pageSizeOptions?: number[]
  selectable?: boolean
  selectedKeys?: (string | number)[]
  loading?: boolean
  emptyMessage?: string
  stickyHeader?: boolean
  maxHeight?: string
  columnFilters?: boolean
  showColumnVisibility?: boolean
  exportable?: boolean
  exportFilename?: string
  paginationVariant?: 'simple' | 'numbered'
}

const props = withDefaults(defineProps<Props>(), {
  rowKey: 'id',
  searchable: true,
  searchPlaceholder: 'Buscar registros...',
  paginated: true,
  pageSize: 10,
  pageSizeOptions: () => [5, 10, 20, 50],
  selectable: false,
  selectedKeys: () => [],
  loading: false,
  emptyMessage: 'No se encontraron registros',
  stickyHeader: false,
  maxHeight: undefined,
  columnFilters: false,
  showColumnVisibility: false,
  exportable: false,
  exportFilename: 'export',
  paginationVariant: 'numbered',
})

const emit = defineEmits<{
  (e: 'update:selectedKeys', keys: (string | number)[]): void
  (e: 'row-click', row: T): void
}>()

const searchQuery = ref('')
const sortColumn = ref<string | null>(null)
const sortDirection = ref<'asc' | 'desc'>('asc')
const currentPage = ref(1)
const currentPageSize = ref(props.pageSize)
const isColumnsMenuOpen = ref(false)

const columnVisibilityMap = ref<Record<string, boolean>>({})
const columnFiltersMap = ref<Record<string, string>>({})

watch(
  () => props.columns,
  (cols) => {
    cols.forEach((c) => {
      if (columnVisibilityMap.value[c.key] === undefined) {
        columnVisibilityMap.value[c.key] = c.visible !== false
      }
    })
  },
  { immediate: true },
)

watch(
  () => props.pageSize,
  (newSize) => {
    currentPageSize.value = newSize
  },
)

watch([searchQuery, currentPageSize, columnFiltersMap], () => {
  currentPage.value = 1
}, { deep: true })

const visibleColumns = computed(() => {
  return props.columns.filter((c) => columnVisibilityMap.value[c.key] !== false)
})

const toggleColumnVisibility = (key: string) => {
  columnVisibilityMap.value[key] = !columnVisibilityMap.value[key]
}

const getRowId = (row: T): string | number => {
  if (typeof props.rowKey === 'function') {
    return props.rowKey(row)
  }
  return row[props.rowKey]
}

const filteredData = computed(() => {
  let result = props.data

  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase().trim()
    const targetKeys = props.searchKeys?.length
      ? props.searchKeys
      : props.columns.map((c) => c.key)

    result = result.filter((item) => {
      return targetKeys.some((key) => {
        const val = item[key]
        if (val === null || val === undefined) return false
        return String(val).toLowerCase().includes(query)
      })
    })
  }

  if (props.columnFilters) {
    Object.entries(columnFiltersMap.value).forEach(([colKey, filterVal]) => {
      if (filterVal && filterVal.trim()) {
        const needle = filterVal.toLowerCase().trim()
        result = result.filter((item) => {
          const val = item[colKey]
          if (val === null || val === undefined) return false
          return String(val).toLowerCase().includes(needle)
        })
      }
    })
  }

  return result
})

const sortedData = computed(() => {
  if (!sortColumn.value) {
    return filteredData.value
  }

  const colKey = sortColumn.value
  const dir = sortDirection.value === 'asc' ? 1 : -1

  return [...filteredData.value].sort((a, b) => {
    const aVal = a[colKey]
    const bVal = b[colKey]

    if (aVal === bVal) return 0
    if (aVal === null || aVal === undefined) return 1
    if (bVal === null || bVal === undefined) return -1

    if (typeof aVal === 'number' && typeof bVal === 'number') {
      return (aVal - bVal) * dir
    }

    return String(aVal).localeCompare(String(bVal)) * dir
  })
})

const totalPages = computed(() => {
  if (!props.paginated || currentPageSize.value <= 0) return 1
  return Math.ceil(sortedData.value.length / currentPageSize.value) || 1
})

const paginatedData = computed(() => {
  if (!props.paginated) {
    return sortedData.value
  }
  const start = (currentPage.value - 1) * currentPageSize.value
  const end = start + currentPageSize.value
  return sortedData.value.slice(start, end)
})

const startRecord = computed(() => {
  if (!sortedData.value.length) return 0
  return (currentPage.value - 1) * currentPageSize.value + 1
})

const endRecord = computed(() => {
  return Math.min(currentPage.value * currentPageSize.value, sortedData.value.length)
})

const isAllSelected = computed(() => {
  if (!paginatedData.value.length) return false
  return paginatedData.value.every((row) =>
    props.selectedKeys.includes(getRowId(row)),
  )
})

const isSomeSelected = computed(() => {
  if (isAllSelected.value) return false
  return paginatedData.value.some((row) =>
    props.selectedKeys.includes(getRowId(row)),
  )
})

const handleSelectAll = (event: Event) => {
  const target = event.target as HTMLInputElement
  const currentIds = paginatedData.value.map((r) => getRowId(r))

  if (target.checked) {
    const set = new Set([...props.selectedKeys, ...currentIds])
    emit('update:selectedKeys', Array.from(set))
  } else {
    emit(
      'update:selectedKeys',
      props.selectedKeys.filter((id) => !currentIds.includes(id)),
    )
  }
}

const toggleRowSelection = (row: T) => {
  const id = getRowId(row)
  const isSelected = props.selectedKeys.includes(id)
  if (isSelected) {
    emit(
      'update:selectedKeys',
      props.selectedKeys.filter((item) => item !== id),
    )
  } else {
    emit('update:selectedKeys', [...props.selectedKeys, id])
  }
}

const isRowSelected = (row: T) => props.selectedKeys.includes(getRowId(row))

const handleSort = (col: DataTableColumn) => {
  if (!col.sortable) return

  if (sortColumn.value === col.key) {
    if (sortDirection.value === 'asc') {
      sortDirection.value = 'desc'
    } else {
      sortColumn.value = null
      sortDirection.value = 'asc'
    }
  } else {
    sortColumn.value = col.key
    sortDirection.value = 'asc'
  }
}

const goToPage = (p: number) => {
  if (p >= 1 && p <= totalPages.value) {
    currentPage.value = p
  }
}

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

const visiblePages = computed(() => {
  const current = currentPage.value
  const total = totalPages.value
  const pages: (number | string)[] = []

  if (total <= 7) {
    for (let i = 1; i <= total; i++) pages.push(i)
    return pages
  }

  pages.push(1)
  if (current > 3) pages.push('...')

  const start = Math.max(2, current - 1)
  const end = Math.min(total - 1, current + 1)

  for (let i = start; i <= end; i++) pages.push(i)

  if (current < total - 2) pages.push('...')
  pages.push(total)

  return pages
})

const totalColumnsCount = computed(() => {
  let count = visibleColumns.value.length
  if (props.selectable) count++
  return count
})
</script>

<template>
  <div class="space-y-3 w-full">
    <div
      v-if="searchable || showColumnVisibility || exportable || $slots.toolbar"
      class="flex flex-col sm:flex-row sm:items-center justify-between gap-3"
    >
      <div class="flex items-center gap-2 flex-1 max-w-sm">
        <div v-if="searchable" class="relative w-full">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-muted-foreground">
            <IconSearch class="size-4" :stroke-width="2" />
          </div>
          <input
            v-model="searchQuery"
            type="text"
            :placeholder="searchPlaceholder"
            class="w-full bg-card border border-border text-foreground placeholder:text-muted-foreground/70 pl-9 pr-8 py-1.5 text-xs sm:text-sm rounded-lg outline-none focus:border-primary focus:ring-1 focus:ring-primary transition"
          />
          <button
            v-if="searchQuery"
            type="button"
            class="absolute inset-y-0 right-0 pr-2.5 flex items-center text-muted-foreground hover:text-foreground cursor-pointer"
            @click="searchQuery = ''"
          >
            <IconX class="size-3.5" :stroke-width="2" />
          </button>
        </div>
      </div>

      <div class="flex items-center gap-2 self-end sm:self-auto shrink-0">
        <div v-if="showColumnVisibility" class="relative">
          <button
            type="button"
            class="inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-foreground bg-card border border-border rounded-lg hover:bg-muted focus:outline-hidden transition cursor-pointer"
            @click="isColumnsMenuOpen = !isColumnsMenuOpen"
          >
            <IconColumns class="size-3.5 text-muted-foreground" :stroke-width="2" />
            <span>Columnas</span>
            <IconChevronDown class="size-3 text-muted-foreground transition-transform" :class="isColumnsMenuOpen ? 'rotate-180' : ''" :stroke-width="2" />
          </button>

          <div
            v-if="isColumnsMenuOpen"
            class="fixed inset-0 z-20"
            @click="isColumnsMenuOpen = false"
          ></div>

          <div
            v-if="isColumnsMenuOpen"
            class="absolute right-0 mt-1.5 w-48 bg-card border border-border rounded-xl shadow-lg p-2 z-30 text-xs space-y-1"
          >
            <div class="px-2 py-1 text-[11px] font-semibold text-muted-foreground uppercase tracking-wider border-b border-border">
              Visibilidad de columnas
            </div>
            <div class="max-h-48 overflow-y-auto py-1 space-y-1">
              <label
                v-for="col in columns"
                :key="col.key"
                class="flex items-center gap-2 px-2 py-1 rounded hover:bg-muted cursor-pointer select-none text-foreground"
              >
                <input
                  type="checkbox"
                  :checked="columnVisibilityMap[col.key] !== false"
                  class="size-3.5 rounded border-border text-primary focus:ring-primary/20"
                  @change="toggleColumnVisibility(col.key)"
                />
                <span class="truncate">{{ col.label }}</span>
              </label>
            </div>
          </div>
        </div>

        <DataTableExport
          v-if="exportable"
          :data="sortedData"
          :columns="visibleColumns.map(c => ({ key: c.key, label: c.label }))"
          :filename="exportFilename"
        />

        <slot name="toolbar" />
      </div>
    </div>

    <div
      v-if="selectable && selectedKeys.length > 0"
      class="flex items-center justify-between px-3 py-2 bg-primary/5 border border-primary/20 rounded-lg text-xs"
    >
      <span class="font-medium text-primary">
        {{ selectedKeys.length }} {{ selectedKeys.length === 1 ? 'registro seleccionado' : 'registros seleccionados' }}
      </span>
      <div class="flex items-center gap-2">
        <slot name="batch-actions" :selected-keys="selectedKeys" />
      </div>
    </div>

    <div
      class="bg-card rounded-xl border border-border overflow-hidden"
      :class="maxHeight ? 'overflow-y-auto' : ''"
      :style="maxHeight ? { maxHeight: maxHeight } : undefined"
    >
      <Table>
        <TableHeader :class="stickyHeader ? 'sticky top-0 z-10 bg-card shadow-xs' : ''">
          <TableRow :hoverable="false">
            <TableHead v-if="selectable" class="w-10 px-4">
              <div class="relative flex items-center justify-center">
                <input
                  type="checkbox"
                  :checked="isAllSelected"
                  :indeterminate="isSomeSelected"
                  class="size-4 rounded border-border text-primary focus:ring-primary/30 cursor-pointer"
                  @change="handleSelectAll"
                />
              </div>
            </TableHead>

            <TableHead
              v-for="col in visibleColumns"
              :key="col.key"
              :align="col.align"
              :style="col.width ? { width: col.width } : undefined"
              :class="[col.sortable ? 'cursor-pointer select-none hover:text-foreground' : '']"
              @click="handleSort(col)"
            >
              <div
                class="inline-flex items-center gap-1.5"
                :class="col.align === 'right' ? 'justify-end w-full' : col.align === 'center' ? 'justify-center w-full' : ''"
              >
                <span>{{ col.label }}</span>
                <span v-if="col.sortable" class="text-muted-foreground/70">
                  <IconChevronUp
                    v-if="sortColumn === col.key && sortDirection === 'asc'"
                    class="size-3.5 text-primary"
                    :stroke-width="2.5"
                  />
                  <IconChevronDown
                    v-else-if="sortColumn === col.key && sortDirection === 'desc'"
                    class="size-3.5 text-primary"
                    :stroke-width="2.5"
                  />
                  <IconSelector
                    v-else
                    class="size-3.5 opacity-40 hover:opacity-100"
                    :stroke-width="2"
                  />
                </span>
              </div>
            </TableHead>
          </TableRow>

          <TableRow v-if="columnFilters" :hoverable="false" class="bg-muted/20 border-t border-border">
            <TableHead v-if="selectable" class="w-10 px-4" />
            <TableHead
              v-for="col in visibleColumns"
              :key="`filter-${col.key}`"
              class="px-3 py-1.5"
            >
              <input
                v-if="col.filterable !== false"
                v-model="columnFiltersMap[col.key]"
                type="text"
                :placeholder="col.filterPlaceholder || `Filtrar ${col.label.toLowerCase()}...`"
                class="w-full bg-card border border-border text-foreground placeholder:text-muted-foreground/60 px-2 py-1 text-xs rounded outline-none focus:border-primary"
                @click.stop
              />
            </TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableEmpty
            v-if="loading"
            :colspan="totalColumnsCount"
          >
            <div class="flex items-center justify-center gap-2 py-6">
              <IconLoader2 class="animate-spin size-5 text-primary" :stroke-width="2" />
              <span class="text-sm font-medium text-foreground">Cargando registros...</span>
            </div>
          </TableEmpty>

          <TableEmpty
            v-else-if="!paginatedData.length"
            :colspan="totalColumnsCount"
            :message="emptyMessage"
          >
            <slot name="empty" />
          </TableEmpty>

          <TableRow
            v-for="(row, idx) in paginatedData"
            :key="String(getRowId(row))"
            :selected="isRowSelected(row)"
            @click="emit('row-click', row)"
          >
            <TableCell v-if="selectable" class="w-10 px-4" @click.stop>
              <div class="relative flex items-center justify-center">
                <input
                  type="checkbox"
                  :checked="isRowSelected(row)"
                  class="size-4 rounded border-border text-primary focus:ring-primary/30 cursor-pointer"
                  @change="toggleRowSelection(row)"
                />
              </div>
            </TableCell>

            <TableCell
              v-for="col in visibleColumns"
              :key="col.key"
              :align="col.align"
            >
              <slot
                :name="`cell-${col.key}`"
                :row="row"
                :value="row[col.key]"
                :index="idx"
              >
                {{ row[col.key] }}
              </slot>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>

      <div
        v-if="paginated && sortedData.length > 0"
        class="p-3 sm:p-4 border-t border-border flex flex-col sm:flex-row items-center justify-between gap-3 bg-muted/10 text-xs text-muted-foreground"
      >
        <div class="flex items-center gap-3">
          <span>
            Mostrando <strong class="text-foreground font-semibold">{{ startRecord }}</strong> a <strong class="text-foreground font-semibold">{{ endRecord }}</strong> de <strong class="text-foreground font-semibold">{{ sortedData.length }}</strong> resultados
          </span>

          <div class="flex items-center gap-1.5">
            <select
              v-model.number="currentPageSize"
              class="bg-card border border-border rounded-lg px-2 py-1 text-foreground text-xs outline-none focus:border-primary"
            >
              <option v-for="size in pageSizeOptions" :key="size" :value="size">
                {{ size }} por pág.
              </option>
            </select>
          </div>
        </div>

        <div v-if="paginationVariant === 'numbered'" class="flex items-center gap-1">
          <button
            type="button"
            class="p-1.5 rounded-lg border border-border bg-card text-foreground hover:bg-muted disabled:opacity-40 disabled:cursor-not-allowed transition"
            :disabled="currentPage <= 1"
            aria-label="Página anterior"
            @click="prevPage"
          >
            <IconChevronLeft class="size-4" :stroke-width="2" />
          </button>

          <template v-for="(p, i) in visiblePages" :key="i">
            <span v-if="p === '...'" class="px-2 text-muted-foreground">...</span>
            <button
              v-else
              type="button"
              class="min-w-8 h-8 px-2 rounded-lg text-xs font-semibold transition"
              :class="currentPage === p ? 'bg-primary text-primary-foreground' : 'bg-card border border-border text-foreground hover:bg-muted'"
              @click="goToPage(Number(p))"
            >
              {{ p }}
            </button>
          </template>

          <button
            type="button"
            class="p-1.5 rounded-lg border border-border bg-card text-foreground hover:bg-muted disabled:opacity-40 disabled:cursor-not-allowed transition"
            :disabled="currentPage >= totalPages"
            aria-label="Página siguiente"
            @click="nextPage"
          >
            <IconChevronRight class="size-4" :stroke-width="2" />
          </button>
        </div>

        <div v-else class="flex items-center gap-1">
          <Button
            variant="outline"
            size="xs"
            :disabled="currentPage <= 1"
            @click="prevPage"
          >
            Anterior
          </Button>

          <span class="px-2 font-medium text-foreground">
            {{ currentPage }} / {{ totalPages }}
          </span>

          <Button
            variant="outline"
            size="xs"
            :disabled="currentPage >= totalPages"
            @click="nextPage"
          >
            Siguiente
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
