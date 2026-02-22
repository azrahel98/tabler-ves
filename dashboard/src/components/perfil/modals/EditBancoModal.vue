<template>
  <Modal :isOpen="isOpen" :title="isEdit ? 'Editar Información Bancaria' : 'Agregar Información Bancaria'" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <!-- Numero de Cuenta -->
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Número de Cuenta</label>
          <input
            type="text"
            v-model="form.numero_cuenta"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="123456789"
            required />
        </div>

        <!-- Tipo de Cuenta -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo de Cuenta</label>
          <select
            v-model="form.tipo_cuenta"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required>
            <option value="AHORRO">AHORRO</option>
            <option value="CORRIENTE">CORRIENTE</option>
          </select>
        </div>

        <!-- Banco -->
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Banco</label>
          <select
            v-model="form.banco"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required>
            <option value="" disabled>Seleccione un banco</option>
            <option v-for="b in bancos" :key="b.id" :value="b.id">{{ b.nombre }}</option>
          </select>
        </div>

        <!-- CCI -->
        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">CCI (Código de Cuenta Interbancaria)</label>
          <input
            type="text"
            v-model="form.cci"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="00212345678901234567" />
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

  const form = ref({
    id: null,
    numero_cuenta: '',
    tipo_cuenta: 'AHORRO',
    cci: '',
    banco: '',
    estado: 1,
  })

  watch(
    () => [props.isOpen, props.infoBancaria],
    async ([isOpen, info]) => {
      if (isOpen) {
        if (bancos.value.length === 0) {
          await tableroStore.obtenerBancos()
        }
        if (props.isEdit && info) {
          form.value = {
            id: info.id,
            numero_cuenta: info.numero_cuenta || '',
            tipo_cuenta: info.tipo_cuenta || 'AHORRO',
            cci: info.cci || '',
            banco: info.banco || '',
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
    emit('close')
  }

  const guardar = () => {
    emit('save', form.value)
  }
</script>
