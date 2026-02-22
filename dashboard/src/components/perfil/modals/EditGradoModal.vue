<template>
  <Modal :isOpen="isOpen" :title="isEdit ? 'Editar Grado Académico' : 'Agregar Grado Académico'" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4">
        <!-- Abreviatura -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Abreviatura</label>
          <input
            type="text"
            v-model="form.abrv"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Ej: Ing., Lic., Mag."
            required />
        </div>

        <!-- Descripción -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Descripción Completa</label>
          <input
            type="text"
            v-model="form.descripcion"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Ingeniero de Sistemas"
            required />
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="close"
        class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-slate-900 shadow-sm ring-1 ring-inset ring-slate-300 hover:bg-slate-50 dark:bg-slate-800 dark:text-slate-100 dark:ring-slate-700 dark:hover:bg-slate-700 sm:mt-0 sm:w-auto">
        Cancelar
      </button>
      <button
        type="button"
        @click="guardar"
        class="inline-flex w-full justify-center rounded-md bg-primary px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary/90 sm:ml-3 sm:w-auto">
        {{ isEdit ? 'Guardar Cambios' : 'Agregar' }}
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import Modal from '../../ui/Modal.vue'

  const props = defineProps<{
    isOpen: boolean
    grado: any
    isEdit: boolean
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const form = ref({
    id: null,
    descripcion: '',
    abrv: '',
  })

  watch(
    () => [props.isOpen, props.grado],
    ([isOpen, info]) => {
      if (isOpen) {
        if (props.isEdit && info) {
          form.value = {
            id: info.id,
            descripcion: info.descripcion || '',
            abrv: info.abrv || '',
          }
        } else {
          form.value = {
            id: null,
            descripcion: '',
            abrv: '',
          }
        }
      }
    }
  )

  const close = () => {
    emit('close')
  }

  const guardar = () => {
    emit('save', form.value)
  }
</script>
