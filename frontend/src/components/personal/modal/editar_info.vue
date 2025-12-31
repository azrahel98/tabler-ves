<template>
  <div>
    <button
      class="inline-flex items-center justify-center h-9 w-9 rounded-xl border border-[#e5e7eb] text-[#6b7280] transition-colors hover:bg-[#5347ce]/5 hover:text-[#5347ce] hover:border-[#5347ce]/30"
      title="Editar Perfil"
      @click="openModal"
    >
      <Edit class="w-4 h-4" />
    </button>

    <Teleport to="body">
      <modal :is-open="isModalOpen" @update:is-open="isModalOpen = $event" title="Editar Información Personal" width="450px">
        <template #body>
          <div class="w-full overflow-hidden p-1">
            <form @submit.prevent="handleSubmit">
              <div class="flex items-center justify-between mb-5 px-1">
                <div>
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Trabajador</label>
                  <h6 class="text-sm font-bold text-[#1a1a1a] leading-tight">
                    {{ store.perfil.nombre }}
                  </h6>
                </div>
                <div class="flex flex-col items-end">
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">DNI</label>
                  <span class="font-mono text-sm font-semibold text-[#1a1a1a] px-2 py-0.5 rounded border-[#e5e7eb]">
                    {{ store.perfil.dni }}
                  </span>
                </div>
              </div>

              <div class="grid grid-cols-2 gap-x-4 gap-y-3">
                <div class="col-span-1">
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Correo Electrónico</label>
                  <input
                    v-model="formData.email"
                    type="email"
                    class="w-full rounded-xl border px-3 py-2 text-sm outline-none transition-all"
                    :class="errors.email ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                  />
                  <span v-if="errors.email" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.email }}</span>
                </div>

                <div class="col-span-1">
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Teléfono</label>
                  <input
                    v-model="formData.telf"
                    type="tel"
                    class="w-full rounded-xl border px-3 py-2 text-sm outline-none transition-all"
                    :class="errors.telf ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                  />
                  <span v-if="errors.telf" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.telf }}</span>
                </div>

                <div class="col-span-2">
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Dirección Domiciliaria</label>
                  <div class="relative">
                    <MapPinIcon class="absolute left-3 top-2.5 w-4 h-4 text-[#94a3b8]" />
                    <input
                      v-model="formData.direccion"
                      type="text"
                      class="w-full rounded-xl border pl-9 pr-3 py-2 text-sm outline-none transition-all"
                      :class="errors.direccion ? 'border-red-500 bg-red-50' : 'border-[#e5e7eb] bg-white focus:border-[#5347ce] focus:ring-2 focus:ring-[#5347ce]/10'"
                    />
                  </div>
                  <span v-if="errors.direccion" class="text-[10px] text-red-500 mt-0.5 ml-1">{{ errors.direccion }}</span>
                </div>

                <div class="col-span-1">
                  <label class="block text-xs font-medium text-[#6b7280] mb-1 ml-0.5">Fecha Nacimiento</label>
                  <input
                    v-model="formData.nacimiento"
                    type="date"
                    disabled
                    class="w-full rounded-xl border border-transparent bg-[#f5f7fa] px-3 py-2 text-sm text-[#6b7280] cursor-not-allowed"
                  />
                </div>
              </div>

              <div class="mt-5 pt-4 border-t border-dashed border-[#e5e7eb]">
                <div class="flex items-center gap-2 mb-3">
                  <ShieldAlertIcon class="w-3.5 h-3.5 text-[#ef4444]" />
                  <h6 class="text-xs font-bold text-[#1a1a1a] uppercase tracking-wider">Contacto de Emergencia</h6>
                </div>

                <div class="rounded-xl p-3 border border-[#e5e7eb]/60">
                  <div class="grid grid-cols-2 gap-3">
                    <div class="col-span-6 sm:col-span-3">
                      <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Nombre</label>
                      <input
                        v-model="formData.emergencia.nombre"
                        type="text"
                        class="w-full rounded-lg border px-2.5 py-1.5 text-sm outline-none transition-all"
                        :class="errors['emergencia.nombre'] ? 'border-red-500' : 'border-[#e5e7eb] focus:border-[#5347ce]'"
                      />
                    </div>

                    <div class="col-span-3 sm:col-span-2">
                      <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Relación</label>
                      <input
                        v-model="formData.emergencia.relacion"
                        type="text"
                        class="w-full rounded-lg border px-2.5 py-1.5 text-sm outline-none transition-all"
                        :class="errors['emergencia.relacion'] ? 'border-red-500' : 'border-[#e5e7eb] focus:border-[#5347ce]'"
                      />
                    </div>

                    <div class="col-span-3 sm:col-span-1">
                      <label class="block text-[10px] font-bold text-[#94a3b8] mb-0.5">Teléfono</label>
                      <input
                        v-model="formData.emergencia.telefono"
                        type="tel"
                        class="w-full rounded-lg border px-2.5 py-1.5 text-sm outline-none transition-all"
                        :class="errors['emergencia.telefono'] ? 'border-red-500' : 'border-[#e5e7eb] focus:border-[#5347ce]'"
                      />
                    </div>
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
              Guardar Cambios
            </button>
          </div>
        </template>
      </modal>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

