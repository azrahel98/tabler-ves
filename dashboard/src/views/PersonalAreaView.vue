<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute, type LocationQueryRaw } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import SearchSelect, { type SelectOption } from '@/components/ui/form/SearchSelect.vue'
import CardDemografiaRegimen from '@/components/dashboard/CardDemografiaRegimen.vue'
import CardConteoBarras from '@/components/dashboard/CardConteoBarras.vue'
import {
  fetchAreas,
  fetchPersonalPorArea,
  type PersonaArea,
} from '@/services/personal'
import { resolveAvatarUrl, type ResumenPersonal } from '@/components/dashboard/types'
import { formatDate } from '@/utils/date'
import {
  IconArrowLeft,
  IconRefresh,
  IconAlertTriangle,
  IconBuildingCommunity,
  IconUsers,
} from '@tabler/icons-vue'

interface FilaArea {
  dni: string
  nombre: string
  ingreso: string | null
  renuncia: string | null
  area: string
  cargo: string
  regimen: string
  sindicato: string
  avatar: string | null
  raw: PersonaArea
}

const router = useRouter()
const route = useRoute()

const areasOpciones = ref<SelectOption<number>[]>([])
const selectedAreaId = ref<number | ''>('')
const selectedAreaNombreDirecto = ref<string>('')

const personal = ref<PersonaArea[]>([])
const isLoadingAreas = ref<boolean>(true)
const isLoadingPersonal = ref<boolean>(false)
const errorMessage = ref<string | null>(null)

const estaCargando = computed(() => isLoadingAreas.value || isLoadingPersonal.value)

const tieneAreaSeleccionada = computed(() => {
  return (typeof selectedAreaId.value === 'number' && selectedAreaId.value > 0) || Boolean(selectedAreaNombreDirecto.value)
})

const areaSeleccionadaNombre = computed(() => {
  if (selectedAreaNombreDirecto.value) {
    return selectedAreaNombreDirecto.value
  }
  if (!selectedAreaId.value) return ''
  const opt = areasOpciones.value.find((o) => o.value === selectedAreaId.value)
  return opt ? opt.label : ''
})

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
    key: 'sindicato',
    label: 'Sindicato',
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

const filasPlanas = computed<FilaArea[]>(() => {
  return personal.value.map((p) => ({
    dni: p.dni,
    nombre: p.nombre,
    ingreso: p.ingreso,
    renuncia: p.renuncia,
    area: p.area || '—',
    cargo: p.cargo || '—',
    regimen: p.regimen || '',
    sindicato: p.sindicato || '',
    avatar: p.avatar,
    raw: p,
  }))
})

const conteoPorRegimen = computed<{ nombre: string; cantidad: number }[]>(() => {
  const mapa = new Map<string, number>()
  for (const p of personal.value) {
    const reg = (p.regimen && p.regimen.trim()) || 'Sin régimen'
    mapa.set(reg, (mapa.get(reg) || 0) + 1)
  }
  return Array.from(mapa.entries()).map(([nombre, cantidad]) => ({
    nombre,
    cantidad,
  })).sort((a, b) => b.cantidad - a.cantidad)
})

const regimenPredominante = computed(() => {
  if (conteoPorRegimen.value.length === 0) return null
  return conteoPorRegimen.value[0]
})

const regimenPredominanteDetalle = computed(() => {
  if (!regimenPredominante.value || personal.value.length === 0) {
    return 'Sin registros'
  }
  const pct = Math.round((regimenPredominante.value.cantidad / personal.value.length) * 100)
  return `${regimenPredominante.value.cantidad} de ${personal.value.length} · ${pct}%`
})

const totalSindicalizados = computed(() => {
  return personal.value.filter((p) => p.sindicato && p.sindicato.trim() !== '' && p.sindicato.trim() !== '—').length
})

const porcentajeSindicalizados = computed(() => {
  if (personal.value.length === 0) return 0
  return Math.round((totalSindicalizados.value / personal.value.length) * 100)
})

