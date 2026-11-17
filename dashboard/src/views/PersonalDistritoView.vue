<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import PageHeader from '@/components/ui/page-header/PageHeader.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import DistritoResumen from '@/components/distrito/DistritoResumen.vue'
import {
  fetchActivosPorDistrito,
  fetchPersonalPorDistrito,
  type DetalleDistrito,
  type PersonaDistrito,
} from '@/services/personal'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import {
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
  { key: 'nombre', label: 'Servidor', sortable: true },
  { key: 'area', label: 'Área', sortable: true },
  { key: 'direccion', label: 'Dirección', sortable: true },
]

const searchKeys = ['nombre', 'dni', 'cargo', 'area', 'direccion']

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

const regimenes = computed(() => {
  const conteo = new Map<string, number>()
  for (const p of detalle.value?.personas ?? []) {
    const n = p.regimen?.nombre
    if (n) conteo.set(n, (conteo.get(n) ?? 0) + 1)
  }
  return [...conteo].map(([nombre, cantidad]) => ({ nombre, cantidad }))
})

const sindicatos = computed(() => {
  const conteo = new Map<string, number>()
  for (const p of detalle.value?.personas ?? []) {
    const n = p.sindicato?.trim() || 'No afiliado'
    conteo.set(n, (conteo.get(n) ?? 0) + 1)
  }
  return [...conteo].map(([nombre, cantidad]) => ({ nombre, cantidad }))
})

const irAArea = (area: string) => router.push({ path: '/personal/area', query: { area } })

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
  <div class="mx-auto w-full max-w-6xl space-y-3 px-4 pb-12">
    <PageHeader title="Trabajadores por Distrito" subtitle="Personal activo según distrito de residencia">
      <template #actions>
        <Button
          variant="ghost"
          size="sm"
          class="gap-1.5 text-meta text-muted-foreground hover:text-foreground cursor-pointer"
          :disabled="estaCargando"
          @click="actualizarVista"
        >
          <IconRefresh class="size-3.5" :class="estaCargando ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </template>
    </PageHeader>

    <div class="w-full sm:w-80">
      <SearchSelect
        :model-value="selectedDistrito"
        :options="distritosOpciones"
        placeholder="Seleccionar distrito..."
        search-placeholder="Buscar distrito..."
        empty-text="No se encontraron distritos"
        :allow-clear="true"
        size="sm"
        @update:model-value="seleccionarDistrito"
      />
    </div>

    <Card v-if="!selectedDistrito" class="p-8 text-center">
      <div class="max-w-sm mx-auto space-y-3">
        <div class="size-9 rounded-full bg-primary/10 text-primary mx-auto flex items-center justify-center">
          <IconMapPin class="size-4" />
        </div>
        <h3 class="text-meta font-semibold text-foreground">Seleccione un distrito</h3>
        <p class="text-meta text-muted-foreground">
          Elija un distrito del selector superior para ver el resumen y los trabajadores activos que residen allí.
        </p>
      </div>
    </Card>

    <div
      v-else-if="errorMessage"
      class="p-4 text-center rounded-xl border border-rose-500/20 bg-rose-500/5 space-y-3"
      role="alert"
    >
      <IconAlertTriangle class="size-8 mx-auto text-rose-500" />
      <p class="text-xs font-bold text-rose-600 dark:text-rose-400">Error al cargar los datos</p>
      <p class="text-meta text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button
        variant="outline"
        size="sm"
        class="gap-1.5 text-meta cursor-pointer"
        @click="cargarDetalleDistrito(selectedDistrito)"
      >
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else-if="isLoadingDetalle" class="space-y-3 animate-pulse" aria-busy="true">
      <div class="h-44 bg-muted/40 rounded-xl"></div>
      <div class="h-64 bg-muted/40 rounded-xl"></div>
    </div>

    <template v-else-if="detalle">
      <DistritoResumen
        :distrito="selectedDistrito"
        :total="detalle.total"
        :areas="detalle.areas"
        :rangos="detalle.rangos_edad"
        :regimenes="regimenes"
        :sindicatos="sindicatos"
        @select-area="irAArea"
      />

      <DataTable
        :columns="columns"
        :data="filasPlanas"
        row-key="dni"
        searchable
        search-placeholder="Buscar por nombre, DNI, área o dirección..."
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
          <div class="flex items-center gap-2.5 min-w-0">
            <div class="size-8 rounded-full bg-muted overflow-hidden shrink-0">
              <img
                v-if="row.avatar || row.dni"
                v-auth-src="resolveAvatarUrl(row.dni)!"
                :alt="row.nombre"
                class="size-full object-cover"
              />
            </div>
            <div class="min-w-0">
              <router-link
                :to="`/perfil/${row.dni}`"
                class="block font-semibold text-foreground hover:text-primary transition-colors text-meta truncate max-w-xs"
                :title="row.nombre"
                @click.stop
              >
                {{ row.nombre }}
              </router-link>
              <p class="text-micro text-muted-foreground truncate max-w-xs">
                {{ row.cargo }} · <span class="font-mono">{{ row.dni }}</span>
              </p>
            </div>
          </div>
        </template>

        <template #cell-area="{ row }">
          <span class="text-meta text-muted-foreground truncate block max-w-56" :title="row.area">{{ row.area }}</span>
        </template>

        <template #cell-direccion="{ row }">
          <span class="text-meta text-muted-foreground line-clamp-2 max-w-64" :title="row.direccion || undefined">
            {{ row.direccion || '—' }}
          </span>
        </template>

        <template #empty>
          <div class="p-10 text-center space-y-2">
            <IconUsers class="size-8 mx-auto text-muted-foreground/60" />
            <h3 class="text-xs font-semibold text-foreground">No se encontraron trabajadores</h3>
            <p class="text-meta text-muted-foreground">
              No hay servidores activos registrados para este distrito con los criterios seleccionados.
            </p>
          </div>
        </template>
      </DataTable>
    </template>
  </div>
</template>
