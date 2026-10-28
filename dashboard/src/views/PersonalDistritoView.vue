<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import CardPersonalPorArea from '@/components/dashboard/CardPersonalPorArea.vue'
import CardDemografiaEdad from '@/components/dashboard/CardDemografiaEdad.vue'
import {
  fetchActivosPorDistrito,
  fetchPersonalPorDistrito,
  type DetalleDistrito,
  type PersonaDistrito,
} from '@/services/personal'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import { formatDate } from '@/utils/date'
import {
  IconArrowLeft,
  IconRefresh,
  IconAlertTriangle,
  IconMapPin,
  IconUsers,
} from '@tabler/icons-vue'

interface FilaDistrito {
  dni: string
  nombre: string
  ingreso: string | null
  direccion: string | null
  area: string
  cargo: string
  regimen: string
  sindicato: string | null
  distrito: string | null
  avatar: string | null
  raw: PersonaDistrito
}

const router = useRouter()
const route = useRoute()

const distritosOpciones = ref<SelectOption<string>[]>([])
const selectedDistrito = ref<string>('')

const detalle = ref<DetalleDistrito | null>(null)
const isLoadingOpciones = ref<boolean>(true)
const isLoadingDetalle = ref<boolean>(false)
const errorMessage = ref<string | null>(null)

const estaCargando = computed(() => isLoadingOpciones.value || isLoadingDetalle.value)

const irAlPerfil = (dni: string) => {
  if (!dni) return
  router.push(`/perfil/${dni}`)
}

const columns: DataTableColumn[] = [
  {
    key: 'nombre',
    label: 'Servidor',
    sortable: true,
  },
  {
    key: 'dni',
    label: 'DNI',
    sortable: true,
    width: '120px',
  },
  {
    key: 'area',
    label: 'Área',
    sortable: true,
  },
  {
    key: 'cargo',
    label: 'Cargo',
    sortable: true,
  },
  {
    key: 'regimen',
    label: 'Régimen',
    sortable: true,
    width: '140px',
  },
  {
    key: 'ingreso',
    label: 'Ingreso',
    sortable: true,
    width: '130px',
  },
]

const searchKeys = ['nombre', 'dni']

const filasPlanas = computed<FilaDistrito[]>(() => {
  if (!detalle.value?.personas) return []
  return detalle.value.personas.map((p) => ({
    dni: p.dni,
    nombre: p.nombre,
    ingreso: p.ingreso,
    direccion: p.direccion,
    area: p.area?.nombre || '—',
    cargo: p.cargo?.nombre || '—',
    regimen: p.regimen?.nombre || '',
    sindicato: p.sindicato,
    distrito: p.distrito,
    avatar: p.avatar,
    raw: p,
  }))
})

const areasParaGrafico = computed(() => {
  if (!detalle.value?.areas) return []
  return detalle.value.areas.map((a) => ({
    nombre: a.nombre,
    cantidad: a.cantidad,
  }))
})

const areaConMasPersonal = computed(() => {
  if (!detalle.value?.areas || detalle.value.areas.length === 0) return null
  return [...detalle.value.areas].sort((a, b) => b.cantidad - a.cantidad)[0] || null
})

const areaConMasPersonalCantidad = computed(() => {
  if (!areaConMasPersonal.value) return 'Sin registros'
  const cant = areaConMasPersonal.value.cantidad
  return `${cant} ${cant === 1 ? 'trabajador' : 'trabajadores'}`
})

const rangoEdadPredominante = computed(() => {
  if (!detalle.value?.rangos_edad || detalle.value.rangos_edad.length === 0) return null
  return [...detalle.value.rangos_edad].sort((a, b) => b.cantidad - a.cantidad)[0] || null
})

const rangoEdadPredominanteCantidad = computed(() => {
  if (!rangoEdadPredominante.value) return 'Sin registros'
  const cant = rangoEdadPredominante.value.cantidad
  return `${cant} ${cant === 1 ? 'trabajador' : 'trabajadores'}`
})

const exportFilename = computed(() => {
  const nombre = selectedDistrito.value
    ? selectedDistrito.value.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')
    : 'general'
  return `trabajadores-distrito-${nombre}`
})

const cargarOpcionesDistritos = async () => {
  isLoadingOpciones.value = true
  try {
    const data = await fetchActivosPorDistrito()
    const ordenados = [...data].sort((a, b) => b.cantidad - a.cantidad)
    distritosOpciones.value = ordenados.map((item) => ({
      value: item.distrito,
      label: `${item.distrito} (${item.cantidad})`,
    }))
  } catch (err: unknown) {
    console.error('Error cargando distritos:', err)
  } finally {
    isLoadingOpciones.value = false
  }
}

