<template>
  <div>
    <button
      class="inline-flex items-center justify-center h-9 w-9 rounded-xl border border-[#e5e7eb] text-[#6b7280] transition-colors hover:bg-[#5347ce]/5 hover:text-[#5347ce] hover:border-[#5347ce]/30"
      title="Registrar Baja"
      @click="openModal"
    >
      <FileMinus class="w-4 h-4" />
    </button>

    <modal :is-open="isModalOpen" @update:is-open="isModalOpen = $event" title="Registrar Cese Laboral" width="600px">
      <template #body>
        <div class="w-full px-1">
          <form @submit.prevent="handleSubmit(vinculo)">
            <div class="grid grid-cols-12 gap-x-4 gap-y-4">
              <div class="col-span-12">
                <label class="block text-[10px] font-bold text-[#94a3b8] uppercase tracking-widest mb-1">Documento de Sustento</label>
                <div class="relative">
                  <select
                    v-model="form.documentType"
                    class="w-full pl-3 pr-8 py-2 bg-white border rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-[#5347ce]/10 transition-all appearance-none"
                    :class="errors.documentType ? 'border-red-500' : 'border-[#e5e7eb] focus:border-[#5347ce]'"
                    @change="() => (form.documentType == '10' ? (sunat = true) : (sunat = false))"
                  >
                    <option value="" disabled>Seleccionar tipo...</option>
                    <option v-for="type in tipos_docs" :key="type.id" :value="type.id">{{ type.nombre }} - {{ type.siglas }}</option>
                  </select>
                  <ChevronDown class="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-[#94a3b8] pointer-events-none" />
                </div>
                <span v-if="errors.documentType" class="text-[10px] text-red-500 mt-0.5">{{ errors.documentType }}</span>
              </div>

              <template v-if="sunat === false">
                <div class="col-span-12 grid grid-cols-12 gap-3 p-3 bg-[#f8fafc] rounded-xl border border-[#e5e7eb]/60">
                  <div class="col-span-5">
                    <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Número</label>
                    <input
                      type="text"
                      v-model="form.documentNumber"
                      placeholder="Ej. 001-2024"
                      class="w-full px-2.5 py-1.5 bg-white border rounded-lg text-sm focus:outline-none focus:border-[#5347ce] transition-all"
                      :class="errors.documentNumber ? 'border-red-500' : 'border-[#e5e7eb]'"
                    />
                  </div>
                  <div class="col-span-3">
                    <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Año</label>
                    <input
                      type="number"
                      v-model="form.documentYear"
                      placeholder="2024"
                      class="w-full px-2.5 py-1.5 bg-white border rounded-lg text-sm focus:outline-none focus:border-[#5347ce] transition-all"
                      :class="errors.documentYear ? 'border-red-500' : 'border-[#e5e7eb]'"
                    />
                  </div>

                  <div class="col-span-4">
                    <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Emisión</label>
                    <input
                      type="date"
                      v-model="form.documentDate"
                      class="w-full px-2.5 py-1.5 bg-white border rounded-lg text-sm focus:outline-none focus:border-[#5347ce] transition-all text-[#6b7280]"
                      :class="errors.documentDate ? 'border-red-500' : 'border-[#e5e7eb]'"
                    />
                  </div>
                </div>
                <div class="col-span-12 -mt-2" v-if="errors.documentYear || errors.documentNumber || errors.documentDate">
                  <span class="text-[10px] text-red-500">Complete los datos del documento físico.</span>
                </div>
              </template>

              <div class="col-span-12 sm:col-span-6">
                <label class="block text-[10px] font-bold text-[#5347ce] uppercase tracking-widest mb-1">Fecha de Cese</label>
                <div class="relative">
                  <input
                    type="date"
                    v-model="form.effectiveDate"
                    class="w-full px-3 py-2 bg-white border-2 rounded-xl text-sm font-medium text-[#1a1a1a] focus:outline-none transition-all"
                    :class="errors.effectiveDate ? 'border-red-500 bg-red-50' : 'border-[#5347ce]/20 focus:border-[#5347ce] focus:shadow-[0_0_0_3px_rgba(83,71,206,0.1)]'"
                  />
                </div>
                <p class="text-[10px] text-[#6b7280] mt-1 ml-0.5">Último día laborable.</p>
                <span v-if="errors.effectiveDate" class="text-[10px] text-red-500">{{ errors.effectiveDate }}</span>
              </div>

              <div class="col-span-12">
                <label class="block text-[10px] font-bold text-[#94a3b8] uppercase tracking-widest mb-1">Observaciones</label>
                <textarea
                  v-model="form.observation"
                  rows="2"
                  placeholder="Motivo o detalles..."
                  class="w-full px-3 py-2 bg-white border border-[#e5e7eb] rounded-xl text-sm focus:outline-none focus:border-[#5347ce] focus:ring-1 focus:ring-[#5347ce] transition-all resize-none"
                />
              </div>
            </div>
          </form>
        </div>
      </template>

      <template #footer>
        <div class="flex gap-2 justify-end w-full">
          <button class="px-4 py-2 text-sm font-medium text-[#6b7280] hover:text-[#1a1a1a] transition-colors" @click="isModalOpen = false">Cancelar</button>
          <button
            @click="handleSubmit(vinculo)"
            :disabled="isSubmitting"
            class="inline-flex items-center justify-center px-4 py-2 text-sm font-semibold rounded-xl bg-[#ef4444] text-white hover:bg-[#dc2626] shadow-sm shadow-red-500/20 disabled:opacity-70 disabled:cursor-not-allowed transition-all gap-2"
          >
            <Loader2 v-if="isSubmitting" class="w-4 h-4 animate-spin" />
            <span v-else>Confirmar Baja</span>
          </button>
        </div>
      </template>
    </modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { FileMinus, ChevronDown, Loader2 } from 'lucide-vue-next'
