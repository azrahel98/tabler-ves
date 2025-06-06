<script setup lang="ts">
defineProps({
  user: { type: Object, required: true }
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
      <span class="avatar fw-bolder avatar-md mb-3 rounded" :class="{ 'bg-secondary-lt': user.estado == 'inactivo', 'bg-primary-lt': user.estado == 'activo' }" v>{{
        nombreabrv(user.nombre)
      }}</span>
      <h5 class="m-0 mb-1">
        <RouterLink :to="{ name: 'perfil', params: { dni: user.dni } }">
          {{ user.nombre }}
        </RouterLink>
      </h5>
      <div class="text-secondary small">{{ user.dni }}</div>
      <div class="mt-3">
        <span
          class="badge text-uppercase fw-medium text-white fs-6"
          style="font-size: 0.7rem"
          :class="{ 'bg-secondary': user.estado == 'inactivo', 'bg-primary': user.estado == 'activo' }"
          >{{ user.estado }}</span
        >
      </div>
    </div>
  </div>
</template>
