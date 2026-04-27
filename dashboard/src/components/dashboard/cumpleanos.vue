<template>
  <div class="rounded-2xl border border-gray-200 bg-card p-4 dark:border-gray-800 dark:bg-white/[0.03] md:p-6 flex flex-col gap-5">

    
    <div class="flex items-center justify-between gap-3">
      <h3 class="text-title-md font-semibold leading-snug text-gray-800 dark:text-white/90">Cumpleaños</h3>

      <div class="flex items-center gap-0.5">
        <button
          @click="prevMonth"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-white/5 dark:hover:text-gray-200 transition-colors">
          <ChevronLeft class="w-3.5 h-3.5" />
        </button>
        <span
          class="text-sm font-medium text-gray-600 dark:text-gray-300 capitalize px-1"
          style="min-width: 6rem; text-align: center;">
          {{ format(viewDate, "MMM yyyy", { locale: es }) }}
        </span>
        <button
          @click="nextMonth"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-white/5 dark:hover:text-gray-200 transition-colors">
          <ChevronRight class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>

    
    <div class="flex flex-col divide-y divide-gray-100 dark:divide-gray-800 overflow-y-auto max-h-128 custom-scrollbar -mx-1 px-1">
      <RouterLink
        v-for="item in listaDelMes"
        :key="item.dni"
        :to="{ name: 'personal-profile', params: { dni: item.dni } }"
        class="flex gap-2 items-start py-1 first:pt-0 last:pb-0 hover:opacity-80 transition-opacity group">

        
        <span
          class="mt-1.5 h-2 w-2 shrink-0 rounded-full"
          :class="dotColor(item.daysUntil)" />

        
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 justify-between">
            <p class="text-sm font-semibold text-gray-800 dark:text-white/90 leading-snug truncate">
              {{ item.nombre }}
            </p>
            <span
              class="shrink-0 inline-flex items-center rounded-md px-2 py-0.5 text-2xs font-medium capitalize tracking-wide"
              :class="diasBadgeClass(item.daysUntil)">
              {{ diasLabel(item.daysUntil) }}
            </span>
          </div>
          <p class="text-xs text-gray-400 uppercase tracking-wide truncate">
            <template v-if="item.area">{{ item.area }}</template>
          </p>
          <div class="flex items-center gap-1 mt-1.5">
            <Calendar class="w-3 h-3 text-gray-400 shrink-0" />
            <span class="text-xs text-gray-400">
              {{ format(item.fechaCumple, "MMM dd", { locale: es }) }} · {{ age(item.nacimiento) }} años
            </span>
          </div>
        </div>
      </RouterLink>

      <div v-if="listaDelMes.length === 0" class="flex items-center justify-center py-10">
        <p class="text-sm text-gray-400">Sin cumpleaños este mes</p>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { RouterLink } from 'vue-router'
  import { useTableroStore } from '../../stores/dashboard'
  import { storeToRefs } from 'pinia'
  import {
    differenceInYears, differenceInCalendarDays,
    format, parseISO, isValid, startOfDay,
  } from 'date-fns'
  import { es } from 'date-fns/locale'
  import { ChevronLeft, ChevronRight, Calendar } from 'lucide-vue-next'

  interface ItemCumple {
    dni: string
    nombre: string
    nacimiento: string
    area?: string | null
    fechaCumple: Date
    daysUntil: number
  }

  const { cumpleanos } = storeToRefs(useTableroStore())

  const today = startOfDay(new Date())
  const viewDate = ref(new Date(today.getFullYear(), today.getMonth(), 1))

  const prevMonth = () => {
    viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() - 1, 1)
  }
  const nextMonth = () => {
    viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() + 1, 1)
  }

  const age = (nacimiento: string) => {
    const d = parseISO(nacimiento)
    if (!isValid(d)) return '?'
    return differenceInYears(today, d)
  }

  const listaDelMes = computed<ItemCumple[]>(() => {
    if (!cumpleanos.value?.length) return []
    const month = viewDate.value.getMonth()
    const year  = viewDate.value.getFullYear()
    const result: ItemCumple[] = []

    for (const b of cumpleanos.value as any[]) {
      const d = parseISO(b.nacimiento)
      if (!isValid(d) || d.getMonth() !== month) continue
      const fechaCumple = new Date(year, month, d.getDate())
      result.push({
        dni:        b.dni,
        nombre:     b.nombre,
        nacimiento: b.nacimiento,
        area:       b.area ?? null,
        fechaCumple,
        daysUntil:  differenceInCalendarDays(fechaCumple, today),
      })
    }

    return result.sort((a, b) => a.fechaCumple.getDate() - b.fechaCumple.getDate())
  })

  function dotColor(daysUntil: number): string {
    if (daysUntil === 0) return 'bg-primary'
    if (daysUntil > 0 && daysUntil <= 7) return 'bg-warning-400'
    if (daysUntil < 0) return 'bg-gray-300 dark:bg-gray-600'
    return 'bg-gray-400 dark:bg-gray-500'
  }

  function diasBadgeClass(daysUntil: number): string {
    if (daysUntil === 0)
      return 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300'
    if (daysUntil > 0 && daysUntil <= 7)
      return 'bg-warning-50 text-warning-700 dark:bg-warning-500/10 dark:text-warning-400'
    if (daysUntil < 0)
      return 'bg-gray-100 text-gray-400 dark:bg-gray-800 dark:text-gray-500'
    return 'bg-gray-100 text-gray-500 dark:bg-gray-800 dark:text-gray-400'
  }

  function diasLabel(daysUntil: number): string {
    if (daysUntil === 0)  return 'Hoy'
    if (daysUntil === 1)  return 'Mañana'
    if (daysUntil > 1)    return `en ${daysUntil} días`
    if (daysUntil === -1) return 'Ayer'
    return `hace ${Math.abs(daysUntil)} días`
  }
</script>
