|
<template>
  <div class="tab-pane p-3 active">
    <div class="card">
      <div class="card-header">
        <div class="d-flex align-items-center justify-content-between w-100">
          <div class="d-flex align-items-center">
            <IconCalendar class="icon text-primary me-4" />
            <div>
              <h3 class="card-title mb-0">Calendario de Asistencia</h3>
              <p class="text-secondary mb-0 small">{{ currentMonthYear }}</p>
            </div>
          </div>

          <div class="d-flex align-items-center gap-2">
            <div class="btn-group">
              <button type="button" class="btn btn-outline-facebook btn-sm px-3" @click="previousMonth" :disabled="loading">
                <IconArrowLeft class="icon" />
              </button>
              <button type="button" class="btn btn-ghost-primary btn" @click="goToToday" :disabled="loading">Hoy</button>
              <button type="button" class="btn btn-outline-primary btn-sm px-3" @click="nextMonth" :disabled="loading">
                <IconArrowRight class="icon" />
              </button>
            </div>

            <button type="button" class="btn btn-ghost-success btn-sm" @click="loadEventos" :disabled="loading">
              <span v-if="loading" class="spinner-border spinner-border-sm"></span>
              <IconRefresh class="icon" v-else />
            </button>
          </div>
        </div>
      </div>

      <div class="card-body p-0">
        <div v-if="loading && eventos.length === 0" class="text-center py-5">
          <div class="spinner-border text-primary mb-3"></div>
          <p class="text-secondary">Cargando eventos...</p>
        </div>

        <div v-else-if="error" class="alert alert-danger m-3">
          <div class="d-flex align-items-center">
            <IconError404 class="alert-icon me-2" />
            <div>{{ error }}</div>
          </div>
        </div>

        <div v-else class="table-responsive">
          <table class="table table-vcenter card-table">
            <thead>
              <tr>
                <th v-for="dia in diasSemana" :key="dia" class="text-center text-secondary fw-medium">
                  <span class="d-none d-md-inline">{{ dia }}</span>
                  <span class="d-md-none">{{ dia.substring(0, 1) }}</span>
                </th>
              </tr>
            </thead>

            <tbody>
              <tr v-for="semana in semanasDelMes" :key="semana[0]?.fecha || 'empty'">
                <td
                  v-for="dia in semana"
                  :key="dia?.fecha || Math.random()"
                  class="calendar-day"
                  :class="{
                    'calendar-day-other-month': dia && !dia.esDelMesActual,
                    'calendar-day-today': dia && dia.esHoy,
                    'calendar-day-has-events': dia && dia.eventos.length > 0
                  }"
                >
                  <div v-if="dia" class="calendar-day-content">
                    <div class="calendar-day-number">
                      {{ dia.numero }}
                    </div>
                    <div v-if="dia.eventos.length > 0" class="calendar-events bg-secondary-subtle">
                      <div
                        v-for="(evento, index) in dia.eventos"
                        :key="index"
                        class="calendar-event"
                        @click="showEventDetails(evento, dia)"
                        :title="`${evento.hora} - DNI: ${evento.dni}`"
                      >
                        <div class="d-flex align-items-center">
                          <IconClock class="icon icon-sm" />
                          <span class="text-truncate">{{ evento.hora }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div class="modal fade" :class="{ show: showModal }" :style="{ display: showModal ? 'block' : 'none' }" tabindex="-1" @click.self="closeModal">
      <div class="modal-dialog modal-dialog-centered">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title d-flex align-items-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="icon me-2 text-primary"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                stroke-width="2"
                stroke="currentColor"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path stroke="none" d="m0 0h24v24H0z" fill="none" />
                <path d="M12 8h.01" />
                <path d="M11 12h1v4h1" />
                <path d="M12 3c7.2 0 9 1.8 9 9s-1.8 9 -9 9s-9 -1.8 -9 -9s1.8 -9 9 -9z" />
              </svg>
              Detalles del Evento
            </h5>
            <button type="button" class="btn-close" @click="closeModal"></button>
          </div>
          <div class="modal-body" v-if="selectedEvent">
            <div class="row g-3">
              <div class="col-12">
                <div class="card card-sm bg-primary-lt">
                  <div class="card-body">
                    <div class="row align-items-center">
                      <div class="col-auto">
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          class="icon text-primary"
                          width="32"
                          height="32"
                          viewBox="0 0 24 24"
                          stroke-width="2"
                          stroke="currentColor"
                          fill="none"
                          stroke-linecap="round"
                          stroke-linejoin="round"
                        >
                          <path stroke="none" d="m0 0h24v24H0z" fill="none" />
                          <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0" />
                          <path d="M6 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2" />
                        </svg>
                      </div>
                      <div class="col">
                        <div class="fw-medium">DNI del Usuario</div>
                        <div class="h4 mb-0">{{ selectedEvent.dni }}</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="col-md-6">
                <div class="card card-sm bg-success-lt">
                  <div class="card-body text-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="icon text-success mb-2"
                      width="24"
                      height="24"
                      viewBox="0 0 24 24"
                      stroke-width="2"
                      stroke="currentColor"
                      fill="none"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    >
                      <path stroke="none" d="m0 0h24v24H0z" fill="none" />
                      <path d="M4 7a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12z" />
                      <path d="M16 3v4" />
                      <path d="M8 3v4" />
                      <path d="M4 11h16" />
                    </svg>
                    <div class="fw-medium">Fecha</div>
                    <div class="h5 mb-0">{{ formatFecha(selectedEvent.fecha) }}</div>
                  </div>
                </div>
              </div>

              <div class="col-md-6">
                <div class="card card-sm bg-info-lt">
                  <div class="card-body text-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="icon text-info mb-2"
                      width="24"
                      height="24"
                      viewBox="0 0 24 24"
                      stroke-width="2"
                      stroke="currentColor"
                      fill="none"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    >
                      <path stroke="none" d="m0 0h24v24H0z" fill="none" />
                      <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" />
                      <path d="M12 7v5l3 3" />
                    </svg>
                    <div class="fw-medium">Hora</div>
                    <div class="h5 mb-0">{{ selectedEvent.hora }}</div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="selectedDayEvents && selectedDayEvents.length > 1" class="mt-4">
              <h6 class="text-secondary">Otros eventos del mismo día:</h6>
              <div class="list-group list-group-flush">
                <div
                  v-for="evento in selectedDayEvents.filter((e) => e !== selectedEvent)"
                  :key="`${evento.dni}-${evento.hora}`"
                  class="list-group-item list-group-item-action px-0"
                >
                  <div class="d-flex align-items-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="icon me-2 text-secondary"
                      width="16"
                      height="16"
                      viewBox="0 0 24 24"
                      stroke-width="2"
                      stroke="currentColor"
                      fill="none"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    >
                      <path stroke="none" d="m0 0h24v24H0z" fill="none" />
                      <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" />
                      <path d="M12 7v5l3 3" />
                    </svg>
                    <div class="flex-fill">
                      <div class="fw-medium">{{ evento.hora }}</div>
                      <div class="text-secondary small">DNI: {{ evento.dni }}</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="closeModal">Cerrar</button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="showModal" class="modal-backdrop fade show"></div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@api/axios'
import { IconArrowLeft, IconArrowRight, IconCalendar, IconClock, IconError404, IconRefresh } from '@tabler/icons-vue'
import { getMonth, getYear } from 'date-fns'
import { ref, computed, onMounted, watch } from 'vue'

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
const loading = ref(false)
const error = ref('')
const showModal = ref(false)
const selectedEvent = ref<EventoAPI | null>(null)
const selectedDayEvents = ref<EventoAPI[]>([])

const diasSemana = ['Domingo', 'Lunes', 'Martes', 'Miércoles', 'Jueves', 'Viernes', 'Sábado']

const currentMonthYear = computed(() => {
  return currentDate.value
    .toLocaleDateString('es-ES', {
      month: 'long',
      year: 'numeric'
    })
    .replace(/^\w/, (c) => c.toUpperCase())
})

const semanasDelMes = computed(() => {
  const year = currentDate.value.getFullYear()
  const month = currentDate.value.getMonth()

  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)

  const startDay = firstDay.getDay()

  const diasAnteriores = []
  for (let i = startDay - 1; i >= 0; i--) {
    const fecha = new Date(year, month, -i)
    diasAnteriores.push(crearDiaCalendario(fecha, false))
  }

  const diasActuales = []
  for (let dia = 1; dia <= lastDay.getDate(); dia++) {
    const fecha = new Date(year, month, dia)
    diasActuales.push(crearDiaCalendario(fecha, true))
  }

  const diasSiguientes = []
  const totalDias = diasAnteriores.length + diasActuales.length
  const diasFaltantes = 42 - totalDias // 6 semanas * 7 días
  for (let dia = 1; dia <= diasFaltantes; dia++) {
    const fecha = new Date(year, month + 1, dia)
    diasSiguientes.push(crearDiaCalendario(fecha, false))
  }

  const todosDias = [...diasAnteriores, ...diasActuales, ...diasSiguientes]

  const semanas = []
  for (let i = 0; i < todosDias.length; i += 7) {
    semanas.push(todosDias.slice(i, i + 7))
  }

  return semanas
})