const cargarDetalleDistrito = async (distrito: string) => {
  if (!distrito) {
    detalle.value = null
    return
  }

  isLoadingDetalle.value = true
  errorMessage.value = null
  try {
    detalle.value = await fetchPersonalPorDistrito(distrito)
  } catch (err: unknown) {
    if (err instanceof Error) {
      errorMessage.value = err.message
    } else {
      errorMessage.value = 'Error al cargar los trabajadores del distrito.'
    }
  } finally {
    isLoadingDetalle.value = false
  }
}

const actualizarVista = async () => {
  await cargarOpcionesDistritos()
  if (selectedDistrito.value) {
    await cargarDetalleDistrito(selectedDistrito.value)
  }
}

const seleccionarDistrito = (distrito: string | '') => {
  selectedDistrito.value = distrito || ''
  router.replace({
    query: {
      ...route.query,
      distrito: distrito || undefined,
    },
  })
}

watch(
  () => selectedDistrito.value,
  (nuevoDistrito) => {
    if (nuevoDistrito) {
      cargarDetalleDistrito(nuevoDistrito)
    } else {
      detalle.value = null
      errorMessage.value = null
    }
  }
)

watch(
  () => route.query.distrito,
  (nuevoQuery) => {
    const val = typeof nuevoQuery === 'string' ? nuevoQuery : ''
    if (val !== selectedDistrito.value) {
      selectedDistrito.value = val
    }
  }
)

