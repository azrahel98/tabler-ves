<script setup lang="ts">
import { IconCake, IconCreditCard } from '@tabler/icons-vue'
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

<template>
  <div class="card">
    <div class="card-body p-4 pb-2 text-center d-flex flex-column gap-0 align-items-center">
      <span class="avatar fs-4 avatar-lg avatar-rounded">{{ nombreabrv(user.nombre) }}</span>
      <h4 class="font-bold tracking-tight w-75 p-0 m-0">{{ user.nombre }}</h4>
      <div class="text-secondary fs-5">
        <IconCreditCard class="icon" />
        {{ user.dni }}
      </div>
      <span class="badge mt-3 badge-lg fs-5 text-secondary" v-if="user.nacimiento">
        <IconCake class="icon text-yellow" />
        {{ getYear(new Date()) - getYear(addDays(parseISO(user.nacimiento), 0)) }} años
      </span>
      <div v-if="vinculo" class="mt-1"></div>
      <div v-if="vinculo" class="text-center">
        <div class="text-primary fw-medium fs-5">
          {{ vinculo.cargo }}
        </div>
        <!-- <span class="badge px-3 bg-primary text-white badge-sm fw-bold text-wrap align-middle">
          {{ vinculo.cargo }}
        </span> -->

        <div class="text-secondary fw-bold fs-6 mb-1 fw-medium">
          {{ vinculo.area }}
        </div>

        <div class="fw-bold small">Fecha Ingreso</div>
        <div class="datagrid-content fs-5 fw-medium">
          {{ vinculo.fecha_ingreso ? format(addDays(parseISO(vinculo.fecha_ingreso), 0), 'dd/MM/yyyy') : 'Fecha no disponible' }}
        </div>
      </div>
    </div>
  </div>
</template>
