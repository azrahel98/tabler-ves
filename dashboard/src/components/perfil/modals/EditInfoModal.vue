<template>
  <Modal :isOpen="isOpen" title="Editar Información Personal" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <!-- Teléfono -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Teléfono / Celular</label>
          <input
            type="text"
            v-model="form.telf"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="999999999" />
        </div>

        <!-- Correo Electrónico -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Correo Electrónico</label>
          <input
            type="email"
            v-model="form.email"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="correo@ejemplo.com" />
        </div>

        <!-- Dirección -->
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Dirección</label>
          <input
            type="text"
            v-model="form.direccion"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Av. Ejemplo 123" />
        </div>

        <!-- RUC -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">RUC</label>
          <input
            type="text"
            v-model="form.ruc"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="10123456789" />
        </div>

        <!-- Sexo -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Sexo</label>
          <select
            v-model="form.sexo"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white">
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
        </div>

        <!-- Fecha de Nacimiento -->
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Fecha de Nacimiento</label>
          <input
            type="date"
            v-model="form.nacimiento"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
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
        Guardar Cambios
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import Modal from '../../ui/Modal.vue'
  import { usePersonalStore } from '../../../stores/personal'

  const props = defineProps<{
    isOpen: boolean
    perfil: any
  }>()

  const personalStore = usePersonalStore()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const form = ref({
    dni: '',
    nombre: '',
    telf: '',
    email: '',
    direccion: '',
    ruc: '',
    nacimiento: '',
    sexo: '',
  })

  watch(
    () => props.perfil,
    (newVal) => {
      if (newVal) {
        form.value = {
          dni: newVal.dni || '',
          nombre: newVal.nombre || '',
          telf: newVal.telf || '',
          email: newVal.email || '',
          direccion: newVal.direccion || '',
          ruc: newVal.ruc || '',
          nacimiento: newVal.nacimiento ? newVal.nacimiento.split('T')[0] : '',
          sexo: newVal.sexo || '',
        }
      }
    },
    { immediate: true, deep: true }
  )

  const close = () => {
    emit('close')
  }

  const guardar = async () => {
    try {
      await personalStore.updateProfile(form.value)
      emit('close')
    } catch (error) {
      console.error('Error al guardar', error)
    }
  }
</script>