onMounted(async () => {
  await cargarOpcionesDistritos()
  const qDistrito = route.query.distrito
  if (typeof qDistrito === 'string' && qDistrito.trim()) {
    selectedDistrito.value = qDistrito.trim()
  }
})
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <div class="flex items-center justify-between gap-4 border-b border-border/70 pb-3">
      <div class="flex items-center gap-2.5 min-w-0">
        <button
          type="button"
          class="size-8 inline-flex items-center justify-center rounded-md text-muted-foreground hover:text-foreground hover:bg-muted/80 transition-colors cursor-pointer shrink-0"
          title="Volver"
          aria-label="Volver a la página anterior"
          @click="router.back()"
        >
          <IconArrowLeft class="size-4" aria-hidden="true" />
        </button>
        <div class="h-4 w-px bg-border shrink-0" aria-hidden="true"></div>
        <div class="min-w-0">
          <h1 class="text-sm sm:text-base font-semibold tracking-tight text-foreground truncate">
            Trabajadores por Distrito
          </h1>
          <p class="text-xs text-muted-foreground truncate hidden sm:block">
            Personal activo según distrito de residencia
          </p>
        </div>
      </div>
      <div class="flex items-center gap-2 shrink-0">
        <Button
          variant="ghost"
          size="sm"
          class="gap-1.5 text-xs text-muted-foreground hover:text-foreground cursor-pointer"
          :disabled="estaCargando"
          @click="actualizarVista"
        >
          <IconRefresh class="size-3.5" :class="estaCargando ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </div>
    </div>

    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div class="w-full sm:w-96">
        <SearchSelect
          :model-value="selectedDistrito"
          :options="distritosOpciones"
          placeholder="Seleccionar distrito..."
          search-placeholder="Buscar distrito..."
          empty-text="No se encontraron distritos"
          :allow-clear="true"
          size="md"
          @update:model-value="seleccionarDistrito"
        />
      </div>
    </div>

    <Card v-if="!selectedDistrito" class="p-12 text-center">
      <div class="max-w-sm mx-auto space-y-3">
        <div class="size-12 rounded-full bg-primary/10 text-primary mx-auto flex items-center justify-center">
          <IconMapPin class="size-6" />
        </div>
        <h3 class="text-sm font-semibold text-foreground">Seleccione un distrito</h3>
        <p class="text-xs text-muted-foreground">
          Elija un distrito del selector superior para ver el resumen y los trabajadores activos que residen allí.
        </p>
      </div>
    </Card>

    <div
      v-else-if="errorMessage"
      class="p-8 text-center rounded-2xl border border-rose-500/20 bg-rose-500/5 space-y-3"
      role="alert"
    >
      <IconAlertTriangle class="size-8 mx-auto text-rose-500" />
      <p class="text-sm font-bold text-rose-600 dark:text-rose-400">Error al cargar los datos</p>
      <p class="text-xs text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-xs cursor-pointer"
        @click="cargarDetalleDistrito(selectedDistrito)"
      >
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else-if="isLoadingDetalle" class="space-y-6 animate-pulse" aria-busy="true">
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <div class="h-20 bg-muted/40 rounded-xl"></div>
        <div class="h-20 bg-muted/40 rounded-xl"></div>
        <div class="h-20 bg-muted/40 rounded-xl"></div>
      </div>
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
        <div class="h-72 bg-muted/40 rounded-xl"></div>
        <div class="h-72 bg-muted/40 rounded-xl"></div>
      </div>
      <div class="h-64 bg-muted/40 rounded-xl"></div>
    </div>

    <template v-else-if="detalle">
      <div class="space-y-4">
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Total trabajadores</p>
            <div class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1">
              {{ detalle.total }}
            </div>
            <p class="text-meta text-muted-foreground mt-0.5">
              {{ detalle.total === 1 ? 'servidor activo' : 'servidores activos' }}
            </p>
          </Card>

          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Área con más personal</p>
            <div
              class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1 truncate"
              :title="areaConMasPersonal?.nombre || '—'"
            >
              {{ areaConMasPersonal?.nombre || '—' }}
            </div>
            <p class="text-meta text-muted-foreground mt-0.5 truncate">
              {{ areaConMasPersonalCantidad }}
            </p>
          </Card>

          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Rango de edad predominante</p>
            <div
              class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1 truncate"
              :title="rangoEdadPredominante?.nombre || '—'"
            >
              {{ rangoEdadPredominante?.nombre || '—' }}
            </div>
            <p class="text-meta text-muted-foreground mt-0.5 truncate">
              {{ rangoEdadPredominanteCantidad }}
            </p>
          </Card>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
          <CardPersonalPorArea
            :areas="areasParaGrafico"
            :is-loading="isLoadingDetalle"
            chart-height="h-60"
          />
          <CardDemografiaEdad
            :rangos="detalle.rangos_edad || []"
            :is-loading="isLoadingDetalle"
            chart-height="h-60"
          />
        </div>
      </div>

      <DataTable
        :columns="columns"
        :data="filasPlanas"
        row-key="dni"
        searchable
        search-placeholder="Buscar por nombre o DNI..."
        :search-keys="searchKeys"
        paginated
        :page-size="10"
        :page-size-options="[10, 20, 50, 100]"
        exportable
        :export-filename="exportFilename"
        :loading="isLoadingDetalle"
        empty-message="No se encontraron trabajadores en este distrito"
        @row-click="(row) => irAlPerfil(row.dni)"
      >
        <template #cell-nombre="{ row }">
          <div class="flex items-center gap-3">
            <div
              class="size-8 rounded-full bg-primary/10 border border-border overflow-hidden shrink-0 flex items-center justify-center font-bold text-xs text-primary"
            >
              <img
                v-if="row.avatar || row.dni"
                v-auth-src="resolveAvatarUrl(row.dni)!"
                :alt="row.nombre"
                class="size-full object-cover"
              />
            </div>
            <router-link
              :to="`/perfil/${row.dni}`"
              class="font-semibold text-foreground hover:text-primary transition-colors text-xs truncate max-w-xs"
              :title="row.nombre"
              @click.stop
            >
              {{ row.nombre }}
            </router-link>
          </div>
        </template>

        <template #cell-dni="{ row }">
          <span class="font-mono text-xs text-muted-foreground">
            {{ row.dni }}
          </span>
        </template>

        <template #cell-area="{ row }">
          <span class="text-xs text-muted-foreground truncate block max-w-50" :title="row.area">
            {{ row.area }}
          </span>
        </template>

        <template #cell-cargo="{ row }">
          <span class="text-xs text-muted-foreground truncate block max-w-45" :title="row.cargo">
            {{ row.cargo }}
          </span>
        </template>

        <template #cell-regimen="{ row }">
          <Badge v-if="row.regimen" variant="secondary" size="xs">
            {{ row.regimen }}
          </Badge>
          <span v-else class="text-muted-foreground text-xs">—</span>
        </template>

        <template #cell-ingreso="{ row }">
          <span class="text-xs font-mono text-muted-foreground whitespace-nowrap">
            {{ row.ingreso ? formatDate(row.ingreso) : '—' }}
          </span>
        </template>

        <template #empty>
          <div class="p-10 text-center space-y-2">
            <IconUsers class="size-8 mx-auto text-muted-foreground/60" />
            <h3 class="text-sm font-semibold text-foreground">No se encontraron trabajadores</h3>
            <p class="text-xs text-muted-foreground">
              No hay servidores activos registrados para este distrito con los criterios seleccionados.
            </p>
          </div>
        </template>
      </DataTable>
    </template>
  </div>
</template>
