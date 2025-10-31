<template>
  <div class="small pt-3 px-2">
    <div class="row align-items-start justify-content-start row-gap-0">
      <div class="col-3">
        <span class="badge text-white text-wrap px-2 align-middle" :class="[JSON.parse(item.detalle).estado === 'archivado' ? 'bg-primary' : 'bg-secondary']">
          {{ JSON.parse(item.detalle).estado === 'archivado' ? 'Devuelto por:' : 'En poder de:' }}
        </span>
      </div>
      <div class="col-auto">
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
  </div>
</template>

<script lang="ts" setup>
import { formatFechaCompleta } from '@api/date'
defineProps({
  item: { type: Object, required: true }
})
</script>
