<template>
  <div class="card rounded-top-4">
    <div class="card-body p-0">
      <div class="text-black rounded-top-4 p-4 py-0 pt-3 position-relative overflow-hidden">
        <div class="text-center position-relative">
          <div class="avatar avatar-xl bg-white mb-2 text-black fw-bold fs-2">
            {{ nombreabrv(user.nombre) }}
          </div>

          <h3 class="fw-bold mb-2">{{ user.nombre }}</h3>

          <div class="d-flex justify-content-center align-items-center gap-3 mb-3">
            <div class="d-flex align-items-center text-secondary">
              <IconCreditCard class="icon icon-sm me-1" />
              <span class="small fw-medium">{{ user.dni }}</span>
            </div>

            <div class="d-flex align-items-center text-secondary" v-if="user.nacimiento">
              <IconCake class="icon icon-sm me-1" />
              <span class="small fw-medium"> {{ getYear(new Date()) - getYear(addDays(parseISO(user.nacimiento), 0)) }} años </span>
            </div>
          </div>
        </div>
      </div>

      <div class="p-2" v-if="vinculo">
        <div class="d-flex flex-column">
          <div class="d-flex align-items-center small bg-light rounded p-2 transition-all">
            <IconUserBolt class="me-2 text-green" />
            <div>
              <small class="text-muted d-block">Cargo</small>
              <span class="fw-medium">{{ vinculo.cargo }}</span>
            </div>
          </div>
          <div class="d-flex align-items-center small bg-light rounded p-2 transition-all">
            <IconBuildingCommunity class="me-2 text-primary" />
            <div>
              <small class="text-muted d-block">Área</small>
              <span class="fw-medium">{{ vinculo.area }}</span>
            </div>
          </div>
          <div class="d-flex align-items-center small bg-light rounded p-2 transition-all">
            <IconCalendar class="me-2 text-primary" />
            <div>
              <small class="text-muted d-block">Fecha de Ingreso</small>
              <span class="fw-bold fs-4">
                {{ vinculo.fecha_ingreso ? format(addDays(parseISO(vinculo.fecha_ingreso), 0), 'dd/MM/yyyy') : 'No disponible' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconBuildingCommunity, IconCake, IconCalendar, IconCreditCard, IconUserBolt } from '@tabler/icons-vue'
import { format, parseISO, addDays, getYear } from 'date-fns'

defineProps({
  user: { type: Object, required: true },
  vinculo: { type: Object }
})

const nombreabrv = (nombrefull: string) => {
  if (!nombrefull) return ''
  const palabras = nombrefull.trim().split(/\s+/)
  const iniciales = palabras
    .slice(0, 2)
    .map((palabra) => palabra.charAt(0).toUpperCase())
    .join('')
  return iniciales
}
</script>

<style scoped>
.text-white-75 {
  color: rgba(255, 255, 255, 0.75) !important;
}

.avatar-xl {
  width: 4rem;
  height: 4rem;
  font-size: 1.5rem;
}

.gap-3 {
  gap: 1rem;
}

.gap-4 {
  gap: 1.5rem;
}
</style>