const topCargos = computed<{ nombre: string; cantidad: number }[]>(() => {
  const mapa = new Map<string, number>()
  for (const p of personal.value) {
    const cargo = (p.cargo && p.cargo.trim()) || 'Sin cargo'
    mapa.set(cargo, (mapa.get(cargo) || 0) + 1)
  }
  return Array.from(mapa.entries())
    .map(([nombre, cantidad]) => ({ nombre, cantidad }))
    .sort((a, b) => b.cantidad - a.cantidad)
    .slice(0, 8)
})

const resumenPersonalCliente = computed<ResumenPersonal>(() => {
  const total = personal.value.length
  return {
    total,
    activos: total,
    por_regimen: conteoPorRegimen.value,
    por_sexo: [],
    por_sindicato: [],
  }
})

const exportFilename = computed(() => {
  const raw = areaSeleccionadaNombre.value || 'area'
  const nombre = raw.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')
  return `trabajadores-area-${nombre}`
})

const normalizarTexto = (s: string): string => {
  if (!s) return ''
  return s
    .trim()
    .toUpperCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
}

const cargarAreasOpciones = async () => {
  isLoadingAreas.value = true
  try {
    const data = await fetchAreas()
    areasOpciones.value = data.map((a) => ({
      value: a.id,
      label: a.nombre,
    }))
  } catch (err: unknown) {
    console.error('Error cargando áreas:', err)
  } finally {
    isLoadingAreas.value = false
  }
}

const cargarPersonalArea = async (filtro: { areaId: number } | { area: string }) => {
  isLoadingPersonal.value = true
  errorMessage.value = null
  try {
    personal.value = await fetchPersonalPorArea(filtro)
  } catch (err: unknown) {
    if (err instanceof Error) {
      errorMessage.value = err.message
    } else {
      errorMessage.value = 'Error al cargar los trabajadores del área.'
    }
  } finally {
    isLoadingPersonal.value = false
  }
}

const reintentarCarga = () => {
  if (typeof selectedAreaId.value === 'number' && selectedAreaId.value > 0) {
    cargarPersonalArea({ areaId: selectedAreaId.value })
  } else if (selectedAreaNombreDirecto.value) {
    cargarPersonalArea({ area: selectedAreaNombreDirecto.value })
  }
}

const actualizarVista = async () => {
  await cargarAreasOpciones()
  if (typeof selectedAreaId.value === 'number' && selectedAreaId.value > 0) {
    await cargarPersonalArea({ areaId: selectedAreaId.value })
  } else if (selectedAreaNombreDirecto.value) {
    await cargarPersonalArea({ area: selectedAreaNombreDirecto.value })
  }
}

const seleccionarArea = (areaId: number | '') => {
  selectedAreaNombreDirecto.value = ''
  selectedAreaId.value = areaId
  const query = { ...route.query }
  delete query.area
  if (areaId) {
    query.area_id = String(areaId)
  } else {
    delete query.area_id
  }
  router.replace({ query })
}

watch(
  () => selectedAreaId.value,
  (nuevoId) => {
    if (typeof nuevoId === 'number' && nuevoId > 0) {
      selectedAreaNombreDirecto.value = ''
      cargarPersonalArea({ areaId: nuevoId })
    } else if (!selectedAreaNombreDirecto.value) {
      personal.value = []
      errorMessage.value = null
    }
  }
)

watch(
  () => route.query.area_id,
  (nuevoQuery) => {
    const parsed = nuevoQuery ? Number(nuevoQuery) : ''
    if (parsed !== selectedAreaId.value) {
      if (typeof parsed === 'number' && !isNaN(parsed) && parsed > 0) {
        selectedAreaNombreDirecto.value = ''
        selectedAreaId.value = parsed
      } else if (!route.query.area) {
        selectedAreaId.value = ''
      }
    }
  }
)

