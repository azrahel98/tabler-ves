<script setup lang="ts">
import { ref, computed } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import {
  resolveAvatarUrl,
  type TrabajadorNuevo,
} from '@/services/dashboard'
import { formatDate } from '@/utils/date'
import {
  IconUsers,
  IconSearch,
  IconX,
  IconUserOff,
  IconChevronLeft,
  IconChevronRight,
  IconFileText,
} from '@tabler/icons-vue'

interface Props {
  nuevos: TrabajadorNuevo[]
  isLoading?: boolean
  selectedArea?: string | null
  selectedRegimen?: string | null
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  selectedArea: null,
  selectedRegimen: null,
})

const emit = defineEmits<{
  (e: 'clear-filter'): void
}>()

const searchQuery = ref('')
const currentPage = ref(1)
const pageSize = ref(8)

const filteredList = computed(() => {
  let list = props.nuevos
  if (props.selectedArea) {
    list = list.filter((t) => t.area.toLowerCase().includes(props.selectedArea!.toLowerCase()))
  }
  if (props.selectedRegimen) {
    list = list.filter((t) => t.regimen.toLowerCase().includes(props.selectedRegimen!.toLowerCase()))
  }
  if (searchQuery.value.trim()) {
    const q = searchQuery.value.trim().toLowerCase()
    list = list.filter((t) =>
      t.nombre.toLowerCase().includes(q) ||
      t.dni.includes(q) ||
      t.cargo.toLowerCase().includes(q) ||
      t.area.toLowerCase().includes(q)
    )
  }
  return list
})

const totalItems = computed(() => filteredList.value.length)
const totalPages = computed(() => Math.max(1, Math.ceil(totalItems.value / pageSize.value)))

const paginatedList = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  return filteredList.value.slice(start, start + pageSize.value)
})

const startRecord = computed(() => {
  if (totalItems.value === 0) return 0
  return (currentPage.value - 1) * pageSize.value + 1
})

const endRecord = computed(() => {
  return Math.min(currentPage.value * pageSize.value, totalItems.value)
})

const onSearchChange = () => {
  currentPage.value = 1
}

