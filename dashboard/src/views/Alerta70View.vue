<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import {
  fetchAlerta70,
  type Alerta70Servidor,
  type Alerta70Estado,
} from '@/services/personal'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconAlertTriangle,
  IconRefresh,
  IconCalendar,
  IconFilter,
  IconUserCheck,
} from '@tabler/icons-vue'

const router = useRouter()

const servidores = ref<Alerta70Servidor[]>([])
const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)

const filtroEstado = ref<string>('todos')
const filtroRegimen = ref<string>('todos')

const obtenerMesNacimiento = (nacimiento: string | null | undefined): number => {
  const d = parseDateSafe(nacimiento)
  return d ? d.getMonth() + 1 : 0
}

const obtenerDiaNacimiento = (nacimiento: string | null | undefined): number => {
  const d = parseDateSafe(nacimiento)
  return d ? d.getDate() : 0
}

const compararPorEdadYMes = (a: Alerta70Servidor, b: Alerta70Servidor, direction: 'asc' | 'desc'): number => {
  const factor = direction === 'asc' ? 1 : -1
  const edadA = a.edad_actual ?? 0
  const edadB = b.edad_actual ?? 0

  if (edadA !== edadB) {
    return (edadA - edadB) * factor
  }

  const mesA = obtenerMesNacimiento(a.nacimiento)
  const mesB = obtenerMesNacimiento(b.nacimiento)

  if (mesA !== mesB) {
    return (mesA - mesB) * factor
  }

  const diaA = obtenerDiaNacimiento(a.nacimiento)
  const diaB = obtenerDiaNacimiento(b.nacimiento)

  return (diaA - diaB) * factor
}

const columns: DataTableColumn[] = [
  {
    key: 'nombre',
    label: 'Servidor Público',
    sortable: true,
  },
  {
    key: 'edad_actual',
    label: 'Edad / Nacimiento',
    sortable: true,
    width: '160px',
    sortFn: (a: unknown, b: unknown, direction: 'asc' | 'desc') => {
      return compararPorEdadYMes(a as Alerta70Servidor, b as Alerta70Servidor, direction)
    },
  },
  {
    key: 'regimen',
    label: 'Régimen y Plaza',
    sortable: true,
  },
  {
    key: 'cargo',
    label: 'Cargo y Área',
    sortable: true,
  },
  {
    key: 'estado',
    label: 'Estado de Alerta',
    sortable: true,
  },
]

const searchKeys = ['nombre', 'dni', 'area', 'cargo', 'plaza', 'regimen']

const cargarDatos = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    servidores.value = await fetchAlerta70()
  } catch (err: unknown) {
    if (err instanceof Error) {
      errorMessage.value = err.message
    } else {
      errorMessage.value = 'Error al obtener la lista de servidores en alerta de edad.'
    }
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  cargarDatos()
})

const regímenesDisponibles = computed<string[]>(() => {
  const map = new Set<string>()
  for (const s of servidores.value) {
    if (s.regimen) map.add(s.regimen)
  }
  return Array.from(map).sort()
})

const conteoPorEstado = computed(() => {
  const totales = {
    total: servidores.value.length,
    excedido: 0,
    alLimite: 0,
    atiempo: 0,
    excepcional: 0,
    otros: 0,
  }

  for (const s of servidores.value) {
    const st = s.estado?.toLowerCase() || ''
    if (st === 'excedido') totales.excedido++
    else if (st === 'allimite') totales.alLimite++
    else if (st === 'atiempo') totales.atiempo++
    else if (st === 'excepcional') totales.excepcional++
    else totales.otros++
  }

  return totales
})

const servidoresFiltrados = computed<Alerta70Servidor[]>(() => {
  const est = filtroEstado.value.toLowerCase()
  const reg = filtroRegimen.value

  const lista = servidores.value.filter((s) => {
    if (est !== 'todos') {
      const sEstado = (s.estado || '').toLowerCase()
      if (sEstado !== est) return false
    }

    if (reg !== 'todos') {
      if (s.regimen !== reg) return false
    }

    return true
  })

  return [...lista].sort((a, b) => {
    const edadDiff = (b.edad_actual ?? 0) - (a.edad_actual ?? 0)
    if (edadDiff !== 0) return edadDiff

    const mesDiff = obtenerMesNacimiento(a.nacimiento) - obtenerMesNacimiento(b.nacimiento)
    if (mesDiff !== 0) return mesDiff

    return obtenerDiaNacimiento(a.nacimiento) - obtenerDiaNacimiento(b.nacimiento)
  })
})

