<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import Button from '@/components/ui/button/Button.vue'
import PageHeader from '@/components/ui/page-header/PageHeader.vue'
import FilterDropdown from '@/components/ui/form/FilterDropdown.vue'
import { fetchAlerta70, type Alerta70Servidor } from '@/services/personal'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import { formatDate, parseDateSafe } from '@/utils/date'
import {
  IconAlertTriangle,
  IconRefresh,
  IconSearch,
  IconChevronLeft,
  IconChevronRight,
  IconCake,
  IconCalendarMonth,
  IconListDetails,
} from '@tabler/icons-vue'

const router = useRouter()

const servidores = ref<Alerta70Servidor[]>([])
const isLoading = ref<boolean>(true)
const errorMessage = ref<string | null>(null)

const hoy = new Date()
const hoyKey = `${hoy.getFullYear()}-${hoy.getMonth()}-${hoy.getDate()}`
const cursor = ref<Date>(new Date(hoy.getFullYear(), hoy.getMonth(), 1))
const diaSeleccionado = ref<Date | null>(null)
const vista = ref<'mes' | 'agenda'>('mes')

const busqueda = ref<string>('')
const filtroRegimen = ref<string>('todos')
const estadosActivos = ref<string[]>(['excedido', 'allimite', 'atiempo', 'excepcional'])

const MAX_CHIPS = 3
const DIAS_SEMANA = ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom']

const ESTADOS = [
  { id: 'excedido', label: 'Excedido', hint: '70+ o cese inminente', dot: 'bg-rose-500', chip: 'bg-rose-500/15 text-rose-700 dark:text-rose-300 border-rose-500', text: 'text-rose-600 dark:text-rose-400' },
  { id: 'allimite', label: 'Al límite', hint: '6 a 15 días', dot: 'bg-amber-500', chip: 'bg-amber-500/15 text-amber-700 dark:text-amber-300 border-amber-500', text: 'text-amber-600 dark:text-amber-400' },
  { id: 'atiempo', label: 'A tiempo', hint: 'Más de 15 días', dot: 'bg-primary', chip: 'bg-primary/10 text-primary border-primary', text: 'text-primary' },
  { id: 'excepcional', label: 'CAS excepcional', hint: '70+ bajo D.L. 1057', dot: 'bg-sky-500', chip: 'bg-sky-500/15 text-sky-700 dark:text-sky-300 border-sky-500', text: 'text-sky-600 dark:text-sky-400' },
] as const

const configEstado = (estado: string | null | undefined) => {
  const e = (estado || '').toLowerCase()
  return (
    ESTADOS.find((x) => x.id === e) ?? {
      id: 'otro', label: estado || 'Evaluación', hint: '', dot: 'bg-muted-foreground',
      chip: 'bg-muted text-muted-foreground border-border', text: 'text-muted-foreground',
    }
  )
}

const normalizar = (t: string | null | undefined) =>
  (t || '').normalize('NFD').replace(/[̀-ͯ]/g, '').toLowerCase()

const cargarDatos = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    servidores.value = await fetchAlerta70()
  } catch (err: unknown) {
    errorMessage.value =
      err instanceof Error ? err.message : 'Error al obtener la lista de servidores en alerta de edad.'
  } finally {
    isLoading.value = false
  }
}

onMounted(cargarDatos)

const regimenOptions = computed(() => [
  { value: 'todos', label: 'Todos los regímenes' },
  ...Array.from(new Set(servidores.value.map((s) => s.regimen).filter(Boolean)))
    .sort()
    .map((r) => ({ value: r, label: r })),
])

const conteos = computed(() => {
  const c: Record<string, number> = {}
  for (const s of servidores.value) {
    const k = (s.estado || '').toLowerCase()
    c[k] = (c[k] || 0) + 1
  }
  return c
})

const hayFiltros = computed(
  () => busqueda.value !== '' || filtroRegimen.value !== 'todos' || estadosActivos.value.length !== 4,
)

const limpiarFiltros = () => {
  busqueda.value = ''
  filtroRegimen.value = 'todos'
  estadosActivos.value = ESTADOS.map((e) => e.id)
}

