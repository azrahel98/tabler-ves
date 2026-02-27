<template>
  <Modal :isOpen="isOpen" @close="close" maxWidth="sm:max-w-4xl lg:max-w-6xl xl:max-w-7xl">
    <template #header>
      <h3 class="text-lg font-bold text-black dark:text-white flex items-center gap-2 m-0 truncate" :title="documentoActual?.original_name">
        {{ documentoActual?.original_name || 'Vista Previa' }}
      </h3>
    </template>
    <div class="w-full h-[65vh] flex flex-col items-center justify-center bg-gray-50 dark:bg-meta-4 rounded-xl overflow-hidden my-4 relative">
      <iframe v-if="urlPrevia" :src="urlPrevia" class="w-full h-full border-0 absolute inset-0" title="Vista previa del documento PDF"> </iframe>
      <div v-else class="flex flex-col items-center justify-center text-gray-500 z-10 w-full h-full absolute inset-0 bg-white dark:bg-boxdark">
        <Loader2 class="h-8 w-8 animate-spin mb-4 text-primary" />
        <p>Cargando vista previa...</p>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-between w-full">
        <a
          v-if="urlPrevia"
          :href="urlPrevia"
          :download="documentoActual?.original_name"
          class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-primary hover:bg-gray-100 dark:border-strokedark dark:hover:bg-meta-4 transition-colors">
          Descargar
        </a>
        <span v-else></span>
        <button @click="close" class="rounded-lg bg-primary px-6 py-2 text-sm font-medium text-white hover:bg-opacity-90 transition-colors">Cerrar</button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import Modal from '../../ui/Modal.vue'
  import { Loader2 } from 'lucide-vue-next'

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
