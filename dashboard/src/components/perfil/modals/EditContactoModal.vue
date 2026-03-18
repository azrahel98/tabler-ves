<template>
  <Modal :isOpen="isOpen" title="Contacto de Emergencia" @close="close">
    <form @submit.prevent="guardar" class="space-y-5">
      <div class="grid grid-cols-1 gap-5">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nombre Completo</label>
          <input
            type="text"
            v-model="form.nombre"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            required
            placeholder="Juan Pérez" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Parentezco / Relación</label>
          <input
            type="text"
            v-model="form.relacion"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            required
            placeholder="Padre / Madre / Esposo(a)" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Teléfono</label>
          <input
            type="text"
            v-model="form.telefono"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="999999999" />
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
        Guardar
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
  import { storeToRefs } from 'pinia'
  import { Loader2 } from 'lucide-vue-next'

  const props = defineProps<{
    isOpen: boolean
    contacto: any
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
  }>()

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const isSubmitting = ref(false)

  const form = ref({
    nombre: '',
    relacion: '',
    telefono: '',
  })

  watch(
    () => props.contacto,
    (newVal) => {
      if (newVal) {
        form.value = {
          nombre: newVal.nombre || '',
          relacion: newVal.relacion || '',
          telefono: newVal.telefono || '',
        }
      } else {
        form.value = { nombre: '', relacion: '', telefono: '' }
      }
    },
    { immediate: true, deep: true }
  )

  const close = () => {
    if (isSubmitting.value) return
    emit('close')
  }

  const guardar = async () => {
    if (!form.value.nombre || !form.value.relacion) return

    isSubmitting.value = true
    try {
      await personalStore.agregarContacto({
        persona_dni: perfilActual.value?.dni,
        ...form.value,
      })
      emit('close')
    } catch (error) {
      console.error('Error al guardar contacto', error)
    } finally {
      isSubmitting.value = false
    }
  }
</script>