import modal from '@comp/ui/modal.vue'
import { api } from '@api/axios'
import { useProfileStore } from '@store/perfil'

const store = useProfileStore()

const isModalOpen = ref(false)
const isSubmitting = ref(false)
const tipos_docs = ref<any[]>([])
const sunat = ref(false)

const form = reactive({
  documentType: '',
  documentYear: new Date().getFullYear().toString(),
  documentNumber: '',
  documentDate: new Date().toISOString().split('T')[0],
  effectiveDate: '',
  observation: ''
})

const errors = ref<Record<string, string>>({})

const openModal = () => {
  isModalOpen.value = true
  fetchDocTypes()
}

const fetchDocTypes = async () => {
  try {
    const { data } = await api.post('/dash/reporte_documentos')
    tipos_docs.value = data
  } catch (error) {
    console.error(error)
  }
}

const validateForm = () => {
  errors.value = {}
  let isValid = true

  if (!form.documentType) {
    errors.value.documentType = 'Requerido'
    isValid = false
  }

  if (!form.documentNumber && !sunat.value) {
    errors.value.documentNumber = 'Requerido'
    isValid = false
  }

  if (!form.documentDate && !sunat.value) {
    errors.value.documentDate = 'Requerido'
    isValid = false
  }

  if (!form.effectiveDate) {
    errors.value.effectiveDate = 'Requerido'
    isValid = false
  }

  return isValid
}

defineProps({
  vinculo: {
    type: Number,
    required: true
  }
})

const handleSubmit = async (id: number) => {
  if (!validateForm()) return

  isSubmitting.value = true

  try {
    await api.post('/personal/renuncia_por_vinculo', {
      tipoDocumento: form.documentType,
      numeroDocumento: sunat.value ? null : parseInt(form.documentNumber),
      añoDocumento: sunat.value ? null : parseInt(form.documentYear),
      fecha: form.effectiveDate,
      fechaValida: form.documentDate,
      descripcion: form.observation,
      id
    })
    isModalOpen.value = false
    store.update_vinculo(store.perfil.dni)
    resetForm()
  } catch (error) {
  } finally {
    isSubmitting.value = false
  }
}

const resetForm = () => {
  form.documentType = ''
  form.documentNumber = ''
  form.effectiveDate = ''
  form.observation = ''
  form.documentYear = new Date().getFullYear().toString()
  form.documentDate = new Date().toISOString().split('T')[0]
  errors.value = {}
}
</script>
