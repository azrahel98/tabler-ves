<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import { IconCake } from '@tabler/icons-vue'
import { formatDayMonth, parseDateSafe } from '@/utils/date'
import { resolveAvatarUrl, type Cumpleanero } from './types'

interface Props {
  cumpleanos: Cumpleanero[]
  isLoading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isLoading: false,
})

const listContainerRef = ref<HTMLElement | null>(null)

const isToday = (nacimiento: string | null | undefined): boolean => {
  const d = parseDateSafe(nacimiento)
  if (!d) return false
  const now = new Date()
  return d.getDate() === now.getDate() && d.getMonth() === now.getMonth()
}

const cumpleanosHoy = computed(() => {
  return props.cumpleanos.filter((c) => isToday(c.nacimiento))
})

const targetIndex = computed(() => {
  if (!props.cumpleanos.length) return -1
  const now = new Date()
  const todayDay = now.getDate()
  const todayMonth = now.getMonth()

  const todayIdx = props.cumpleanos.findIndex((c) => {
    const d = parseDateSafe(c.nacimiento)
    return d && d.getDate() === todayDay && d.getMonth() === todayMonth
  })
  if (todayIdx !== -1) return todayIdx

  const upcomingIdx = props.cumpleanos.findIndex((c) => {
    const d = parseDateSafe(c.nacimiento)
    if (!d) return false
    if (d.getMonth() === todayMonth) {
      return d.getDate() >= todayDay
    }
    return d.getMonth() > todayMonth
  })
  return upcomingIdx !== -1 ? upcomingIdx : 0
})

const scrollToTarget = (smooth = true) => {
  if (!listContainerRef.value || targetIndex.value === -1) return
  const container = listContainerRef.value
  const targetEl = container.querySelector(`[data-index="${targetIndex.value}"]`) as HTMLElement | null
  if (targetEl) {
    const containerRect = container.getBoundingClientRect()
    const targetRect = targetEl.getBoundingClientRect()
    const offset = targetRect.top - containerRect.top + container.scrollTop
    container.scrollTo({
      top: Math.max(0, offset - 8),
      behavior: smooth ? 'smooth' : 'auto',
    })
  }
}

watch(
  () => [props.cumpleanos, props.isLoading],
  async ([list, loading]) => {
    if (!loading && Array.isArray(list) && list.length > 0) {
      await nextTick()
      setTimeout(() => {
        scrollToTarget(true)
      }, 100)
    }
  },
  { deep: true }
)

onMounted(async () => {
  if (!props.isLoading && props.cumpleanos.length > 0) {
    await nextTick()
    setTimeout(() => {
      scrollToTarget(false)
    }, 100)
  }
})
</script>

<template>
  <Card :no-padding="true" class="shadow-2xs">
    <div class="flex items-center justify-between border-b border-border p-3.5 sm:px-4">
      <div class="flex items-center gap-2">
        <IconCake class="size-4 text-pink-500" aria-hidden="true" />
        <div>
          <h3 class="font-semibold text-foreground tracking-tight text-sm">Cumpleaños Próximos</h3>
          <p class="text-[11px] text-muted-foreground">Onomásticos del personal activo</p>
        </div>
      </div>
      <div class="flex items-center gap-1.5">
        <button v-if="cumpleanosHoy.length > 0" type="button"
          class="inline-flex cursor-pointer transition-transform active:scale-95" @click="scrollToTarget(true)">
          <Badge variant="success" size="xs" class="gap-1">
            <span class="size-1.5 rounded-full bg-emerald-500 animate-ping"></span>
            {{ cumpleanosHoy.length }} Hoy
          </Badge>
        </button>
        <Badge variant="primary" size="xs">{{ cumpleanos.length }} Próximos</Badge>
      </div>
    </div>

    <div ref="listContainerRef" class="max-h-65 overflow-y-auto scroll-smooth">
      <div v-if="isLoading" class="p-4 space-y-3 animate-pulse" aria-busy="true">
        <div v-for="i in 2" :key="i" class="flex items-center justify-between">
          <div class="flex items-center gap-2.5">
            <div class="size-8 rounded-full bg-muted"></div>
            <div class="space-y-1">
              <div class="h-3 w-24 bg-muted rounded"></div>
              <div class="h-2.5 w-32 bg-muted rounded"></div>
            </div>
          </div>
          <div class="h-3 w-10 bg-muted rounded"></div>
        </div>
      </div>

      <div v-else-if="cumpleanos.length === 0" class="py-8 px-4 text-center space-y-1.5 text-muted-foreground">
        <div class="size-8 rounded-full bg-muted/60 flex items-center justify-center mx-auto text-muted-foreground">
          <IconCake class="size-4" aria-hidden="true" />
        </div>
        <p class="font-medium text-xs text-foreground">Sin cumpleaños este mes</p>
        <p class="text-[11px]">No hay onomásticos registrados.</p>
      </div>

      <div v-else class="divide-y divide-border">
        <router-link v-for="(c, index) in cumpleanos" :key="c.dni" :to="{ name: 'perfil', params: { dni: c.dni } }"
          :data-index="index"
          class="p-3 hover:bg-muted/30 focus-visible:bg-muted/40 focus-visible:outline-hidden transition-colors flex items-center justify-between gap-2.5 text-xs group cursor-pointer">
          <div class="flex items-center gap-2.5 min-w-0">
            <img v-if="resolveAvatarUrl(c.avatar)" v-auth-src="resolveAvatarUrl(c.avatar)!" :alt="c.nombre"
              class="size-8 rounded-full object-cover border border-border shrink-0 shadow-2xs group-hover:border-primary transition-colors"
              :class="isToday(c.nacimiento) ? 'border-pink-500 ring-2 ring-pink-500/30' : ''" />
            <div v-else
              class="size-8 rounded-full bg-pink-500/10 text-pink-600 dark:text-pink-400 flex items-center justify-center font-bold text-xs shrink-0 border border-pink-500/20 group-hover:border-pink-500 transition-colors"
              :class="isToday(c.nacimiento) ? 'border-pink-500 ring-2 ring-pink-500/30' : ''">
              {{ c.nombre.charAt(0) }}
            </div>

            <div class="space-y-0.5 min-w-0">
              <p
                class="font-medium text-[11.5px] text-foreground group-hover:text-primary transition-colors wrap-break-word">
                {{ c.nombre }}
              </p>
              <p class="text-[11px] text-muted-foreground truncate">{{ c.regimen }} &bull; DNI {{ c.dni }}</p>
            </div>
          </div>

          <div class="text-right shrink-0 flex flex-col items-end gap-0.5">
            <span class="text-xs font-bold text-foreground block font-mono">{{ formatDayMonth(c.nacimiento) }}</span>
            <div class="flex items-center gap-1">
              <Badge v-if="isToday(c.nacimiento)" variant="success" size="xs">¡Hoy!</Badge>
              <Badge variant="outline" size="xs">{{ c.edad }} años</Badge>
            </div>
          </div>
        </router-link>
      </div>
    </div>
  </Card>
</template>
