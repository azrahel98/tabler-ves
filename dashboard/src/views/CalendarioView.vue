<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import PageHeader from '@/components/ui/page-header/PageHeader.vue'
import { api } from '@/services/api'
import { resolveAvatarUrl } from '@/components/dashboard/types'
import {
  IconChevronLeft,
  IconChevronRight,
  IconCake,
  IconCalendarEvent,
  IconLoader2,
} from '@tabler/icons-vue'

interface Cumpleanero {
  dni: string
  nombre: string | null
  nacimiento: string
  avatar?: string | null
  regimen?: string | null
}

interface DiaEspecial {
  fecha: string
  dia_semana: string
  tipo: 'feriado' | 'no_laborable'
  motivo: string
}

interface MesCalendario {
  mes: number
  nombre: string
  dias: DiaEspecial[]
}

const router = useRouter()

const MESES = [
  'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
  'Julio', 'Agosto', 'Setiembre', 'Octubre', 'Noviembre', 'Diciembre',
]
const DIAS_SEMANA = ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom']

const hoy = new Date()
const anio = ref(hoy.getFullYear())
const mes = ref(hoy.getMonth()) // 0-11
const seleccionado = ref<string>(clave(hoy.getFullYear(), hoy.getMonth(), hoy.getDate()))

// Cumpleaños por mes (1-12). Se piden al visitar el mes y se guardan: no dependen del año.
const cumplesPorMes = ref<Record<number, Cumpleanero[]>>({})
const mesesCargando = ref<Set<number>>(new Set())
const errorCumples = ref(false)
const especiales = ref<Record<string, DiaEspecial>>({})
const aniosCargados = new Set<number>()
const errorCalendario = ref(false)

const verCumples = ref(true)
const verFeriados = ref(true)
const verNoLaborables = ref(true)

function clave(y: number, m: number, d: number): string {
  return `${y}-${String(m + 1).padStart(2, '0')}-${String(d).padStart(2, '0')}`
}

// Régimen a resaltar: D.L 1057-F (tolera espacios y mayúsculas).
const esRegimenF = (r?: string | null) => /1057\s*-\s*F\b/i.test(r ?? '')

const esBisiesto = (y: number) => (y % 4 === 0 && y % 100 !== 0) || y % 400 === 0

const cargarCumpleanos = async (m: number) => {
  if (cumplesPorMes.value[m] || mesesCargando.value.has(m)) return
  mesesCargando.value = new Set(mesesCargando.value).add(m)
  errorCumples.value = false
  try {
    const lista = await api<Cumpleanero[]>(`/api/dash/cumpleanos_mes?mes=${m}`)
    cumplesPorMes.value = { ...cumplesPorMes.value, [m]: lista ?? [] }
  } catch {
    errorCumples.value = true
  } finally {
    const s2 = new Set(mesesCargando.value)
    s2.delete(m)
    mesesCargando.value = s2
  }
}

const cargarEspeciales = async (y: number) => {
  if (aniosCargados.has(y)) return
  try {
    const meses = await api<MesCalendario[]>(`/api/dash/dias_habiles?anio=${y}`)
    const mapa = { ...especiales.value }
    for (const m of meses ?? []) for (const d of m.dias) mapa[d.fecha] = d
    especiales.value = mapa
    aniosCargados.add(y)
    errorCalendario.value = false
  } catch {
    errorCalendario.value = true
  }
}

onMounted(() => {
  cargarCumpleanos(mes.value + 1)
  cargarEspeciales(anio.value)
})

watch(anio, (y) => cargarEspeciales(y))

// Al navegar a otro mes se consulta ese mes y se ajusta el día seleccionado.
watch([anio, mes], () => {
  cargarCumpleanos(mes.value + 1)
  const [y, m] = seleccionado.value.split('-').map(Number)
  if (y !== anio.value || m - 1 !== mes.value) {
    const esMesActual = hoy.getFullYear() === anio.value && hoy.getMonth() === mes.value
    seleccionado.value = clave(anio.value, mes.value, esMesActual ? hoy.getDate() : 1)
  }
})

const cargandoCumples = computed(() => mesesCargando.value.has(mes.value + 1))

// cumpleaños del mes mostrado por "MM-DD" (29 feb → 28 feb en años no bisiestos)
const cumplesPorDia = computed(() => {
  const mapa: Record<string, Cumpleanero[]> = {}
  for (const c of cumplesPorMes.value[mes.value + 1] ?? []) {
    const [, mm, dd] = (c.nacimiento || '').split('-')
    if (!mm || !dd) continue
    let k = `${mm}-${dd}`
    if (k === '02-29' && !esBisiesto(anio.value)) k = '02-28'
    ;(mapa[k] ||= []).push(c)
  }
  return mapa
})

