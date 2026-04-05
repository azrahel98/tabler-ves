<template>
  <div class="rounded-2xl border border-gray-100 bg-white dark:border-white/[0.06] dark:bg-white/[0.03] flex flex-col h-full">

    <div class="flex items-center justify-between px-5 pt-5 pb-4 border-b border-gray-100 dark:border-white/[0.06]">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-xl bg-brand-500 flex items-center justify-center shrink-0">
          <Cake class="w-4 h-4 text-white" />
        </div>
        <div>
          <h3 class="text-sm font-semibold text-gray-800 dark:text-white/90 leading-none">Cumpleaños</h3>
          <p class="text-[11px] text-gray-400 dark:text-gray-500 mt-1 leading-none">
            <template v-if="totalThisMonth > 0">
              <span class="text-brand-500 font-medium">{{ totalThisMonth }}</span> este mes
            </template>
            <template v-else>Ninguno este mes</template>
          </p>
        </div>
      </div>

      <div class="flex items-center gap-0.5">
        <button
          @click="prevMonth"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-white/5 dark:hover:text-gray-200 transition-colors">
          <ChevronLeft class="w-3.5 h-3.5" />
        </button>
        <span class="text-xs font-medium text-gray-600 dark:text-gray-300 capitalize px-1" style="min-width: 6rem; text-align: center;">
          {{ format(viewDate, "MMM yyyy", { locale: es }) }}
        </span>
        <button
          @click="nextMonth"
          class="w-7 h-7 rounded-lg flex items-center justify-center text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-white/5 dark:hover:text-gray-200 transition-colors">
          <ChevronRight class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>

    <div class="px-4 py-4 flex flex-col gap-3 h-full">

      <div class="grid grid-cols-7">
        <div
          v-for="d in DAY_NAMES"
          :key="d"
          class="text-center text-[10px] font-medium text-gray-300 dark:text-gray-600 uppercase tracking-widest py-1">
          {{ d }}
        </div>
      </div>

      <div class="grid grid-cols-7 gap-y-1 h-full">
        <div
          v-for="(cell, i) in calendarCells"
          :key="i"
          class="flex justify-center">

          <template v-if="cell.day !== null">

            <div
              v-if="!cell.birthdays.length"
              :class="[
                'w-8 h-8 flex items-center justify-center rounded-full',
                cell.isToday ? 'bg-gray-900 dark:bg-white' : '',
              ]">
              <span :class="[
                'text-sm font-medium',
                cell.isToday
                  ? 'text-white dark:text-gray-900 font-semibold'
                  : 'text-gray-400 dark:text-gray-500',
              ]">{{ cell.day }}</span>
            </div>

            <Popover
              v-else
              posicion="arriba"
              alineacion="centro"
              ancho="240px"
              :mostrarCerrar="false"
              :cerrarAlClickFuera="true">
              <template #disparador>
                <div :class="[
                  'w-8 h-8 flex flex-col items-center justify-center rounded-full cursor-pointer transition-all duration-150 relative',
                  cell.isToday
                    ? 'bg-brand-500'
                    : 'hover:bg-brand-50 dark:hover:bg-brand-500/10',
                ]">
                  <span :class="[
                    'text-sm font-semibold leading-none',
                    cell.isToday ? 'text-white' : 'text-brand-600 dark:text-brand-400',
                  ]">{{ cell.day }}</span>
                  <span :class="[
                    'w-1 h-1 rounded-full mt-0.5',
                    cell.isToday ? 'bg-white/60' : 'bg-brand-400 dark:bg-brand-500',
                  ]" />
                </div>
              </template>

              <div class="flex flex-col gap-3">
                <p class="text-[10px] font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wider">
                  {{ format(new Date(viewDate.getFullYear(), viewDate.getMonth(), cell.day!), "EEEE d 'de' MMMM", { locale: es }) }}
                </p>
                <div class="flex flex-col gap-1">
                  <RouterLink
                    v-for="p in cell.birthdays"
                    :key="p.dni"
                    :to="{ name: 'personal-profile', params: { dni: p.dni } }"
                    class="flex items-center gap-2.5 px-2 py-1.5 rounded-xl hover:bg-gray-50 dark:hover:bg-white/5 transition-colors group -mx-1">
                    <div class="w-8 h-8 rounded-full bg-brand-50 dark:bg-brand-500/15 flex items-center justify-center shrink-0">
                      <span class="text-brand-600 dark:text-brand-400 text-xs font-bold">{{ initials(p.nombre) }}</span>
                    </div>
                    <div class="min-w-0 flex-1">
                      <p class="text-sm font-medium text-gray-800 dark:text-white/90 truncate group-hover:text-brand-600 dark:group-hover:text-brand-400 transition-colors">
                        {{ p.nombre }}
                      </p>
                      <p class="text-xs text-gray-400">{{ age(p.nacimiento) }} años</p>
                    </div>
                    <Cake class="w-3.5 h-3.5 text-brand-400 dark:text-brand-500 shrink-0" />
                  </RouterLink>
                </div>
              </div>
            </Popover>

          </template>
        </div>
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
    differenceInYears,
    format, parseISO, isValid, startOfMonth, getDay, getDaysInMonth,
  } from 'date-fns'
  import { es } from 'date-fns/locale'
  import { ChevronLeft, ChevronRight, Cake } from 'lucide-vue-next'
  import Popover from '../ui/Popover.vue'

  const DAY_NAMES = ['L', 'M', 'X', 'J', 'V', 'S', 'D']

  const { cumpleanos } = storeToRefs(useTableroStore())

  const today = new Date()
  today.setHours(0, 0, 0, 0)

  const viewDate = ref(new Date(today.getFullYear(), today.getMonth(), 1))

  const prevMonth = () => {
    viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() - 1, 1)
  }

  const nextMonth = () => {
    viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() + 1, 1)
  }

  const initials = (nombre: string) => {
    const parts = nombre.trim().split(/\s+/)
    if (parts.length === 1) return (parts[0]?.[0] ?? '').toUpperCase()
    return ((parts[0]?.[0] ?? '') + (parts[parts.length - 1]?.[0] ?? '')).toUpperCase()
  }

  const age = (nacimiento: string) => {
    const d = parseISO(nacimiento)
    if (!isValid(d)) return '?'
    return differenceInYears(today, d)
  }

  const birthdayMap = computed(() => {
    const map: Record<number, any[]> = {}
    if (!cumpleanos.value?.length) return map

    const month = viewDate.value.getMonth()

    cumpleanos.value.forEach((b: any) => {
      const date = parseISO(b.nacimiento)
      if (!isValid(date)) return
      if (date.getMonth() === month) {
        const day = date.getDate()
        if (!map[day]) map[day] = []
        map[day].push(b)
      }
    })

    return map
  })

  const calendarCells = computed(() => {
    const year = viewDate.value.getFullYear()
    const month = viewDate.value.getMonth()
    const daysInMonth = getDaysInMonth(viewDate.value)

    let firstDow = getDay(startOfMonth(viewDate.value))
    firstDow = firstDow === 0 ? 6 : firstDow - 1

    const cells: { day: number | null; birthdays: any[]; isToday: boolean }[] = []

    for (let i = 0; i < firstDow; i++) {
      cells.push({ day: null, birthdays: [], isToday: false })
    }

    for (let d = 1; d <= daysInMonth; d++) {
      cells.push({
        day: d,
        birthdays: birthdayMap.value[d] ?? [],
        isToday:
          today.getFullYear() === year &&
          today.getMonth() === month &&
          today.getDate() === d,
      })
    }

    return cells
  })

  const totalThisMonth = computed(() =>
    Object.values(birthdayMap.value).reduce((sum, arr) => sum + arr.length, 0),
  )
</script>