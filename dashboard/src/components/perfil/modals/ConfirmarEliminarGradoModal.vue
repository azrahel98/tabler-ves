<template>
  <Modal :isOpen="isOpen" @close="close" title="Eliminar grado académico">
    <div class="space-y-4">
      <div class="rounded-md bg-red-50 p-4 dark:bg-red-900/20">
        <div class="flex">
          <div class="shrink-0">
            <AlertTriangle class="h-5 w-5 text-red-400" aria-hidden="true" />
          </div>
          <div class="ml-3">
            <h3 class="text-sm font-medium text-red-800 dark:text-red-300">Acción irreversible</h3>
            <div class="mt-2 text-sm text-red-700 dark:text-red-400">
              <p>Está a punto de eliminar este grado académico del trabajador. Esta acción no se puede deshacer.</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="grado" class="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-3 text-sm space-y-2">
        <div class="flex flex-col">
          <span class="text-xs font-medium text-slate-500 uppercase">Profesión</span>
          <span class="font-medium text-slate-900 dark:text-white">{{ grado.profesion }}</span>
        </div>
        <div class="flex flex-col" v-if="grado.universidad">
          <span class="text-xs font-medium text-slate-500 uppercase">Universidad</span>
          <span class="font-medium text-slate-900 dark:text-white">{{ grado.universidad }}</span>
        </div>
        <div class="flex flex-col" v-if="grado.nivel_academico">
          <span class="text-xs font-medium text-slate-500 uppercase">Nivel</span>
          <span class="font-medium text-slate-900 dark:text-white">{{ grado.nivel_academico }}</span>
        </div>
      </div>
    </div>

    <template #footer>
      <button
        type="button"
        class="inline-flex w-full justify-center rounded-lg bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto transition-colors"
        @click="confirm"
        :disabled="isSubmitting">
        <Loader2 v-if="isSubmitting" class="animate-spin -ml-1 mr-2 h-4 w-4" />
        Sí, eliminar grado
      </button>
      <button
        type="button"
        class="mt-3 inline-flex w-full justify-center rounded-lg bg-white px-3 py-2 text-sm font-semibold text-slate-900 shadow-sm ring-1 ring-inset ring-slate-300 hover:bg-slate-50 sm:mt-0 sm:w-auto dark:bg-slate-800 dark:text-slate-300 dark:ring-slate-700 dark:hover:bg-slate-700 transition-colors"
        @click="close"
        :disabled="isSubmitting">
        Cancelar
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import Modal from '../../ui/Modal.vue'
  import { AlertTriangle, Loader2 } from 'lucide-vue-next'
  import type { GradoAcademico } from '../../../types'

  const props = defineProps<{
    isOpen: boolean
    grado: GradoAcademico | null
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'confirm', id: number): void
  }>()

  const isSubmitting = ref(false)

  const close = () => {
    if (isSubmitting.value) return
    emit('close')
  }

  const confirm = async () => {
    if (!props.grado || props.grado.id == null) return
    isSubmitting.value = true
    try {
      emit('confirm', props.grado.id)
    } finally {
      isSubmitting.value = false
    }
  }
</script>
