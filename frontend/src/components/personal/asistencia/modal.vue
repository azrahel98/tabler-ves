<template>
  <div>
    <slot name="trigger" :open="openModal">
      <button
        @click="openModal"
        class="group flex items-center gap-2 bg-background border border-border hover:border-primary/50 text-foreground px-3 py-1.5 rounded-xl text-xs font-semibold transition-all shadow-sm hover:shadow-md active:scale-95"
      >
        <Calendar :size="14" class="text-primary group-hover:scale-110 transition-transform" />
        <span>Historial</span>
      </button>
    </slot>

    <Teleport to="body">
      <modal :is-open="isModalOpen" @update:is-open="isModalOpen = $event" width="750px">
        <template #title>
          <div class="flex items-center justify-between w-full pr-8">
            <div class="flex flex-col pr-9">
              <span class="text-[9px] text-muted-foreground font-bold uppercase tracking-wider mb-0.5">Asistencia</span>
              <div class="flex items-center gap-2">
                <h2 class="text-foreground text-sm font-bold capitalize">{{ currentMonthName }}</h2>
                <span class="text-primary text-sm font-medium">{{ currentYear }}</span>
              </div>
            </div>

            <div class="flex items-center gap-1 bg-muted/30 p-0.5 rounded-lg border border-border/50">
              <button @click="changeMonth(-1)" class="p-1 hover:bg-background hover:text-primary rounded-md transition-all text-muted-foreground">
                <ChevronLeft :size="14" />
              </button>
              <div class="w-[1px] h-3 bg-border mx-0.5"></div>
              <button @click="changeMonth(1)" class="p-1 hover:bg-background hover:text-primary rounded-md transition-all text-muted-foreground">
                <ChevronRight :size="14" />
              </button>
            </div>
          </div>
        </template>

        <template #body>
          <div class="px-3 pb-3 -mt-2">
            <div v-if="isLoading" class="grid grid-cols-7 gap-1 animate-pulse">
              <div v-for="i in 35" :key="'sk-' + i" class="h-16 bg-muted/20 rounded-lg border border-border/40"></div>
            </div>

            <div v-else class="select-none">
              <div class="grid grid-cols-7 gap-1 mb-2">
                <span v-for="day in daysOfWeek" :key="day" class="text-center text-[10px] font-bold text-muted-foreground/70 uppercase">
                  {{ day }}
                </span>
              </div>

              <div class="grid grid-cols-7 gap-1.5">
                <div v-for="empty in firstDayOfMonth" :key="'empty-' + empty" class="h-16 opacity-10">
                  <div class="w-full h-full border border-dashed border-border rounded-xl"></div>
                </div>

                <div
                  v-for="day in daysInMonth"
                  :key="day"
                  class="group relative h-16 border rounded-sm p-1.5 transition-all duration-200 flex flex-col hover:shadow-sm"
                  :class="[
                    getAttendanceForDay(day).length > 0 ? 'bg-blue-50/30 border-blue-200/50 hover:border-blue-300' : 'bg-card border-border hover:border-primary/50',
                    isToday(day) ? 'ring-2 ring-primary ring-offset-2 scale-[1.02] shadow-md z-10' : ''
                  ]"
                >
                  <div class="flex justify-between items-start mb-0.5">
                    <span class="text-[0.7rem] font-black leading-none" :class="isToday(day) ? 'text-primary' : 'text-foreground'">
                      {{ String(day).padStart(2, '0') }}
                    </span>
                    <div v-if="isToday(day)" class="w-1 h-1 bg-primary rounded-full shadow-sm"></div>
                  </div>

                  <div class="flex-1 overflow-hidden flex flex-wrap gap-0.5 content-start">
                    <div
                      v-for="(log, idx) in getAttendanceForDay(day)"
                      :key="idx"
                      class="px-1 py-px rounded-[3px] text-[0.6rem] font-bold transition-colors leading-tight"
                      :class="[
                        log.tipo === 'Entrada'
                          ? 'bg-emerald-50 text-emerald-700 border border-emerald-100/50 group-hover:bg-emerald-100 group-hover:text-emerald-800'
                          : 'bg-rose-50 text-rose-700 border border-rose-100/50 group-hover:bg-rose-100 group-hover:text-rose-800'
                      ]"
                    >
                      {{ log.hora.substring(0, 5) }}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </template>
      </modal>
    </Teleport>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import { Calendar, ChevronLeft, ChevronRight } from 'lucide-vue-next'
import modal from '@comp/ui/modal.vue'
import { useProfileStore } from '@store/perfil'
import { useRoute } from 'vue-router'

interface AsistenciaRaw {
  dni: string
  hora: string
  fecha: string
  tipo: string
}

const store = useProfileStore()
const route = useRoute()

const isModalOpen = ref<boolean>(false)
const isLoading = ref<boolean>(false)
const currentDate = ref<Date>(new Date())

const daysOfWeek: string[] = ['Lun', 'Mar', 'Mie', 'Jue', 'Vie', 'Sab', 'Dom']

const currentMonth = computed<number>(() => currentDate.value.getMonth())
const currentYear = computed<number>(() => currentDate.value.getFullYear())

const currentMonthName = computed<string>(() => {
  return new Intl.DateTimeFormat('es-ES', { month: 'long' }).format(currentDate.value)
})

const daysInMonth = computed<number>(() => {
  return new Date(currentYear.value, currentMonth.value + 1, 0).getDate()
})

const firstDayOfMonth = computed<number>(() => {
  let day = new Date(currentYear.value, currentMonth.value, 1).getDay()
  return day === 0 ? 6 : day - 1
})

const workedDaysCount = computed<number>(() => {
  if (!store.asistencia) return 0
  const uniqueDays = new Set((store.asistencia as AsistenciaRaw[]).map((a) => a.fecha))
  return uniqueDays.size
})

const isToday = (day: number): boolean => {
  const today = new Date()
  return today.getDate() === day && today.getMonth() === currentMonth.value && today.getFullYear() === currentYear.value
}

const openModal = (): void => {
  isModalOpen.value = true
  fetchData()
}

const changeMonth = (step: number): void => {
  currentDate.value = new Date(currentYear.value, currentMonth.value + step, 1)
  fetchData()
}

const fetchData = async (): Promise<void> => {
  isLoading.value = true
  const dni = route.params.dni?.toString() || ''
  try {
    await store.update_asistencia(dni, currentMonth.value + 1, currentYear.value)
  } finally {
    setTimeout(() => {
      isLoading.value = false
    }, 400)
  }
}

const getAttendanceForDay = (day: number): AsistenciaRaw[] => {
  if (!store.asistencia) return []
  const dateStr = `${currentYear.value}-${String(currentMonth.value + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}`

  return (store.asistencia as AsistenciaRaw[]).filter((a) => a.fecha === dateStr).sort((a, b) => a.hora.localeCompare(b.hora))
}
</script>

<style scoped>
.select-none {
  user-select: none;
}

/* Efecto suave para las celdas */
.content-start {
  align-content: flex-start;
}

/* Personalización de la tipografía para que se vea más "App" y menos "Web" */
h2 {
  letter-spacing: -0.02em;
}
</style>
