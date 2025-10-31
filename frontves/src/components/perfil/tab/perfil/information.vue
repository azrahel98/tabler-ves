<template>
  <div class="card shadow-sm">
    <div class="card-header bg-white border-0 pb-0">
      <div class="d-flex align-items-center justify-content-between w-100">
        <div class="d-flex align-items-center">
          <IconUserEdit class="me-2 text-primary" :size="24" />
          <div>
            <h3 class="card-title mb-0 h5">Información Básica</h3>
            <p class="text-muted mb-0 small">Actualiza tu información personal</p>
          </div>
        </div>
        <div v-if="!store.isUser" class="form-check form-switch">
          <input id="editModeSwitch" v-model="isEditMode" class="form-check-input" type="checkbox" :disabled="isLoading" role="switch" />
          <label class="form-check-label text-muted" for="editModeSwitch">
            {{ isEditMode ? 'Modo edición' : 'Solo lectura' }}
          </label>
        </div>
      </div>
    </div>

    <div class="card-body">
      <!-- Alert de Error General -->
      <div v-if="generalError" class="alert alert-danger alert-dismissible fade show" role="alert">
        <div class="d-flex align-items-start">
          <IconAlertCircle class="me-2 flex-shrink-0" :size="20" />
          <div class="flex-grow-1">{{ generalError }}</div>
        </div>
        <button type="button" class="btn-close" @click="clearGeneralError" aria-label="Cerrar"></button>
      </div>

      <!-- Formulario -->
      <form @submit.prevent="handleSubmit" novalidate>
        <div class="row g-3">
          <div class="col-12">
            <label for="direccion" class="form-label mb-1" :class="{ 'fw-semibold': isEditMode }">
              <MapPin class="me-1" :size="16" :class="isEditMode ? 'text-danger' : 'text-primary'" />
              Dirección
              <span v-if="isEditMode" class="text-danger">*</span>
            </label>
            <div v-if="isEditMode" class="input-group">
              <span class="input-group-text bg-light">
                <MapPin :size="18" />
              </span>
              <input
                id="direccion"
                v-model.trim="formData.direccion"
                type="text"
                placeholder="Ingresa tu dirección completa"
                class="form-control"
                :class="{ 'is-invalid': fieldErrors.direccion }"
                :disabled="isLoading"
                :aria-describedby="fieldErrors.direccion ? 'direccion-error' : undefined"
                @blur="validateSingleField('direccion')"
              />
              <div v-if="fieldErrors.direccion" id="direccion-error" class="invalid-feedback">
                {{ fieldErrors.direccion }}
              </div>
            </div>
            <div v-else class="mt-0">
              <span class="text-body">{{ formData.direccion || '—' }}</span>
            </div>
          </div>

          <div class="col-md-6">
            <label for="telf" class="form-label mb-1" :class="{ 'fw-semibold': isEditMode }">
              <Phone class="me-1" :size="16" :class="isEditMode ? 'text-danger' : 'text-primary'" />
              Teléfono
              <span v-if="isEditMode" class="text-danger">*</span>
            </label>
            <div v-if="isEditMode">
              <div class="input-group">
                <span class="input-group-text bg-light">
                  <Phone :size="18" />
                </span>
                <input
                  id="telf"
                  v-model="formData.telf"
                  type="tel"
                  placeholder="999 999 999"
                  class="form-control"
                  :class="{ 'is-invalid': fieldErrors.telf }"
                  :disabled="isLoading"
                  maxlength="9"
                  inputmode="numeric"
                  pattern="[0-9]*"
                  :aria-describedby="fieldErrors.telf ? 'telf-error' : 'telf-help'"
                  @input="handlePhoneInput"
                  @blur="validateSingleField('telf')"
                />
                <div v-if="fieldErrors.telf" id="telf-error" class="invalid-feedback">
                  {{ fieldErrors.telf }}
                </div>
              </div>
              <small id="telf-help" class="form-text text-muted"> Formato: 9 dígitos sin espacios </small>
            </div>
            <div v-else class="mt-1">
              <span class="text-body">{{ formData.telf || '—' }}</span>
            </div>
          </div>

          <div class="col-md-6">
            <label for="email" class="form-label mb-1" :class="{ 'fw-semibold': isEditMode }">
              <Mail class="me-1" :size="16" :class="isEditMode ? 'text-danger' : 'text-primary'" />
              Correo Electrónico
              <span v-if="isEditMode" class="text-danger">*</span>
            </label>
            <div v-if="isEditMode" class="input-group">
              <span class="input-group-text bg-light">
                <Mail :size="18" />
              </span>
              <input
                id="email"
                v-model.trim="formData.email"
                type="email"
                placeholder="ejemplo@correo.com"
                class="form-control"
                :class="{ 'is-invalid': fieldErrors.email }"
                :disabled="isLoading"
                autocomplete="email"
                :aria-describedby="fieldErrors.email ? 'email-error' : undefined"
                @blur="validateSingleField('email')"
              />
              <div v-if="fieldErrors.email" id="email-error" class="invalid-feedback">
                {{ fieldErrors.email }}
              </div>
            </div>
            <div v-else class="mt-1">
              <span class="text-body">{{ formData.email || '—' }}</span>
            </div>
          </div>

          <div class="col-md-6">
            <label for="ruc" class="form-label mb-0" :class="{ 'fw-semibold': isEditMode }">
              <FileText class="me-1 text-muted" :size="16" />
              RUC
              <span v-if="isEditMode" class="text-muted small">(Opcional)</span>
            </label>
            <div v-if="isEditMode">
              <div class="input-group">
                <span class="input-group-text bg-light">
                  <FileText :size="18" />
                </span>
                <input
                  id="ruc"
                  v-model="formData.ruc"
                  type="text"
                  placeholder="20123456789"
                  class="form-control"
                  :class="{ 'is-invalid': fieldErrors.ruc }"
                  :disabled="isLoading"
                  maxlength="11"
                  inputmode="numeric"
                  pattern="[0-9]*"
                  :aria-describedby="fieldErrors.ruc ? 'ruc-error' : 'ruc-help'"
                  @input="handleRucInput"
                  @blur="validateSingleField('ruc')"
                />
                <div v-if="fieldErrors.ruc" id="ruc-error" class="invalid-feedback">
                  {{ fieldErrors.ruc }}
                </div>
              </div>
              <small id="ruc-help" class="form-text text-muted"> RUC de 11 dígitos (opcional) </small>
            </div>
            <div v-else class="mt-1">
              <span class="text-body">{{ formData.ruc || '—' }}</span>
            </div>
          </div>
        </div>

        <div v-if="isEditMode && !store.isUser" class="mt-4 pt-3 border-top">
          <div class="d-flex justify-content-between align-items-center flex-wrap gap-3">
            <small class="text-muted d-flex align-items-center">
              <IconAlertCircle class="me-1" :size="16" />
              Los campos marcados con * son obligatorios
            </small>

            <div class="d-flex gap-2">
              <button type="button" class="btn btn-outline-secondary" @click="handleCancel" :disabled="isLoading">
                <IconX class="me-1" :size="18" />
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary" :disabled="isLoading || !isFormValid">
                <span v-if="isLoading" class="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                <IconCheck v-else class="me-1" :size="18" />
                {{ isLoading ? 'Guardando...' : 'Guardar Cambios' }}
              </button>
            </div>
          </div>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import { FileText, Mail, MapPin, Phone } from 'lucide-vue-next'
