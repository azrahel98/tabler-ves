<template>
  <Modal :isOpen="isOpen" @close="close">
    <template #header>
      <h3 class="text-title-md font-semibold leading-snug text-black dark:text-white flex items-center gap-2 m-0">
        <ShieldOff class="h-5 w-5 text-amber-500" />
        Desafiliar del Sindicato
      </h3>
    </template>

    <form @submit.prevent="guardar" class="space-y-4">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Registra el documento que respalda la desafiliación de
        <span class="font-semibold text-gray-700 dark:text-gray-200">{{ sindicato }}</span>.
      </p>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo Documento</label>
          <select
            v-model="form.tipoDocumento"
            :disabled="cargandoDocumentos"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
            <option value="" disabled>{{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}</option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Número</label>
            <input
              type="number"
              v-model="form.numeroDocumento"
              placeholder="001"
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
            <input
              type="number"
              v-model="form.añoDocumento"
              placeholder="2024"
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha <span class="text-red-500">*</span></label>
          <input
            type="date"
            v-model="form.fecha"
            required
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha del Documento</label>
          <input
            type="date"
            v-model="form.fechaValida"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>

        <div class="sm:col-span-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción / Motivo <span class="text-red-500">*</span></label>
          <textarea
            v-model="form.descripcion"
            rows="3"
            required
            placeholder="Motivo de la desafiliación..."
            class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"></textarea>
        </div>
      </div>
    </form>

    <template #footer>
      <button
        type="button"
        @click="guardar"
        :disabled="isSubmitting || !puedeGuardar"
        class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-amber-500 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-amber-600 transition disabled:opacity-50 disabled:cursor-not-allowed sm:ml-3 sm:w-auto">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 animate-spin" />
        <ShieldOff v-else class="h-4 w-4" />
        Registrar Desafiliación
      </button>
      <button
        type="button"
        @click="close"
        :disabled="isSubmitting"
        class="mt-3 inline-flex w-full justify-center rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700 sm:mt-0 sm:w-auto">
        Cancelar
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, computed, watch, toRef } from 'vue'
  import { storeToRefs } from 'pinia'
  import { useTableroStore } from '../../../stores/dashboard'
  import Modal from '../../ui/Modal.vue'
  import { Loader2, ShieldOff } from 'lucide-vue-next'

  const props = defineProps<{
    isOpen: boolean
    sindicato?: string
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
    tipoDocumento: '' as string | number,
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaValida: null as string | null,
    descripcion: '',
  })

  const puedeGuardar = computed(() => form.value.fecha && form.value.descripcion.trim())

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
    if (!puedeGuardar.value) return
    isSubmitting.value = true
    emit('save', {
      tipoDocumento: form.value.tipoDocumento ? String(form.value.tipoDocumento) : null,
      numeroDocumento: form.value.numeroDocumento,
      añoDocumento: form.value.añoDocumento,
      fecha: form.value.fecha,
      fechaValida: form.value.fechaValida,
      descripcion: form.value.descripcion,
    })
  }
</script>