const crearDiaCalendario = (fecha: Date, esDelMesActual: boolean): DiaCalendario => {
  const fechaStr = fecha.toISOString().split('T')[0]
  const hoy = new Date().toISOString().split('T')[0]

  return {
    numero: fecha.getDate(),
    fecha: fechaStr,
    esDelMesActual,
    esHoy: fechaStr === hoy,
    eventos: eventos.value.filter((evento) => evento.fecha === fechaStr)
  }
}

const formatFecha = (fecha: string) => {
  return new Date(fecha + 'T00:00:00').toLocaleDateString('es-ES', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
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

const showEventDetails = (evento: EventoAPI, dia: DiaCalendario) => {
  selectedEvent.value = evento
  selectedDayEvents.value = dia.eventos
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  selectedEvent.value = null
  selectedDayEvents.value = []
}

const loadEventos = async () => {
  try {
    loading.value = true
    error.value = ''

    const data = await (await api.post('/personal/asistencia', { año: getYear(currentDate.value), mes: getMonth(currentDate.value) + 1, dni: props.dni })).data
    eventos.value = data as EventoAPI[]
  } catch (err: any) {
    error.value = err.message || 'Error al cargar los eventos'
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
.calendar-day {
  height: 10vh;
  width: 14.28%;
  vertical-align: top;
  border: 1px solid var(--tblr-border-color);
  position: relative;
  padding: 4px;
}

.calendar-day-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.calendar-day-number {
  font-weight: 600;
  font-size: 0.875rem;
  margin-bottom: 4px;
  color: var(--tblr-body-color);
}

.calendar-day-other-month {
  background-color: var(--tblr-bg-surface-secondary);
}

.calendar-day-other-month .calendar-day-number {
  color: var(--tblr-text-muted);
}

.calendar-day-today {
  background-color: var(--tblr-primary-lt);
}

.calendar-day-today .calendar-day-number {
  color: var(--tblr-primary);
  font-weight: 700;
}

.calendar-day-has-events {
  background-color: var(--tblr-blue-lt);
}

.calendar-events {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow: hidden;
}

.calendar-event {
  background: white;
  border: 1px solid var(--tblr-border-color);
  border-radius: 4px;
  padding: 2px 4px;
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.2s;
}

.calendar-event:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.calendar-event-morning {
  border-left: 3px solid var(--tblr-success);
  background-color: var(--tblr-success-lt);
}

.calendar-event-afternoon {
  border-left: 3px solid var(--tblr-warning);
  background-color: var(--tblr-warning-lt);
}

.calendar-event-evening {
  border-left: 3px solid var(--tblr-info);
  background-color: var(--tblr-info-lt);
}

.calendar-event-more {
  text-align: center;
  padding: 2px;
  cursor: pointer;
}

@media (max-width: 768px) {
  .calendar-day {
    height: 80px;
    padding: 2px;
  }

  .calendar-day-number {
    font-size: 0.75rem;
  }

  .calendar-event {
    font-size: 0.625rem;
    padding: 1px 2px;
  }
}

@media (max-width: 576px) {
  .calendar-day {
    height: 60px;
  }

  .calendar-events {
    gap: 1px;
  }
}
</style>