import { IconAlertCircle, IconCheck, IconUserEdit, IconX } from '@tabler/icons-vue'
import { z } from 'zod'
import { userStore } from '../../../../store/user'
import { useProfileStore } from '@store/perfil'
import { api } from '@api/axios'
import { router } from '@router/router'

interface PerfilData {
  direccion: string
  telf: string
  email: string
  ruc?: string | null
}

// const props = defineProps<{
//   perfil: PerfilData
// }>()

const props = defineProps({
  perfil: { type: Object, required: true }
})

const store = userStore()
const profileStore = useProfileStore()

const isEditMode = ref(false)
const isLoading = ref(false)
const generalError = ref('')

const validationSchema = z.object({
  telf: z.string({ required_error: 'El teléfono es requerido' }).regex(/^\d{9}$/, 'El teléfono debe tener exactamente 9 dígitos'),
  direccion: z
    .string({ required_error: 'La dirección es requerida' })
    .min(5, 'La dirección debe tener al menos 5 caracteres')
    .max(200, 'La dirección no puede exceder 200 caracteres'),
  email: z.string({ required_error: 'El correo electrónico es requerido' }).email('Ingresa un correo electrónico válido').max(100, 'El correo no puede exceder 100 caracteres'),
  ruc: z
    .string()
    .regex(/^\d{11}$/, 'El RUC debe tener exactamente 11 dígitos')
    .optional()
    .or(z.literal(''))
})