interface Celda {
  fecha: string
  dia: number
  delMes: boolean
  esHoy: boolean
  finDeSemana: boolean
  cumples: Cumpleanero[]
  cumplesF: number
  especial: DiaEspecial | null
}

const celdas = computed<Celda[]>(() => {
  const primero = new Date(anio.value, mes.value, 1)
  const offset = (primero.getDay() + 6) % 7 // lunes = 0
  const inicio = new Date(anio.value, mes.value, 1 - offset)
  const hoyKey = clave(hoy.getFullYear(), hoy.getMonth(), hoy.getDate())
  const total = Math.ceil((offset + new Date(anio.value, mes.value + 1, 0).getDate()) / 7) * 7
  return Array.from({ length: total }, (_, i) => {
    const d = new Date(inicio.getFullYear(), inicio.getMonth(), inicio.getDate() + i)
    const fecha = clave(d.getFullYear(), d.getMonth(), d.getDate())
    const mmdd = fecha.slice(5)
    const delMes = d.getMonth() === mes.value
    const esp = especiales.value[fecha] ?? null
    const cumples = verCumples.value && delMes ? (cumplesPorDia.value[mmdd] ?? []) : []
    return {
      fecha,
      dia: d.getDate(),
      delMes,
      esHoy: fecha === hoyKey,
      finDeSemana: d.getDay() === 0 || d.getDay() === 6,
      cumples,
      cumplesF: cumples.filter((c) => esRegimenF(c.regimen)).length,
      especial:
        esp && ((esp.tipo === 'feriado' && verFeriados.value) || (esp.tipo === 'no_laborable' && verNoLaborables.value))
          ? esp
          : null,
    }
  })
})

const cumplesDelMes = computed(() =>
  celdas.value.filter((c) => c.delMes).reduce((a, c) => a + c.cumples.length, 0),
)
const cumplesFDelMes = computed(() =>
  celdas.value.filter((c) => c.delMes).reduce((a, c) => a + c.cumplesF, 0),
)
// En el panel del día, los D.L 1057-F van primero.
const cumplesOrdenados = computed(() =>
  [...detalle.value.cumples].sort((a, b) => Number(esRegimenF(b.regimen)) - Number(esRegimenF(a.regimen))),
)
const especialesDelMes = computed(() => celdas.value.filter((c) => c.delMes && c.especial))

const detalle = computed(() => {
  const celda = celdas.value.find((c) => c.fecha === seleccionado.value)
  if (celda) return celda
  const [y, m, d] = seleccionado.value.split('-').map(Number)
  const esp = especiales.value[seleccionado.value] ?? null
  return {
    fecha: seleccionado.value,
    dia: d,
    delMes: m - 1 === mes.value,
    esHoy: false,
    finDeSemana: false,
    cumples: [],
    cumplesF: 0,
    especial: esp,
    anioSel: y,
  } as Celda
})