const clearSearch = () => {
  searchQuery.value = ''
  currentPage.value = 1
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

const regimenBadgeVariant = (regimen: string): 'primary' | 'success' | 'warning' | 'secondary' | 'neutral' => {
  if (regimen.includes('276')) return 'primary'
  if (regimen.includes('1057')) return 'success'
  if (regimen.includes('728')) return 'warning'
  return 'secondary'
}
</script>

<template>
  <Card :no-padding="true" class="shadow-2xs w-full overflow-hidden border border-border bg-card">
    <div class="flex flex-col gap-3.5 border-b border-border p-4 sm:px-5 lg:flex-row lg:items-center lg:justify-between">
      <div class="flex items-center gap-3">
        <div class="flex size-7 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
          <IconUsers class="size-4" aria-hidden="true" />
        </div>
        <div>
  
                    <h3 class="font-semibold text-foreground tracking-tight text-sm">    Nómina de Personal e Incorporacionesa</h3>
          <p class="text-xs text-muted-foreground">
            Altas recientes y servidores en gestión activa
          </p>
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-2.5">
        <div class="relative w-full sm:w-64 lg:w-72">
          <IconSearch class="pointer-events-none absolute left-3 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground" aria-hidden="true" />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Buscar por DNI, nombre, área..."
            class="h-8.5 w-full rounded-lg border border-border bg-muted/30 pl-8.5 pr-8 text-xs text-foreground placeholder:text-muted-foreground transition-colors focus:border-primary focus:outline-hidden focus:ring-1 focus:ring-primary"
            @input="onSearchChange"
          />
          <button
            v-if="searchQuery"
            type="button"
            class="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground focus-visible:outline-hidden"
            aria-label="Limpiar búsqueda"
            @click="clearSearch"
          >
            <IconX class="size-3.5" aria-hidden="true" />
          </button>
        </div>

        <div v-if="selectedArea || selectedRegimen" class="flex items-center gap-1">
          <Badge variant="primary" size="xs">
            Filtro: {{ selectedArea || selectedRegimen }}
          </Badge>
          <button
            type="button"
            class="flex size-6 items-center justify-center rounded-full text-muted-foreground transition-colors hover:bg-muted hover:text-foreground focus-visible:outline-hidden"
            title="Quitar filtro"
            aria-label="Quitar filtro activo"
            @click="emit('clear-filter')"
          >
            <IconX class="size-3.5" aria-hidden="true" />
          </button>
        </div>

        <Badge variant="success" size="xs" dot>
          {{ totalItems }} {{ totalItems === 1 ? 'Servidor' : 'Servidores' }}
        </Badge>
      </div>
    </div>

    <div v-if="isLoading" class="space-y-3.5 p-5 animate-pulse" aria-busy="true">
      <div v-for="i in 5" :key="i" class="h-11 rounded-lg bg-muted/60"></div>
    </div>

    <div
      v-else-if="totalItems === 0"
      class="flex flex-col items-center justify-center px-4 py-14 text-center"
    >
      <div class="flex size-11 items-center justify-center rounded-full bg-muted/60 text-muted-foreground mb-3">
        <IconUserOff class="size-5.5" aria-hidden="true" />
      </div>
      <p class="text-sm font-semibold text-foreground">
        No se encontraron trabajadores
      </p>
      <p class="mt-1 max-w-sm text-xs text-muted-foreground leading-relaxed">
        {{ (selectedArea || selectedRegimen || searchQuery) ? 'No existen registros que coincidan con la búsqueda o el filtro aplicado.' : 'No se registran altas de personal en el período consultado.' }}
      </p>
      <div v-if="selectedArea || selectedRegimen || searchQuery" class="mt-4">
        <Button variant="outline" size="sm" class="gap-1.5 text-xs" @click="clearSearch(); emit('clear-filter')">
          <IconX class="size-3.5" aria-hidden="true" />
          Restablecer listado
        </Button>
      </div>
    </div>

    <div v-else class="overflow-x-auto">
      <table class="w-full text-left text-xs text-muted-foreground" aria-label="Tabla de nómina de personal e incorporaciones">
        <thead class="border-b border-border bg-muted/30 text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">
          <tr>
            <th scope="col" class="px-5 py-3 font-semibold">Servidor</th>
            <th scope="col" class="px-5 py-3 font-semibold">Área / Gerencia</th>
            <th scope="col" class="px-5 py-3 font-semibold">Cargo Funcional</th>
            <th scope="col" class="px-5 py-3 font-semibold">Régimen & Plaza</th>
            <th scope="col" class="px-5 py-3 font-semibold">Documento</th>
            <th scope="col" class="px-5 py-3 text-right font-semibold">Fecha Ingreso</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-border">
          <tr
            v-for="t in paginatedList"
            :key="t.id"
            tabindex="0"
            class="transition-colors hover:bg-muted/30 focus-visible:bg-muted/40 focus-visible:outline-hidden"
          >
            <td class="px-5 py-3.5">
              <router-link
                :to="{ name: 'perfil', params: { dni: t.dni } }"
                class="flex items-center gap-3 group cursor-pointer"
              >
                <img
                  v-if="resolveAvatarUrl(t.avatar)"
                  :src="resolveAvatarUrl(t.avatar)!"
                  :alt="t.nombre"
                  class="size-8.5 rounded-full border border-border object-cover shrink-0 shadow-2xs group-hover:border-primary transition-colors"
                />
                <div
                  v-else
                  class="flex size-8.5 shrink-0 items-center justify-center rounded-full bg-primary/10 font-bold text-xs text-primary border border-primary/20 group-hover:border-primary transition-colors"
                >
                  {{ t.nombre.charAt(0) }}
                </div>
                <div class="min-w-0">
                  <div class="text-xs font-medium text-foreground group-hover:text-primary transition-colors wrap-break-word">
                    {{ t.nombre }}
                  </div>
                  <div class="font-mono text-[11px] text-muted-foreground">
                    DNI: {{ t.dni }}
                  </div>
                </div>
              </router-link>
            </td>
            <td class="px-5 py-3.5">
              <div class="text-xs font-medium text-foreground wrap-break-word" :title="t.area">
                {{ t.area }}
              </div>
            </td>
            <td class="px-5 py-3.5">
              <div class="text-xs text-muted-foreground wrap-break-word" :title="t.cargo">
                {{ t.cargo }}
              </div>
            </td>
            <td class="px-5 py-3.5">
              <div class="flex flex-col items-start gap-1">
                <Badge :variant="regimenBadgeVariant(t.regimen)" size="xs">
                  {{ t.regimen }}
                </Badge>
                <span class="font-mono text-[11px] text-muted-foreground" :title="'Plaza Presupuestada: ' + t.plaza">
                  Plaza: {{ t.plaza }}
                </span>
              </div>
            </td>
            <td class="px-5 py-3.5">
              <Badge variant="outline" size="xs" class="font-mono gap-1">
                <IconFileText class="size-3" aria-hidden="true" />
                {{ t.documento }}
              </Badge>
            </td>
            <td class="px-5 py-3.5 text-right font-mono text-xs font-semibold text-foreground tabular-nums">
              {{ formatDate(t.ingreso) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div
      v-if="!isLoading && totalItems > 0"
      class="flex flex-col items-center justify-between gap-3 border-t border-border bg-muted/15 px-5 py-3 sm:flex-row text-xs text-muted-foreground"
    >
      <div>
        Mostrando <span class="font-semibold text-foreground font-mono">{{ startRecord }}</span> a <span class="font-semibold text-foreground font-mono">{{ endRecord }}</span> de <span class="font-semibold text-foreground font-mono">{{ totalItems }}</span> registros
      </div>

      <div v-if="totalPages > 1" class="flex items-center gap-1.5">
        <Button
          variant="outline"
          size="xs"
          :disabled="currentPage === 1"
          aria-label="Página anterior"
          @click="prevPage"
        >
          <IconChevronLeft class="size-3.5" aria-hidden="true" />
          Anterior
        </Button>

        <span class="px-2 text-xs font-medium text-foreground font-mono">
          {{ currentPage }} / {{ totalPages }}
        </span>

        <Button
          variant="outline"
          size="xs"
          :disabled="currentPage === totalPages"
          aria-label="Página siguiente"
          @click="nextPage"
        >
          Siguiente
          <IconChevronRight class="size-3.5" aria-hidden="true" />
        </Button>
      </div>
    </div>
  </Card>
</template>