type ValidationSchema = z.infer<typeof validationSchema>
type FieldName = keyof ValidationSchema

const formData = reactive<PerfilData>({
  direccion: props.perfil.direccion,
  telf: props.perfil.telf,
  email: props.perfil.email,
  ruc: props.perfil.ruc || ''
})

const fieldErrors = reactive<Partial<Record<FieldName, string>>>({})

const isFormValid = computed(() => {
  const result = validationSchema.safeParse(formData)
  return result.success
})

watch(
  () => props.perfil,
  (newPerfil) => {
    if (!isEditMode.value) {
      Object.assign(formData, {
        direccion: newPerfil.direccion,
        telf: newPerfil.telf,
        email: newPerfil.email,
        ruc: newPerfil.ruc || ''
      })
    }
  },
  { deep: true }
)

const clearGeneralError = () => {
  generalError.value = ''
}

const validateSingleField = (fieldName: FieldName) => {
  try {
    const fieldSchema = validationSchema.shape[fieldName]
    fieldSchema.parse(formData[fieldName])
    delete fieldErrors[fieldName]

    if (Object.keys(fieldErrors).length === 0) {
      clearGeneralError()
    }
  } catch (error) {
    if (error instanceof z.ZodError) {
      fieldErrors[fieldName] = error.errors[0]?.message || 'Error de validación'
      generalError.value = 'Corrige los errores en el formulario'
    }
  }
}

const handlePhoneInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const cleaned = target.value.replace(/\D/g, '').slice(0, 9)
  formData.telf = cleaned
}

const handleRucInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const cleaned = target.value.replace(/\D/g, '').slice(0, 11)
  formData.ruc = cleaned
}

const handleCancel = () => {
  Object.assign(formData, {
    direccion: props.perfil.direccion,
    telf: props.perfil.telf,
    email: props.perfil.email,
    ruc: props.perfil.ruc || ''
  })

  Object.keys(fieldErrors).forEach((key) => delete fieldErrors[key as FieldName])
  clearGeneralError()

  isEditMode.value = false
}

const handleSubmit = async () => {
  try {
    isLoading.value = true
    Object.keys(fieldErrors).forEach((key) => delete fieldErrors[key as FieldName])
    clearGeneralError()

    const validationResult = validationSchema.safeParse(formData)

    if (!validationResult.success) {
      validationResult.error.errors.forEach((error) => {
        const fieldName = error.path[0] as FieldName
        fieldErrors[fieldName] = error.message
      })
      generalError.value = 'Por favor, corrige los errores en el formulario'
      return
    }

    const dataToSend = {
      dni: router.currentRoute.value.params.dni,
      telf: formData.telf,
      direccion: formData.direccion,
      email: formData.email,
      ruc: formData.ruc || null,
      nacimiento: '2025-01-01'
    }

    await api.post('/personal/editar_por_dni', dataToSend)

    isEditMode.value = false

    await profileStore.update_perfil(router.currentRoute.value.params.dni as string)
  } catch (error: any) {
    console.error('Error al guardar:', error)
    generalError.value = error.response?.data?.message || 'Error al guardar la información. Por favor, inténtalo nuevamente.'
  } finally {
    isLoading.value = false
  }
}
</script>
