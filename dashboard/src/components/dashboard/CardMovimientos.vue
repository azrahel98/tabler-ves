<script setup lang="ts">
import { ref, computed } from 'vue'
import { resolveAvatarUrl, type EventoVinculo } from './types'
import { formatDate } from '@/utils/date'
import { IconArrowsExchange, IconArrowRight } from '@tabler/icons-vue'

interface Props {
  eventos: EventoVinculo[]
  isLoading?: boolean
  maxHeightClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  maxHeightClass: 'max-h-64',
})

const avatarErrors = ref<Record<string, boolean>>({})
const soloActivos = ref(true)

interface TipoMeta {
  label: string
  dot: string
  badge: string
}

const TIPOS: Record<string, TipoMeta> = {
  rotacion: { label: 'Rotación', dot: 'bg-sky-500', badge: 'bg-sky-500/10 text-sky-700 dark:text-sky-400 border-sky-500/20' },
  destaque: { label: 'Destaque', dot: 'bg-violet-500', badge: 'bg-violet-500/10 text-violet-700 dark:text-violet-400 border-violet-500/20' },
  encargatura: { label: 'Encargatura', dot: 'bg-blue-500', badge: 'bg-blue-500/10 text-blue-700 dark:text-blue-400 border-blue-500/20' },
  encargo_puesto: { label: 'Encargo de puesto', dot: 'bg-blue-500', badge: 'bg-blue-500/10 text-blue-700 dark:text-blue-400 border-blue-500/20' },
  encargo_funciones: { label: 'Encargo de funciones', dot: 'bg-indigo-500', badge: 'bg-indigo-500/10 text-indigo-700 dark:text-indigo-400 border-indigo-500/20' },
  suspension: { label: 'Suspensión', dot: 'bg-rose-500', badge: 'bg-rose-500/10 text-rose-700 dark:text-rose-400 border-rose-500/20' },
  abandono: { label: 'Abandono', dot: 'bg-rose-500', badge: 'bg-rose-500/10 text-rose-700 dark:text-rose-400 border-rose-500/20' },
  'licencia sin goce': { label: 'Licencia sin goce', dot: 'bg-amber-500', badge: 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20' },
  licencia: { label: 'Licencia', dot: 'bg-amber-500', badge: 'bg-amber-500/10 text-amber-700 dark:text-amber-400 border-amber-500/20' },
  vacaciones: { label: 'Vacaciones', dot: 'bg-emerald-500', badge: 'bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border-emerald-500/20' },
}

const FALLBACK: TipoMeta = { label: 'Otro', dot: 'bg-neutral-400', badge: 'bg-muted text-muted-foreground border-border' }

const meta = (tipo: string): TipoMeta => TIPOS[tipo] ?? { ...FALLBACK, label: tipo.replace(/_/g, ' ') }

const activos = computed(() => props.eventos.filter((e) => e.estado === 'activo'))

const lista = computed(() => (soloActivos.value ? activos.value : props.eventos))

const resumenTipos = computed(() => {
  const conteo = new Map<string, number>()
  for (const e of activos.value) conteo.set(e.tipo_evento, (conteo.get(e.tipo_evento) ?? 0) + 1)
  return [...conteo.entries()]
    .map(([tipo, cantidad]) => ({ tipo, cantidad, ...meta(tipo) }))
    .sort((a, b) => b.cantidad - a.cantidad)
})

const iniciales = (nombre: string) =>
  nombre
    .split(' ')
    .filter(Boolean)
    .slice(0, 2)
    .map((p) => p[0])
    .join('')

/** Lo que cambió con el evento: área, cargo o ambos; vacío si no aplica. */
const destino = (e: EventoVinculo): string => {
  const partes: string[] = []
  if (e.area_nueva && e.area_nueva !== e.area_original) partes.push(e.area_nueva)
  if (e.cargo_nuevo && e.cargo_nuevo !== e.cargo) partes.push(e.cargo_nuevo)
  return partes.join(' · ')
}

const vigencia = (e: EventoVinculo): string => {
  const desde = e.fecha_desde || e.fecha_inicio
  const hasta = e.fecha_hasta || e.fecha_salida
  if (desde && hasta) return `${formatDate(desde)} → ${formatDate(hasta)}`
  if (desde) return `Desde ${formatDate(desde)}`
  return '-'
}
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden flex flex-col">
    <div class="px-3.5 py-2 border-b border-border flex items-center justify-between gap-2 shrink-0">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconArrowsExchange class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Eventos de Vínculo</span>
        </h3>
        <p class="text-micro text-muted-foreground">Encargos, destaques, licencias y otros movimientos del personal</p>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        <div class="inline-flex rounded-md border border-border bg-card overflow-hidden text-nano font-medium">
          <button
            type="button"
            class="px-2 py-0.5 transition cursor-pointer"
            :class="soloActivos ? 'bg-muted text-foreground' : 'text-muted-foreground hover:text-foreground'"
            @click="soloActivos = true"
          >
            Vigentes
          </button>
          <button
            type="button"
            class="px-2 py-0.5 border-s border-border transition cursor-pointer"
            :class="!soloActivos ? 'bg-muted text-foreground' : 'text-muted-foreground hover:text-foreground'"
            @click="soloActivos = false"
          >
            Todos
          </button>
        </div>
        <span class="px-1.5 py-0.5 text-nano font-mono rounded-full bg-muted border border-border text-foreground">
          {{ lista.length }}
        </span>
      </div>
    </div>

    <div v-if="!isLoading && resumenTipos.length > 0" class="px-3.5 py-2 border-b border-border flex flex-wrap gap-1.5 shrink-0">
      <span
        v-for="t in resumenTipos"
        :key="t.tipo"
        class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md border text-nano font-medium"
        :class="t.badge"
      >
        <span class="size-1.5 rounded-full" :class="t.dot"></span>
        <span>{{ t.label }}</span>
        <strong class="font-mono font-semibold tabular-nums">{{ t.cantidad }}</strong>
      </span>
    </div>

    <div class="relative flex-1 min-h-0 overflow-hidden flex flex-col">
      <div :class="['flex-1 overflow-y-auto overscroll-contain', maxHeightClass]">
        <div v-if="isLoading" class="p-3.5 space-y-2.5 animate-pulse" aria-busy="true">
          <div v-for="i in 5" :key="i" class="flex items-center justify-between gap-2">
            <div class="flex items-center gap-2.5">
              <div class="size-7 rounded-full bg-muted shrink-0"></div>
              <div class="space-y-1">
                <div class="h-3 w-32 bg-muted rounded"></div>
                <div class="h-2.5 w-24 bg-muted/60 rounded"></div>
              </div>
            </div>
            <div class="h-4 w-16 bg-muted rounded"></div>
          </div>
        </div>

        <div v-else-if="lista.length === 0" class="py-8 px-4 text-center space-y-1.5 text-muted-foreground">
          <IconArrowsExchange class="size-6 mx-auto text-muted-foreground/40" aria-hidden="true" />
          <p class="font-semibold text-micro text-foreground">
            {{ soloActivos ? 'Sin movimientos vigentes' : 'Sin movimientos registrados' }}
          </p>
          <p class="text-micro">No constan eventos de vínculo en el sistema.</p>
        </div>

        <div v-else class="divide-y divide-border">
          <router-link
            v-for="e in lista"
            :key="e.id"
            :to="{ name: 'perfil', params: { dni: e.dni } }"
            class="px-3.5 py-2 hover:bg-muted/40 focus-visible:bg-muted/40 focus-visible:outline-hidden transition-colors flex items-center justify-between gap-2.5 text-micro cursor-pointer"
            :class="e.estado !== 'activo' ? 'opacity-70' : ''"
          >
            <div class="flex items-center gap-2.5 min-w-0">
              <img
                v-if="e.avatar && !avatarErrors[e.dni]"
                v-auth-src="resolveAvatarUrl(e.dni)!"
                :alt="e.nombre"
                class="size-7 rounded-full object-cover border border-border shrink-0"
                @error="avatarErrors[e.dni] = true"
              />
              <div
                v-else
                class="size-7 rounded-full bg-muted border border-border flex items-center justify-center text-nano font-semibold text-muted-foreground shrink-0"
              >
                {{ iniciales(e.nombre) }}
              </div>

              <div class="min-w-0">
                <p class="font-semibold text-foreground truncate" :title="e.nombre">{{ e.nombre }}</p>
                <p class="text-nano text-muted-foreground truncate" :title="`${e.area_original} · ${e.cargo}`">
                  <span>{{ e.area_original }}</span>
                  <template v-if="destino(e)">
                    <IconArrowRight class="inline size-2.5 mx-0.5 align-middle" aria-hidden="true" />
                    <span class="text-foreground/80">{{ destino(e) }}</span>
                  </template>
                </p>
              </div>
            </div>

            <div class="text-end shrink-0">
              <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md border text-nano font-medium" :class="meta(e.tipo_evento).badge">
                {{ meta(e.tipo_evento).label }}
              </span>
              <p class="mt-0.5 text-nano font-mono text-muted-foreground tabular-nums">{{ vigencia(e) }}</p>
            </div>
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>
