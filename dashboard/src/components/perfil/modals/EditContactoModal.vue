<template>
  <Modal :isOpen="isOpen" title="Contacto de Emergencia" @close="close">
    <form @submit.prevent="guardar" class="space-y-5">
      <div class="grid grid-cols-1 gap-5">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nombre Completo</label>
          <input
            type="text"
            v-model="form.nombre"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.nombre ? 'border-red-500' : 'border-gray-300'"
            placeholder="Juan Pérez" />
          <p v-if="errores.nombre" class="mt-1 text-xs text-red-500">{{ errores.nombre }}</p>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Parentezco / Relación</label>
          <input
            type="text"
            v-model="form.relacion"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.relacion ? 'border-red-500' : 'border-gray-300'"
            placeholder="Padre / Madre / Esposo(a)" />
          <p v-if="errores.relacion" class="mt-1 text-xs text-red-500">{{ errores.relacion }}</p>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Teléfono</label>
          <input
            type="text"
            v-model="form.telefono"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.telefono ? 'border-red-500' : 'border-gray-300'"
            placeholder="999999999" />
          <p v-if="errores.telefono" class="mt-1 text-xs text-red-500">{{ errores.telefono }}</p>
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
  import { ref, reactive, watch } from 'vue'
  import { z } from 'zod/v4'
  import Modal from '../../ui/Modal.vue'
  import { usePersonalStore } from '../../../stores/personal'
  import { storeToRefs } from 'pinia'
  import { Loader2 } from 'lucide-vue-next'
  import type { ContactoEmergencia } from '../../../types'

  const props = defineProps<{
    isOpen: boolean
    contacto: ContactoEmergencia | null
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
  }>()

  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  const contactoSchema = z.object({
    nombre: z.string().min(1, 'El nombre es requerido'),
    relacion: z.string().min(1, 'La relación es requerida'),
    telefono: z.string().regex(/^\d{9}$/, 'El teléfono debe tener 9 dígitos').or(z.literal('')).optional(),
  })

  const isSubmitting = ref(false)

  const form = ref({
    nombre: '',
    relacion: '',
    telefono: '',
  })

  const errores = reactive<Record<string, string>>({})

  function limpiarErrores() {
    Object.keys(errores).forEach((k) => delete errores[k])
  }

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
    limpiarErrores()
    emit('close')
  }

  const guardar = async () => {
    limpiarErrores()
    const resultado = contactoSchema.safeParse(form.value)
    if (!resultado.success) {
      for (const issue of resultado.error.issues) {
        const campo = issue.path.join('.')
        if (!errores[campo]) {
          errores[campo] = issue.message
        }
      }
      return
    }

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
