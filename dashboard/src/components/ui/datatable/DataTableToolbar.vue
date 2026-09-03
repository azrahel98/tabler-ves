<script setup lang="ts">
import { ref } from 'vue'
import {
  IconSearch,
  IconX,
  IconColumns,
  IconChevronDown,
} from '@tabler/icons-vue'
import DataTableExport from './DataTableExport.vue'

interface ColumnVisibilityOption {
  key: string
  label: string
  visible: boolean
}

const props = withDefaults(
  defineProps<{
    searchable?: boolean
    searchPlaceholder?: string
    showColumnVisibility?: boolean
    columns?: ColumnVisibilityOption[]
    exportable?: boolean
    exportData?: Record<string, any>[]
    exportFilename?: string
  }>(),
  {
    searchable: true,
    searchPlaceholder: 'Buscar registros...',
    showColumnVisibility: false,
    columns: () => [],
    exportable: false,
    exportData: () => [],
    exportFilename: 'export',
  }
)

const emit = defineEmits<{
  (e: 'update:search', query: string): void
  (e: 'toggle-column', key: string): void
}>()

const searchQuery = ref('')
const isColumnsMenuOpen = ref(false)

const handleSearchInput = (e: Event) => {
  const target = e.target as HTMLInputElement
  searchQuery.value = target.value
  emit('update:search', target.value)
}

const clearSearch = () => {
  searchQuery.value = ''
  emit('update:search', '')
}
</script>

<template>
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 w-full">
    <div class="flex items-center gap-2 flex-1 max-w-md">
      <div v-if="searchable" class="relative w-full">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-muted-foreground">
          <IconSearch class="size-4" :stroke-width="2" />
        </div>
        <input
          :value="searchQuery"
          type="text"
          :placeholder="searchPlaceholder"
          class="w-full bg-card border border-border text-foreground placeholder:text-muted-foreground/70 pl-9 pr-8 py-1.5 text-xs sm:text-sm rounded-lg outline-none focus:border-primary focus:ring-1 focus:ring-primary transition"
          @input="handleSearchInput"
        />
        <button
          v-if="searchQuery"
          type="button"
          class="absolute inset-y-0 right-0 pr-2.5 flex items-center text-muted-foreground hover:text-foreground cursor-pointer"
          @click="clearSearch"
        >
          <IconX class="size-3.5" :stroke-width="2" />
        </button>
      </div>

      <slot name="filters" />
    </div>

    <div class="flex items-center gap-2 self-end sm:self-auto shrink-0">
      <div v-if="showColumnVisibility && columns.length" class="relative">
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
            Columnas Visibles
          </div>
          <div class="max-h-48 overflow-y-auto py-1 space-y-1">
            <label
              v-for="col in columns"
              :key="col.key"
              class="flex items-center gap-2 px-2 py-1 rounded hover:bg-muted cursor-pointer select-none text-foreground"
            >
              <input
                type="checkbox"
                :checked="col.visible"
                class="size-3.5 rounded border-border text-primary focus:ring-primary/20"
                @change="emit('toggle-column', col.key)"
              />
              <span class="truncate">{{ col.label }}</span>
            </label>
          </div>
        </div>
      </div>

      <DataTableExport
        v-if="exportable"
        :data="exportData"
        :columns="columns.map(c => ({ key: c.key, label: c.label }))"
        :filename="exportFilename"
      />

      <slot name="actions" />
    </div>
  </div>
</template>
