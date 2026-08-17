<template>
  <Modal :isOpen="isOpen" title="Cambio Definitivo de Área" @close="cerrar" maxWidth="sm:max-w-xl">
    <div class="space-y-5">

      <div v-if="vinculo" class="rounded-xl border border-indigo-200 bg-indigo-50 dark:border-indigo-800 dark:bg-indigo-900/20 p-4">
        <div class="flex items-center gap-2 mb-3">
          <ArrowRightLeft class="h-4 w-4 text-indigo-500" />
          <p class="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400">Vínculo seleccionado</p>
        </div>
        <div class="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
          <div>
            <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Cargo</p>
            <p class="font-medium text-gray-800 dark:text-gray-200">{{ vinculo.cargo }}</p>
          </div>
          <div>
            <p class="text-2xs font-medium uppercase tracking-wider text-gray-400">Área actual</p>
            <p class="font-medium text-gray-800 dark:text-gray-200">{{ vinculo.area }}</p>
          </div>
        </div>
      </div>

      <div class="flex items-start gap-2 rounded-lg bg-amber-50 border border-amber-200 dark:bg-amber-900/20 dark:border-amber-800 p-3">
        <AlertTriangle class="h-4 w-4 text-amber-500 mt-0.5 shrink-0" />
        <p class="text-xs text-amber-700 dark:text-amber-400">
          El cambio definitivo actualiza el área del vínculo de forma permanente. Si existe una rotación abierta, debe cerrarse primero.
        </p>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nueva área <span class="text-red-500">*</span></label>
        <select
          v-model="formulario.nuevaAreaId"
          :disabled="cargandoAreas"
          class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
          <option :value="null">{{ cargandoAreas ? 'Cargando áreas...' : 'Seleccionar área de destino' }}</option>
          <option v-for="area in areasFiltradas" :key="area.id" :value="area.id">{{ area.nombre }}</option>
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha del cambio <span class="text-red-500">*</span></label>
        <input
          type="date"
          v-model="formulario.fechaCambio"
          required
          class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
      </div>

      <div>
        <p class="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-3">Documento de soporte</p>

        <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo</label>
            <select
              v-model="formulario.tipoDocumento"
              :disabled="cargandoDocumentos"
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-60">
              <option value="" disabled>{{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}</option>
              <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
            </select>
          </div>

          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">N°</label>
              <input
                type="number"
                v-model="formulario.numeroDocumento"
                placeholder="001"
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Año</label>
              <input
                type="number"
                v-model="formulario.anioDocumento"
                placeholder="2025"
                class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha del documento <span class="text-red-500">*</span></label>
            <input
              type="date"
              v-model="formulario.fechaDocumento"
              required
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
          </div>

          <div class="sm:col-span-2">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción <span class="text-red-500">*</span></label>
            <textarea
              v-model="formulario.descripcion"
              rows="3"
              required
              placeholder="Motivo o detalle del cambio definitivo de área..."
              class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
            </textarea>
          </div>
        </div>
      </div>

      <div v-if="errorMensaje" class="rounded-lg border border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-900/20 p-3 flex items-start gap-2">
        <AlertTriangle class="h-4 w-4 text-red-500 mt-0.5 shrink-0" />
        <p class="text-xs text-red-700 dark:text-red-400">{{ errorMensaje }}</p>
      </div>
    </div>

    <template #footer>
      <div class="flex w-full flex-col gap-2 sm:flex-row sm:justify-end">
        <button
          type="button"
          @click="cerrar"
          :disabled="guardando"
          class="inline-flex w-full justify-center rounded-lg bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 transition dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-700 dark:hover:bg-gray-700 sm:w-auto">
          Cancelar
        </button>
        <button
          type="button"
          @click="guardar"
          :disabled="!puedeGuardar || guardando"
          class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 transition sm:w-auto disabled:cursor-not-allowed disabled:opacity-60">
          <Loader2 v-if="guardando" class="h-4 w-4 animate-spin" />
          Registrar cambio
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import { storeToRefs } from 'pinia'
  import { ArrowRightLeft, AlertTriangle, Loader2 } from 'lucide-vue-next'
  import Modal from '../../ui/Modal.vue'
  import api from '../../../services/api'
  import { useTableroStore } from '../../../stores/dashboard'
  import { usePersonalStore } from '../../../stores/personal'
  import type { Area, CambioAreaPayload } from '../../../types'

  const props = defineProps<{
    isOpen: boolean
    vinculo: any | null
  }>()

  const emit = defineEmits<{
    (e: 'close'): void
    (e: 'guardado'): void
  }>()

  const tableroStore = useTableroStore()
  const personalStore = usePersonalStore()
  const { documentos } = storeToRefs(tableroStore)

  const guardando = ref(false)
  const cargandoAreas = ref(false)
  const cargandoDocumentos = ref(false)
  const areas = ref<Area[]>([])
  const errorMensaje = ref('')
  let datosCargados = false

  const formulario = ref({
    nuevaAreaId: null as number | null,
    fechaCambio: new Date().toISOString().split('T')[0]!,
    tipoDocumento: '' as string | number,
    numeroDocumento: null as number | null,
    anioDocumento: new Date().getFullYear() as number | null,
    fechaDocumento: new Date().toISOString().split('T')[0]!,
    descripcion: '',
  })

  const areasFiltradas = computed(() => {
    if (!props.vinculo) return areas.value
    return areas.value.filter((a) => a.id !== props.vinculo.area_id)
  })

  const puedeGuardar = computed(() => {
    if (!formulario.value.nuevaAreaId) return false
    if (!formulario.value.fechaCambio) return false
    if (!formulario.value.fechaDocumento) return false
    if (!formulario.value.descripcion.trim()) return false
    return true
  })

  watch(
    () => props.isOpen,
    async (abierto) => {
      if (!abierto) return
      reiniciarFormulario()

      if (datosCargados) return

      cargandoAreas.value = true
      cargandoDocumentos.value = true
      try {
        const [resAreas] = await Promise.all([api.post('/personal/buscar_areas'), tableroStore.obtenerDocumentos()])
        areas.value = resAreas.data
        datosCargados = true
      } catch (error) {
        console.error('Error cargando datos del modal de cambio de área', error)
      } finally {
        cargandoAreas.value = false
        cargandoDocumentos.value = false
      }
    }
  )

  function reiniciarFormulario() {
    formulario.value = {
      nuevaAreaId: null,
      fechaCambio: new Date().toISOString().split('T')[0]!,
      tipoDocumento: '',
      numeroDocumento: null,
      anioDocumento: new Date().getFullYear(),
      fechaDocumento: new Date().toISOString().split('T')[0]!,
      descripcion: '',
    }
    errorMensaje.value = ''
  }

  function cerrar() {
    if (guardando.value) return
    emit('close')
  }

  async function guardar() {
    if (!puedeGuardar.value || !props.vinculo) return
    guardando.value = true
    errorMensaje.value = ''

    const payload: CambioAreaPayload = {
      vinculo_id: props.vinculo.id,
      nueva_area_id: formulario.value.nuevaAreaId as number,
      fecha_cambio: formulario.value.fechaCambio,
      documento: {
        tipoDocumento: formulario.value.tipoDocumento ? String(formulario.value.tipoDocumento) : null,
        numeroDocumento: formulario.value.numeroDocumento,
        añoDocumento: formulario.value.anioDocumento,
        fecha: formulario.value.fechaDocumento,
        descripcion: formulario.value.descripcion,
      },
    }

    try {
      await personalStore.cambioArea(payload)
      emit('guardado')
    } catch (error: any) {
      const apiError = error?.response?.data?.error
      if (apiError && /rotaci[oó]n/i.test(apiError)) {
        errorMensaje.value = 'Existe una rotación abierta en este vínculo. Ciérrela antes de registrar el cambio definitivo.'
      } else if (apiError) {
        errorMensaje.value = apiError
      } else {
        errorMensaje.value = 'Ocurrió un error al registrar el cambio de área.'
      }
      console.error('Error al registrar cambio de área', error)
    } finally {
      guardando.value = false
    }
  }
</script>
