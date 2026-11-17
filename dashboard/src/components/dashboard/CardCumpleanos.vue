<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import { IconCake, IconCalendarEvent } from '@tabler/icons-vue'
import { formatDayMonth, parseDateSafe } from '@/utils/date'
import { resolveAvatarUrl, type Cumpleanero } from './types'

interface Props {
  cumpleanos: Cumpleanero[]
  isLoading?: boolean
  maxHeightClass?: string
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
  maxHeightClass: 'max-h-76',
})

const listContainerRef = ref<HTMLElement | null>(null)
const soloHoy = ref(false)
const avatarErrors = ref<Record<string, boolean>>({})

const isToday = (nacimiento: string | null | undefined): boolean => {
  const d = parseDateSafe(nacimiento)
  if (!d) return false
  const now = new Date()
  return d.getDate() === now.getDate() && d.getMonth() === now.getMonth()
}

const getInitials = (nombre: string): string => {
  const parts = (nombre || '').trim().split(/\s+/).filter(Boolean)
  if (parts.length === 0) return 'SP'
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase()
  return (parts[0][0] + (parts[1]?.[0] || '')).toUpperCase()
}

const cumpleanosHoy = computed(() => {
  return props.cumpleanos.filter((c) => isToday(c.nacimiento))
})

const filteredCumpleanos = computed(() => {
  if (soloHoy.value) {
    return props.cumpleanos.filter((c) => isToday(c.nacimiento))
  }
  return props.cumpleanos
})

const daysUntilBirthday = (d: Date, today: Date): number => {
  const next = new Date(today.getFullYear(), d.getMonth(), d.getDate())
  if (next.getTime() < today.getTime()) {
    next.setFullYear(today.getFullYear() + 1)
  }
  return Math.round((next.getTime() - today.getTime()) / 86400000)
}

const targetIndex = computed(() => {
  if (!filteredCumpleanos.value.length) return -1
  const now = new Date()
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())

  let bestIdx = 0
  let bestDays = Number.MAX_SAFE_INTEGER

  filteredCumpleanos.value.forEach((c, idx) => {
    const d = parseDateSafe(c.nacimiento)
    if (!d) return
    const diff = daysUntilBirthday(d, today)
    if (diff < bestDays) {
      bestDays = diff
      bestIdx = idx
    }
  })

  return bestIdx
})

const scrollToTarget = (smooth = false) => {
  if (!listContainerRef.value || targetIndex.value === -1) return
  const container = listContainerRef.value
  const targetEl = container.querySelector(`[data-index="${targetIndex.value}"]`) as HTMLElement | null
  if (!targetEl) return
  const containerRect = container.getBoundingClientRect()
  const targetRect = targetEl.getBoundingClientRect()
  const offset = targetRect.top - containerRect.top + container.scrollTop
  const top = Math.max(0, offset - 4)
  if (smooth) {
    container.scrollTo({ top, behavior: 'smooth' })
  } else {
    container.scrollTop = top
  }
}

const scheduleScrollToTarget = async (smooth = false) => {
  await nextTick()
  requestAnimationFrame(() => {
    scrollToTarget(smooth)
  })
  setTimeout(() => {
    scrollToTarget(false)
  }, 120)
}

const handleTodayClick = () => {
  soloHoy.value = !soloHoy.value
  if (!soloHoy.value) {
    scheduleScrollToTarget(true)
  }
}

watch(
  () => [props.cumpleanos, props.isLoading],
  ([list, loading]) => {
    if (!loading && Array.isArray(list) && list.length > 0) {
      scheduleScrollToTarget(false)
    }
  },
  { deep: true, immediate: true }
)

onMounted(() => {
  if (!props.isLoading && props.cumpleanos.length > 0) {
    scheduleScrollToTarget(false)
  }
})
</script>