const filtrados = computed(() => {
  const q = normalizar(busqueda.value.trim())
  return servidores.value.filter((s) => {
    const est = (s.estado || '').toLowerCase()
    const esConocido = ESTADOS.some((e) => e.id === est)
    if (esConocido && !estadosActivos.value.includes(est)) return false
    if (filtroRegimen.value !== 'todos' && s.regimen !== filtroRegimen.value) return false
    if (q && !normalizar([s.nombre, s.dni, s.area, s.cargo, s.plaza, s.regimen].join(' ')).includes(q)) return false
    return true
  })
})

interface Cumple {
  s: Alerta70Servidor
  fecha: Date
  edad: number
}

const diasDelMes = (y: number, m: number) => new Date(y, m + 1, 0).getDate()

// Cumpleaños de un servidor dentro de un año dado (29 feb → 28 feb en años no bisiestos).
const cumpleEnAnio = (s: Alerta70Servidor, anio: number): Cumple | null => {
  const n = parseDateSafe(s.nacimiento)
  if (!n) return null
  const mes = n.getMonth()
  const dia = Math.min(n.getDate(), diasDelMes(anio, mes))
  return { s, fecha: new Date(anio, mes, dia), edad: anio - n.getFullYear() }
}

const claveDia = (d: Date) => `${d.getFullYear()}-${d.getMonth()}-${d.getDate()}`

const cumplesDelMes = computed(() => {
  const mapa = new Map<string, Cumple[]>()
  const anio = cursor.value.getFullYear()
  const mes = cursor.value.getMonth()
  for (const s of filtrados.value) {
    const c = cumpleEnAnio(s, anio)
    if (!c || c.fecha.getMonth() !== mes) continue
    const k = claveDia(c.fecha)
    if (!mapa.has(k)) mapa.set(k, [])
    mapa.get(k)!.push(c)
  }
  for (const lista of mapa.values()) lista.sort((a, b) => b.edad - a.edad)
  return mapa
})

const totalDelMes = computed(() => {
  let n = 0
  for (const l of cumplesDelMes.value.values()) n += l.length
  return n
})

interface Celda {
  fecha: Date
  key: string
  fueraDeMes: boolean
  esHoy: boolean
  items: Cumple[]
}

const celdas = computed<Celda[]>(() => {
  const y = cursor.value.getFullYear()
  const m = cursor.value.getMonth()
  const offset = (new Date(y, m, 1).getDay() + 6) % 7 // lunes = 0
  const inicio = new Date(y, m, 1 - offset)
  const filas = Math.ceil((offset + diasDelMes(y, m)) / 7)
  return Array.from({ length: filas * 7 }, (_, i) => {
    const fecha = new Date(inicio.getFullYear(), inicio.getMonth(), inicio.getDate() + i)
    const key = claveDia(fecha)
    const fueraDeMes = fecha.getMonth() !== m
    return { fecha, key, fueraDeMes, esHoy: key === hoyKey, items: fueraDeMes ? [] : (cumplesDelMes.value.get(key) ?? []) }
  })
})

const agenda = computed(() =>
  celdas.value.filter((c) => c.items.length > 0),
)

const tituloMes = computed(() => {
  const t = cursor.value.toLocaleDateString('es-PE', { month: 'long', year: 'numeric' })
  return t.charAt(0).toUpperCase() + t.slice(1)
})

const esMesActual = computed(
  () => cursor.value.getFullYear() === hoy.getFullYear() && cursor.value.getMonth() === hoy.getMonth(),
)

const moverMes = (delta: number) => {
  cursor.value = new Date(cursor.value.getFullYear(), cursor.value.getMonth() + delta, 1)
  diaSeleccionado.value = null
}
const irAHoy = () => {
  cursor.value = new Date(hoy.getFullYear(), hoy.getMonth(), 1)
  diaSeleccionado.value = null
}

const seleccionarDia = (c: Celda) => {
  if (c.fueraDeMes) return
  diaSeleccionado.value = diaSeleccionado.value && claveDia(diaSeleccionado.value) === c.key ? null : c.fecha
}

const itemsDiaSeleccionado = computed(() =>
  diaSeleccionado.value ? (cumplesDelMes.value.get(claveDia(diaSeleccionado.value)) ?? []) : [],
)

const tituloDia = computed(() =>
  diaSeleccionado.value
    ? diaSeleccionado.value.toLocaleDateString('es-PE', { weekday: 'long', day: 'numeric', month: 'long' })
    : '',
)

