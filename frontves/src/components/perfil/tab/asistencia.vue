<template>
  <div class="tab-pane p-3 active">
    <div class="card">
      <div class="card-header">
        <div class="d-flex align-items-center justify-content-between w-100 flex-wrap gap-2">
          <div class="d-flex align-items-center">
            <IconCalendar class="icon icon-lg text-primary me-3" />
            <div>
              <h3 class="card-title mb-0">Calendario de Asistencia</h3>
              <p class="text-secondary mb-0 small">{{ currentMonthYear }}</p>
            </div>
          </div>

          <div class="d-flex align-items-center gap-2">
            <div class="btn-group">
              <button type="button" class="btn btn-outline-secondary btn-sm px-3" @click="previousMonth" :disabled="loading" title="Mes anterior">
                <IconArrowLeft class="icon" />
              </button>
              <button type="button" class="btn btn-ghost-primary btn-sm" @click="goToToday" :disabled="loading">Hoy</button>
              <button type="button" class="btn btn-outline-secondary btn-sm px-3" @click="nextMonth" :disabled="loading" title="Mes siguiente">
                <IconArrowRight class="icon" />
              </button>
            </div>

            <button type="button" class="btn btn-ghost-secondary btn-sm" @click="loadEventos" :disabled="loading" title="Refrescar">
              <span v-if="loading" class="spinner-border spinner-border-sm" role="status"></span>
              <IconRefresh class="icon" v-else />
            </button>
          </div>
        </div>
      </div>

      <div class="card-body">
        <div class="calendar-container">
          <div v-if="loading" class="skeleton-grid">
            <template v-for="i in 49" :key="i">
              <div v-if="i <= 7" class="skeleton-cell skeleton-header"></div>
              <div v-else class="skeleton-cell skeleton-day"></div>
            </template>
          </div>

          <div v-else-if="error" class="calendar-error-container">
            <div class="alert alert-danger w-100">
              <div class="d-flex align-items-center">
                <IconAlertTriangle class="icon alert-icon me-2" />
                <div>{{ error }}</div>
              </div>
            </div>
          </div>

          <div v-else class="calendar-grid">
            <div v-for="dia in diasSemana" :key="dia" class="calendar-header">
              <span class="d-none d-md-inline">{{ dia }}</span>
              <span class="d-md-none">{{ dia.substring(0, 3) }}</span>
            </div>

            <div
              v-for="dia in diasDelMes"
              :key="dia.fecha"
              class="calendar-day"
              :class="{
                'calendar-day-other-month': !dia.esDelMesActual
              }"
            >
              <div
                class="calendar-day-number"
                :class="{
                  'calendar-day-today': dia.esHoy
                }"
              >
                {{ dia.numero }}
              </div>

              <div v-if="dia.eventos.length > 0" class="calendar-events">
                <div v-for="(evento, index) in dia.eventos" :key="index" class="calendar-event" :title="`${evento.hora} - DNI: ${evento.dni}`">
                  <IconClock class="icon icon-sm me-1 flex-shrink-0" />
                  <span class="text-truncate">{{ evento.hora }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@api/axios'
import { IconArrowLeft, IconArrowRight, IconCalendar, IconClock, IconRefresh, IconAlertTriangle } from '@tabler/icons-vue'
import { getMonth, getYear } from 'date-fns'
import { ref, computed, watch, onMounted } from 'vue'

interface EventoAPI {
  dni: string
  hora: string
  fecha: string
}

interface DiaCalendario {
  numero: number
  fecha: string
  esDelMesActual: boolean
  esHoy: boolean
  eventos: EventoAPI[]
}

const props = defineProps({
  dni: { type: String, required: true }
})

const currentDate = ref(new Date())
const eventos = ref<EventoAPI[]>([])
const loading = ref(true)
const error = ref('')

const diasSemana = ['Domingo', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado']

const currentMonthYear = computed(() => {
  return currentDate.value
    .toLocaleString('es-ES', {
      month: 'long',
      year: 'numeric'
    })
    .replace(/^\w/, (c) => c.toUpperCase())
})

const diasDelMes = computed((): DiaCalendario[] => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth()

  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)
  const startDayOfWeek = firstDay.getDay()

  const dias: DiaCalendario[] = []

  for (let i = startDayOfWeek; i > 0; i--) {
    const fecha = new Date(year, month, 1 - i)
    dias.push(crearDiaCalendario(fecha, false))
  }

  for (let dia = 1; dia <= lastDay.getDate(); dia++) {
    const fecha = new Date(year, month, dia)
    dias.push(crearDiaCalendario(fecha, true))
  }

  const totalDiasMostrados = dias.length
  const diasFaltantes = 42 - totalDiasMostrados
  for (let dia = 1; dia <= diasFaltantes; dia++) {
    const fecha = new Date(year, month + 1, dia)
    dias.push(crearDiaCalendario(fecha, false))
  }

  return dias
})

