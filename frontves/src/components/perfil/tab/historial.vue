<template>
  <div class="card">
    <div class="card" style="max-height: 70vh">
      <div class="card-header">
        <h3 class="card-title d-flex align-items-center">
          <IconReport class="icon me-2" />
          {{ title }}
        </h3>
        <div class="card-actions">
          <span class="badge bg-primary text-white fw-bold">{{ lista.length }}</span>
        </div>
      </div>

      <div class="card-body card-body-scrollable card-body-scrollable-shadow p-0">
        <div v-if="lista.length === 0" class="empty">
          <div class="empty-img">
            <IconPaperBagOff class="icon me-3" />
          </div>
          <p class="empty-title">Sin resultados</p>
          <p class="empty-subtitle text-secondary">No se encontraron registros para el criterio de búsqueda</p>
        </div>

        <div v-else class="list-group list-group-flush">
          <div v-for="(item, index) in lista" :key="index" class="list-group-item list-group-item-action">
            <div class="row align-items-start">
              <div class="col-auto">
                <span class="avatar avatar-sm avatar-rounded bg-primary-lt">
                  {{ getInitials(item.nombre) }}
                </span>
              </div>

              <div class="col">
                <div class="d-flex align-items-center justify-content-between mb-2">
                  <div class="d-flex align-items-center flex-wrap">
                    <div class="d-flex align-items-center me-3">
                      <IconUser class="icon me-1 icon-sm text-secondary" />
                      <strong class="text-body fs-4">{{ item.nombre }}</strong>
                    </div>
                    <span class="badge fs-6">
                      {{ item.operacion }}
                    </span>
                  </div>

                  <div class="text-secondary d-flex align-items-center">
                    <IconClock class="icon icon-sm" />
                    <small>{{ formatFecha(item.fecha) }}</small>
                  </div>
                </div>

                <div v-if="item.detalle && item.detalle.trim()" class="mt-2">
                  <div class="card card-sm">
                    <div class="card-body p-0 m-0">
                      <Legajo v-if="item.operacion === 'cambiar legajo'" :item="item" />
                      <Banco v-else-if="item.operacion === 'actualizar cuenta bancaria'" :item="JSON.parse(item.detalle)" />
                      <Information v-else-if="item.operacion === 'editar informacion personal'" :perfil="JSON.parse(item.detalle)" />
                      <div class="flex-fill" v-else>
                        <pre class="mb-0 text-white small">{{ item.detalle }}</pre>
                      </div>
                    </div>
                  </div>
                </div>

                <div class="mt-2 d-flex align-items-center text-muted">
                  <IconCalendar class="icon icon-sm" />
                  <small>{{ formatFechaCompleta(item.fecha) }}</small>
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
import { IconCalendar, IconClock, IconPaperBagOff, IconReport, IconUser } from '@tabler/icons-vue'
import { formatFechaCompleta } from '@api/date'
import Legajo from './historial/legajo.vue'
import Banco from './historial/banco.vue'
import Information from './historial/info.vue'

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
  title: 'Reporte de Historial'
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

    if (diffInMinutes < 1) {
      return 'Ahora'
    } else if (diffInMinutes < 60) {
      return `${diffInMinutes}m`
    } else if (diffInMinutes < 1440) {
      const hours = Math.floor(diffInMinutes / 60)
      return `${hours}h`
    } else if (diffInMinutes < 2880) {
      return 'Ayer'
    } else {
      const days = Math.floor(diffInMinutes / 1440)
      return `${days}d`
    }
  } catch {
    return fecha
  }
}
</script>
