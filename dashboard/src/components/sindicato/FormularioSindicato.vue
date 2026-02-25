<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Datos de la Afiliación</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Complete los datos del documento de afiliación sindical</p>
    </div>

    <form @submit.prevent="enviar" class="p-5 space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sindicato</label>
        <select
          v-model="form.sindicato"
          required
          class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90">
          <option value="" disabled>Seleccionar sindicato</option>
          <option v-for="s in sindicatos" :key="s.id" :value="s.id">{{ s.nombre }}</option>
        </select>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo Documento</label>
          <select
            v-model="form.tipoDocumento"
            :disabled="cargandoDocumentos"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-50">
            <option value="" disabled>
              {{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar' }}
            </option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>
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
            :placeholder="String(new Date().getFullYear())"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha</label>
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
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción / Motivo</label>
        <textarea
          v-model="form.descripcion"
          rows="3"
          required
          placeholder="Motivo de la afiliación..."
          class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"></textarea>
      </div>

      <div class="flex items-center justify-end gap-3 pt-2">
        <button
          type="button"
          @click="limpiarTodo"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          Limpiar
        </button>
        <button
          type="submit"
          :disabled="enviando || !puedeEnviar"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          <Loading v-if="enviando" size="xs" />
          <Shield v-else class="h-4 w-4" />
          Registrar Afiliación
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { useSindicatoStore } from '../../stores/sindicato'
  import { storeToRefs } from 'pinia'
  import { Shield } from 'lucide-vue-next'
  import Loading from '../ui/Loading.vue'

  const emit = defineEmits<{
    (e: 'enviado'): void
    (e: 'error', msg: string): void
  }>()

  const store = useSindicatoStore()
  const { documentos, enviando, trabajadoresAgregados } = storeToRefs(store)

  const cargandoDocumentos = ref(false)

  const sindicatos = [
    { id: 1, nombre: 'SOMUVES' },
    { id: 2, nombre: 'SUTRAMUVES' },
  ]

  const form = ref({
    sindicato: '' as number | string,
    tipoDocumento: '' as number | string,
    numeroDocumento: null as number | null,
    añoDocumento: new Date().getFullYear() as number | null,
    fecha: new Date().toISOString().split('T')[0],
    fechaValida: null as string | null,
    descripcion: '',
  })

  const puedeEnviar = computed(() => {
    const tieneVinculos = trabajadoresAgregados.value.some((t) => t.vinculos.some((v: any) => v.seleccionado))
    return form.value.sindicato && form.value.fecha && form.value.descripcion && tieneVinculos
  })

  const enviar = async () => {
    if (!puedeEnviar.value) return
    try {
      await store.enviarAfiliacion({
        sindicato: Number(form.value.sindicato),
        tipoDocumento: form.value.tipoDocumento ? String(form.value.tipoDocumento) : null,
        numeroDocumento: form.value.numeroDocumento,
        añoDocumento: form.value.añoDocumento,
        fecha: form.value.fecha,
        fechaValida: form.value.fechaValida,
        descripcion: form.value.descripcion,
      })
      emit('enviado')
      resetForm()
    } catch (err: any) {
      emit('error', err?.response?.data?.error || err.message || 'Error al registrar')
    }
  }

  const resetForm = () => {
    form.value = {
      sindicato: '',
      tipoDocumento: '',
      numeroDocumento: null,
      añoDocumento: new Date().getFullYear(),
      fecha: new Date().toISOString().split('T')[0],
      fechaValida: null,
      descripcion: '',
    }
  }

  const limpiarTodo = () => {
    resetForm()
    store.limpiar()
  }
</script>
