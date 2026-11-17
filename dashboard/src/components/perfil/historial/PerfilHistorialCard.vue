<script setup lang="ts">
import { ref, computed, watch, onMounted, type Component } from 'vue'
import { fetchHistorialPersona, type HistorialEntrada } from '@/services/personal'
import { formatDate } from '@/utils/date'
import {
  IconHistory,
  IconPlus,
  IconPencil,
  IconTrash,
  IconFileX,
  IconChevronLeft,
  IconChevronRight,
  IconChevronDown,
  IconAlertTriangle,
  IconLoader2,
} from '@tabler/icons-vue'

const props = defineProps<{ dni: string }>()

const PAGE_SIZE = 8
const VISIBLES = 4

const items = ref<HistorialEntrada[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)
const cargando = ref(false)
const error = ref<string | null>(null)
const expandidos = ref<Set<number>>(new Set())

const cargar = async () => {
  if (!props.dni) return
  cargando.value = true
  error.value = null
  try {
    const res = await fetchHistorialPersona(props.dni, page.value, PAGE_SIZE)
    items.value = res.items ?? []
    total.value = res.total ?? 0
    totalPages.value = Math.max(1, res.total_pages ?? 1)
    expandidos.value = new Set()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'No se pudo cargar el historial.'
  } finally {
    cargando.value = false
  }
}

onMounted(cargar)
watch(
  () => props.dni,
  () => {
    page.value = 1
    cargar()
  },
)

const irA = (n: number) => {
  page.value = Math.min(Math.max(1, n), totalPages.value)
  cargar()
}

interface Estilo {
  icon: Component
  tono: string
}