// Próximos cumpleaños (30 días) de los servidores filtrados.
const proximos = computed(() => {
  const base = new Date(hoy.getFullYear(), hoy.getMonth(), hoy.getDate())
  const limite = new Date(base.getFullYear(), base.getMonth(), base.getDate() + 30)
  const out: Cumple[] = []
  for (const s of filtrados.value) {
    for (const anio of [base.getFullYear(), base.getFullYear() + 1]) {
      const c = cumpleEnAnio(s, anio)
      if (c && c.fecha >= base && c.fecha <= limite) {
        out.push(c)
        break
      }
    }
  }
  return out.sort((a, b) => a.fecha.getTime() - b.fecha.getTime()).slice(0, 8)
})

// CAS excepcional: ya superaron los 70, se listan todos por su próximo cumpleaños.
const casExcepcionales = computed(() => {
  const base = new Date(hoy.getFullYear(), hoy.getMonth(), hoy.getDate())
  const out: Cumple[] = []
  for (const s of filtrados.value) {
    if ((s.estado || '').toLowerCase() !== 'excepcional') continue
    let c = cumpleEnAnio(s, base.getFullYear())
    if (c && c.fecha < base) c = cumpleEnAnio(s, base.getFullYear() + 1)
    if (c) out.push(c)
  }
  return out.sort((a, b) => a.fecha.getTime() - b.fecha.getTime())
})

const diasHasta = (f: Date) => {
  const base = new Date(hoy.getFullYear(), hoy.getMonth(), hoy.getDate())
  const d = Math.round((f.getTime() - base.getTime()) / 86400000)
  return d === 0 ? 'Hoy' : d === 1 ? 'Mañana' : `En ${d} días`
}

const alternarEstado = (id: string) => {
  estadosActivos.value = estadosActivos.value.includes(id)
    ? estadosActivos.value.filter((e) => e !== id)
    : [...estadosActivos.value, id]
}

const obtenerIniciales = (nombre: string): string => {
  if (!nombre) return 'SP'
  const p = nombre.trim().split(/\s+/)
  return p.length >= 2 ? (p[0][0] + p[1][0]).toUpperCase() : nombre.slice(0, 2).toUpperCase()
}

const ocultarAvatarErroneo = (event: Event) => {
  const t = event.target as HTMLElement | null
  if (t) t.style.display = 'none'
}

const irAlPerfil = (dni: string) => {
  if (dni) router.push({ name: 'perfil', params: { dni } })
}
</script>

