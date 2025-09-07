<template>
  <div class="p-4">
    <!-- v-if="item.operacion === 'cambiar legajo'" -->
    <div class="estado d-flex align-items-start justify-content-start column-gap-4 row-gap-0">
      <span class="badge text-white fs-6" :class="[JSON.parse(item.detalle).estado === 'archivado' ? 'bg-primary' : 'bg-secondary']">{{
        JSON.parse(item.detalle).estado === 'archivado' ? 'Devuelto por:' : 'En poder de:'
      }}</span>
      <div class="d-flex align-items-start gap-2">
        <div class="avatar avatar-md" />
        <div>
          <h4 class="m-0 p-0">{{ JSON.parse(item.detalle).persona }}</h4>
          <p class="small m-0" v-if="JSON.parse(item.detalle).descrip">{{ JSON.parse(item.detalle).descrip }}</p>
          <div class="d-flex align-content-center align-items-center">
            <IconCalendar class="icon" :class="[JSON.parse(item.detalle).estado === 'archivado' ? 'text-primary' : 'text-danger']" />
            <p class="small m-0 fw-normal">{{ formatFechaCompleta(JSON.parse(item.detalle).fecha) }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { formatFechaCompleta } from '@api/date'
defineProps({
  item: { type: Object, required: true }
})
</script>
