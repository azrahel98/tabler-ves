<script setup lang="ts">
import { format, parseISO, addDays } from 'date-fns'

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

<template>
  <div class="card">
    <div class="card-body p-3 text-center">
      <span class="avatar avatar-md mb-3 rounded">{{ nombreabrv(user.nombre) }}</span>
      <p class="m-0 mb-1  ">
        <RouterLink class="text-black fw-medium text-decoration-none text-body" :to="{ name: 'perfil', params: { dni: user.dni } }">
          {{ user.nombre }}
        </RouterLink>
      </p>
      <div class="text-secondary  text-hint">{{ user.dni }}</div>
      <div class="mt-3"></div>
      <div v-if="vinculo" class="text-center mt-2 lista text-hint">
        <span class="badge px-3 bg-primary text-white fw-bold text-wrap align-middle">
          {{ vinculo.cargo }}
        </span>

        <div class="text-muted text-hint mb-3 fw-medium">
          {{ vinculo.area }}
        </div>

        <div class="text-uppercase fw-medium small">Fecha Ingreso</div>
        <div class="datagrid-content fs-5 fw-medium">
          {{ vinculo.fecha_ingreso ? format(addDays(parseISO(vinculo.fecha_ingreso), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.badge {
  width: auto;
}
</style>
