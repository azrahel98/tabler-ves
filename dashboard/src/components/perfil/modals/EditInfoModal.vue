<template>
  <Modal :isOpen="isOpen" title="Editar Información Personal" @close="close">
    <form @submit.prevent="guardar" class="space-y-5">
      <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Teléfono / Celular</label>
          <input
            type="text"
            v-model="form.telf"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="999999999" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Correo Electrónico</label>
          <input
            type="email"
            v-model="form.email"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="correo@ejemplo.com" />
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Dirección</label>
          <input
            type="text"
            v-model="form.direccion"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="Av. Ejemplo 123" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">RUC</label>
          <input
            type="text"
            v-model="form.ruc"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="10123456789" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sexo</label>
          <select
            v-model="form.sexo"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha de Nacimiento</label>
          <input
            type="date"
            v-model="form.nacimiento"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            required />
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="guardar"
        class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-brand-700 transition sm:ml-3 sm:w-auto"
        :disabled="isSubmitting">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 animate-spin" />
        Guardar Cambios
      </button>
      <button
        type="button"
        @click="close"
        class="mt-3 inline-flex w-full justify-center rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700 sm:mt-0 sm:w-auto"
        :disabled="isSubmitting">
        Cancelar
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import Modal from '../../ui/Modal.vue'
  import { usePersonalStore } from '../../../stores/personal'
  import { Loader2 } from 'lucide-vue-next'

  const props = defineProps<{
    isOpen: boolean
    perfil: any
  }>()

  const personalStore = usePersonalStore()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const isSubmitting = ref(false)

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
    if (isSubmitting.value) return
    emit('close')
  }

  const guardar = async () => {
    isSubmitting.value = true
    try {
      await personalStore.updateProfile(form.value)
      emit('close')
    } catch (error) {
      console.error('Error al guardar', error)
    } finally {
      isSubmitting.value = false
    }
  }
</script>
