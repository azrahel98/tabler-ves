<template>
  <Modal :isOpen="isOpen" :title="isEdit ? 'Editar Grado Académico' : 'Agregar Grado Académico'" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div class="col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Profesión</label>
          <input
            type="text"
            v-model="form.profesion"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Ej: Ingeniero de Sistemas"
            required />
        </div>

        <div class="col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Universidad</label>
          <input
            type="text"
            v-model="form.universidad"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Ej: Universidad Nacional de Piura" />
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Nivel Académico</label>
          <select
            v-model="form.nivel_academico"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required>
            <option value="">Seleccione</option>
            <option value="TECNICO">Técnico</option>
            <option value="BACHILLER">Bachiller</option>
            <option value="TITULADO">Titulado</option>
            <option value="MAGISTER">Maestría</option>
            <option value="DOCTOR">Doctorado</option>
          </select>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Abreviatura</label>
          <input
            type="text"
            v-model="form.abrv"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Ej: ING, LIC, BACH"
            required />
        </div>

        <div class="col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Colegiatura</label>
          <input
            type="text"
            v-model="form.colegiatura"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Nro. de colegiatura (opcional)" />
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
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'
  import Modal from '../../ui/Modal.vue'

  const store = usePersonalStore()
  const { perfilActual } = storeToRefs(store)

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
    id: null as number | null,
    profesion: '',
    universidad: '',
    colegiatura: '',
    nivel_academico: '',
    abrv: '',
  })

  watch(
    () => [props.isOpen, props.grado],
    ([isOpen]) => {
      if (isOpen) {
        if (props.isEdit && props.grado) {
          form.value = {
            id: props.grado.id,
            profesion: props.grado.profesion || '',
            universidad: props.grado.universidad || '',
            colegiatura: props.grado.colegiatura || '',
            nivel_academico: props.grado.nivel_academico || '',
            abrv: props.grado.abrv || '',
          }
        } else {
          form.value = {
            id: null,
            profesion: '',
            universidad: '',
            colegiatura: '',
            nivel_academico: '',
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
    const payload = {
      ...form.value,
      dni: perfilActual.value?.dni,
    }
    emit('save', payload)
  }
</script>