const crearDiaCalendario = (fecha: Date, esDelMesActual: boolean): DiaCalendario => {
  const fechaStr = fecha.toISOString().split('T')[0]
  const hoyStr = new Date().toISOString().split('T')[0]

  return {
    numero: fecha.getDate(),
    fecha: fechaStr,
    esDelMesActual,
    esHoy: fechaStr === hoyStr && esDelMesActual,
    eventos: eventos.value.filter((evento) => evento.fecha === fechaStr)
  }
}

const previousMonth = () => {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() - 1, 1)
}

const nextMonth = () => {
  currentDate.value = new Date(currentDate.value.getFullYear(), currentDate.value.getMonth() + 1, 1)
}

const goToToday = () => {
  currentDate.value = new Date()
}

const loadEventos = async () => {
  try {
    loading.value = true
    error.value = ''
    eventos.value = []

    const params = {
      año: getYear(currentDate.value),
      mes: getMonth(currentDate.value) + 1,
      dni: props.dni
    }
    const response = await api.post('/personal/asistencia', params)
    eventos.value = response.data as EventoAPI[]
  } catch (err: any) {
    error.value = err.response?.data?.message || err.message || 'Error al cargar los eventos'
    console.error('Error loading eventos:', err)
  } finally {
    loading.value = false
  }
}

watch(currentDate, () => {
  loadEventos()
})

onMounted(() => {
  loadEventos()
})
</script>

<style scoped>
@keyframes skeleton-pulse {
  0% {
    background-color: var(--tblr-bg-surface-secondary);
  }
  50% {
    background-color: var(--tblr-bg-surface-tertiary);
  }
  100% {
    background-color: var(--tblr-bg-surface-secondary);
  }
}

.calendar-container {
  min-height: 708px;
  position: relative;
}

.calendar-error-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 708px;
  padding: 1rem;
}

.skeleton-grid,
.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 1px;
  background-color: var(--tblr-border-color);
  border: 1px solid var(--tblr-border-color);
  border-radius: var(--tblr-border-radius);
  overflow: hidden;
}

.calendar-header {
  text-align: center;
  padding: 0.5rem 0.25rem;
  font-weight: 600;
  font-size: 0.75rem;
  text-transform: uppercase;
  color: var(--tblr-text-muted);
  background-color: var(--tblr-bg-surface-secondary);
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.skeleton-cell {
  background-color: var(--tblr-bg-surface);
  animation: skeleton-pulse 1.5s ease-in-out infinite;
}

.skeleton-header {
  height: 42px;
}

.skeleton-day {
  height: 110px;
}

.calendar-day {
  padding: 0.5rem;
  height: 110px;
  background-color: var(--tblr-bg-surface);
  position: relative;
  display: flex;
  flex-direction: column;
}

.calendar-day-number {
  font-weight: 600;
  font-size: 0.875rem;
  color: var(--tblr-body-color);
  text-align: left;
  margin-bottom: 0.25rem;
  height: 1.75rem;
}

.calendar-day-other-month {
  background-color: var(--tblr-bg-surface-secondary);
}

.calendar-day-other-month .calendar-day-number {
  color: var(--tblr-text-muted);
  font-weight: 400;
}

.calendar-day-today {
  background-color: var(--tblr-primary);
  color: var(--tblr-white);
  border-radius: 50%;
  width: 1.75rem;
  height: 1.75rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  text-align: center;
}

.calendar-events {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 3px;
  overflow-y: auto;
  max-height: 70px;
}

.calendar-event {
  display: flex;
  align-items: center;
  background: var(--tblr-primary-lt);
  border-left: 3px solid var(--tblr-primary);
  border-radius: var(--tblr-border-radius-sm);
  padding: 2px 4px;
  font-size: 0.75rem;
  color: var(--tblr-primary-dark);
}

@media (max-width: 767px) {
  .calendar-container,
  .calendar-error-container {
    min-height: 0;
  }

  .calendar-day,
  .skeleton-day {
    height: 90px;
  }
  .calendar-day-number {
    font-size: 0.75rem;
  }
  .calendar-day-today {
    width: 1.5rem;
    height: 1.5rem;
  }
  .calendar-event {
    font-size: 0.625rem;
  }
  .calendar-events {
    max-height: 50px;
  }
}
</style>
