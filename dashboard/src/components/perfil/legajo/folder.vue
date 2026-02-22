<template>
  <div class="rounded-2xl border border-stroke bg-white shadow-sm dark:border-strokedark dark:bg-boxdark overflow-hidden">
    <div class="flex items-center justify-between border-b border-stroke p-6 dark:border-strokedark">
      <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white">
        <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
          <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z" />
        </svg>
        VIRTUAL FOLDER (LEGAJO)
      </div>
    </div>

    <div class="p-6">
      <div v-if="legajo.length === 0" class="text-center text-gray-500">No documents uploaded yet.</div>
      <div v-else class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
        <legajo-item v-for="(item, i) in legajo" :key="i" :item="item" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import LegajoItem from './items.vue'

  const legajo = ref([
    { type: 'pdf', title: 'Contrato de Trabajo', meta: 'PDF • 1.2 MB • Subido 12 Ene, 2021' },
    { type: 'image', title: 'Documento de Identidad (DNI)', meta: 'JPEG • 450 KB • Subido 12 Ene, 2021' },
    { type: 'data', title: 'Formularios de Impuestos', meta: 'PDF • 890 KB • Actualizado 24 Feb, 2024' },
    { type: 'pdf', title: 'Certificado de Estudios', meta: 'PDF • 2 MB • Subido 15 Mar, 2022' },
  ])
</script>
