<template>
  <Modal :isOpen="isOpen" @close="cerrar">
    <template #header>
      <h3 class="text-title-md font-semibold leading-snug text-black dark:text-white flex items-center gap-2 m-0">
        <Link2 class="h-5 w-5 text-primary" />
        Vincular PDF externo
      </h3>
    </template>

    <div class="space-y-4">
      <div class="rounded-lg border border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-900/20 p-3 text-xs text-blue-700 dark:text-blue-300">
        Registra un PDF que vive en una URL pública (portal de transparencia, Drive, etc.) sin subirlo al servidor. El sistema lo tratará igual que un archivo local.
      </div>

      <div>
        <label class="mb-2 block text-sm font-medium text-black dark:text-white">Nombre del archivo <span class="text-red-500">*</span></label>
        <input
          v-model="originalName"
          type="text"
          placeholder="RA-123-2026.pdf"
          class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary" />
        <p class="mt-1 text-2xs text-gray-500">Debe terminar en <code>.pdf</code>.</p>
      </div>

      <div>
        <label class="mb-2 block text-sm font-medium text-black dark:text-white">URL del PDF <span class="text-red-500">*</span></label>
        <input
          v-model="externalUrl"
          type="url"
          placeholder="https://transparencia.munives.gob.pe/docs/RA-123-2026.pdf"
          class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary" />
        <p class="mt-1 text-2xs text-gray-500">Debe empezar con <code>https://</code>.</p>
      </div>

      <div>
        <label class="mb-2 block text-sm font-medium text-black dark:text-white">Asociar a documento (opcional)</label>
        <select
          v-model="documentoSeleccionado"
          class="w-full rounded-lg border-[1.5px] border-stroke bg-transparent px-4 py-2.5 outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:focus:border-primary">
          <option :value="null">Sin asociar</option>
          <option v-for="doc in documentosDisponibles" :key="doc.id" :value="doc.id">
            {{ doc.original_name || doc.tipo_documento || doc.sigla }}
          </option>
        </select>
      </div>

      <p v-if="errorMensaje" class="text-xs text-red-500 font-medium">{{ errorMensaje }}</p>
      <p v-if="mensajeExito" class="text-xs text-green-500 font-medium">{{ mensajeExito }}</p>
    </div>

    <template #footer>
      <div class="flex justify-end gap-3 w-full">
        <button
          @click="cerrar"
          :disabled="cargando"
          class="rounded-lg border border-stroke px-4 py-2 text-sm font-medium text-black hover:bg-gray-100 dark:border-strokedark dark:text-white dark:hover:bg-meta-4 transition-colors">
          Cancelar
        </button>
        <button
          @click="registrar"
          :disabled="cargando"
          class="flex items-center justify-center gap-2 rounded-lg bg-primary px-6 py-2 text-sm font-medium text-white hover:bg-opacity-90 disabled:bg-opacity-50 disabled:cursor-not-allowed transition-all">
          <Loader2 v-if="cargando" class="h-4 w-4 animate-spin" />
          <span>{{ cargando ? 'Registrando...' : 'Registrar URL' }}</span>
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, watch } from 'vue'
  import { Link2, Loader2 } from 'lucide-vue-next'
  import Modal from '../../ui/Modal.vue'
  import api from '../../../services/api'
  import type { RegistrarUrlPayload } from '../../../types'

  const props = defineProps<{
    isOpen: boolean
    dni: string | null
    documentosDisponibles: any[]
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'registrado', archivo: any): void
  }>()

  const originalName = ref('')
  const externalUrl = ref('')
  const documentoSeleccionado = ref<number | null>(null)
  const errorMensaje = ref('')
  const mensajeExito = ref('')
  const cargando = ref(false)

  watch(
    () => props.isOpen,
    (abierto) => {
      if (!abierto) return
      originalName.value = ''
      externalUrl.value = ''
      documentoSeleccionado.value = null
      errorMensaje.value = ''
      mensajeExito.value = ''
    }
  )

  function cerrar() {
    if (cargando.value) return
    emit('close')
  }

  async function registrar() {
    errorMensaje.value = ''
    mensajeExito.value = ''

    if (!props.dni) {
      errorMensaje.value = 'No se encontró el DNI asociado.'
      return
    }

    // Validar URL primero
    const url = externalUrl.value.trim()
    if (!url.toLowerCase().startsWith('https://')) {
      errorMensaje.value = 'La URL debe empezar con https://'
      return
    }

    // Derivar originalName desde la URL si está vacío o no termina en .pdf
    const urlPath = url.split('?')[0]
    const nombreDesdeUrl = urlPath?.split('/').pop() || ''

    if (!originalName.value.trim()) {
      originalName.value = nombreDesdeUrl
    }

    if (!originalName.value.trim().toLowerCase().endsWith('.pdf')) {
      originalName.value = originalName.value.trim().replace(/\.[^.]+$/, '') + '.pdf'
    }

    const payload: RegistrarUrlPayload = {
      dni_asociado: props.dni,
      original_name: originalName.value.trim(),
      external_url: url,
    }

    if (documentoSeleccionado.value) payload.documento_id = documentoSeleccionado.value

    cargando.value = true
    try {
      const response = await api.post('/fileserver/registrar_url', payload)
      mensajeExito.value = 'URL registrada con éxito'
      emit('registrado', response.data)
      setTimeout(() => {
        mensajeExito.value = ''
        emit('close')
      }, 1200)
    } catch (error: any) {
      console.error('Error al registrar URL', error)
      errorMensaje.value = error?.response?.data?.error || 'No se pudo registrar la URL'
    } finally {
      cargando.value = false
    }
  }
</script>
