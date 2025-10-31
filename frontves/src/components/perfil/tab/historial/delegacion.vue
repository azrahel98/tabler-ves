<template>
  <div class="border rounded-2 p-3 mt-2 bg-light-lt">
    <div class="d-flex align-items-center">
      <div>
        <span :class="['badge me-3', badgeClass]">{{ detalle.estado }}</span>
      </div>
      <div class="d-flex align-items-center">
        <span class="avatar avatar-sm avatar-rounded me-2">
          {{ getInitials(detalle.usuario) }}
        </span>
        <div>
          <div class="fw-bold">{{ detalle.usuario }}</div>
          <div class="text-muted small">{{ detalle.comentario }}</div>
        </div>
      </div>
    </div>
    <div class="text-muted small mt-2">
      {{ detalle.fecha_accion }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

// --- Interfaces para Props ---
interface DetalleDelegacion {
  estado: 'Devuelto por' | 'En poder de'
  usuario: string
  comentario: string
  fecha_accion: string
}

const props = defineProps<{
  detalle: DetalleDelegacion
}>()

// --- Lógica del Componente ---
const getInitials = (name: string): string => {
  return name
    .split(' ')
    .map((word) => word.charAt(0))
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

const badgeClass = computed(() => {
  return props.detalle.estado === 'En poder de' ? 'bg-secondary-lt' : 'bg-primary-lt'
})
</script>
