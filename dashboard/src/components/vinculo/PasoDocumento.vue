<template>
  <div class="rounded-xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/3">
    <div class="border-b border-gray-200 px-5 py-4 dark:border-gray-800">
      <h4 class="text-sm font-semibold text-gray-800 dark:text-white/90">Documento de Ingreso</h4>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Complete los datos del documento que respalda el ingreso del trabajador</p>
    </div>

    <div class="p-5 space-y-4">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Tipo Documento</label>
          <select
            v-model="formulario.tipoDocumento"
            :disabled="cargandoDocumentos"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90 disabled:opacity-50">
            <option value="" disabled>
              {{ cargandoDocumentos ? 'Cargando...' : 'Seleccionar tipo' }}
            </option>
            <option v-for="doc in documentos" :key="doc.id" :value="doc.id">{{ doc.sigla }} — {{ doc.nombre }}</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Número</label>
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
            v-model="formulario.añoDocumento"
            :placeholder="String(new Date().getFullYear())"
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
      </div>

      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Fecha de Ingreso *</label>
          <input
            type="date"
            v-model="formulario.fecha"
            required
            class="h-11 w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
        </div>
        <div v-if="!store.vacanteSeleccionada?.sueldo">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sueldo *</label>
          <div class="relative">
            <span class="absolute top-1/2 left-3 -translate-y-1/2 text-sm text-gray-500 dark:text-gray-400">S/</span>
            <input
              type="number"
              v-model="formulario.sueldo"
              step="0.01"
              placeholder="0.00"
              required
              class="h-11 w-full rounded-lg border border-gray-300 bg-transparent py-2.5 pr-3 pl-10 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90" />
          </div>
        </div>
        <div v-else>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Sueldo (Asignado)</label>
          <div class="relative">
            <span class="absolute top-1/2 left-3 -translate-y-1/2 text-sm text-gray-400 dark:text-gray-500">S/</span>
            <input
              type="number"
              :value="store.vacanteSeleccionada.sueldo"
              disabled
              class="h-11 w-full rounded-lg border border-gray-200 bg-gray-50 py-2.5 pr-3 pl-10 text-sm text-gray-500 cursor-not-allowed focus:outline-hidden dark:border-gray-800 dark:bg-gray-900/50 dark:text-gray-500" />
          </div>
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Descripción / Motivo</label>
        <textarea
          v-model="formulario.descripcion"
          rows="3"
          placeholder="Descripción del documento de ingreso..."
          class="w-full rounded-lg border border-gray-300 bg-transparent px-3 py-2.5 text-sm text-gray-800 placeholder:text-gray-400 focus:border-brand-300 focus:ring-3 focus:ring-brand-500/10 focus:outline-hidden dark:border-gray-700 dark:bg-gray-900 dark:text-white/90"></textarea>
      </div>

      <div class="flex items-center justify-between pt-4">
        <button
          @click="store.retroceder()"
          class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-white/3">
          <ArrowLeft class="h-4 w-4" />
          Anterior
        </button>
        <button
          @click="confirmar"
          :disabled="!esValido"
          class="inline-flex items-center gap-2 rounded-lg bg-brand-500 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-600 disabled:opacity-50 disabled:cursor-not-allowed">
          Continuar
          <ArrowRight class="h-4 w-4" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { useVinculoStore } from '../../stores/vinculo'
  import { storeToRefs } from 'pinia'
  import { ArrowLeft, ArrowRight } from 'lucide-vue-next'

  const store = useVinculoStore()
  const { formularioDocumento: formulario, documentos } = storeToRefs(store)

  const cargandoDocumentos = ref(false)

  onMounted(async () => {
    if (documentos.value.length === 0) {
      cargandoDocumentos.value = true
      try {
        await store.cargarDocumentos()
      } finally {
        cargandoDocumentos.value = false
      }
    }
  })

  const esValido = computed(() => {
    const f = formulario.value
    return f.fecha && f.sueldo !== null && f.sueldo > 0
  })

  const confirmar = () => {
    if (esValido.value) {
      store.avanzar()
    }
  }
</script>
