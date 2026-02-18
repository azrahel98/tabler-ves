<template>
  <div>
    <button class="w-full flex items-center gap-3 px-3 text-sm font-medium text-gray-600 hover:rounded-md hover:text-[#5347ce] transition-all group" @click="openModal">
      <div class="p-1.5 rounded-lg bg-gray-100 text-gray-500 group-hover:bg-[#5347ce]/10 group-hover:text-[#5347ce] transition-colors">
        <Building2 class="w-3 h-3" v-if="store.isbanco()" />
        <Edit3Icon class="w-3 h-3" v-else />
      </div>
      <span v-if="edit">Información Bancaria</span>
    </button>

    <Teleport to="body">
      <modal :is-open="isModalOpen" @update:is-open="isModalOpen = $event" :title="isEditing ? 'Editar Información Bancaria' : 'Agregar Información Bancaria'" width="500px">
        <template #body>
          <div class="w-full overflow-hidden p-1">
            <form @submit.prevent="handleSubmit">
              <div class="grid grid-cols-1 gap-y-3">
                <div>
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Entidad Bancaria</label>
                  <div class="relative">
                    <Building2 class="absolute left-3 top-2.5 w-4 h-4 text-[#94a3b8] pointer-events-none" />
                    <select
                      v-model="formData.banco"
                      class="w-full rounded-xl border pl-9 pr-3 py-2 text-sm outline-none transition-all appearance-none cursor-pointer"
                      :class="errors.banco ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                      :disabled="loadingBanks"
                    >
                      <option :value="0" disabled selected>Seleccione un banco</option>
                      <option v-for="bank in banks" :key="bank.id" :value="bank.id.toString()">
                        {{ bank.nombre }}
                      </option>
                    </select>
                    <ChevronDown class="absolute right-3 top-2.5 w-4 h-4 text-gray-400 pointer-events-none" />
                  </div>
                  <span v-if="errors.banco" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.banco }}</span>
                </div>

                <div>
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Tipo de Cuenta</label>
                  <div class="relative">
                    <select
                      v-model="formData.tipo_cuenta"
                      class="w-full rounded-xl border px-3 py-2 text-sm outline-none transition-all appearance-none cursor-pointer"
                      :class="errors.tipo_cuenta ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                    >
                      <option value="" disabled selected>Seleccione el tipo de cuenta</option>
                      <option v-for="type in accountTypes" :key="type" :value="type">
                        {{ type }}
                      </option>
                    </select>
                    <ChevronDown class="absolute right-3 top-2.5 w-4 h-4 text-gray-400 pointer-events-none" />
                  </div>
                  <span v-if="errors.tipo_cuenta" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.tipo_cuenta }}</span>
                </div>

                <!-- Número de Cuenta -->
                <div>
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Número de Cuenta</label>
                  <input
                    v-model="formData.numero_cuenta"
                    type="text"
                    class="w-full rounded-xl border px-3 py-2 text-sm font-mono outline-none transition-all"
                    :class="errors.numero_cuenta ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                    placeholder="000-00000000-0-00"
                  />
                  <span v-if="errors.numero_cuenta" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.numero_cuenta }}</span>
                </div>

                <!-- CCI -->
                <div>
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">CCI (Código de Cuenta Interbancario)</label>
                  <input
                    v-model="formData.cci"
                    type="text"
                    maxlength="20"
                    class="w-full rounded-xl border px-3 py-2 text-sm font-mono outline-none transition-all"
                    :class="errors.cci ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                    placeholder="002-000-000000000000-00"
                  />
                  <div class="flex justify-between items-center mt-0.5 ml-1">
                    <span v-if="errors.cci" class="text-[10px] text-red-500">{{ errors.cci }}</span>
                    <span v-else class="text-[10px] text-gray-400">20 dígitos</span>
                    <span class="text-[10px]" :class="formData.cci.length === 20 ? 'text-green-500 font-bold' : 'text-gray-400'"> {{ formData.cci.length }}/20 </span>
                  </div>
                </div>
              </div>
            </form>
          </div>
        </template>

        <template #footer>
          <div class="flex gap-2 justify-end w-full">
            <button class="px-4 py-2 text-sm font-medium text-[#6b7280] hover:text-[#1a1a1a] transition-colors" @click="isModalOpen = false">Cancelar</button>
            <button
              :disabled="isSubmitting"
              @click="handleSubmit"
              class="inline-flex items-center justify-center px-4 py-2 text-sm font-semibold rounded-xl bg-[#5347ce] text-white hover:bg-[#4338ca] shadow-sm shadow-[#5347ce]/20 disabled:opacity-70 disabled:cursor-not-allowed transition-all"
            >
              <Loader2 v-if="isSubmitting" class="w-4 h-4 mr-2 animate-spin" />
              <span v-if="!isSubmitting">{{ isEditing ? 'Actualizar Información' : 'Guardar Información' }}</span>
              <span v-else>Guardando...</span>
            </button>
          </div>
        </template>
      </modal>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { Loader2, Building2, ChevronDown, Edit3Icon } from 'lucide-vue-next'
