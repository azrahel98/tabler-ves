<template>
  <div class="card">
    <div class="card-header border-0 pb-0">
      <div class="d-flex align-items-center justify-content-between w-100">
        <div class="d-flex align-items-center">
          <IconUserEdit class="icon me-2 text-primary" />
          <div>
            <h3 class="card-title mb-0">Información Básica</h3>
            <p class="text-secondary mb-0 small">Actualiza tu información personal</p>
          </div>
        </div>
        <div class="d-flex gap-2">
          <div v-if="!store.isUser" class="form-check form-switch">
            <input class="form-check-input" type="checkbox" v-model="editMode" :disabled="loading" />
            <label class="form-check-label text-secondary">
              {{ editMode ? 'Modo edición' : 'Solo lectura' }}
            </label>
          </div>
        </div>
      </div>
    </div>

    <div class="card-body">
      <div v-if="generalError" class="alert alert-danger alert-dismissible fade show" role="alert">
        <div class="d-flex align-items-center">
          <IconAlertCircle class="icon alert-icon me-2" />

          <div>{{ generalError }}</div>
        </div>
        <button type="button" class="btn-close" @click="generalError = ''" aria-label="Close"></button>
      </div>

      <form @submit.prevent="guardar(perfil)">
        <div class="row g-3">
          <div class="col-md-6">
            <label class="form-label required">
              <MapPin class="icon icon-sm me-1 text-red" />
              Dirección
            </label>
            <div class="input-group">
              <span class="input-group-text">
                <MapPin class="icon icon-sm" />
              </span>
              <input
                type="text"
                v-model="perfil.direccion"
                placeholder="Ingresa tu dirección completa"
                class="form-control"
                :class="{ 'is-invalid': errors?.direccion }"
                :disabled="!editMode || loading"
                @blur="validateField('direccion')"
              />
              <div v-if="errors?.direccion" class="invalid-feedback">
                <div v-for="error in errors.direccion._errors" :key="error">
                  {{ error }}
                </div>
              </div>
            </div>
          </div>

          <div class="col-md-6">
            <label class="form-label required">
              <Phone class="icon icon-sm me-1 text-red" />
              Teléfono
            </label>
            <div class="input-group">
              <span class="input-group-text">
                <Phone class="icon icon-sm" />
              </span>
              <input
                type="tel"
                v-model="perfil.telf"
                placeholder="999 999 999"
                class="form-control"
                :class="{ 'is-invalid': errors?.telf }"
                :disabled="!editMode || loading"
                maxlength="9"
                @input="formatPhone"
                @blur="validateField('telf')"
              />
              <div v-if="errors?.telf" class="invalid-feedback">
                <div v-for="error in errors.telf._errors" :key="error">
                  {{ error }}
                </div>
              </div>
            </div>
            <div class="small text-secondary" v-if="editMode">Formato: 9 dígitos sin espacios</div>
          </div>

          <div class="col-md-6">
            <label class="form-label required">
              <Mail class="icon icon-sm me-1 text-red" />
              Correo Electrónico
            </label>
            <div class="input-group">
              <span class="input-group-text">
                <Mail class="icon icon-sm" />
              </span>
              <input
                type="email"
                v-model="perfil.email"
                placeholder="ejemplo@correo.com"
                class="form-control"
                :class="{ 'is-invalid': errors?.email }"
                :disabled="!editMode || loading"
                @blur="validateField('email')"
              />
              <div v-if="errors?.email" class="invalid-feedback">
                <div v-for="error in errors.email._errors" :key="error">
                  {{ error }}
                </div>
              </div>
            </div>
          </div>

          <div class="col-md-6">
            <label class="form-label">
              <FileText class="icon icon-sm me-1 text-secondary" />
              RUC
              <span class="text-secondary" v-if="editMode">(Opcional)</span>
            </label>
            <div class="input-group">
              <span class="input-group-text">
                <FileText class="icon icon-sm" />
              </span>
              <input type="text" v-model="perfil.ruc" placeholder="20123456789" class="form-control" :disabled="!editMode || loading" maxlength="11" />
            </div>
            <div class="text-secondary small" v-if="editMode">RUC de 11 dígitos (opcional)</div>
          </div>
        </div>

        <div class="mt-4 d-flex justify-content-between align-items-center" v-if="editMode">
          <div class="text-secondary small">
            <IconAlertCircle class="icon icon-sm" />
            Los campos marcados con * son obligatorios
          </div>

          <div class="d-flex gap-2" v-if="editMode && !store.isUser">
            <button type="button" class="btn btn-secondary fs-5 p-1 px-2" @click="cancelarEdicion" :disabled="loading">
              <IconX class="icon" />
              Cancelar
            </button>
            <button type="submit" class="btn btn-primary fs-5 p-1 px-2" :disabled="loading">
              <span v-if="loading" class="spinner-border spinner-border-sm me-2" role="status"></span>
              <IconCheck class="icon" />
              {{ loading ? 'Guardando...' : 'Guardar Cambios' }}
            </button>
          </div>
        </div>
      </form>
    </div>

    <!-- <div class="card-footer bg-transparent text-secondary">
      <div class="row align-items-center">
        <div class="col">
          <div class="d-flex align-items-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="icon icon-sm me-1"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              stroke-width="2"
              stroke="currentColor"
              fill="none"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path stroke="none" d="m0 0h24v24H0z" fill="none" />
              <path d="M12 8h.01" />
              <path d="M11 12h1v4h1" />
              <path d="M12 3c7.2 0 9 1.8 9 9s-1.8 9 -9 9s-9 -1.8 -9 -9s1.8 -9 9 -9z" />
            </svg>
            <small>Tu información está protegida y encriptada</small>
          </div>
        </div>
        <div class="col-auto"></div>
      </div>
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { FileText, Mail, MapPin, Phone } from 'lucide-vue-next'
import { IconAlertCircle, IconCheck, IconUserEdit, IconX } from '@tabler/icons-vue'
import { userStore } from '../../store/user'
import { api } from '@api/axios'
import { router } from '@router/router'
import { ref } from 'vue'
import { z } from 'zod'

