<template>
  <Modal :isOpen="isOpen" title="Registrar Renuncia" @close="close">
    <form @submit.prevent="guardar" class="space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo Documento</label>
          <select
            v-model="form.tipoDocumento"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60 disabled:bg-gray-50 dark:disabled:bg-gray-800"
            :disabled="cargandoDocumentos">
            <option value="" disabled>
              {{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}
            </option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Número</label>
            <input
              type="number"
              v-model="form.numeroDocumento"
              :disabled="esSunat"
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60 disabled:bg-gray-50 dark:disabled:bg-gray-800"
              placeholder="001" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
            <input
              type="number"
              v-model="form.añoDocumento"
              :disabled="esSunat"
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60 disabled:bg-gray-50 dark:disabled:bg-gray-800"
              placeholder="2024" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha de Cese</label>
          <input
            type="date"
            v-model="form.fecha"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"
            required />
        </div>

        <div v-if="!esSunat">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha del Documento</label>
          <input
            type="date"
            v-model="form.fechaValida"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción / Motivo</label>
          <textarea
            v-model="form.descripcion"
            rows="3"
            :disabled="esSunat"
            class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60 disabled:bg-gray-50 dark:disabled:bg-gray-800"
            placeholder="Motivos de la renuncia..."
            required></textarea>
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="guardar"
        class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-red-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-red-700 transition sm:ml-3 sm:w-auto"
        :disabled="isSubmitting">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 animate-spin" />
        Registrar Renuncia
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
  import { ref, watch, toRef, computed } from 'vue'
  import { storeToRefs } from 'pinia'
  import { useTableroStore } from '../../../stores/dashboard'
  import Modal from '../../ui/Modal.vue'
  import { Loader2 } from 'lucide-vue-next'

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
  const isSubmitting = ref(false)
  let yaCargados = false

  const form = ref({
    tipoDocumento: '',
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaValida: null as string | null,
    descripcion: '',
  })

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
    if (abierto) {
      isSubmitting.value = false
      
      form.value = {
        tipoDocumento: '',
        numeroDocumento: null,
        añoDocumento: new Date().getFullYear(),
        fecha: new Date().toISOString().split('T')[0],
        fechaValida: null,
        descripcion: '',
      }

      if (!yaCargados) {
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
    }
  })

  const close = () => {
    if (isSubmitting.value) return
    emit('close')
  }

  const guardar = () => {
    isSubmitting.value = true
    const payload = {
      ...form.value,
      tipoDocumento: form.value.tipoDocumento.toString(),
      numeroDocumento: esSunat.value ? null : form.value.numeroDocumento,
      añoDocumento: esSunat.value ? null : form.value.añoDocumento,
    }
    emit('save', payload)
    
  }
</script>