const fechaLarga = computed(() => {
  const [y, m, d] = seleccionado.value.split('-').map(Number)
  const f = new Date(y, m - 1, d)
  const dia = ['Domingo', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado'][f.getDay()]
  return `${dia} ${d} de ${MESES[m - 1].toLowerCase()} de ${y}`
})

const mover = (delta: number) => {
  const f = new Date(anio.value, mes.value + delta, 1)
  anio.value = f.getFullYear()
  mes.value = f.getMonth()
}

const irAHoy = () => {
  anio.value = hoy.getFullYear()
  mes.value = hoy.getMonth()
  seleccionado.value = clave(hoy.getFullYear(), hoy.getMonth(), hoy.getDate())
}

const edadQueCumple = (c: Cumpleanero) => {
  const y = Number((c.nacimiento || '').slice(0, 4))
  return y ? Number(seleccionado.value.slice(0, 4)) - y : null
}

const iniciales = (n: string | null) => {
  const p = (n || '').trim().split(/\s+/).filter(Boolean)
  return ((p[0]?.[0] || '') + (p[1]?.[0] || '')).toUpperCase() || 'SP'
}

const avatarFallo = ref<Record<string, boolean>>({})
</script>

<template>
  <div class="mx-auto w-full max-w-6xl space-y-3 px-4 pb-12">
    <PageHeader title="Calendario" subtitle="Cumpleaños, feriados y días no laborables del sector público" />

    <div class="flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-1.5">
        <button
          type="button"
          class="size-control-sm inline-flex items-center justify-center rounded-md border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer"
          aria-label="Mes anterior"
          @click="mover(-1)"
        >
          <IconChevronLeft class="size-3.5" />
        </button>
        <h2 class="min-w-36 text-center text-sm font-semibold text-foreground tabular-nums">
          {{ MESES[mes] }} {{ anio }}
        </h2>
        <button
          type="button"
          class="size-control-sm inline-flex items-center justify-center rounded-md border border-border bg-card text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer"
          aria-label="Mes siguiente"
          @click="mover(1)"
        >
          <IconChevronRight class="size-3.5" />
        </button>
        <button
          type="button"
          class="ms-1 h-control-sm px-3 rounded-md border border-border bg-card text-micro font-medium text-foreground hover:bg-muted transition cursor-pointer"
          @click="irAHoy"
        >
          Hoy
        </button>
      </div>

      <div class="flex flex-wrap items-center gap-1.5 text-micro">
        <button
          type="button"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border transition cursor-pointer"
          :class="verCumples ? 'border-sky-500/30 bg-sky-500/10 text-sky-600 dark:text-sky-400' : 'border-border text-muted-foreground'"
          :aria-pressed="verCumples"
          @click="verCumples = !verCumples"
        >
          <span class="size-2 rounded-full bg-sky-500"></span> Cumpleaños
        </button>
        <button
          type="button"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border transition cursor-pointer"
          :class="verFeriados ? 'border-rose-500/30 bg-rose-500/10 text-rose-600 dark:text-rose-400' : 'border-border text-muted-foreground'"
          :aria-pressed="verFeriados"
          @click="verFeriados = !verFeriados"
        >
          <span class="size-2 rounded-full bg-rose-500"></span> Feriados
        </button>
        <button
          type="button"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-md border transition cursor-pointer"
          :class="verNoLaborables ? 'border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400' : 'border-border text-muted-foreground'"
          :aria-pressed="verNoLaborables"
          @click="verNoLaborables = !verNoLaborables"
        >
          <span class="size-2 rounded-full bg-amber-500"></span> No laborables
        </button>
      </div>
    </div>

    <p v-if="errorCumples" class="text-micro text-amber-600 dark:text-amber-400">
      No se pudieron cargar los cumpleaños de {{ MESES[mes] }}.
      <button type="button" class="underline cursor-pointer" @click="cargarCumpleanos(mes + 1)">Reintentar</button>
    </p>
    <p v-if="errorCalendario" class="text-micro text-amber-600 dark:text-amber-400">
      No se pudo cargar el calendario de feriados y días no laborables.
    </p>

    <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_19rem] gap-3 items-start">
      <section class="rounded-xl border border-border bg-card shadow-2xs overflow-hidden">
        <div class="grid grid-cols-7 border-b border-border text-center text-micro font-medium text-muted-foreground">
          <div v-for="d in DIAS_SEMANA" :key="d" class="py-2">{{ d }}</div>
        </div>

        <div class="grid grid-cols-7" role="grid" :aria-label="`Calendario de ${MESES[mes]} ${anio}`">
          <button
            v-for="c in celdas"
            :key="c.fecha"
            type="button"
            role="gridcell"
            class="relative min-h-16 sm:min-h-20 p-1.5 text-left border-b border-e border-border/70 transition-colors cursor-pointer focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-primary focus-visible:z-1"
            :class="[
              c.delMes ? '' : 'bg-muted/20 text-muted-foreground/50',
              c.finDeSemana && c.delMes ? 'bg-muted/30' : '',
              seleccionado === c.fecha ? 'ring-2 ring-inset ring-primary bg-primary/5' : 'hover:bg-muted/50',
            ]"
            :aria-label="`${c.dia} de ${MESES[Number(c.fecha.slice(5, 7)) - 1]}`"
            :aria-selected="seleccionado === c.fecha"
            @click="seleccionado = c.fecha"
          >
            <span
              class="inline-flex size-5 items-center justify-center rounded-full text-micro font-medium tabular-nums"
              :class="c.esHoy ? 'bg-primary text-primary-foreground' : c.delMes ? 'text-foreground' : ''"
            >
              {{ c.dia }}
            </span>

            <span
              v-if="c.especial"
              class="mt-0.5 hidden sm:block truncate rounded px-1 text-nano font-medium"
              :class="
                c.especial.tipo === 'feriado'
                  ? 'bg-rose-500/10 text-rose-600 dark:text-rose-400'
                  : 'bg-amber-500/10 text-amber-600 dark:text-amber-400'
              "
              :title="c.especial.motivo"
            >
              {{ c.especial.motivo }}
            </span>

            <span class="absolute bottom-1 end-1.5 flex items-center gap-1">
              <span v-if="c.especial" class="size-1.5 rounded-full sm:hidden" :class="c.especial.tipo === 'feriado' ? 'bg-rose-500' : 'bg-amber-500'"></span>
              <span
                v-if="c.cumplesF"
                class="inline-flex items-center rounded-full bg-violet-500 px-1.5 py-0.5 text-nano font-semibold text-white"
                :title="`${c.cumplesF} con D.L 1057-F`"
              >
                F{{ c.cumplesF > 1 ? ' ' + c.cumplesF : '' }}
              </span>
              <span
                v-if="c.cumples.length"
                class="inline-flex items-center gap-0.5 rounded-full bg-sky-500/15 px-1.5 py-0.5 text-nano font-semibold text-sky-600 dark:text-sky-400"
                :title="`${c.cumples.length} cumpleaños`"
              >
                <IconCake class="size-2.5" aria-hidden="true" />{{ c.cumples.length }}
              </span>
            </span>
          </button>
        </div>

        <div class="px-4 py-2.5 flex flex-wrap items-center gap-x-4 gap-y-1 text-micro text-muted-foreground">
          <span v-if="cargandoCumples" class="inline-flex items-center gap-1.5"><IconLoader2 class="size-3 animate-spin" /> Cargando cumpleaños...</span>
          <template v-else>
            <span><strong class="font-semibold text-foreground">{{ cumplesDelMes }}</strong> cumpleaños este mes</span>
            <span v-if="cumplesFDelMes" class="inline-flex items-center gap-1.5">
              <span class="size-2 rounded-full bg-violet-500"></span>
              <strong class="font-semibold text-foreground">{{ cumplesFDelMes }}</strong> con D.L 1057-F
            </span>
            <span><strong class="font-semibold text-foreground">{{ especialesDelMes.length }}</strong> {{ especialesDelMes.length === 1 ? 'día especial' : 'días especiales' }}</span>
          </template>
        </div>
      </section>

      <aside class="rounded-xl border border-border bg-card shadow-2xs overflow-hidden lg:sticky lg:top-4">
        <header class="px-4 py-3 border-b border-border">
          <p class="text-micro text-muted-foreground">Día seleccionado</p>
          <h3 class="text-meta font-semibold text-foreground">{{ fechaLarga }}</h3>
        </header>

        <div class="p-4 space-y-4">
          <div v-if="detalle.especial" class="rounded-lg px-3 py-2 text-micro"
            :class="detalle.especial.tipo === 'feriado' ? 'bg-rose-500/10 text-rose-600 dark:text-rose-400' : 'bg-amber-500/10 text-amber-600 dark:text-amber-400'">
            <p class="font-semibold">{{ detalle.especial.tipo === 'feriado' ? 'Feriado' : 'Día no laborable (sector público)' }}</p>
            <p class="opacity-90">{{ detalle.especial.motivo }}</p>
          </div>

          <div>
            <h4 class="text-micro font-medium text-muted-foreground mb-2 flex items-center gap-1.5">
              <IconCake class="size-3.5" aria-hidden="true" />
              Cumpleaños ({{ detalle.cumples.length }})
            </h4>
            <ul v-if="detalle.cumples.length" class="space-y-1.5 max-h-80 overflow-y-auto overscroll-contain pe-1">
              <li v-for="c in cumplesOrdenados" :key="c.dni">
                <button
                  type="button"
                  class="w-full flex items-center gap-2.5 rounded-lg px-2 py-1.5 text-left transition-colors cursor-pointer"
                  :class="
                    esRegimenF(c.regimen)
                      ? 'bg-violet-500/10 ring-1 ring-inset ring-violet-500/40 hover:bg-violet-500/15'
                      : 'hover:bg-muted/60'
                  "
                  @click="router.push(`/perfil/${c.dni}`)"
                >
                  <span class="size-8 rounded-full bg-muted overflow-hidden shrink-0 inline-flex items-center justify-center text-micro font-semibold text-muted-foreground">
                    <img
                      v-if="!avatarFallo[c.dni]"
                      v-auth-src="resolveAvatarUrl(c.dni)!"
                      :alt="c.nombre || ''"
                      class="size-full object-cover"
                      @error="avatarFallo[c.dni] = true"
                    />
                    <span v-else>{{ iniciales(c.nombre) }}</span>
                  </span>
                  <span class="min-w-0 flex-1">
                    <span class="flex items-center gap-1.5 min-w-0">
                      <span class="text-meta font-medium text-foreground truncate">{{ c.nombre }}</span>
                      <span
                        v-if="esRegimenF(c.regimen)"
                        class="shrink-0 rounded bg-violet-500 px-1 py-px text-nano font-semibold text-white"
                      >
                        1057-F
                      </span>
                    </span>
                    <span class="block text-micro text-muted-foreground truncate">
                      Cumple {{ edadQueCumple(c) ?? '—' }} años<template v-if="c.regimen"> · {{ c.regimen }}</template>
                    </span>
                  </span>
                </button>
              </li>
            </ul>
            <p v-else class="text-micro text-muted-foreground">Sin cumpleaños este día.</p>
          </div>

          <div v-if="!detalle.especial && !detalle.cumples.length" class="flex items-center gap-2 text-micro text-muted-foreground">
            <IconCalendarEvent class="size-4" aria-hidden="true" /> Día laborable sin eventos.
          </div>
        </div>
      </aside>
    </div>
  </div>
</template>