const store = userStore()
const editMode = ref(false)
const loading = ref(false)

const generalError = ref('')

const props = defineProps({
  perfil: {
    type: Object,
    required: true
  }
})

const schema_validate = z.object({
  telf: z
    .union([z.string({ message: 'Se espera un telefono' }), z.number({ message: '' })])
    .refine((val) => /^\d{9}$/.test(String(val)), {
      message: 'El teléfono debe tener exactamente 9 dígitos'
    })
    .transform((val) => String(val)),
  direccion: z.string({ message: 'Se espera una direccion' }).min(5, 'La dirección debe tener al menos 5 caracteres').max(200, 'La dirección no puede exceder 200 caracteres'),
  email: z.string({ message: 'Se espera un correo electronico' }).email('Ingresa un correo electrónico válido').max(100, 'El correo no puede exceder 100 caracteres'),
  ruc: z
    .string()
    .nullable()
    .optional()
    .refine((val) => !val || /^\d{11}$/.test(val), {
      message: 'El RUC debe tener exactamente 11 dígitos'
    })
})

type schema_validateType = z.infer<typeof schema_validate>
type FormattedErrors = Partial<z.ZodFormattedError<schema_validateType>>

const errors = ref<FormattedErrors | null>(null)

const fieldNames = ['telf', 'direccion', 'email', 'ruc'] as const
type FieldName = (typeof fieldNames)[number]

const validateField = (fieldName: FieldName) => {
  try {
    const singleFieldSchema = z.object({
      [fieldName]: schema_validate.shape[fieldName]
    } as Pick<typeof schema_validate.shape, FieldName>)

    singleFieldSchema.parse({ [fieldName]: props.perfil[fieldName] })

    if (errors.value?.[fieldName]) {
      delete errors.value[fieldName]
      if (Object.keys(errors.value).length === 0) {
        errors.value = null
        generalError.value = ''
      }
    }
  } catch (error) {
    if (error instanceof z.ZodError) {
      const formatted = error.format() as z.ZodFormattedError<schema_validateType>

      if (!errors.value) {
        errors.value = {}
      }

      if (formatted[fieldName]) {
        errors.value[fieldName] = formatted[fieldName]
        generalError.value = 'Corrige los errores en el formulario'
      }
    }
  }
}

const emit = defineEmits(['update:perfil'])

const formatPhone = (event: Event) => {
  const target = event.target as HTMLInputElement
  let value = target.value.replace(/\D/g, '').slice(0, 9)
  props.perfil.telf = value
}

const cancelarEdicion = () => {
  editMode.value = false
  errors.value = null
  emit('update:perfil', true)
  generalError.value = ''
}

const guardar = async (user: any) => {
  try {
    loading.value = true
    errors.value = null
    generalError.value = ''

    const valid = schema_validate.safeParse(user)

    if (!valid.success) {
      errors.value = valid.error.format()
      generalError.value = 'Por favor, corrige los errores en el formulario'
      return
    }

    await api.post('/personal/editar_por_dni', {
      dni: router.currentRoute.value.params.dni,
      telf: user.telf,
      direccion: user.direccion,
      email: user.email,
      ruc: user.ruc || null,
      nacimiento: '2025-01-01'
    })

    editMode.value = false
  } catch (error: any) {
    console.error('Error al guardar:', error)
    generalError.value = error.response?.data?.message || 'Error al guardar la información. Inténtalo nuevamente.'
  } finally {
    loading.value = false
    emit('update:perfil', true)
  }
}
</script>
