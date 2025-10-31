<template>
  <div id="cambio_pass" class="modal fade" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    <div class="modal-dialog modal-sm modal-dialog-centered" @click.stop>
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">Cambiar contraseña</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>

        <form @submit.prevent="handleSubmit">
          <div class="modal-body">
            <div class="mb-3">
              <label class="form-label">Contraseña actual</label>
              <input
                v-model="form.currentPassword"
                type="password"
                class="form-control"
                :class="{ 'is-invalid': errors.currentPassword }"
                placeholder="Ingresa tu contraseña actual"
                required
              />
              <div v-if="errors.currentPassword" class="invalid-feedback">
                {{ errors.currentPassword }}
              </div>
            </div>

            <div class="mb-3">
              <label class="form-label">Nueva contraseña</label>
              <input
                v-model="form.newPassword"
                type="password"
                class="form-control"
                :class="{ 'is-invalid': errors.newPassword }"
                placeholder="Ingresa tu nueva contraseña"
                required
              />
              <div v-if="errors.newPassword" class="invalid-feedback">
                {{ errors.newPassword }}
              </div>
              <div class="form-hint">La contraseña debe tener al menos 8 caracteres.</div>
            </div>

            <div class="mb-3">
              <label class="form-label">Confirmar nueva contraseña</label>
              <input
                v-model="form.confirmPassword"
                type="password"
                class="form-control"
                :class="{ 'is-invalid': errors.confirmPassword }"
                placeholder="Confirma tu nueva contraseña"
                required
              />
              <div v-if="errors.confirmPassword" class="invalid-feedback">
                {{ errors.confirmPassword }}
              </div>
            </div>
          </div>

          <div class="modal-footer">
            <button type="button" class="btn me-auto" data-bs-dismiss="modal" aria-label="Close">Cancelar</button>
            <button type="submit" class="btn btn-primary" :disabled="isLoading">
              <span v-if="isLoading" class="spinner-border spinner-border-sm me-2"></span>
              {{ isLoading ? 'Cambiando...' : 'Cambiar contraseña' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { userStore } from '@store/user'
import { ref, reactive } from 'vue'

const store = userStore()

interface PasswordForm {
  currentPassword: string
  newPassword: string
  confirmPassword: string
}

interface FormErrors {
  currentPassword?: string
  newPassword?: string
  confirmPassword?: string
}

const form = reactive<PasswordForm>({
  currentPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const errors = reactive<FormErrors>({})
const isLoading = ref(false)

const validateForm = (): boolean => {
  Object.keys(errors).forEach((key) => {
    delete errors[key as keyof FormErrors]
  })

  let isValid = true

  if (!form.currentPassword.trim()) {
    errors.currentPassword = 'La contraseña actual es requerida'
    isValid = false
  }

  if (!form.newPassword.trim()) {
    errors.newPassword = 'La nueva contraseña es requerida'
    isValid = false
  } else if (form.newPassword.length < 8) {
    errors.newPassword = 'La contraseña debe tener al menos 8 caracteres'
    isValid = false
  }

  if (!form.confirmPassword.trim()) {
    errors.confirmPassword = 'Debes confirmar la nueva contraseña'
    isValid = false
  } else if (form.newPassword !== form.confirmPassword) {
    errors.confirmPassword = 'Las contraseñas no coinciden'
    isValid = false
  }

  if (form.currentPassword === form.newPassword) {
    errors.newPassword = 'La nueva contraseña debe ser diferente a la actual'
    isValid = false
  }

  return isValid
}

const handleSubmit = async () => {
  if (!validateForm()) return

  isLoading.value = true

  try {
    await api.post('/login/changepass', {
      newpass: form.newPassword,
      oldpass: 'ad',
      id: store.id
    })

    router.go(0)
  } catch (error) {
    console.error('Error changing password:', error)
    errors.currentPassword = 'La contraseña es incorrecta'
  } finally {
    isLoading.value = false
  }
}
</script>
