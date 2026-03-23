<template>
  <Modal :isOpen="isOpen" title="Editar Información Personal" @close="close">
    <form @submit.prevent="guardar" class="space-y-5">
      <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Teléfono / Celular</label>
          <input
            type="text"
            v-model="form.telf"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.telf ? 'border-red-500' : 'border-gray-300'"
            placeholder="999999999" />
          <p v-if="errores.telf" class="mt-1 text-xs text-red-500">{{ errores.telf }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Correo Electrónico</label>
          <input
            type="email"
            v-model="form.email"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.email ? 'border-red-500' : 'border-gray-300'"
            placeholder="correo@ejemplo.com" />
          <p v-if="errores.email" class="mt-1 text-xs text-red-500">{{ errores.email }}</p>
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
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Región</label>
          <input
            type="text"
            v-model="form.region"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="Lima" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Distrito</label>
          <input
            type="text"
            v-model="form.distrito"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            placeholder="Miraflores" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">RUC</label>
          <input
            type="text"
            v-model="form.ruc"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.ruc ? 'border-red-500' : 'border-gray-300'"
            placeholder="10123456789" />
          <p v-if="errores.ruc" class="mt-1 text-xs text-red-500">{{ errores.ruc }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sexo</label>
          <select
            v-model="form.sexo"
            class="h-11 w-full rounded-lg border bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            :class="errores.sexo ? 'border-red-500' : 'border-gray-300'">
            <option value="">Seleccione</option>
            <option value="M">Masculino</option>
            <option value="F">Femenino</option>
          </select>
          <p v-if="errores.sexo" class="mt-1 text-xs text-red-500">{{ errores.sexo }}</p>
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha de Nacimiento</label>
          <input
            type="date"
            v-model="form.nacimiento"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
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
  import { ref, reactive, watch } from 'vue'
  import { z } from 'zod/v4'
  import Modal from '../../ui/Modal.vue'
  import { usePersonalStore } from '../../../stores/personal'
  import { Loader2 } from 'lucide-vue-next'
  import type { Persona } from '../../../types'

  const props = defineProps<{
    isOpen: boolean
    perfil: Persona | null
  }>()

  const personalStore = usePersonalStore()

  const emit = defineEmits<{
    (e: 'close'): void
  }>()

  const infoSchema = z.object({
    telf: z.string().regex(/^\d{9}$/, 'El teléfono debe tener 9 dígitos').or(z.literal('')).optional(),
    email: z.email('Correo electrónico inválido').or(z.literal('')).optional(),
    ruc: z.string().regex(/^\d{11}$/, 'El RUC debe tener 11 dígitos').or(z.literal('')).optional(),
    sexo: z.enum(['M', 'F', ''], { error: 'Seleccione un sexo' }).optional(),
  })

  const isSubmitting = ref(false)

  const form = ref({
    dni: '',
    nombre: '',
    telf: '',
    email: '',
    direccion: '',
    region: '',
    distrito: '',
    ruc: '',
    nacimiento: '',
    sexo: '',
  })

  const errores = reactive<Record<string, string>>({})

  function limpiarErrores() {
    Object.keys(errores).forEach((k) => delete errores[k])
  }

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
          region: newVal.region || '',
          distrito: newVal.distrito || '',
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
    limpiarErrores()
    emit('close')
  }

  const guardar = async () => {
    limpiarErrores()
    const resultado = infoSchema.safeParse(form.value)
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
      const payload = Object.fromEntries(
        Object.entries(form.value).map(([k, v]) => [k, v === '' ? null : v])
      )
      await personalStore.actualizarPerfil(payload)
      emit('close')
    } catch (error) {
      console.error('Error al guardar', error)
    } finally {
      isSubmitting.value = false
    }
  }
</script>