import { Loader2, MapPin as MapPinIcon, ShieldAlert as ShieldAlertIcon, Edit } from 'lucide-vue-next'

import modal from '@comp/ui/modal.vue'

import { z } from 'zod'

import { useProfileStore } from '@store/perfil'

import { api } from '@api/axios'

const store = useProfileStore()

const profileSchema = z.object({
  email: z.string().email('Ingresa un correo electrónico válido').optional(),

  telf: z.string().min(9, 'El teléfono debe tener al menos 9 dígitos').regex(/^\d+$/, 'Solo números').optional(),

  direccion: z.string().min(5, 'La dirección es muy corta').optional(),

  emergencia: z

    .object({
      nombre: z.string().min(2, 'Nombre de contacto requerido'),

      relacion: z.string().min(2, 'Parentesco requerido'),

      telefono: z.string().min(9, 'Teléfono de contacto requerido')
    })

    .optional()
})

const isModalOpen = ref(false)

const isSubmitting = ref(false)

const errors = ref<Record<string, string>>({})

const formData = reactive({
  email: '',

  telf: '',

  direccion: '',

  nacimiento: '',

  emergencia: {
    nombre: '',

    relacion: '',

    telefono: ''
  }
})

const loadFormData = () => {
  const perfilData = store.perfil

  const contactoData = store.contacto

  if (perfilData) {
    formData.email = perfilData.correo || perfilData.email || ''

    formData.telf = perfilData.celular || perfilData.telf || ''

    formData.direccion = perfilData.direccion || ''

    formData.nacimiento = perfilData.fecha_nacimiento || perfilData.nacimiento || ''
  }

  formData.emergencia = { nombre: '', relacion: '', telefono: '' }

  if (contactoData) {
    formData.emergencia.nombre = contactoData.nombre || contactoData.nombre_contacto || ''

    formData.emergencia.relacion = contactoData.parentesco || contactoData.relacion || ''

    formData.emergencia.telefono = contactoData.celular || contactoData.telefono || ''
  }
}

const validateForm = () => {
  errors.value = {}

  const dataToValidate = { ...formData }

  const isEmergenciaEmpty = !dataToValidate.emergencia.nombre && !dataToValidate.emergencia.relacion && !dataToValidate.emergencia.telefono

  if (isEmergenciaEmpty) {
    // @ts-ignore

    dataToValidate.emergencia = undefined
  }

  if (!dataToValidate.email) {
    // @ts-ignore

    dataToValidate.email = undefined
  }

  if (!dataToValidate.telf) {
    // @ts-ignore

    dataToValidate.telf = undefined
  }

  if (!dataToValidate.direccion) {
    // @ts-ignore

    dataToValidate.direccion = undefined
  }

  const result = profileSchema.safeParse(dataToValidate)

  if (!result.success) {
    result.error.issues.forEach((issue) => {
      const key = issue.path.join('.')

      errors.value[key] = issue.message
    })

    return false
  }

  return true
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isSubmitting.value = true

  try {
    const payload: any = {
      dni: store.perfil.dni,

      email: formData.email,

      telf: formData.telf,

      direccion: formData.direccion,

      nacimiento: formData.nacimiento
    }

    if (formData.emergencia.nombre) {
      payload.emergencia = {
        persona_dni: store.perfil.dni,

        nombre: formData.emergencia.nombre,

        relacion: formData.emergencia.relacion,

        telefono: formData.emergencia.telefono
      }

      if (formData.emergencia.nombre != store.contacto.nombre || formData.emergencia.relacion != store.contacto.relacion || formData.emergencia.telefono != store.contacto.telefono) {
        await api.post('personal/agregar_contacto', payload.emergencia)

        await store.update_contacto(store.perfil.dni)
      }
    }

    if (formData.email != store.perfil.email || formData.direccion != store.perfil.direccion || formData.telf != store.perfil.telf) {
      await api.post('personal/editar_por_dni', payload)

      await store.update_perfil(store.perfil.dni)
    }

    isModalOpen.value = false
  } catch (error) {
    console.error(error)
  } finally {
    isSubmitting.value = false
  }
}

const openModal = () => {
  errors.value = {}

  loadFormData()

  isModalOpen.value = true
}
</script>