<template>
  <div class="mx-auto w-full max-w-7xl space-y-3 px-4 pb-12">
    <PageHeader title="Alerta 70 Años" subtitle="Calendario de cumpleaños de servidores en edad límite de cese">
      <template #actions>
        <Button variant="ghost" size="sm" class="gap-1.5 text-meta text-muted-foreground hover:text-foreground cursor-pointer"
          :disabled="isLoading" @click="cargarDatos">
          <IconRefresh class="size-3.5" :class="isLoading ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </template>
    </PageHeader>

    <div v-if="errorMessage" class="p-4 text-center rounded-xl border border-rose-500/20 bg-rose-500/5 space-y-3" role="alert">
      <IconAlertTriangle class="size-6 mx-auto text-rose-500" />
      <p class="text-meta font-bold text-rose-600 dark:text-rose-400">Error al cargar la alerta</p>
      <p class="text-meta text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" class="gap-1.5 text-meta cursor-pointer" @click="cargarDatos">
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <div v-else class="grid grid-cols-1 lg:grid-cols-[15rem_minmax(0,1fr)] gap-3 items-start">
      <!-- Barra lateral -->
      <aside class="space-y-3">
        <section class="bg-card border border-border shadow-2xs rounded-xl p-3 space-y-2.5" aria-label="Filtros">
          <div class="flex items-center gap-2.5">
            <span class="size-8 rounded-lg bg-rose-500/10 text-rose-600 dark:text-rose-400 flex items-center justify-center">
              <IconAlertTriangle class="size-4" />
            </span>
            <div>
              <p class="text-micro text-muted-foreground">Total en alerta</p>
              <p class="text-kpi font-bold font-mono tabular-nums text-foreground leading-none">{{ servidores.length }}</p>
            </div>
          </div>

          <label class="relative block">
            <IconSearch class="size-3 absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <input v-model="busqueda" type="search" placeholder="Buscar servidor, DNI, área…"
              class="w-full ps-7 pe-2 h-control-sm py-0 rounded-md bg-card border border-border text-micro text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-primary" />
          </label>

          <FilterDropdown v-model="filtroRegimen" :options="regimenOptions" aria-label="Filtrar por régimen" />

          <div>
            <p class="text-micro font-semibold uppercase tracking-wider text-muted-foreground mb-1">Estados</p>
            <ul class="space-y-0.5">
              <li v-for="e in ESTADOS" :key="e.id">
                <label class="flex items-center gap-2 rounded-md px-1.5 py-1 cursor-pointer hover:bg-muted/60 select-none">
                  <input type="checkbox" class="size-3.5 rounded border-border text-primary focus:ring-primary/30"
                    :checked="estadosActivos.includes(e.id)" @change="alternarEstado(e.id)" />
                  <span class="size-2 rounded-full shrink-0" :class="e.dot"></span>
                  <span class="text-meta text-foreground flex-1 truncate">{{ e.label }}</span>
                  <span class="text-micro font-mono tabular-nums" :class="e.text">{{ conteos[e.id] || 0 }}</span>
                </label>
              </li>
            </ul>
          </div>

          <Button v-if="hayFiltros" variant="ghost" size="xs"
            class="w-full text-micro text-muted-foreground hover:text-foreground cursor-pointer" @click="limpiarFiltros">
            Limpiar filtros
          </Button>
        </section>

        <section class="bg-card border border-border shadow-2xs rounded-xl p-3" aria-live="polite">
          <template v-if="diaSeleccionado">
            <div class="flex items-center justify-between gap-2 mb-1.5">
              <p class="text-meta font-semibold text-foreground capitalize">{{ tituloDia }}</p>
              <button type="button" class="text-micro text-muted-foreground hover:text-foreground cursor-pointer" @click="diaSeleccionado = null">Cerrar</button>
            </div>
            <p v-if="itemsDiaSeleccionado.length === 0" class="text-micro text-muted-foreground">Sin cumpleaños este día.</p>
          </template>
          <p v-else class="text-micro font-semibold uppercase tracking-wider text-muted-foreground mb-1.5">Próximos 30 días</p>

          <ul class="space-y-1">
            <li v-for="c in (diaSeleccionado ? itemsDiaSeleccionado : proximos)" :key="c.s.dni">
              <button type="button" class="w-full flex items-center gap-2 rounded-md px-1.5 py-1.5 text-left hover:bg-muted/60 cursor-pointer"
                @click="irAlPerfil(c.s.dni)">
                <span class="relative shrink-0">
                  <span class="size-7 rounded-full bg-primary/10 border border-border overflow-hidden flex items-center justify-center font-bold text-micro text-primary">
                    <img v-if="c.s.avatar" v-auth-src="resolveAvatarUrl(c.s.dni)!" :alt="c.s.nombre" class="size-full object-cover" @error="ocultarAvatarErroneo" />
                    <span v-else>{{ obtenerIniciales(c.s.nombre) }}</span>
                  </span>
                  <span class="absolute -bottom-0.5 -end-0.5 size-2.5 rounded-full border-2 border-card" :class="configEstado(c.s.estado).dot"></span>
                </span>
                <span class="min-w-0 flex-1">
                  <span class="block text-meta font-medium text-foreground truncate">{{ c.s.nombre }}</span>
                  <span class="block text-micro text-muted-foreground truncate">
                    Cumple {{ c.edad }} · {{ diaSeleccionado ? c.s.cargo : diasHasta(c.fecha) }}
                  </span>
                </span>
              </button>
            </li>
            <li v-if="!diaSeleccionado && proximos.length === 0" class="text-micro text-muted-foreground">
              Sin cumpleaños en los próximos 30 días.
            </li>
          </ul>

          <template v-if="!diaSeleccionado">
            <p class="text-micro font-semibold uppercase tracking-wider text-muted-foreground mt-3 mb-1.5 pt-2.5 border-t border-border/70">
              CAS excepcional · {{ casExcepcionales.length }}
            </p>
            <ul class="space-y-1 max-h-64 overflow-y-auto pe-0.5">
              <li v-for="c in casExcepcionales" :key="c.s.dni">
                <button type="button" class="w-full flex items-center gap-2 rounded-md px-1.5 py-1.5 text-left hover:bg-muted/60 cursor-pointer"
                  @click="irAlPerfil(c.s.dni)">
                  <span class="relative shrink-0">
                    <span class="size-7 rounded-full bg-primary/10 border border-border overflow-hidden flex items-center justify-center font-bold text-micro text-primary">
                      <img v-if="c.s.avatar" v-auth-src="resolveAvatarUrl(c.s.dni)!" :alt="c.s.nombre" class="size-full object-cover" @error="ocultarAvatarErroneo" />
                      <span v-else>{{ obtenerIniciales(c.s.nombre) }}</span>
                    </span>
                    <span class="absolute -bottom-0.5 -end-0.5 size-2.5 rounded-full border-2 border-card bg-sky-500"></span>
                  </span>
                  <span class="min-w-0 flex-1">
                    <span class="block text-meta font-medium text-foreground truncate">{{ c.s.nombre }}</span>
                    <span class="block text-micro text-muted-foreground truncate">
                      Cumple {{ c.edad }} · {{ diasHasta(c.fecha) }}
                    </span>
                  </span>
                </button>
              </li>
              <li v-if="casExcepcionales.length === 0" class="text-micro text-muted-foreground">Sin servidores CAS excepcional.</li>
            </ul>
          </template>
        </section>
      </aside>

      <!-- Calendario -->
      <section class="bg-card border border-border shadow-2xs rounded-xl overflow-hidden min-w-0" aria-label="Calendario mensual">
        <div class="flex flex-wrap items-center justify-between gap-2 px-3 py-2 border-b border-border">
          <div class="flex items-center gap-2">
            <Button variant="outline" size="xs" class="text-micro cursor-pointer" :disabled="esMesActual" @click="irAHoy">Hoy</Button>
            <div class="flex items-center">
              <button type="button" class="size-7 inline-flex items-center justify-center rounded-lg text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer"
                aria-label="Mes anterior" @click="moverMes(-1)">
                <IconChevronLeft class="size-3.5" />
              </button>
              <button type="button" class="size-7 inline-flex items-center justify-center rounded-lg text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer"
                aria-label="Mes siguiente" @click="moverMes(1)">
                <IconChevronRight class="size-3.5" />
              </button>
            </div>
            <h2 class="text-title font-semibold text-foreground">{{ tituloMes }}</h2>
            <span class="text-micro text-muted-foreground">{{ totalDelMes }} {{ totalDelMes === 1 ? 'cumpleaños' : 'cumpleaños' }}</span>
          </div>

          <div class="inline-flex rounded-lg border border-border p-0.5" role="group" aria-label="Tipo de vista">
            <button type="button" class="inline-flex items-center gap-1 px-2 h-6 rounded-md text-micro cursor-pointer transition"
              :class="vista === 'mes' ? 'bg-muted text-foreground font-medium' : 'text-muted-foreground hover:text-foreground'"
              @click="vista = 'mes'">
              <IconCalendarMonth class="size-3.5" />Mes
            </button>
            <button type="button" class="inline-flex items-center gap-1 px-2 h-6 rounded-md text-micro cursor-pointer transition"
              :class="vista === 'agenda' ? 'bg-muted text-foreground font-medium' : 'text-muted-foreground hover:text-foreground'"
              @click="vista = 'agenda'">
              <IconListDetails class="size-3.5" />Agenda
            </button>
          </div>
        </div>

        <div v-if="isLoading" class="h-96 animate-pulse bg-muted/30" aria-busy="true"></div>

        <template v-else>
          <!-- Vista mensual -->
          <div v-if="vista === 'mes'" class="hidden md:block">
            <div class="grid grid-cols-7 border-b border-border bg-muted/20">
              <div v-for="d in DIAS_SEMANA" :key="d" class="px-2 py-1.5 text-micro font-semibold uppercase tracking-wider text-muted-foreground text-center">
                {{ d }}
              </div>
            </div>
            <div class="grid grid-cols-7">
              <div v-for="(c, i) in celdas" :key="c.key"
                class="min-h-24 p-1 border-border min-w-0 transition"
                :class="[
                  i % 7 !== 6 ? 'border-e' : '',
                  i < celdas.length - 7 ? 'border-b' : '',
                  c.fueraDeMes ? 'bg-muted/20' : 'cursor-pointer hover:bg-muted/30',
                  diaSeleccionado && claveDia(diaSeleccionado) === c.key ? 'bg-primary/5 ring-1 ring-inset ring-primary/40' : '',
                ]"
                @click="seleccionarDia(c)">
                <div class="flex justify-end">
                  <span class="size-5 inline-flex items-center justify-center rounded-full text-micro tabular-nums"
                    :class="c.esHoy ? 'bg-primary text-primary-foreground font-bold' : c.fueraDeMes ? 'text-muted-foreground/40' : 'text-foreground'">
                    {{ c.fecha.getDate() }}
                  </span>
                </div>
                <ul class="mt-0.5 space-y-0.5">
                  <li v-for="it in c.items.slice(0, MAX_CHIPS)" :key="it.s.dni">
                    <button type="button"
                      class="w-full flex items-center gap-1 rounded-sm border-s-2 px-1 py-0.5 text-left text-micro leading-tight cursor-pointer hover:brightness-110 transition"
                      :class="configEstado(it.s.estado).chip"
                      :title="`${it.s.nombre} · cumple ${it.edad} · ${configEstado(it.s.estado).label}`"
                      @click.stop="irAlPerfil(it.s.dni)">
                      <span class="font-mono font-bold tabular-nums shrink-0">{{ it.edad }}</span>
                      <span class="truncate">{{ it.s.nombre }}</span>
                    </button>
                  </li>
                  <li v-if="c.items.length > MAX_CHIPS">
                    <button type="button" class="px-1 text-micro text-muted-foreground hover:text-foreground cursor-pointer"
                      @click.stop="seleccionarDia(c)">
                      {{ c.items.length - MAX_CHIPS }} más
                    </button>
                  </li>
                </ul>
              </div>
            </div>
          </div>

          <!-- Agenda (también en móvil) -->
          <div :class="vista === 'agenda' ? '' : 'md:hidden'">
            <p v-if="agenda.length === 0" class="p-8 text-center text-meta text-muted-foreground">
              <IconCake class="size-6 mx-auto mb-2 text-muted-foreground/50" />
              Sin cumpleaños en {{ tituloMes }} con los filtros actuales.
            </p>
            <ul v-else class="divide-y divide-border">
              <li v-for="c in agenda" :key="c.key" class="flex gap-3 px-3 py-2.5">
                <div class="w-10 shrink-0 text-center">
                  <p class="text-micro uppercase text-muted-foreground">{{ DIAS_SEMANA[(c.fecha.getDay() + 6) % 7] }}</p>
                  <p class="text-title font-bold font-mono tabular-nums leading-tight"
                    :class="c.esHoy ? 'text-primary' : 'text-foreground'">{{ c.fecha.getDate() }}</p>
                </div>
                <ul class="flex-1 min-w-0 space-y-1">
                  <li v-for="it in c.items" :key="it.s.dni">
                    <button type="button"
                      class="w-full flex items-center gap-2 rounded-md border-s-2 px-2 py-1.5 text-left cursor-pointer hover:brightness-110 transition"
                      :class="configEstado(it.s.estado).chip" @click="irAlPerfil(it.s.dni)">
                      <span class="size-7 shrink-0 rounded-full bg-card border border-border overflow-hidden flex items-center justify-center font-bold text-micro text-primary">
                        <img v-if="it.s.avatar" v-auth-src="resolveAvatarUrl(it.s.dni)!" :alt="it.s.nombre" class="size-full object-cover" @error="ocultarAvatarErroneo" />
                        <span v-else>{{ obtenerIniciales(it.s.nombre) }}</span>
                      </span>
                      <span class="min-w-0 flex-1">
                        <span class="block text-meta font-semibold text-foreground truncate">{{ it.s.nombre }}</span>
                        <span class="block text-micro text-muted-foreground truncate">{{ it.s.cargo || 'Sin cargo' }} · {{ it.s.regimen || 'Sin régimen' }}</span>
                      </span>
                      <span class="text-end shrink-0">
                        <span class="block text-meta font-bold font-mono tabular-nums text-foreground">{{ it.edad }} años</span>
                        <span class="block text-micro" :class="configEstado(it.s.estado).text">{{ configEstado(it.s.estado).label }}</span>
                      </span>
                    </button>
                  </li>
                </ul>
              </li>
            </ul>
          </div>
        </template>

        <div class="flex flex-wrap items-center gap-x-3 gap-y-1 px-3 py-2 border-t border-border text-micro text-muted-foreground">
          <span v-for="e in ESTADOS" :key="e.id" class="inline-flex items-center gap-1.5">
            <span class="size-1.5 rounded-full" :class="e.dot"></span>{{ e.label }}
          </span>
          <span class="ms-auto">El número indica la edad que cumple · {{ formatDate(hoy.toISOString()) }}</span>
        </div>
      </section>
    </div>
  </div>
</template>
