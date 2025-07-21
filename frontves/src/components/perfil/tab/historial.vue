<template>
  <div class="tab-pane p-3 active">
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
                      <IconUser class="icon me-1 text-secondary" />
                      <strong class="text-body">{{ item.nombre }}</strong>
                    </div>
                    <span class="badge small" :class="getOperationBadge(item.operacion)">
                      {{ formatOperacion(item.operacion) }}
                    </span>
                  </div>

                  <div class="text-secondary d-flex align-items-center">
                    <IconClock class="icon icon-sm" />
                    <small>{{ formatFecha(item.fecha) }}</small>
                  </div>
                </div>

                <div v-if="item.detalle && item.detalle.trim()" class="mt-2">
                  <div class="card card-sm">
                    <div class="card-body">
                      <div class="flex-fill">
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

const getOperationBadge = (operacion: string): string => {
  const op = operacion.toLowerCase()

  if (op.includes('crear') || op.includes('insert') || op.includes('add') || op.includes('nuevo')) {
    return 'bg-success text-white'
  }
  if (op.includes('editar') || op.includes('update') || op.includes('modificar') || op.includes('cambiar')) {
    return 'bg-primary text-white'
  }
  if (op.includes('eliminar') || op.includes('delete') || op.includes('borrar') || op.includes('quitar')) {
    return 'bg-danger text-white'
  }
  if (op.includes('login') || op.includes('acceso') || op.includes('ingreso') || op.includes('entrar')) {
    return 'bg-success text-white'
  }
  if (op.includes('logout') || op.includes('salir') || op.includes('cerrar')) {
    return 'bg-warning text-white'
  }
  if (op.includes('ver') || op.includes('consultar') || op.includes('select') || op.includes('buscar')) {
    return 'bg-info text-white'
  }
  if (op.includes('exportar') || op.includes('descargar') || op.includes('imprimir')) {
    return 'bg-cyan text-white'
  }

  return 'bg-secondary text-white'
}

const formatOperacion = (operacion: string): string => {
  return operacion
    .toLowerCase()
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ')
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

const formatFechaCompleta = (fecha: string): string => {
  try {
    const date = new Date(fecha)
    return date.toLocaleString('es-ES', {
      weekday: 'short',
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return fecha
  }
}
</script>
