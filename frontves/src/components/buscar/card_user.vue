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
    <div class="card-body p-4 pb-2 row-gap-sm-6 text-start d-flex flex-column justify-content-between">
      <div class="d-flex align-items-start gap-2">
        <span
          class="avatar avatar-rounded avatar-sm flex-shrink-0 d-flex align-items-center justify-content-center"
          :class="{ 'bg-secondary-lt ': user.estado == 'inactivo', 'bg-green-lt ': user.estado == 'activo' }"
        >
          {{ nombreabrv(user.nombre) }}
        </span>
        <div>
          <h4 class="m-0 mb-1">
            <RouterLink :to="{ name: 'perfil', params: { dni: user.dni } }">
              {{ user.nombre }}
            </RouterLink>
          </h4>
          <div class="text-secondary">{{ user.dni }}</div>
        </div>
      </div>

      <div class="text-end">
        <h6 class="badge" :class="{ 'bg-secondary-lt ': user.estado == 'inactivo', 'bg-green-lt': user.estado == 'activo' }">
          {{ user.estado != 'activo' ? 'Inactivo' : 'Activo' }}
        </h6>
      </div>
    </div>
  </div>
</template>