import modal from '@comp/ui/modal.vue'
import { z } from 'zod'
import { useProfileStore } from '@store/perfil'
import { api } from '@api/axios'

const store = useProfileStore()

defineProps({
  edit: { type: Boolean, default: true }
})

interface Bank {
  id: number
  nombre: string
}

const banks = ref<Bank[]>([])
const loadingBanks = ref(false)

onMounted(async () => {
  try {
    banks.value = await store.report_bancos()
  } catch (e) {
    console.error('Falied to load banks', e)
    banks.value = []
  }
})

const bankSchema = z.object({
  banco: z.string().min(1, 'Selecciona una entidad bancaria'),
  tipo_cuenta: z.string().min(1, 'Selecciona el tipo de cuenta'),
  numero_cuenta: z.string().min(8, 'El número de cuenta es muy corto').regex(/^\d+$/, 'Solo números'),
  cci: z.string().length(20, 'El CCI debe tener exactamente 20 dígitos').regex(/^\d+$/, 'Solo números')
})

const isModalOpen = ref(false)
const isSubmitting = ref(false)
const errors = ref<Record<string, string>>({})

const formData = reactive({
  banco: 0,
  tipo_cuenta: '',
  numero_cuenta: '',
  cci: '',
  estado: 1,
  id: 0
})

const accountTypes = ['Ahorro', 'Corriente', 'Sueldo', 'CTS']

const isEditing = computed(() => {
  return !!store.banco && !!store.banco.id
})

const loadFormData = () => {
  if (store.banco && store.banco.id) {
    let bancoid = banks.value.find((bank) => bank.nombre === store.banco.banco)
    let tipo = accountTypes.find((type) => type.toLowerCase() === store.banco.tipo_cuenta.toLowerCase())
    formData.banco = bancoid?.id || 0
    formData.tipo_cuenta = tipo || ''
    formData.numero_cuenta = store.banco.numero_cuenta || ''
    formData.cci = store.banco.cci || ''
    formData.id = store.banco.id || 0
  } else {
    formData.banco = 0
    formData.tipo_cuenta = ''
    formData.numero_cuenta = ''
    formData.cci = ''
  }
}

const validateForm = () => {
  errors.value = {}
  const result = bankSchema.safeParse(formData)

  if (!result.success) {
    result.error.issues.forEach((issue) => {
      // @ts-ignore - Valid property access
      errors.value[issue.path[0]] = issue.message
    })
    return false
  }
  return true
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isSubmitting.value = true
  try {
    formData.banco = parseInt(formData.banco.toString())
    const payload: any = {
      dni: store.perfil.dni,
      ...formData
    }

    // Add ID if editing
    if (isEditing.value) {
      payload.id = store.banco.id
    }

    const endpoint = isEditing.value ? '/personal/editar_infobancaria' : '/personal/agregar_infobancaria'

    await api.post(endpoint, payload)

    await store.update_banco(store.perfil.dni)

    isModalOpen.value = false
  } catch (error) {
    console.error('Error saving bank details:', error)
  } finally {
    isSubmitting.value = false
  }
}

const openModal = async () => {
  errors.value = {}
  loadFormData()
  isModalOpen.value = true
}
</script>
