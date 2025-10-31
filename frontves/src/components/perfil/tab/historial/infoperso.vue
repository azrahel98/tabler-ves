<template>
  <div class="border rounded-2 p-3 mt-2 bg-light-lt">
    <div class="info-grid">
      <div v-for="(item, index) in detalle" :key="index" class="info-grid-item">
        <div class="d-flex align-items-center text-muted">
          <IconMail v-if="item.campo === 'Correo'" class="icon icon-sm me-2" />
          <IconPhone v-if="item.campo === 'Teléfono'" class="icon icon-sm me-2" />
          <span>{{ item.campo }}</span>
        </div>
        <div class="text-body fw-bold">
          {{ item.valor }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IconMail, IconPhone } from '@tabler/icons-vue'

// --- Interfaces para Props ---
interface DetalleInfo {
  campo: 'Correo' | 'Teléfono' | string // Extensible para otros campos
  valor: string
}

defineProps<{
  detalle: DetalleInfo[]
}>()
</script>

<style scoped>
.info-grid {
  display: grid;
  gap: 0.75rem 1.5rem;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
}
.info-grid-item > div:first-child {
  margin-bottom: 0.15rem;
}
</style>