const seleccionarEstado = (estado: string) => {
  filtroEstado.value = filtroEstado.value === estado ? 'todos' : estado
}

const limpiarFiltros = () => {
  filtroEstado.value = 'todos'
  filtroRegimen.value = 'todos'
}

const obtenerConfigEstado = (estado: Alerta70Estado) => {
  const normalizado = (estado || '').toLowerCase()

  switch (normalizado) {
    case 'excedido':
      return {
        label: 'Excedido / Cese Inmediato',
        badgeVariant: 'danger' as const,
        dotClass: 'bg-rose-500',
        bgClass: 'bg-rose-500/10 text-rose-700 dark:text-rose-400 border-rose-500/20',
        desc: '70 años o más en régimen ordinario / 5 días o menos para cumplir',
      }
    case 'allimite':
      return {
        label: 'Al Límite (6 a 15 días)',
        badgeVariant: 'warning' as const,
        dotClass: 'bg-amber-500',
        bgClass: 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20',
        desc: 'Mes de cumpleaños 70 con pocos días restantes',
      }
    case 'atiempo':
      return {
        label: 'A Tiempo / Preventivo',
        badgeVariant: 'primary' as const,
        dotClass: 'bg-primary',
        bgClass: 'bg-primary/10 text-primary border-primary/20',
        desc: '69 años con más de 15 días o meses previos al cumpleaños 70',
      }
    case 'excepcional':
      return {
        label: 'CAS Excepcional',
        badgeVariant: 'secondary' as const,
        dotClass: 'bg-purple-500',
        bgClass: 'bg-purple-500/10 text-purple-700 dark:text-purple-400 border-purple-500/20',
        desc: 'Servidor con 70 años o más bajo régimen D.L. 1057 (CAS)',
      }
    case 'diciembre':
      return {
        label: 'Cierre Diciembre',
        badgeVariant: 'outline' as const,
        dotClass: 'bg-teal-500',
        bgClass: 'bg-teal-500/10 text-teal-700 dark:text-teal-400 border-teal-500/20',
        desc: 'Extensión o corte proyectado al cierre anual',
      }
    default:
      return {
        label: estado || 'Evaluación',
        badgeVariant: 'outline' as const,
        dotClass: 'bg-muted-foreground',
        bgClass: 'bg-muted text-muted-foreground border-border',
        desc: 'Sin clasificación específica',
      }
  }
}

const obtenerIniciales = (nombre: string): string => {
  if (!nombre) return 'SP'
  const partes = nombre.trim().split(/\s+/)
  if (partes.length >= 2) {
    return (partes[0][0] + partes[1][0]).toUpperCase()
  }
  return nombre.slice(0, 2).toUpperCase()
}

const ocultarAvatarErroneo = (event: Event) => {
  const target = event.target as HTMLElement | null
  if (target) {
    target.style.display = 'none'
  }
}

