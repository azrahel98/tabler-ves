<script setup lang="ts">
import { format, parseISO, addDays } from 'date-fns'
import { IconBrandWhatsapp, IconMail } from '@tabler/icons-vue'
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
      <h4 class="m-0 mb-1">
        <RouterLink :to="{ name: 'perfil', params: { dni: user.dni } }">
          {{ user.nombre }}
        </RouterLink>
      </h4>
      <div class="text-secondary small">{{ user.dni }}</div>
      <div class="mt-3"></div>
      <div v-if="vinculo" class="text-center mt-2 lista small">
        <span class="badge bg-primary text-white fw-bold px-2 text-wrap align-middle">
          {{ vinculo.cargo }}
        </span>

        <div class="text-muted mb-3 fw-medium">
          {{ vinculo.area }}
        </div>

        <div class="text-uppercase fw-medium text-black">Fecha Ingreso</div>
        <div class="datagrid-content">
          {{ vinculo.fecha_ingreso ? format(addDays(parseISO(vinculo.fecha_ingreso), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}
        </div>
      </div>
      <div class="d-flex mt-3" v-if="user.telf">
        <a class="card-btn" :href="`https://wa.me/${user.telf}?text=Hola%2C%20quisiera%20más%20información.`">
          <IconMail class="icon me-2 text-muted icon-3" />
          Email</a
        >
        <!-- <a class="card-btn" v-if="user.email">
          <IconBrandWhatsapp class="icon text-muted me-3" />

          Call</a
        > -->
      </div>
    </div>
  </div>
</template>
