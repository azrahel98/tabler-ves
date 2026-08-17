<template>
  <Modal :isOpen="isOpen" :title="isEdit ? 'Editar Información Bancaria' : 'Agregar Información Bancaria'" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Número de Cuenta</label>
          <input
            type="text"
            v-model="form.numero_cuenta"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.numero_cuenta ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'"
            placeholder="123456789" />
          <p v-if="errores.numero_cuenta" class="mt-1 text-xs text-red-500">{{ errores.numero_cuenta }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo de Cuenta</label>
          <select
            v-model="form.tipo_cuenta"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.tipo_cuenta ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'">
            <option value="AHORRO">AHORRO</option>
            <option value="CORRIENTE">CORRIENTE</option>
          </select>
          <p v-if="errores.tipo_cuenta" class="mt-1 text-xs text-red-500">{{ errores.tipo_cuenta }}</p>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Banco</label>
          <select
            v-model="form.banco"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.banco ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'">
            <option v-for="b in bancos" :key="b.id" :value="b.id">
              {{ b.nombre }}
            </option>
          </select>
          <p v-if="errores.banco" class="mt-1 text-xs text-red-500">{{ errores.banco }}</p>
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">CCI (Código de Cuenta Interbancaria)</label>
          <input
            type="text"
            v-model="form.cci"
            maxlength="20"
            class="mt-1 block w-full rounded-md border px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:bg-slate-800 dark:text-white"
            :class="errores.cci ? 'border-red-500' : 'border-slate-300 dark:border-slate-700'"
            placeholder="00212345678901234567" />
          <div class="mt-1 flex items-center justify-between">
            <p v-if="errores.cci" class="text-xs text-red-500">{{ errores.cci }}</p>
            <p class="text-xs text-slate-400 ml-auto">{{ form.cci.length }}/20</p>
          </div>
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
  import { ref, reactive, watch } from 'vue'
  import { z } from 'zod/v4'
  import Modal from '../../ui/Modal.vue'
  import { useTableroStore } from '../../../stores/dashboard'
  import { storeToRefs } from 'pinia'

  const props = defineProps<{
    isOpen: boolean
    infoBancaria: any
    isEdit: boolean
  }>()

  const tableroStore = useTableroStore()
  const { bancos } = storeToRefs(tableroStore)

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const bancoSchema = z.object({
    numero_cuenta: z.string().min(1, 'El número de cuenta es requerido'),
    tipo_cuenta: z.enum(['AHORRO', 'CORRIENTE'], { error: 'Seleccione un tipo de cuenta' }),
    cci: z.string().length(20, 'El CCI debe tener exactamente 20 dígitos').regex(/^\d+$/, 'El CCI solo debe contener números'),
    banco: z.union([z.string(), z.number()]).refine((v) => v !== '' && v !== null, 'Seleccione un banco'),
  })

  const form = ref({
    id: null as number | null,
    numero_cuenta: '',
    tipo_cuenta: 'AHORRO',
    cci: '',
    banco: '' as string | number,
    estado: 1,
  })

  const errores = reactive<Record<string, string>>({})

  function limpiarErrores() {
    Object.keys(errores).forEach((k) => delete errores[k])
  }

  watch(
    () => [props.isOpen, props.infoBancaria],
    async ([isOpen, info]) => {
      if (isOpen) {
        limpiarErrores()
        if (bancos.value.length === 0) {
          await tableroStore.obtenerBancos()
        }

        if (props.isEdit && info) {
          const bancoEncontrado = bancos.value.find((b) => b.nombre.toLowerCase() === info.banco?.toLowerCase())

          form.value = {
            id: info.id,
            numero_cuenta: info.numero_cuenta || '',
            tipo_cuenta: info.tipo_cuenta || 'AHORRO',
            cci: info.cci || '',
            banco: bancoEncontrado ? bancoEncontrado.id : info.banco || '',
            estado: info.estado ?? 1,
          }
        } else {
          form.value = {
            id: null,
            numero_cuenta: '',
            tipo_cuenta: 'AHORRO',
            cci: '',
            banco: '',
            estado: 1,
          }
        }
      }
    }
  )

  const close = () => {
    limpiarErrores()
    emit('close')
  }

  const guardar = () => {
    limpiarErrores()
    const resultado = bancoSchema.safeParse(form.value)
    if (!resultado.success) {
      for (const issue of resultado.error.issues) {
        const campo = issue.path.join('.')
        if (!errores[campo]) {
          errores[campo] = issue.message
        }
      }
      return
    }
    emit('save', form.value)
  }
</script>