const estiloDe = (op: string): Estilo => {
  const o = op.toLowerCase()
  if (o.includes('eliminar')) return { icon: IconTrash, tono: 'bg-rose-500/10 text-rose-600 dark:text-rose-400' }
  if (o.includes('renuncia')) return { icon: IconFileX, tono: 'bg-amber-500/10 text-amber-600 dark:text-amber-400' }
  if (/(registrar|crear|ingresar|agregar|nuevo)/.test(o))
    return { icon: IconPlus, tono: 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' }
  if (/(editar|actualizar|cambio|modificar)/.test(o))
    return { icon: IconPencil, tono: 'bg-sky-500/10 text-sky-600 dark:text-sky-400' }
  return { icon: IconHistory, tono: 'bg-muted text-muted-foreground' }
}

const titulo = (op: string) => op.charAt(0).toUpperCase() + op.slice(1)

const humanizar = (k: string) => {
  const t = k.replace(/_/g, ' ').trim().toLowerCase()
  return t ? t.charAt(0).toUpperCase() + t.slice(1) : k
}

const fechaHora = (f: string) => {
  const [d, h] = (f || '').split(' ')
  return `${formatDate(d)}${h ? ' · ' + h.slice(0, 5) : ''}`
}

interface Fila {
  campo: string
  antes?: string
  despues?: string
  valor?: string
}

const txt = (v: unknown): string =>
  v === null || v === undefined || v === '' ? '—' : typeof v === 'object' ? JSON.stringify(v) : String(v)

const filas = (data: unknown): Fila[] => {
  if (data === undefined || data === null) return []
  if (typeof data !== 'object') return [{ campo: 'Detalle', valor: txt(data) }]
  const out: Fila[] = []
  for (const [k, v] of Object.entries(data as Record<string, unknown>)) {
    if (Array.isArray(v)) {
      if (v.length === 0) continue
      const todosObjetos = v.every((item) => item && typeof item === 'object' && !Array.isArray(item))
      if (todosObjetos) {
        for (const item of v as Record<string, unknown>[]) {
          if ('campo' in item && ('antes' in item || 'despues' in item)) {
            out.push({
              campo: humanizar(String(item.campo ?? k)),
              antes: txt(item.antes),
              despues: txt(item.despues),
            })
          } else if ('campo' in item && 'valor' in item) {
            out.push({
              campo: humanizar(String(item.campo ?? k)),
              valor: txt(item.valor),
            })
          } else {
            out.push(...filas(item))
          }
        }
      } else {
        out.push({ campo: humanizar(k), valor: v.map(txt).join(', ') })
      }
    } else if (v && typeof v === 'object') {
      const o = v as Record<string, unknown>
      if ('antes' in o || 'despues' in o) {
        out.push({ campo: humanizar(k), antes: txt(o.antes), despues: txt(o.despues) })
      } else {
        out.push(...filas(v))
      }
    } else {
      out.push({ campo: humanizar(k), valor: txt(v) })
    }
  }
  return out
}

const detalleDe = (e: HistorialEntrada): { filas: Fila[]; ip: string } => {
  const d = e.detalle
  if (!d) return { filas: [], ip: '' }
  if (typeof d === 'string') return { filas: [{ campo: 'Detalle', valor: d }], ip: '' }
  return { filas: filas(d.data), ip: d.metadata?.ip ?? '' }
}

const filasVisibles = (e: HistorialEntrada, i: number) => {
  const todas = detalleDe(e).filas
  return expandidos.value.has(i) ? todas : todas.slice(0, VISIBLES)
}

const alternar = (i: number) => {
  const s = new Set(expandidos.value)
  if (s.has(i)) s.delete(i)
  else s.add(i)
  expandidos.value = s
}

const vacio = computed(() => !cargando.value && !error.value && items.value.length === 0)
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden">
    <div class="px-4 py-3 border-b border-border flex items-center justify-between gap-3">
      <div>
        <h3 class="text-meta font-semibold text-foreground">Historial de cambios</h3>
        <p class="text-micro text-muted-foreground">Registro de las operaciones realizadas sobre este servidor</p>
      </div>
      <span v-if="total" class="text-micro text-muted-foreground shrink-0">
        <span class="font-semibold text-foreground">{{ total }}</span> {{ total === 1 ? 'registro' : 'registros' }}
      </span>
    </div>

    <div
      v-if="cargando && items.length === 0"
      class="py-12 flex items-center justify-center gap-2 text-micro text-muted-foreground"
    >
      <IconLoader2 class="size-4 animate-spin" aria-hidden="true" /> Cargando historial...
    </div>

    <div v-else-if="error" class="py-10 text-center space-y-2" role="alert">
      <IconAlertTriangle class="size-6 mx-auto text-rose-500" aria-hidden="true" />
      <p class="text-micro text-muted-foreground">{{ error }}</p>
      <button type="button" class="text-micro text-primary font-medium hover:underline cursor-pointer" @click="cargar">
        Reintentar
      </button>
    </div>

    <div v-else-if="vacio" class="py-12 text-center space-y-1.5">
      <IconHistory class="size-7 mx-auto text-muted-foreground/40" aria-hidden="true" />
      <p class="text-meta font-semibold text-foreground">Sin historial</p>
      <p class="text-micro text-muted-foreground">Aún no hay operaciones registradas para este servidor.</p>
    </div>

    <ol v-else class="px-4 py-4" :class="cargando ? 'opacity-60 transition-opacity' : ''">
      <li v-for="(e, i) in items" :key="e.fecha + e.operacion + i" class="relative flex gap-3 pb-5 last:pb-0">
        <span v-if="i < items.length - 1" class="absolute start-3.5 top-8 bottom-0 w-px bg-border" aria-hidden="true" />
        <span
          class="size-7 rounded-full inline-flex items-center justify-center shrink-0 z-1"
          :class="estiloDe(e.operacion).tono"
        >
          <component :is="estiloDe(e.operacion).icon" class="size-3.5" aria-hidden="true" />
        </span>

        <div class="min-w-0 flex-1">
          <div class="flex flex-wrap items-baseline justify-between gap-x-3">
            <p class="text-meta font-semibold text-foreground">{{ titulo(e.operacion) }}</p>
            <p class="text-micro text-muted-foreground font-mono">{{ fechaHora(e.fecha) }}</p>
          </div>
          <p class="text-micro text-muted-foreground">por {{ e.nombre || 'Sistema' }}</p>

          <template v-if="detalleDe(e).filas.length">
            <dl class="mt-2 rounded-lg border border-border bg-muted/20 divide-y divide-border">
              <div
                v-for="(f, j) in filasVisibles(e, i)"
                :key="j"
                class="px-2.5 py-1.5 flex flex-wrap items-baseline gap-x-3 text-micro"
              >
                <dt class="text-muted-foreground w-40 sm:w-44 shrink-0 truncate" :title="f.campo">{{ f.campo }}</dt>
                <dd class="min-w-0 text-foreground wrap-break-word">
                  <template v-if="f.valor !== undefined">{{ f.valor }}</template>
                  <template v-else>
                    <span class="text-muted-foreground line-through">{{ f.antes }}</span>
                    <span class="mx-1 text-muted-foreground">→</span>
                    <span class="font-medium">{{ f.despues }}</span>
                  </template>
                </dd>
              </div>
            </dl>
            <div class="mt-1.5 flex items-center justify-between gap-3">
              <button
                v-if="detalleDe(e).filas.length > VISIBLES"
                type="button"
                class="inline-flex items-center gap-1 text-micro text-primary font-medium hover:underline cursor-pointer"
                @click="alternar(i)"
              >
                <IconChevronDown
                  class="size-3 transition-transform"
                  :class="expandidos.has(i) ? 'rotate-180' : ''"
                  aria-hidden="true"
                />
                {{ expandidos.has(i) ? 'Ver menos' : `Ver los ${detalleDe(e).filas.length} campos` }}
              </button>
              <span v-else />
              <span v-if="detalleDe(e).ip" class="text-nano font-mono text-muted-foreground">IP {{ detalleDe(e).ip }}</span>
            </div>
          </template>
        </div>
      </li>
    </ol>

    <div
      v-if="totalPages > 1"
      class="px-4 py-3 border-t border-border flex items-center justify-between gap-3 text-meta text-muted-foreground"
    >
      <span>Página <span class="font-semibold text-foreground">{{ page }}</span> de {{ totalPages }}</span>
      <nav class="flex items-center gap-1" aria-label="Paginación del historial">
        <button
          type="button"
          class="size-7 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
          :disabled="page <= 1 || cargando"
          aria-label="Página anterior"
          @click="irA(page - 1)"
        >
          <IconChevronLeft class="size-3.5" />
        </button>
        <button
          type="button"
          class="size-7 inline-flex items-center justify-center rounded-lg hover:bg-muted disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
          :disabled="page >= totalPages || cargando"
          aria-label="Página siguiente"
          @click="irA(page + 1)"
        >
          <IconChevronRight class="size-3.5" />
        </button>
      </nav>
    </div>
  </div>
</template>
