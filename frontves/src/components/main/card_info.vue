<template>
  <div class="card p-0 card-sm">
    <div class="card-body py-2 px-3">
      <div class="row align-items-center">
        <div class="col-auto">
          <slot />
        </div>
        <div class="col">
          <div class="font-weight-medium">{{ title }}</div>
          <div class="text-secondary small">{{ cantidad }} {{ descripcion }}</div>
        </div>
        <span class="col-2 cursor-pointer" v-if="descarga">
          <IconDownload class="icon icon text-primary" @click="funcion()" />
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconDownload } from '@tabler/icons-vue'
defineProps({
  title: { type: String, required: true },
  cantidad: { type: Number, required: false, default: 50 },
  descripcion: { type: String, required: false, default: '' },
  descarga: { type: Boolean, required: false, default: false },
  funcion: { type: Function, required: false, default: () => {} }
})
</script>
