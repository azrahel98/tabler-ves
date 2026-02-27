<template>
  <Modal :isOpen="isOpen" @close="close" title="Vista Previa de Documento">
    <div class="w-full h-[70vh] flex flex-col items-center justify-center bg-gray-50 dark:bg-meta-4 rounded-xl overflow-hidden mt-4">
      <iframe
        v-if="url"
        :src="url"
        class="w-full h-full border-0"
        title="Vista previa del documento PDF">
      </iframe>
      <div v-else class="flex flex-col items-center justify-center text-gray-500">
        <Loader2 class="h-8 w-8 animate-spin mb-4" />
        <p>Cargando vista previa...</p>
      </div>
    </div>
    
    <template #footer>
      <div class="flex justify-end w-full">
        <button
          @click="close"
          class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-black hover:bg-gray-100 dark:border-strokedark dark:text-white dark:hover:bg-meta-4 transition-colors">
          Cerrar
        </button>
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
  }>()

  const emit = defineEmits(['close'])

  const close = () => {
    emit('close')
  }
</script>
