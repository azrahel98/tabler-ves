<template>
  <div class="card">
    <div class="card-header d-flex justify-content-between align-items-center">
      <div>
        <h3 class="card-title mb-1">Historial</h3>
        <p class="text-muted mb-0 small">Cambios realizados al perfil</p>
      </div>
      <span class="badge bg-blue-lt rounded-pill">{{ lista.length }}</span>
    </div>

    <div class="card-body card-body-scrollable card-body-scrollable-shadow">
      <div v-if="!lista.length" class="empty">
        <div class="empty-img">
          <IconPaperBagOff class="icon" />
        </div>
        <p class="empty-title">Sin resultados</p>
        <p class="empty-subtitle text-secondary">No se encontraron registros para mostrar.</p>
      </div>

      <ul v-else class="timeline">
        <li v-for="(item, index) in lista" :key="index" class="timeline-item">
          <div class="timeline-item-point">
            <span class="avatar avatar-sm avatar-rounded bg-primary-lt">
              {{ getInitials(item.nombre) }}
            </span>
          </div>
          <div class="timeline-item-content">
            <div class="d-flex flex-wrap align-items-center justify-content-between">
              <div class="fw-bold me-3">{{ item.nombre }}</div>
              <div class="d-flex align-items-center text-muted small flex-shrink-0">
                <span class="badge bg-secondary-lt me-2">{{ item.operacion }}</span>
                <IconClock class="icon icon-sm me-1" />
                <span>{{ formatFecha(item.fecha) }}</span>
              </div>
            </div>

            <div class="mt-2" v-if="item.detalle && item.detalle.trim()">
              <div v-if="item.operacion === 'cambiar legajo' || item.operacion === 'devolver legajo'">
                <div class="card card-sm bg-light border-0">
                  <div class="card-body">
                    <div class="d-flex align-items-center justify-content-between">
                      <div class="lh-sm">
                        <div class="text-secondary small">
                          {{ item.operacion === 'cambiar legajo' ? 'En poder de:' : 'Devuelto por:' }}
                        </div>
                        <div class="fw-bold">{{ JSON.parse(item.detalle).persona }}</div>
                      </div>
                      <span class="avatar avatar-xs avatar-rounded">{{ getInitials(JSON.parse(item.detalle).persona) }}</span>
                    </div>
                    <div class="text-secondary small mt-1">
                      {{ JSON.parse(item.detalle).descrip }}
                    </div>
                    <div class="text-secondary small mt-1">
                      {{ formatFechaCompleta(item.fecha) }}
                    </div>
                  </div>
                </div>
              </div>

              <div v-else-if="item.operacion === 'editar informacion personal'" class="card card-sm bg-light border-0">
                <div class="card-body">
                  <div class="d-flex align-items-center" v-if="JSON.parse(item.detalle).email">
                    <IconMail class="icon icon-sm text-secondary me-2" />
                    <span class="text-secondary small">Correo:&nbsp;</span>
                    <span class="small">{{ JSON.parse(item.detalle).email }}</span>
                  </div>

                  <div class="d-flex align-items-center mt-1" v-if="JSON.parse(item.detalle).telf && JSON.parse(item.detalle).telf != null">
                    <IconPhone class="icon icon-sm text-secondary me-2" />
                    <span class="text-secondary small">Teléfono:&nbsp;</span>
                    <span class="small">{{ JSON.parse(item.detalle).telf }}</span>
                  </div>
                  <div class="d-flex align-items-center mt-1" v-if="JSON.parse(item.detalle).direccion && JSON.parse(item.detalle).direccion != null">
                    <IconPhone class="icon icon-sm text-secondary me-2" />
                    <span class="text-secondary small">Direccion:&nbsp;</span>
                    <span class="small">{{ JSON.parse(item.detalle).direccion }}</span>
                  </div>
                </div>
              </div>

              <Banco :item="JSON.parse(item.detalle)" v-else-if="item.operacion === 'actualizar cuenta bancaria'" />
              <Renuncia :item="JSON.parse(item.detalle)" v-else-if="item.operacion === 'registro de renuncia'" />

              <div class="flex-fill" v-else>
                <pre class="mb-0 text-white small">{{ item.detalle }}</pre>
              </div>
            </div>

            <div class="mt-2 text-muted d-flex align-items-center small">
              <IconCalendar class="icon icon-sm me-1" />
              <span>{{ formatFechaCompleta(item.fecha) }}</span>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconCalendar, IconClock, IconPaperBagOff, IconMail, IconPhone } from '@tabler/icons-vue'
import Banco from './historial/banco.vue'
import Renuncia from './historial/renuncia.vue'

const formatFechaCompleta = (date: string) => new Date(date).toLocaleString()

interface HistorialItem {
  operacion: string
  detalle: string
  fecha: string
  nombre: string
}

interface Props {
  lista: HistorialItem[]
  title?: string
}

withDefaults(defineProps<Props>(), {
  title: 'Reporte de Historial',
  lista: () => []
})

const getInitials = (nombre: string): string => {
  return nombre
    .split(' ')
    .map((word) => word.charAt(0))
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

const formatFecha = (fecha: string): string => {
  try {
    const date = new Date(fecha)
    const now = new Date()
    const diffInMinutes = Math.floor((now.getTime() - date.getTime()) / (1000 * 60))

    if (diffInMinutes < 1) return 'Ahora'
    if (diffInMinutes < 60) return `${diffInMinutes}m`

    const hours = Math.floor(diffInMinutes / 60)
    if (hours < 24) return `${hours}h`

    const days = Math.floor(hours / 24)
    return `${days}d`
  } catch {
    return fecha
  }
}
</script>

<style lang="scss" scoped>
.timeline {
  list-style: none;
  padding: 0;
  margin: 0;

  .timeline-item {
    position: relative;
    display: flex;
    gap: 1rem;
    padding-bottom: 1.5rem;

    &:last-child {
      padding-bottom: 0;
    }

    &::before {
      content: '';
      position: absolute;
      left: 1rem;
      top: 0.25rem;
      bottom: 0;
      width: 2px;
      background-color: var(--tblr-border-color);
      transform: translateX(-50%);
    }

    .timeline-item-point {
      position: relative;
      z-index: 1;
      .avatar {
        border: 2px solid var(--tblr-card-bg);
      }
    }

    .timeline-item-content {
      flex: 1;
      padding-top: 0.25rem;
      min-width: 0;
    }
  }
}

.card-body-scrollable {
  max-height: 65vh;
}
</style>