const irAlPerfil = (dni: string) => {
  if (!dni) return
  router.push({ name: 'perfil', params: { dni } })
}
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3 sm:gap-4">
      <Card
        class="p-4 cursor-pointer transition-all hover:border-primary/40"
        :class="filtroEstado === 'todos' ? 'ring-2 ring-primary/20 border-primary/40' : ''"
        @click="seleccionarEstado('todos')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-medium text-muted-foreground">Total en Alerta</span>
          <span class="size-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-foreground font-mono">
            {{ conteoPorEstado.total }}
          </span>
          <span class="text-[11px] text-muted-foreground">Servidores</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Edad 69 años o más</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-rose-500/40"
        :class="filtroEstado === 'excedido' ? 'ring-2 ring-rose-500/20 border-rose-500/40 bg-rose-500/5' : ''"
        @click="seleccionarEstado('excedido')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-rose-600 dark:text-rose-400">Excedido</span>
          <span class="size-2 rounded-full bg-rose-500 animate-pulse"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-rose-600 dark:text-rose-400 font-mono">
            {{ conteoPorEstado.excedido }}
          </span>
          <span class="text-[11px] text-rose-600/70 dark:text-rose-400/70">Urgente</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">70+ o cese inminente</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-amber-500/40"
        :class="filtroEstado === 'allimite' ? 'ring-2 ring-amber-500/20 border-amber-500/40 bg-amber-500/5' : ''"
        @click="seleccionarEstado('allimite')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-amber-600 dark:text-amber-400">Al Límite</span>
          <span class="size-2 rounded-full bg-amber-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-amber-600 dark:text-amber-400 font-mono">
            {{ conteoPorEstado.alLimite }}
          </span>
          <span class="text-[11px] text-amber-600/70 dark:text-amber-400/70">6 a 15 días</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Mes de cumpleaños</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-primary/40"
        :class="filtroEstado === 'atiempo' ? 'ring-2 ring-primary/20 border-primary/40 bg-primary/5' : ''"
        @click="seleccionarEstado('atiempo')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-primary">A Tiempo</span>
          <span class="size-2 rounded-full bg-sky-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-primary font-mono">
            {{ conteoPorEstado.atiempo }}
          </span>
          <span class="text-[11px] text-muted-foreground">Preventivo</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">Más de 15 días</p>
      </Card>

      <Card
        class="p-4 cursor-pointer transition-all hover:border-purple-500/40 col-span-2 sm:col-span-1"
        :class="filtroEstado === 'excepcional' ? 'ring-2 ring-purple-500/20 border-purple-500/40 bg-purple-500/5' : ''"
        @click="seleccionarEstado('excepcional')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="text-xs font-semibold text-purple-600 dark:text-purple-400">CAS Excepcional</span>
          <span class="size-2 rounded-full bg-purple-500"></span>
        </div>
        <div class="mt-2 flex items-baseline justify-between">
          <span class="text-2xl font-bold tracking-tight text-purple-600 dark:text-purple-400 font-mono">
            {{ conteoPorEstado.excepcional }}
          </span>
          <span class="text-[11px] text-purple-600/70 dark:text-purple-400/70">D.L. 1057</span>
        </div>
        <p class="text-[10px] text-muted-foreground mt-1 truncate">70+ bajo CAS</p>
      </Card>
    </div>

    <div v-if="errorMessage" class="p-8 text-center rounded-2xl border border-rose-500/20 bg-rose-500/5 space-y-3">
      <IconAlertTriangle class="size-8 mx-auto text-rose-500" />
      <p class="text-sm font-bold text-rose-600 dark:text-rose-400">Error al cargar la alerta</p>
      <p class="text-xs text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" class="gap-1.5 text-xs cursor-pointer" @click="cargarDatos">
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <DataTable
      v-else
      :columns="columns"
      :data="servidoresFiltrados"
      row-key="dni"
      default-sort-column="edad_actual"
      default-sort-direction="desc"
      searchable
      search-placeholder="Buscar por servidor, DNI, área, cargo o plaza..."
      :search-keys="searchKeys"
      show-column-visibility
      exportable
      export-filename="alerta_servidores_70_anios"
      :page-size="10"
      :page-size-options="[10, 25, 50, 100]"
      :loading="isLoading"
      empty-message="No se encontraron servidores en alerta de edad"
      @row-click="(row) => irAlPerfil(row.dni)"
    >
      <template #toolbar>
        <div class="flex items-center gap-2 flex-wrap">
          <div class="flex items-center gap-1.5">
            <IconFilter class="size-3.5 text-muted-foreground shrink-0" />
            <select
              v-model="filtroRegimen"
              class="bg-card border border-border rounded-lg px-2.5 py-1.5 text-xs text-foreground outline-hidden focus:border-primary cursor-pointer"
            >
              <option value="todos">Todos los regímenes</option>
              <option v-for="r in regímenesDisponibles" :key="r" :value="r">
                {{ r }}
              </option>
            </select>
          </div>

          <Button
            v-if="filtroEstado !== 'todos' || filtroRegimen !== 'todos'"
            variant="ghost"
            size="xs"
            class="text-xs text-muted-foreground hover:text-foreground cursor-pointer"
            @click="limpiarFiltros"
          >
            Limpiar filtros
          </Button>

          <Button
            variant="outline"
            size="xs"
            class="gap-1.5 text-xs cursor-pointer"
            :disabled="isLoading"
            @click="cargarDatos"
          >
            <IconRefresh class="size-3.5" :class="isLoading ? 'animate-spin' : ''" />
            <span class="hidden sm:inline">Actualizar</span>
          </Button>
        </div>
      </template>

      <template #empty>
        <div class="p-10 text-center space-y-3">
          <IconUserCheck class="size-10 mx-auto text-muted-foreground/60" />
          <h3 class="text-sm font-bold text-foreground">Sin servidores coincidentes</h3>
          <p class="text-xs text-muted-foreground max-w-sm mx-auto">
            No se encontraron servidores que coincidan con los filtros aplicados o no hay personal en el rango de alerta en este momento.
          </p>
          <Button
            v-if="filtroEstado !== 'todos' || filtroRegimen !== 'todos'"
            variant="outline"
            size="xs"
            class="text-xs cursor-pointer"
            @click="limpiarFiltros"
          >
            Restablecer filtros
          </Button>
        </div>
      </template>

      <template #cell-nombre="{ row }">
        <div class="flex items-center gap-3">
          <div
            class="size-9 rounded-full bg-primary/10 border border-border overflow-hidden shrink-0 flex items-center justify-center font-bold text-xs text-primary"
          >
            <img
              v-if="row.avatar"
              v-auth-src="resolveAvatarUrl(row.dni)!"
              :alt="row.nombre"
              class="size-full object-cover"
              @error="ocultarAvatarErroneo"
            />
            <span v-else>{{ obtenerIniciales(row.nombre) }}</span>
          </div>
          <div class="min-w-0">
            <p
              class="font-semibold text-foreground truncate group-hover:text-primary transition-colors text-xs"
              :title="row.nombre"
            >
              {{ row.nombre }}
            </p>
            <span class="font-mono text-[11px] text-muted-foreground">
              DNI: {{ row.dni }}
            </span>
          </div>
        </div>
      </template>

      <template #cell-edad_actual="{ row }">
        <div class="space-y-0.5">
          <div class="flex items-center gap-1.5">
            <span class="font-bold text-sm text-foreground font-mono">
              {{ row.edad_actual }}
            </span>
            <span class="text-xs text-muted-foreground">años</span>
          </div>
          <div class="flex items-center gap-1 text-[11px] text-muted-foreground font-mono">
            <IconCalendar class="size-3 shrink-0" />
            <span>{{ row.nacimiento ? formatDate(row.nacimiento) : '-' }}</span>
          </div>
        </div>
      </template>

      <template #cell-regimen="{ row }">
        <div class="space-y-1">
          <Badge variant="secondary" size="xs" class="font-medium truncate max-w-42.5">
            {{ row.regimen || 'Sin régimen' }}
          </Badge>
          <div v-if="row.plaza" class="text-[11px] font-mono text-muted-foreground">
            Plaza: <strong class="text-foreground">{{ row.plaza }}</strong>
          </div>
        </div>
      </template>

      <template #cell-cargo="{ row }">
        <div class="space-y-0.5 min-w-50 max-w-sm">
          <p class="font-medium text-foreground text-xs whitespace-normal text-wrap wrap-break-word" :title="row.cargo">
            {{ row.cargo || 'Sin cargo' }}
          </p>
          <p class="text-[11px] text-muted-foreground whitespace-normal text-wrap wrap-break-word" :title="row.area">
            {{ row.area || 'Sin área' }}
          </p>
        </div>
      </template>

      <template #cell-estado="{ row }">
        <div class="space-y-1">
          <span
            class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium border"
            :class="obtenerConfigEstado(row.estado).bgClass"
          >
            <span class="size-1.5 rounded-full" :class="obtenerConfigEstado(row.estado).dotClass"></span>
            <span>{{ obtenerConfigEstado(row.estado).label }}</span>
          </span>
          <p class="text-[10px] text-muted-foreground max-w-47.5 leading-tight">
            {{ obtenerConfigEstado(row.estado).desc }}
          </p>
        </div>
      </template>
    </DataTable>
  </div>
</template>
