<template>
  <Modal :isOpen="isOpen" title="Registrar Renuncia" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Tipo Documento</label>
          <select
            v-model="form.tipoDocumento"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            :disabled="cargandoDocumentos">
            <option value="" disabled>
              {{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}
            </option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Número</label>
            <input
              type="number"
              v-model="form.numeroDocumento"
              :disabled="esSunat"
              class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white disabled:opacity-50 disabled:cursor-not-allowed"
              placeholder="001" />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Año</label>
            <input
              type="number"
              v-model="form.añoDocumento"
              :disabled="esSunat"
              class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white disabled:opacity-50 disabled:cursor-not-allowed"
              placeholder="2024" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Fecha de Cesse</label>
          <input
            type="date"
            v-model="form.fecha"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            required />
        </div>

        <!-- Fecha Válida -->
        <div v-if="!esSunat">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Fecha del Documento</label>
          <input
            type="date"
            v-model="form.fechaValida"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white" />
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-slate-700 dark:text-slate-300">Descripción / Motivo</label>
          <textarea
            v-model="form.descripcion"
            rows="3"
            :disabled="esSunat"
            class="mt-1 block w-full rounded-md border border-slate-300 px-3 py-2 shadow-sm focus:border-red-500 focus:outline-none focus:ring-1 focus:ring-red-500 dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            placeholder="Motivos de la renuncia..."
            required></textarea>
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
        class="inline-flex w-full justify-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 sm:ml-3 sm:w-auto">
        Registrar Renuncia
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch, toRef, computed } from 'vue'
  import { storeToRefs } from 'pinia'
  import { useTableroStore } from '../../../stores/dashboard'
  import Modal from '../../ui/Modal.vue'

  const props = defineProps<{
    isOpen: boolean
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'save', data: any): void
  }>()

  const tableroStore = useTableroStore()
  const { documentos } = storeToRefs(tableroStore)

  const cargandoDocumentos = ref(false)
  let yaCargados = false

  const esSunat = computed(() => {
    const seleccionado = documentos.value.find((d: any) => d.id == form.value.tipoDocumento)
    return seleccionado?.nombre?.toUpperCase() === 'SUNAT'
  })

  watch(esSunat, (val) => {
    if (val) {
      form.value.numeroDocumento = null
      form.value.fechaValida = null
      form.value.añoDocumento = null as any
      form.value.descripcion = 'REGISTRADO VALIDADO POR TREGISTRO'
    } else {
      form.value.añoDocumento = new Date().getFullYear()
      form.value.descripcion = ''
    }
  })

  watch(toRef(props, 'isOpen'), async (abierto) => {
    if (abierto && !yaCargados) {
      cargandoDocumentos.value = true
      try {
        await tableroStore.obtenerDocumentos()
        yaCargados = true
      } catch (e) {
        console.error('Error al cargar tipos de documento', e)
      } finally {
        cargandoDocumentos.value = false
      }
    }
  })

  const form = ref({
    tipoDocumento: '',
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaValida: null as string | null,
    descripcion: '',
  })

  const close = () => {
    emit('close')
  }

  const guardar = () => {
    const payload = {
      ...form.value,
      tipoDocumento: form.value.tipoDocumento.toString(),
      numeroDocumento: esSunat.value ? null : form.value.numeroDocumento,
      añoDocumento: esSunat.value ? null : form.value.añoDocumento,
    }
    emit('save', payload)
  }
</script>