<template>
  <div class="rounded-xl border border-border bg-card overflow-hidden flex flex-col">
    <div class="px-3.5 py-2 border-b border-border flex items-center justify-between gap-2 shrink-0">
      <div class="min-w-0">
        <h3 class="text-meta font-semibold text-foreground tracking-tight flex items-center gap-1.5">
          <IconCake class="size-3.5 text-primary shrink-0" aria-hidden="true" />
          <span>Cumpleaños Próximos</span>
        </h3>
        <p class="text-micro text-muted-foreground">Onomásticos del personal activo</p>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        <router-link
          to="/calendario"
          class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md border border-border bg-card text-nano font-medium text-muted-foreground hover:text-foreground hover:bg-muted transition cursor-pointer"
          title="Abrir el calendario de cumpleaños, feriados y días no laborables"
        >
          <IconCalendarEvent class="size-3" aria-hidden="true" />
          <span>Calendario</span>
        </router-link>
        <button
          v-if="cumpleanosHoy.length > 0"
          type="button"
          class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-nano font-semibold transition cursor-pointer"
          :class="
            soloHoy
              ? 'bg-emerald-600 text-white'
              : 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500/15'
          "
          title="Filtrar onomásticos de hoy"
          @click="handleTodayClick"
        >
          <span class="size-1.5 rounded-full" :class="soloHoy ? 'bg-white' : 'bg-emerald-500'"></span>
          <span>{{ cumpleanosHoy.length }} hoy</span>
        </button>
        <span class="px-1.5 py-0.5 text-nano font-mono rounded-full bg-muted border border-border text-foreground">
          {{ cumpleanos.length }}
        </span>
      </div>
    </div>

    <div class="relative flex-1 min-h-0 overflow-hidden flex flex-col">
      <div
        ref="listContainerRef"
        :class="['flex-1 overflow-y-auto overscroll-contain', maxHeightClass]"
      >
        <div v-if="isLoading" class="p-3.5 space-y-2.5 animate-pulse" aria-busy="true">
          <div v-for="i in 5" :key="i" class="flex items-center justify-between gap-2">
            <div class="flex items-center gap-2.5">
              <div class="size-7 rounded-full bg-muted shrink-0"></div>
              <div class="space-y-1">
                <div class="h-3 w-28 bg-muted rounded"></div>
                <div class="h-2.5 w-20 bg-muted/60 rounded"></div>
              </div>
            </div>
            <div class="h-4 w-12 bg-muted rounded"></div>
          </div>
        </div>

        <div
          v-else-if="filteredCumpleanos.length === 0"
          class="py-8 px-4 text-center space-y-1.5 text-muted-foreground my-auto"
        >
          <IconCake class="size-6 mx-auto text-muted-foreground/40" aria-hidden="true" />
          <p class="font-semibold text-micro text-foreground">
            {{ soloHoy ? 'Sin coincidencias' : 'Sin cumpleaños próximos' }}
          </p>
          <p class="text-micro">
            {{
              soloHoy
                ? 'No hay servidores que coincidan con el filtro aplicado.'
                : 'No constan onomásticos próximos registrados.'
            }}
          </p>
        </div>

        <div v-else class="divide-y divide-border">
          <router-link
            v-for="(c, index) in filteredCumpleanos"
            :key="c.dni"
            :to="{ name: 'perfil', params: { dni: c.dni } }"
            :data-index="index"
            class="px-3.5 py-2 hover:bg-muted/40 focus-visible:bg-muted/40 focus-visible:outline-hidden transition-colors flex items-center justify-between gap-2.5 text-micro group cursor-pointer"
            :class="isToday(c.nacimiento) ? 'bg-emerald-500/5' : ''"
          >
            <div class="flex items-center gap-2.5 min-w-0">
              <img
                v-if="c.avatar && !avatarErrors[c.dni]"
                v-auth-src="resolveAvatarUrl(c.dni)!"
                :alt="c.nombre"
                class="size-7 rounded-full object-cover border border-border shrink-0 group-hover:border-primary transition-colors"
                @error="avatarErrors[c.dni] = true"
              />
              <div
                v-else
                class="size-7 rounded-full bg-primary/10 text-primary flex items-center justify-center font-semibold text-nano shrink-0 border border-primary/20 select-none"
              >
                {{ getInitials(c.nombre) }}
              </div>

              <div class="min-w-0">
                <p class="font-medium text-micro text-foreground group-hover:text-primary transition-colors truncate" :title="c.nombre">
                  {{ c.nombre }}
                </p>
                <p class="text-nano text-muted-foreground font-mono truncate">
                  DNI {{ c.dni }}<span v-if="c.regimen"> · {{ c.regimen }}</span>
                </p>
              </div>
            </div>

            <div class="shrink-0 flex items-center gap-1.5">
              <span
                v-if="isToday(c.nacimiento)"
                class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 text-nano font-semibold"
              >
                <span class="size-1.5 rounded-full bg-emerald-500"></span>
                Hoy
              </span>
              <span class="text-micro font-semibold text-foreground font-mono tabular-nums">
                {{ formatDayMonth(c.nacimiento) }}
              </span>
              <span class="px-1.5 py-0.5 rounded bg-muted text-muted-foreground font-mono text-nano tabular-nums">
                {{ c.edad }}a
              </span>
            </div>
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>
