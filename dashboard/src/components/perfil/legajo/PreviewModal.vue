<template>
  <Modal :isOpen="isOpen" @close="close" maxWidth="w-full sm:max-w-3xl md:max-w-5xl lg:max-w-6xl xl:max-w-[90vw]">
    <template #header>
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2.5 m-0 pr-8 truncate" :title="documentoActual?.original_name">
        <FileText class="h-5 w-5 text-gray-500 dark:text-gray-400 shrink-0" />
        <span class="truncate">{{ documentoActual?.original_name || 'Vista Previa del Documento' }}</span>
      </h3>
    </template>

    <!-- Contenedor del PDF (Ajustado para ocupar más pantalla) -->
    <div
      class="w-full h-[65vh] md:h-[80vh] flex flex-col items-center justify-center bg-gray-100/50 dark:bg-gray-900/50 rounded-xl border border-gray-200 dark:border-gray-800 overflow-hidden relative">
      <iframe v-if="urlPrevia" :src="urlPrevia" class="w-full h-full border-0 absolute inset-0 z-20" title="Vista previa del documento PDF"> </iframe>

      <!-- Estado de Carga -->
      <div v-else class="flex flex-col items-center justify-center text-gray-500 z-10 w-full h-full absolute inset-0 bg-white dark:bg-gray-900">
        <Loader2 class="h-8 w-8 animate-spin mb-4 text-brand-500" />
        <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Cargando vista previa...</p>
      </div>
    </div>

    <template #footer>
      <!-- En móvil, botones apilados (cerrar arriba, descargar abajo). En PC, botones separados -->
      <div class="flex flex-col-reverse sm:flex-row justify-between w-full gap-3 sm:gap-0">
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          :download="documentoActual?.original_name"
          class="inline-flex w-full sm:w-auto justify-center items-center gap-2 rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700">
          <Download class="h-4 w-4" />
          Descargar Archivo
        </a>
        <span v-else></span>

        <button
          @click="close"
          class="inline-flex w-full sm:w-auto justify-center items-center rounded-lg bg-brand-600 px-8 py-2 text-sm font-medium text-white shadow-sm hover:bg-brand-700 transition">
          Cerrar Vista
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import Modal from '../../ui/Modal.vue'
  import { Loader2, FileText, Download } from 'lucide-vue-next'

  defineProps<{
    isOpen: boolean
    url: string | null
    documentoActual: any
    urlPrevia: string | null
  }>()

  const emit = defineEmits(['close'])

  const close = () => {
    emit('close')
  }
</script>
