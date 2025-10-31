<template>
  <div class="card">
    <div class="card-body">
      <div class="d-flex justify-content-between align-items-center">
        <div>
          <h4 class="card-title mb-1">Legajo</h4>
          <p class="text-muted small mb-0">Historial de movimientos</p>
        </div>
        <button
          v-if="lista.length === 0 || verificar()?.estado != 'prestamo' || verificar()?.user == store.id"
          class="btn btn-ghost-orange btn-icon"
          data-bs-toggle="modal"
          data-bs-target="#add_legajo-1"
          aria-label="Gestionar legajo"
          title="Gestionar legajo"
        >
          <IconFolderShare class="icon" />
        </button>
      </div>

      <hr class="my-2" />

      <div v-if="lista.length > 0">
        <div class="d-flex align-items-center gap-3">
          <span class="avatar avatar-md">
            {{ getInitials(verificar().persona) }}
          </span>

          <div class="flex-fill" style="min-width: 0">
            <div class="d-flex justify-content-between align-items-center flex-wrap">
              <span class="fw-bold me-2">{{ verificar().persona }}</span>
              <span class="badge" :class="verificar().estado === 'archivado' ? 'bg-success-lt' : 'bg-secondary-lt'">
                {{ verificar().estado === 'archivado' ? 'Devuelto' : 'En préstamo' }}
              </span>
            </div>

            <div class="text-muted small mt-1">
              <span v-if="verificar().descrip">{{ verificar().descrip }}</span>
            </div>

            <div class="d-flex align-items-center text-muted small" :class="{ 'mt-1': !verificar().descrip }">
              <IconCalendar class="icon icon-sm me-1" />
              <span>{{ formatFechaCompleta(verificar().fecha) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-center text-muted py-3">
        <IconFileOff class="icon icon-lg mb-2" />
        <p class="mb-0">Sin movimientos registrados.</p>
      </div>
    </div>

    <modallegajo :prestado="verificar()?.estado == 'prestamo'" :usuario="verificar()?.persona" :create="verificar()?.fecha" />
  </div>
</template>

<script setup lang="ts">
import { IconCalendar, IconFolderShare, IconFileOff } from '@tabler/icons-vue'
import modallegajo from '@comp/perfil/modal/agregar_legajo.vue'
import { userStore } from '@store/user'
import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import { formatFechaCompleta } from '@api/date'
import { router } from '@router/router'

const store = userStore()
const lista = ref<any>([])

const verificar = () => {
  return lista.value[0]
}

const getInitials = (nombre: string): string => {
  if (!nombre) return ''
  return nombre
    .split(' ')
    .map((word) => word.charAt(0))
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

onMounted(async () => {
  try {
    const response = await api.post('/personal/legajo_por_dni', {
      dni: router.currentRoute.value.params.dni
    })
    lista.value = response.data
  } catch (error) {
    console.error('Error al cargar el historial del legajo:', error)
    lista.value = []
  }
})
</script>