onMounted(async () => {
  await cargarAreasOpciones()
  const qAreaId = route.query.area_id
  const qAreaNombre = route.query.area

  if (qAreaId) {
    const num = Number(qAreaId)
    if (!isNaN(num) && num > 0) {
      selectedAreaId.value = num
      return
    }
  }

  if (typeof qAreaNombre === 'string' && qAreaNombre.trim()) {
    const nombreBuscado = qAreaNombre.trim()
    const normBuscado = normalizarTexto(nombreBuscado)
    const encontrada = areasOpciones.value.find(
      (opt) => normalizarTexto(opt.label) === normBuscado
    )

    if (encontrada) {
      selectedAreaNombreDirecto.value = ''
      selectedAreaId.value = encontrada.value
      const query: LocationQueryRaw = { ...route.query, area_id: String(encontrada.value) }
      delete query.area
      router.replace({ query })
    } else {
      selectedAreaId.value = ''
      selectedAreaNombreDirecto.value = nombreBuscado
      await cargarPersonalArea({ area: nombreBuscado })
    }
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
            Trabajadores por Área
          </h1>
          <p class="text-xs text-muted-foreground truncate hidden sm:block">
            Personal activo asignado a cada área
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
          :model-value="selectedAreaId"
          :options="areasOpciones"
          placeholder="Seleccionar área..."
          search-placeholder="Buscar área por nombre..."
          empty-text="No se encontraron áreas"
          :allow-clear="true"
          size="md"
          @update:model-value="seleccionarArea"
        />
      </div>
    </div>

    <Card v-if="!tieneAreaSeleccionada" class="p-12 text-center">
      <div class="max-w-sm mx-auto space-y-3">
        <div class="size-12 rounded-full bg-primary/10 text-primary mx-auto flex items-center justify-center">
          <IconBuildingCommunity class="size-6" />
        </div>
        <h3 class="text-sm font-semibold text-foreground">Seleccione un área</h3>
        <p class="text-xs text-muted-foreground">
          Elija un área institucional para visualizar el personal asignado, su régimen y cargos.
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
        @click="reintentarCarga"
      >
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else-if="isLoadingPersonal" class="space-y-6 animate-pulse" aria-busy="true">
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

    <template v-else>
      <div class="space-y-4">
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Total trabajadores</p>
            <div class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1">
              {{ personal.length }}
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              {{ personal.length === 1 ? 'servidor activo' : 'servidores activos' }}
            </p>
          </Card>

          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Régimen predominante</p>
            <div
              class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1 truncate"
              :title="regimenPredominante?.nombre || '—'"
            >
              {{ regimenPredominante?.nombre || '—' }}
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5 truncate">
              {{ regimenPredominanteDetalle }}
            </p>
          </Card>

          <Card class="p-4 bg-card border border-border shadow-2xs">
            <p class="text-xs text-muted-foreground">Sindicalizados</p>
            <div class="text-2xl font-bold font-mono tabular-nums text-foreground mt-1">
              {{ totalSindicalizados }}
            </div>
            <p class="text-[11px] text-muted-foreground mt-0.5">
              {{ porcentajeSindicalizados }}% del total
            </p>
          </Card>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
          <CardDemografiaRegimen
            :resumen="resumenPersonalCliente"
            :is-loading="isLoadingPersonal"
            chart-height="h-60"
          />
          <CardConteoBarras
            titulo="Cargos"
            subtitulo="Top cargos con más personal"
            badge-label="Top 8"
            :items="topCargos"
            :is-loading="isLoadingPersonal"
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
        :loading="isLoadingPersonal"
        empty-message="No se encontraron trabajadores en esta área"
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

        <template #cell-cargo="{ row }">
          <span class="text-xs text-muted-foreground truncate block max-w-50" :title="row.cargo">
            {{ row.cargo }}
          </span>
        </template>

        <template #cell-regimen="{ row }">
          <Badge v-if="row.regimen" variant="secondary" size="xs">
            {{ row.regimen }}
          </Badge>
          <span v-else class="text-muted-foreground text-xs">—</span>
        </template>

        <template #cell-sindicato="{ row }">
          <Badge v-if="row.sindicato" variant="neutral" size="xs">
            {{ row.sindicato }}
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
              Esta área no tiene trabajadores asignados actualmente o no hay coincidencias con la búsqueda.
            </p>
          </div>
        </template>
      </DataTable>
    </template>
  </div>
</template>
