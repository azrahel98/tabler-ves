<script setup lang="ts">
import { IconBrandWhatsapp, IconMail } from '@tabler/icons-vue'

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
      <span class="avatar avatar-lg mb-3 rounded">{{ nombreabrv(user.nombre) }}</span>
      <h4 class="m-0 mb-1">
        <a href="#">{{ user.nombre }}</a>
      </h4>
      <div class="text-secondary small">{{ user.dni }}</div>
      <div class="mt-3">
        <span class="badge text-uppercase fw-medium" style="font-size: 0.7rem" :class="{ 'bg-danger-lt': user.estado == 'inactivo', 'bg-success-lt': user.estado == 'activo' }">{{
          user.estado
        }}</span>
      </div>
    </div>
    <div class="d-flex">
      <a href="#" class="card-btn">
        <IconMail class="icon me-2 text-muted icon-3" />
        Email</a
      >
      <a href="#" class="card-btn">
        <IconBrandWhatsapp class="icon text-muted me-3" />

        Call</a
      >
    </div>
  </div>
</template>
