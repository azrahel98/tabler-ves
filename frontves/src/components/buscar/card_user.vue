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
  <div class="card h-100" style="min-height: 15vh">
    <div class="card-body p-4 pb-2 text-start d-flex flex-column justify-content-between">
      <div class="d-flex align-items-start gap-2">
        <span
          class="avatar avatar-rounded fs-5 flex-shrink-0 d-flex align-items-center justify-content-center"
          :class="{ 'bg-secondary-lt ': user.estado == 'inactivo', 'bg-facebook text-white': user.estado == 'activo' }"
        >
          {{ nombreabrv(user.nombre) }}
        </span>
        <div>
          <h4 class="m-0 mb-1">
            <RouterLink :to="{ name: 'perfil', params: { dni: user.dni } }">
              {{ user.nombre }}
            </RouterLink>
          </h4>
          <div class="text-secondary small">{{ user.dni }}</div>
        </div>
      </div>

      <div class="text-end">
        <span class="badge text-lowercase fw-normal fs-6 text-white" :class="{ 'bg-secondary-lt ': user.estado == 'inactivo', 'bg-primary-lt': user.estado == 'activo' }">{{
          user.estado
        }}</span>
      </div>
    </div>
  </div>
</template>
