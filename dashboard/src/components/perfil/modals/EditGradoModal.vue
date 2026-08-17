<template>
  <Modal :isOpen="isOpen" :title="isEdit ? 'Editar Grado Académico' : 'Agregar Grado Académico'" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div class="col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Profesión</label>
          <input
            type="text"
            v-model="form.profesion"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.profesion ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'"
            placeholder="Ej: Ingeniero de Sistemas" />
          <p v-if="errores.profesion" class="mt-1 text-xs text-red-500">{{ errores.profesion }}</p>
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
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.nivel_academico ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'">
            <option value="">Seleccione</option>
            <option value="TECNICO">Técnico</option>
            <option value="BACHILLER">Bachiller</option>
            <option value="TITULADO">Titulado</option>
            <option value="MAGISTER">Maestría</option>
            <option value="DOCTOR">Doctorado</option>
          </select>
          <p v-if="errores.nivel_academico" class="mt-1 text-xs text-red-500">{{ errores.nivel_academico }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Abreviatura</label>
          <input
            type="text"
            v-model="form.abrv"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.abrv ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'"
            placeholder="Ej: ING, LIC, BACH" />
          <p v-if="errores.abrv" class="mt-1 text-xs text-red-500">{{ errores.abrv }}</p>
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
        class="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-slate-900 shadow-sm ring-1 ring-inset ring-slate-300 hover:bg-slate-50 dark:bg-slate-800 dark:text-slate-100 dark:ring-slate-700 dark:hover:bg-slate-700 sm:mt-0 sm:w-auto"
        :disabled="isSubmitting">
        Cancelar
      </button>
      <button
        type="button"
        @click="guardar"
        class="inline-flex w-full justify-center items-center gap-2 rounded-md bg-primary px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-primary/90 sm:ml-3 sm:w-auto"
        :disabled="isSubmitting">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 animate-spin" />
        {{ isEdit ? 'Guardar Cambios' : 'Agregar' }}
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, reactive, watch } from 'vue'
  import { z } from 'zod/v4'
  import { storeToRefs } from 'pinia'
  import { usePersonalStore } from '../../../stores/personal'
  import Modal from '../../ui/Modal.vue'
  import { Loader2 } from 'lucide-vue-next'
  import type { GradoAcademico } from '../../../types'

  const store = usePersonalStore()
  const { perfilActual } = storeToRefs(store)

  const props = defineProps<{
    isOpen: boolean
    grado: GradoAcademico | null
    isEdit: boolean
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: GradoAcademico & { dni: string }): void
  }>()

  const gradoSchema = z.object({
    profesion: z.string().min(1, 'La profesión es requerida'),
    nivel_academico: z.string().min(1, 'Seleccione un nivel académico'),
    abrv: z.string().min(1, 'La abreviatura es requerida'),
  })

  const isSubmitting = ref(false)

  const form = ref({
    id: null as number | null,
    profesion: '',
    universidad: '',
    colegiatura: '',
    nivel_academico: '',
    abrv: '',
  })

  const errores = reactive<Record<string, string>>({})

  function limpiarErrores() {
    Object.keys(errores).forEach((k) => delete errores[k])
  }

  watch(
    () => [props.isOpen, props.grado],
    ([isOpen]) => {
      if (isOpen) {
        limpiarErrores()
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
    if (isSubmitting.value) return
    limpiarErrores()
    emit('close')
  }

  const guardar = () => {
    limpiarErrores()
    const resultado = gradoSchema.safeParse(form.value)
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
    const payload = {
      ...form.value,
      dni: perfilActual.value?.dni ?? '',
    } as GradoAcademico & { dni: string }
    emit('save', payload)
    isSubmitting.value = false
  }
</script>
